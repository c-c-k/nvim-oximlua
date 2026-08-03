//! # Rust bindings to all things Neovim
//!
//! This library provides safe bindings to the API exposed by the [Neovim] text
//! editor.
//!
//! [Neovim]: https://neovim.io

#![doc(html_root_url = "https://docs.rs/nvim_oximlua/latest")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

#[doc(hidden)]
pub mod entrypoint;
mod error;
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

pub mod mlua {
    //! This is an almost 1:1 re-export of the [`mlua`] crate
    //! ([github:mlua-rs/mlua]),
    //! please consult it's documentation for details.
    //!
    //! The sole exception to the 1:1 re-export is the addition
    //! of the deprecated [`lua()`] function kept here to ease conversion
    //! from the original [`nvim-oxi`].
    //!
    //! [`mlua`]: ::mlua
    //! [github:mlua-rs/mlua]: https://github.com/mlua-rs/mlua
    //! [`lua()`]: ::olua::lua
    //! [`nvim-oxi`]: https://docs.rs/nvim-oxi

    pub use mlua::*;
    #[deprecated = "Including this in what is otherwise a 1:1 re-export of \
                    the `mlua` crate can cause confusion for new users. \
                    Please use \
                    [`nvim_oximlua::olua::get_nvim_lua`](::olua::get_nvim_lua) \
                    or [`nvim_oximlua::olua::lua`](::olua::lua) instead."]
    pub use olua::lua;
}

#[cfg(feature = "mlua-extras")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua-extras")))]
pub mod mlua_extras {
    //! This is a 1:1 re-export of the [`mlua-extras`] crate
    //! ([github:tired-fox/mlua-extras]),
    //! please consult it's documentation for details.
    //!
    //! [`mlua-extras`]: ::mlua_extras
    //! [github:tired-fox/mlua-extras]: https://github.com/tired-fox/mlua-extras

    pub use mlua_extras::*;
}

pub mod olua {
    //! Integrations to work alongside the [`mlua`] crate.
    //!
    //! [`mlua`]: ::mlua

    #[doc(inline)]
    pub use olua::*;
}

#[cfg(true)] // TODO: Adjust to nvim-oximlua
pub use entrypoint::init;
#[cfg(false)] // TODO: Adjust to nvim-oximlua
pub use entrypoint::plugin;
pub use error::{Error, Result};
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use macros::test;
pub use olua::{IntoResult, dbg, print};
pub use types::*;
#[cfg(feature = "test")]
pub mod tests;
pub use toplevel::*;
pub use types::string;
