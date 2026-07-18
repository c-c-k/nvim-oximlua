//! This module contains bindings to
//! [Neovim's Vimscript functions](https://neovim.io/doc/user/vimfn/#stdpath),
//! exposed in Lua through the `vim.fn` table.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod registers;
mod system;

use error::Result;
pub use registers::*;
pub use system::*;
