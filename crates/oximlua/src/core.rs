use core::ffi::c_int;
use std::{
    cell::{Cell, OnceCell},
    sync::atomic::{AtomicBool, Ordering},
};

use mlua::{IntoLua, Lua};

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct CachedLuaRegistry(mlua::Table);

static DID_INIT: AtomicBool = AtomicBool::new(false);

thread_local! {
    static IS_NVIM_THREAD: Cell<bool> = const { Cell::new(false) };
    static MAIN_LUA_STATE: OnceCell<*mut mlua::ffi::lua_State> = const { OnceCell::new() };
}

#[inline]
pub fn init(lua: &Lua) -> Result<()> {
    if DID_INIT.load(Ordering::Relaxed) {
        return Err(Error::multiple_init_error());
    };
    DID_INIT.store(true, Ordering::Release);
    IS_NVIM_THREAD.set(true);

    cache_main_lua_state(lua)?;
    cache_registry_ref(lua)?;
    Ok(())
}

fn cache_main_lua_state(lua: &Lua) -> Result<()> {
    MAIN_LUA_STATE
        .try_with(|cell| unsafe {
            lua.exec_raw::<()>((), |state| {
                cell.set(state).expect(
                    "Reinitialization attempt should fail before \
                     `cache_main_lua_state`",
                );
            })
        })
        .expect("`MAIN_LUA_STATE` should not be dropped")?;
    Ok(())
}

fn cache_registry_ref(lua: &Lua) -> Result<()> {
    let lua_registry = lua
        .load("debug.getregistry()")
        .eval()
        .expect("the Lua registry should be available");
    let attempeted_reinitialization = lua
        .try_set_app_data(CachedLuaRegistry(lua_registry))
        .expect(
            "the cached Lua registry should be inaccessible before \
             initialization finishes",
        )
        .is_some();
    if attempeted_reinitialization {
        panic!(
            "Reinitialization attempt should fail before \
             `cache_registry_as_app_data`",
        )
    };
    Ok(())
}

pub fn is_nvim_thread() -> bool {
    IS_NVIM_THREAD.get()
}

macro_rules! initialized {
    () => {
        if !DID_INIT.load(Ordering::Relaxed) {
            return Err(Error::missing_init_error());
        };
    };
}

macro_rules! initialized_on_main_thread {
    ($main_thread_value:literal) => {
        initialized!();
        if !is_nvim_thread() {
            return Err(Error::main_thread_only_error($main_thread_value));
        };
    };
}

/// Returns the main NVIM [`mlua::Lua`] instance.
///
/// [`mlua::Lua`]: ::mlua::Lua
///
/// Will return an [`Error::Access`] if used in a secondary thread.
/// Might cause UB if used in a secondary Lua thread or a Lua coroutine.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
/// use mlua::prelude::*;
///
/// fn hello_oximlua() -> LuaResult<()> {
///     nvim::print!("Hello from nvim-oxi..");
///
///     let lua = nvim::olua::get_nvim_lua()?;
///     let print = lua.globals().get::<_, LuaFunction>("print")?;
///     print.call("..and goodbye from mlua!")?;
///
///     Ok(())
/// }
/// ```
pub fn get_nvim_lua() -> Result<Lua> {
    initialized_on_main_thread!("the main Lua State");
    Ok(MAIN_LUA_STATE
        .try_with(|cell| {
            let state = cell.get().expect("`MAIN_LUA` should be initalized");
            unsafe { mlua::Lua::get_or_init_from_ptr(*state).clone() }
        })
        .expect("`MAIN_LUA_STATE` should not be dropped"))
}

/// Returns the main NVIM [`mlua::Lua`] instance.
///
/// [`mlua::Lua`]: ::mlua::Lua
///
/// # Panics
///
/// Will panic if used in a secondary thread.
///
/// Might cause UB if used in a secondary Lua thread or a Lua coroutine.
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
/// use mlua::prelude::*;
///
/// fn hello_oximlua() -> LuaResult<()> {
///     nvim::print!("Hello from nvim-oxi..");
///
///     let print = lua().globals().get::<_, LuaFunction>("print")?;
///     print.call("..and goodbye from mlua!")?;
///
///     Ok(())
/// }
/// ```
pub fn lua() -> Lua {
    get_nvim_lua().unwrap()
}

/// Get the Lua registry table.
///
/// This function is exposed mostly for debugging,
/// prefer to use the [`mlua::RegistryKey`] interface if you need access to the registry.
///
/// [`mlua::RegistryKey`]: ::mlua::RegistryKey
///
/// # Safety
///
/// This table contains global Lua state shared between NVIM and all plugins
/// modifying values that do not belong to your plugin or modifying the array part of this
/// table directly will likely result in undefined behavior.
///
/// see [Lua Registry](https://www.lua.org/manual/5.1/manual.html#3.5).
#[doc(hidden)]
pub unsafe fn get_registry() -> Result<mlua::Table> {
    initialized_on_main_thread!("The Lua Registry");
    let lua = get_nvim_lua()?;
    Ok(lua
        .try_app_data_ref::<CachedLuaRegistry>()
        .map_err(Error::custom)?
        .expect("Cached Lua registry should be initalized")
        .0
        .clone())
}

/// Returns a [`mlua::Value`] from the NVIM Lua registry by it's `lua_ref`.
///
/// This function is exposed mostly for debugging,
/// prefer to use the [`mlua::RegistryKey`] interface if you need access to the registry.
///
/// [`mlua::Value`]: ::mlua::Value
/// [`mlua::RegistryKey`]: ::mlua::RegistryKey
///
/// see [Lua Registry](https://www.lua.org/manual/5.1/manual.html#3.5).
#[doc(hidden)]
pub fn get_registry_value(lua_ref: c_int) -> Result<mlua::Value> {
    let registry = unsafe { get_registry() }?;
    registry.get(lua_ref).map_err(Error::custom)
}

/// Inserts a [`mlua::Value`] into the NVIM Lua registry and returns it's `lua_ref`.
///
/// This function is exposed mostly for debugging,
/// prefer to use the [`mlua::RegistryKey`] interface if you need access to the registry.
///
/// [`mlua::Value`]: ::mlua::Value
/// [`mlua::RegistryKey`]: ::mlua::RegistryKey
///
/// # Safety
///
/// Values inserted into the registry this way will stay there until they are explicitly removed.
///
/// see [Lua Registry](https://www.lua.org/manual/5.1/manual.html#3.5).
#[doc(hidden)]
pub unsafe fn add_registry_value<T: IntoLua>(value: T) -> Result<c_int> {
    use mlua::ffi;
    let lua = get_nvim_lua()?;
    let registry = unsafe { get_registry() }?;

    let lua_ref = unsafe {
        lua.exec_raw::<i32>((), |state| {
            ffi::lua_pushboolean(state, 0);
            let lua_ref = ffi::luaL_ref(state, ffi::LUA_REGISTRYINDEX);
            ffi::lua_pushinteger(state, lua_ref as i64);
        })
    }?;
    registry.set(lua_ref, value)?;

    Ok(lua_ref)
}

/// Frees a NVIM Lua registry slot by it's `lua_ref`.
///
/// This function is exposed mostly for debugging,
/// prefer to use the [`mlua::RegistryKey`] interface if you need access to the registry.
///
/// [`mlua::RegistryKey`]: ::mlua::RegistryKey
///
/// # Safety
///
/// The registry table contains global Lua state shared between NVIM and all plugins
/// Removing a value that does not belong to your plugin might result in undefined behavior.
///
/// see [Lua Registry](https://www.lua.org/manual/5.1/manual.html#3.5).
#[doc(hidden)]
pub unsafe fn free_registry_lua_ref(lua_ref: c_int) -> Result<()> {
    use mlua::ffi;
    let lua = get_nvim_lua()?;

    unsafe {
        lua.exec_raw::<()>((), move |state| {
            ffi::luaL_unref(state, ffi::LUA_REGISTRYINDEX, lua_ref);
        })
    }?;

    Ok(())
}
