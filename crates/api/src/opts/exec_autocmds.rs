use crate::Buffer;
use crate::{StringOrInt, StringOrListOfStrings};

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[cfg_attr( // On 0.12 and Nightly.
        feature = "neovim-0-12",
        deprecated = "NVIM v0.12.2 soft deprecates `buffer` for `ExecAutocmdsOpts`, use `buf` instead",
    )]
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly.
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buf: types::BufHandle,

    /// The autocommand group name or id to match against.
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Whether to process the modeline after the autocommands.
    #[builder(argtype = "bool")]
    modeline: types::Boolean,

    /// Patterns to match against. Cannot be used together with
    /// [`buffer`](ExecAutocmdsOptsBuilder::buffer).
    #[builder(
        generics = "P: StringOrListOfStrings",
        method = "patterns",
        argtype = "P",
        inline = "{0}.to_object()"
    )]
    pattern: types::Object,

    #[builder(
        generics = "D: Into<types::Object>",
        argtype = "D",
        inline = "{0}.into()"
    )]
    data: types::Object,
}
