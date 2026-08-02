pub mod error;
mod into_result;
mod macros;
pub(crate) mod mlua_shim;
pub mod utils;
mod wrap_fn;

pub use crate::error::Error;
pub use crate::into_result::IntoResult;
pub use crate::macros::__print;
pub use crate::mlua_shim::{
    add_registry_value,
    free_registry_lua_ref,
    get_nvim_lua,
    get_registry,
    get_registry_value,
    init,
    is_nvim_thread,
    lua,
};
pub use crate::wrap_fn::WrapFn;
