use std::fmt;
use std::marker::PhantomData;

use mlua::{ExternalError, FromLuaMulti, IntoLuaMulti};
use olua::IntoResult;
use olua::WrapFn;
use oximlua as olua;

use crate::LuaRef;

/// A wrapper around a Lua reference to a function stored in the Lua registry.
#[derive(Eq, PartialEq, Hash)]
pub struct Function<A, R> {
    pub(crate) lua_ref: LuaRef,
    _pd: (PhantomData<A>, PhantomData<R>),
}

impl<A, R> Clone for Function<A, R>
where
    A: IntoLuaMulti,
    R: FromLuaMulti,
{
    fn clone(&self) -> Self {
        Self::from_ref(self.lua_ref)
    }
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
            self.lua_ref,
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
        let lua_ref = unsafe { olua::add_registry_value(fun) }.unwrap();
        Self::from_ref(lua_ref)
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

impl<A, R> mlua::FromLua for Function<A, R> {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::Function(fun) = value {
            Ok(Function::from(fun))
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

impl<A, R> mlua::IntoLua for Function<A, R> {
    fn into_lua(self, _lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Function(mlua::Function::from(self)))
    }
}

impl<A, R> Function<A, R> {
    pub(crate) fn from_ref(lua_ref: LuaRef) -> Self {
        Self { lua_ref, _pd: (PhantomData, PhantomData) }
    }

    #[doc(hidden)]
    pub fn lua_ref(&self) -> LuaRef {
        self.lua_ref
    }

    pub fn from_fn<F, O>(fun: F) -> Self
    where
        F: Fn(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Function::from(WrapFn::wrap(fun).unwrap())
    }

    pub fn from_fn_mut<F, O>(fun: F) -> Self
    where
        F: FnMut(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Function::from(WrapFn::wrap_mut(fun).unwrap())
    }

    pub fn from_fn_once<F, O>(fun: F) -> Self
    where
        F: FnOnce(A) -> O + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        O: IntoResult<R>,
        R: IntoLuaMulti,
        O::Error: ExternalError + 'static,
    {
        Function::from(WrapFn::wrap_once(fun).unwrap())
    }

    pub fn call(&self, args: A) -> mlua::Result<R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        mlua::Function::from(self).call::<R>(args)
    }

    /// Consumes the `Function`, removing the reference stored in the Lua
    /// registry.
    #[doc(hidden)]
    pub unsafe fn remove_from_lua_registry(self) {
        unsafe {
            olua::free_registry_lua_ref(self.lua_ref).unwrap();
        }
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
