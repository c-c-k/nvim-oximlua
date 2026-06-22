//! # Rust bindings to all things Neovim
//!
//! This library provides safe bindings to the API exposed by the [Neovim] text
//! editor.
//!
//! [Neovim]: https://neovim.io

#![doc(html_root_url = "https://docs.rs/nvim_oxi/latest")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(not(feature = "oximlua"))]
#[doc(hidden)]
pub mod entrypoint;
mod error;
#[cfg(feature = "oximlua")]
mod init;
mod toplevel;

pub mod api {
    //! Bindings to the [Neovim C API][api].
    //!
    //! [api]: https://neovim.io/doc/user/api.html
    #[doc(inline)]
    pub use api::*;
}

pub mod vimfn {
    //! Bindings to the [Neovim Vimscript functions][fn].
    //!
    //! [fn]: https://neovim.io/doc/user/vimfn/#vimscript-functions
    #[doc(inline)]
    pub use vimfn::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    //! Bindings to the [Neovim event loop][loop] powered by [libuv].
    //!
    //! [loop]: https://neovim.io/doc/user/lua.html#vim.loop
    //! [libuv]: https://libuv.org/
    #[doc(inline)]
    pub use libuv::*;
}

#[cfg(not(feature = "oximlua"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "oximlua"))))]
pub mod lua {
    //! Low-level Rust bindings to [LuaJIT], the Lua version used by Neovim.
    //!
    //! [LuaJIT]: https://luajit.org/
    #[doc(inline)]
    pub use luajit::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    //! Integrations with the [mlua] Rust crate providing safe Lua bindings.
    //!
    //! [mlua]: https://github.com/khvzak/mlua

    pub use mlua::*;

    #[cfg(feature = "oximlua")]
    #[deprecated = "including this in what is otherwise a 1:1 re-export of \
                    the `mlua` crate can cause confusion for new users. \
                    Please use \
                    [`nvim_oxi::olua::get_nvim_lua`](oximlua::get_nvim_lua) \
                    or [`nvim_oxi::olua::lua`](oximlua::lua) instead."]
    pub fn lua() -> mlua::Lua {
        oximlua::lua()
    }

    #[cfg(not(feature = "oximlua"))]
    /// Returns a
    /// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
    /// instance which can be used to interact with Lua plugins.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use mlua::prelude::LuaFunction;
    /// use nvim_oxi as nvim;
    ///
    /// #[nvim::plugin]
    /// fn mlua() -> nvim::Result<()> {
    ///     nvim::print!("Hello from nvim-oxi..");
    ///
    ///     let lua = nvim::mlua::lua();
    ///     let print = lua.globals().get::<_, LuaFunction>("print")?;
    ///     print.call("..and goodbye from mlua!")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lua() -> mlua::Lua {
        unsafe {
            luajit::with_state(|lua_state| {
                mlua::Lua::get_or_init_from_ptr(lua_state as *mut _).clone()
            })
        }
    }
}

#[cfg(feature = "oximlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "oximlua")))]
pub mod olua {
    //! Integrations to work alongside the [mlua] Rust crate.
    //!
    //! [mlua]: https://github.com/khvzak/mlua

    pub use oximlua::*;
}

pub use error::{Error, Result};
#[cfg(feature = "oximlua")]
pub use init::init;
#[cfg(not(feature = "oximlua"))]
pub use luajit::{IntoResult, dbg, print};
#[cfg(not(feature = "oximlua"))]
pub use macros::plugin;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use macros::test;
#[cfg(feature = "oximlua")]
pub use oximlua::{IntoResult, dbg, print};
pub use types::*;
#[cfg(feature = "test")]
pub mod tests;
pub use toplevel::*;
pub use types::string;
