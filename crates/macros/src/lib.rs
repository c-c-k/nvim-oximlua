use proc_macro::TokenStream;
use syn::parse_macro_input;

mod common;
mod derive_opts;

#[cfg(feature = "plugin")]
mod plugin;

#[cfg(feature = "test")]
mod test;

/// TODO: docs
#[proc_macro_derive(OptsBuilder, attributes(builder))]
pub fn derive_opts_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    derive_opts::expand_derive_opts_builder(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// MEMO: Docs are in the main crate re-export
#[cfg(all(feature = "plugin", false))] // TODO: Adjust to nvim-oximlua
#[proc_macro_attribute]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    plugin::plugin(attr, item)
}

/// Tests a piece of code from inside Neovim.
///
/// # Examples
///
/// ```ignore
/// use nvim_oximlua::api;
///
/// #[nvim_oximlua::test]
/// fn set_get_del_var() {
///     api::set_var("foo", 42).unwrap();
///     assert_eq!(Ok(42), api::get_var("foo"));
///     assert_eq!(Ok(()), api::del_var("foo"));
/// }
/// ```
///
/// The test function can also return a `Result<(), T>` if `T` implements
/// `Debug`:
///
/// ```ignore
/// # use nvim_oximlua::api;
/// #[nvim_oximlua::test]
/// fn print_42() -> Result<(), api::Error> {
///     api::command("lua print(42)")
/// }
/// ```
///
/// # Attributes
///
/// ## `nvim-oximlua`
///
/// Exactly the same as the `nvim-oximlua` attribute on the [`macro@plugin`] macro.
/// See [its documentation](macro@plugin#nvim-oximlua) for more information.
///
/// ## `cmd`
///
/// The `cmd` attribute is used to specify an Ex command that will be executed
/// by Neovim before the test's body. This can be useful to configure the
/// environment in which the test will run.
///
/// ```ignore
/// # use nvim_oximlua::api;
/// #[nvim_oximlua::test(cmd = "lua print('The answer is...')")]
/// fn print_42() -> Result<(), api::Error> {
///     api::command("lua print(42)")
/// }
/// ```
///
/// If the given string spans multiple lines, it will be joined into a single
/// line using `;` as the separator.
#[cfg(feature = "test")]
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::test(attr, item)
}
