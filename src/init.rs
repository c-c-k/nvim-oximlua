use mlua::{ExternalResult, Lua};

/// Initializes the connection between `nvim-oxi` and
/// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html).
/// This must be called exactly once at the
/// [mlua module entry point](https://github.com/mlua-rs/mlua#module-mode).
///
/// # Examples
///
/// ```ignore
/// use nvim_oxi as nvim;
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
    oximlua::init(lua).into_lua_err()
}
