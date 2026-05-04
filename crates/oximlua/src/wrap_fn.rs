use std::{cell::RefCell, marker::PhantomData};

use mlua::{ExternalError, ExternalResult, FromLuaMulti, IntoLuaMulti};

use crate::IntoResult;

pub struct WrapFn<F, A, R> {
    _pd: (PhantomData<F>, PhantomData<A>, PhantomData<R>),
}

impl<F, A, R> WrapFn<F, A, R> {
    pub fn wrap<O>(fun: F) -> mlua::Result<mlua::Function>
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        wrap_fn(fun)
    }

    pub fn wrap_mut<O>(fun: F) -> mlua::Result<mlua::Function>
    where
        F: FnMut(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        wrap_fn_mut(fun)
    }

    pub fn wrap_once<O>(fun: F) -> mlua::Result<mlua::Function>
    where
        F: FnOnce(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        wrap_fn_once(fun)
    }
}

/// Wraps a Rust function or closure in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
///
/// # Examples
///
/// ```ignore
/// use mlua::prelude::*;
/// use nvim_oxi::lua::olua;
///
/// fn to_int(str_num: String) -> anyhow::Result<i32> {
///    Ok(str_num.parse::<i32>()?)
/// }
///
/// fn util_module() -> mlua::Result<mlua::Table> {
///     let lua = olua::get_nvim_lua();
///     let util_module = lua.create_table()?;
///     util_module.set("to_int", olua::wrap_fn(to_int)?)?;
///
///     Ok(util_module)
/// }
/// ```
#[inline]
fn wrap_fn<F, A, R, O>(fun: F) -> mlua::Result<mlua::Function>
where
    F: Fn(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    O::Error: ExternalError + 'static,
{
    let lua = crate::get_nvim_lua().into_lua_err()?;
    lua.create_function(move |_lua, args| {
        fun(args).into_result().into_lua_err()
    })
}

/// Wraps a Rust `<FnMut>` function in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
#[inline]
fn wrap_fn_mut<F, A, R, O>(fun: F) -> mlua::Result<mlua::Function>
where
    F: FnMut(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    O::Error: ExternalError + 'static,
{
    let lua = crate::get_nvim_lua().into_lua_err()?;
    let fun = RefCell::new(fun);

    lua.create_function(move |_lua, args| {
        let fun = &mut *fun
            .try_borrow_mut()
            .map_err(|_| mlua::Error::RecursiveMutCallback)?;

        fun(args).into_result().into_lua_err()
    })
}

/// Wraps a Rust `<FnOnce>` function in a
/// [`mlua::Function`](https://docs.rs/mlua/latest/mlua/struct.Function.html).
#[inline]
fn wrap_fn_once<F, A, R, O>(fun: F) -> mlua::Result<mlua::Function>
where
    F: FnOnce(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    O::Error: ExternalError + 'static,
{
    let lua = crate::get_nvim_lua().into_lua_err()?;
    let fun = RefCell::new(Some(fun));

    lua.create_function(move |_lua, args| {
        let fun = fun
            .try_borrow_mut()
            .map_err(|_| mlua::Error::RecursiveMutCallback)?
            .take()
            .ok_or("Cannot call function twice")
            .into_lua_err()?;

        fun(args).into_result().into_lua_err()
    })
}
