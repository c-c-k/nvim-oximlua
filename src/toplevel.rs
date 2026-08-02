use olua::{self, WrapFn};

/// Binding to [`vim.schedule()`][1].
///
/// Schedules a callback to be invoked soon by the main event-loop. Useful to
/// avoid [`textlock`][2] or other temporary restrictions.
///
/// [1]: https://neovim.io/doc/user/lua.html#vim.schedule()
/// [2]: https://neovim.io/doc/user/eval.html#textlock
pub fn schedule<F>(fun: F)
where
    F: FnOnce(()) + 'static,
{
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/lua/executor.c#L363
    //
    // Unfortunately the `nlua_schedule` C function is not exported, so we have
    // to call the Lua function instead.
    let lua = olua::get_nvim_lua().unwrap();
    let schedule: mlua::Function = lua
        .globals()
        .get("vim.schedule")
        .expect("`vim.schedule` should exist");
    let fun = WrapFn::wrap_once(fun).unwrap();
    schedule.call::<()>(fun).unwrap();
}
