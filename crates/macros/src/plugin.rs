use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, Path, Token, parse_macro_input, parse_quote};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};

#[allow(dead_code, reason = "disabled")] // TODO: Adjust to nvim-oximlua
#[inline]
pub fn plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attributes);

    let entrypoint = parse_macro_input!(item as ItemFn);

    let plugin_name = &entrypoint.sig.ident;

    let lua_module =
        Ident::new(&format!("luaopen_{plugin_name}"), Span::call_site());

    let nvim_oximlua = attrs.nvim_oximlua;

    quote! {
        #entrypoint

        #[unsafe(no_mangle)]
        unsafe extern "C" fn #lua_module(
            state: *mut #nvim_oximlua::lua::ffi::State,
        ) -> ::core::ffi::c_int {
            #nvim_oximlua::entrypoint::entrypoint(state, #plugin_name)
        }
    }
    .into()
}

#[allow(unused, reason = "unused by default")] // TODO: Adjust to nvim-oximlua
#[derive(Default)]
struct Attributes {
    nvim_oximlua: NvimOximlua,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut has_parsed_nvim_oxi = false;

        while !input.is_empty() {
            let keypair = input.parse::<Attribute>()?;

            match keypair {
                Attribute::NvimOximlua(nvim_oximlua) => {
                    if has_parsed_nvim_oxi {
                        return Err(DuplicateError(nvim_oximlua).into());
                    }
                    this.nvim_oximlua = nvim_oximlua;
                    has_parsed_nvim_oxi = true;
                },
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(this)
    }
}

#[allow(unused, reason = "unused by default")] // TODO: Adjust to nvim-oximlua
enum Attribute {
    NvimOximlua(NvimOximlua),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<NvimOximlua>().map(Self::NvimOximlua)
    }
}

pub(crate) struct NvimOximlua {
    key_span: Span,
    value: Path,
}

impl Default for NvimOximlua {
    #[inline]
    fn default() -> Self {
        Self {
            key_span: Span::call_site(),
            value: parse_quote!(::nvim_oximlua),
        }
    }
}

impl Parse for NvimOximlua {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            value: input.parse::<Keyed<Self>>()?.value,
        })
    }
}

impl KeyedAttribute for NvimOximlua {
    const KEY: &'static str = "nvim_oximlua";

    type Value = Path;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl ToTokens for NvimOximlua {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens);
    }
}
