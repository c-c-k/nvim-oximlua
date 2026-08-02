use mlua::{ExternalResult, Lua};

/// Initializes the connection between `nvim-oximlua` and
/// [`mlua::Lua`].
/// This must be called exactly once at the [mlua module entry point].
///
/// [`mlua::Lua`]: ::mlua::Lua
/// [mlua module entry point]: https://github.com/mlua-rs/mlua#module-mode
///
/// # Examples
///
/// ```ignore
/// use nvim_oximlua as nvim;
/// use mlua::prelude::*;
///
/// #[mlua::lua_module]
/// fn plugin_entry_point(lua: &Lua) -> LuaResult<LuaTable> {
///     nvim::init(lua);
///     let plugin_entry_point = lua.create_table()?;
///     
///     ...
///
///     Ok(plugin_entry_point)
/// }
/// ```
pub fn init(lua: &Lua) -> mlua::Result<()> {
    types::arena_init();

    olua::init(lua).into_lua_err()?;

    #[cfg(feature = "libuv")]
    unsafe {
        lua.exec_raw::<()>((), |state| {
            libuv::init(state);
        })?;
    }

    Ok(())
}
