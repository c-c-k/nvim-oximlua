#[allow(unused)]
use std::fmt;
use std::{cell::Cell, marker::PhantomData};

use mlua::{ExternalError, FromLua, FromLuaMulti, IntoLua, IntoLuaMulti};
use olua::{self, IntoResult, WrapFn};

use crate::LuaRef;

/// A wrapper around a Lua reference to a function stored in the Lua registry.
// #[derive(Eq, PartialEq, Hash)]
#[derive(PartialEq, Clone)]
pub struct Function<A, R> {
    pub(crate) lua_fun: mlua::Function,
    pub(crate) lua_ref: Cell<Option<LuaRef>>,
    _pd: (PhantomData<A>, PhantomData<R>),
}

impl<A, R> fmt::Debug for Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<function {}: {} -> {}>",
            self.lua_ref.get().unwrap_or_default(),
            std::any::type_name::<A>(),
            std::any::type_name::<R>()
        )
    }
}

impl<A, R, F, O> From<F> for Function<A, R>
where
    F: Fn(A) -> O + mlua::MaybeSend + 'static,
    A: FromLuaMulti,
    O: IntoResult<R>,
    R: IntoLuaMulti,
    O::Error: ExternalError + 'static,
{
    fn from(fun: F) -> Function<A, R> {
        Function::from_fn_mut(fun)
    }
}

impl<A, R> From<mlua::Function> for Function<A, R> {
    fn from(fun: mlua::Function) -> Self {
        Self::from_lua_fn(fun)
    }
}

impl<A, R> From<&Function<A, R>> for mlua::Function {
    fn from(fun: &Function<A, R>) -> Self {
        olua::get_registry_value(fun.lua_ref())
            .unwrap()
            .as_function()
            .unwrap()
            .to_owned()
    }
}

impl<A, R> From<Function<A, R>> for mlua::Function {
    fn from(fun: Function<A, R>) -> Self {
        Self::from(&fun)
    }
}

impl<A, R> FromLua for Function<A, R> {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::Function(fun) = value {
            Ok(Self::from(fun))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: std::any::type_name_of_val(&value),
                to: std::any::type_name::<Self>().to_string(),
                message: Some(
                    "expected `<mlua::Value::Function>`".to_string(),
                ),
            })
        }
    }
}

impl<A, R> IntoLua for Function<A, R> {
    fn into_lua(self, _lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Function(mlua::Function::from(self)))
    }
}

impl<A, R> Function<A, R> {
    #[deprecated = "Use `Function::try_from_ref` instead."]
    pub(crate) fn from_ref(lua_ref: LuaRef) -> Self {
        Self::try_from_ref(lua_ref).unwrap()
    }

    pub(crate) fn try_from_ref(lua_ref: LuaRef) -> mlua::Result<Self> {
        let lua = olua::get_nvim_lua()?;
        let value = olua::get_registry_value(lua_ref)?;
        Self::from_lua(value, &lua)
    }

    pub(crate) fn from_lua_fn(fun: mlua::Function) -> Self {
        Self {
            lua_fun: fun,
            lua_ref: Cell::new(None),
            _pd: (PhantomData, PhantomData),
        }
    }

    #[deprecated = "Please avoid using LuaRefs directly"]
    #[doc(hidden)]
    pub fn lua_ref(&self) -> LuaRef {
        let mut lua_ref = self.lua_ref.take();
        if lua_ref.is_none() {
            lua_ref = Some(unsafe {
                olua::add_registry_value(self.lua_fun.clone()).unwrap()
            });
        }
        self.lua_ref.replace(lua_ref.clone());
        lua_ref.unwrap()
    }

    #[deprecated = "Please use `Function::try_from_fn` instead."]
    pub fn from_fn<F, O>(fun: F) -> Self
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Self::try_from_fn(fun).unwrap()
    }

    pub fn try_from_fn<F, O>(fun: F) -> mlua::Result<Self>
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Ok(Self::from_lua_fn(WrapFn::wrap(fun)?))
    }

    #[deprecated = "Please use `Function::try_from_fn_mut` instead."]
    pub fn from_fn_mut<F, O>(fun: F) -> Self
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Self::try_from_fn_mut(fun).unwrap()
    }

    pub fn try_from_fn_mut<F, O>(fun: F) -> mlua::Result<Self>
    where
        F: FnMut(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Ok(Self::from_lua_fn(WrapFn::wrap_mut(fun)?))
    }

    #[deprecated = "Please use `Function::try_from_fn_once` instead."]
    pub fn from_fn_once<F, O>(fun: F) -> Self
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Self::try_from_fn_once(fun).unwrap()
    }

    pub fn try_from_fn_once<F, O>(fun: F) -> mlua::Result<Self>
    where
        F: FnOnce(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Ok(Self::from_lua_fn(WrapFn::wrap_once(fun)?))
    }

    pub fn call(&self, args: A) -> mlua::Result<R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        self.lua_fun.call::<R>(args)
    }

    #[deprecated = "TODO: Deprecation notice"]
    /// Consumes the `Function`, removing the reference stored in the Lua
    /// registry.
    #[doc(hidden)]
    pub unsafe fn remove_from_lua_registry(self) {
        todo!()
        // let lua_ref =
        //     self.lua_ref.expect(&format!("not in Lua Registry: {:?}", self));
        // unsafe {
        //     olua::free_registry_lua_ref(lua_ref).unwrap();
        // }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use mlua::{FromLuaMulti, IntoLuaMulti};
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};

    use super::Function;
    use crate::LuaRef;

    impl<A, R> Serialize for Function<A, R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_f32(self.lua_ref as f32)
        }
    }

    impl<'de, A, R> Deserialize<'de> for Function<A, R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use std::marker::PhantomData;

            struct FunctionVisitor<A, R>(PhantomData<A>, PhantomData<R>);

            impl<A, R> Visitor<'_> for FunctionVisitor<A, R>
            where
                A: IntoLuaMulti,
                R: FromLuaMulti,
            {
                type Value = Function<A, R>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("an f32 representing a Lua reference")
                }

                fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Function::from_ref(value as LuaRef))
                }
            }

            deserializer
                .deserialize_f32(FunctionVisitor(PhantomData, PhantomData))
        }
    }
}
