//! This module contains bindings to
//! [Neovim's Vimscript functions](https://neovim.io/doc/user/vimfn/#stdpath),
//! exposed in Lua through the `vim.fn` table.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub mod opts;
mod registers;
mod system;
pub mod types;

use error::Error;
use error::Result;
pub use registers::*;
pub use system::*;
