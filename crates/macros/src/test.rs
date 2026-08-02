use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{AttrStyle, ItemFn, LitStr, Meta, Token, parse_macro_input};

use crate::common::{DuplicateError, Keyed, KeyedAttribute};
use crate::plugin::NvimOximlua;

#[inline]
pub fn test(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attributes);

    let ItemFn { attrs: test_attrs, sig, block, .. } =
        parse_macro_input!(item as syn::ItemFn);

    let should_panic = test_attrs.iter().any(|attr| {
        let AttrStyle::Outer = &attr.style else { return false };
        let Meta::Path(path) = &attr.meta else { return false };
        path.segments.iter().any(|segment| segment.ident == "should_panic")
    });

    let test_name = sig.ident;

    let test_ret = if should_panic {
        quote!()
    } else {
        quote! {
            -> ::core::result::Result<(), impl ::core::fmt::Debug>
        }
    };

    let nvim_oximlua = &attrs.nvim_oximlua;

    let ret = &sig.output;

    let plugin_name = Ident::new(&format!("__{test_name}"), Span::call_site());

    let extra_cmd = match &attrs.cmd {
        Some(Cmd { cmd, .. }) => quote! { ::core::option::Option::Some(#cmd) },
        None => quote! { ::core::option::Option::None },
    };

    let maybe_ignore_err = should_panic.then(|| quote!(let _ = ));

    let maybe_semicolon = should_panic.then(|| quote!(;));

    #[cfg(feature = "test-terminator")]
    let plugin_body = match &sig.inputs.first() {
        Some(terminator) => quote! {
           fn __test_fn(#terminator) #ret {
               #block
           }
           #nvim_oximlua::tests::test_macro::plugin_body_with_terminator(__test_fn)
        },
        None => quote! {
            fn __test_fn() #ret {
                #block
            }
            #nvim_oximlua::tests::test_macro::plugin_body(__test_fn)
        },
    };

    #[cfg(not(feature = "test-terminator"))]
    let plugin_body = quote! {
        fn __test_fn() #ret {
            #block
        }
        #nvim_oximlua::tests::test_macro::plugin_body(__test_fn)
    };

    #[cfg(false)] // TODO: Adjust to nvim-oximlua
    let plugin_entry_point = quote! {
        #[#nvim_oximlua::plugin(nvim_oximlua = #nvim_oximlua)]
        fn #plugin_name()  {
            #plugin_body
        }
    };

    #[cfg(true)] // TODO: Adjust to nvim-oximlua
    let plugin_entry_point = quote! {
        #[mlua::lua_module]
        fn #plugin_name(lua: &mlua::Lua) -> mlua::Result<bool> {
            #nvim_oximlua::init(lua)?;
            #plugin_body;
            Ok(false)
        }
    };

    quote! {
        #[test]
        #(#test_attrs)*
        fn #test_name() #test_ret {
            #maybe_ignore_err #nvim_oximlua::tests::test_macro::test_body(
                env!("CARGO_MANIFEST_PATH"),
                stringify!(#plugin_name),
                #extra_cmd,
            )#maybe_semicolon
        }

        #plugin_entry_point
    }
    .into()
}

#[derive(Default)]
struct Attributes {
    cmd: Option<Cmd>,
    nvim_oximlua: NvimOximlua,
}

impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut has_parsed_nvim_oximlua = false;

        while !input.is_empty() {
            match input.parse::<Attribute>()? {
                Attribute::Cmd(cmd) => {
                    if this.cmd.is_some() {
                        return Err(DuplicateError(cmd).into());
                    }
                    this.cmd = Some(cmd);
                },
                Attribute::NvimOxi(nvim_oximlua) => {
                    if has_parsed_nvim_oximlua {
                        return Err(DuplicateError(nvim_oximlua).into());
                    }
                    this.nvim_oximlua = nvim_oximlua;
                    has_parsed_nvim_oximlua = true;
                },
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(this)
    }
}

enum Attribute {
    Cmd(Cmd),
    NvimOxi(NvimOximlua),
}

impl Parse for Attribute {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input
            .parse::<Cmd>()
            .map(Self::Cmd)
            .or_else(|_| input.parse::<NvimOximlua>().map(Self::NvimOxi))
    }
}

/// The command that will be passed to the Neovim CLI.
struct Cmd {
    key_span: Span,
    cmd: LitStr,
}

impl KeyedAttribute for Cmd {
    const KEY: &'static str = "cmd";

    type Value = LitStr;

    #[inline]
    fn key_span(&self) -> Span {
        self.key_span
    }
}

impl Parse for Cmd {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_span: Span::call_site(),
            cmd: input.parse::<Keyed<Self>>()?.value,
        })
    }
}

impl ToTokens for Cmd {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let str = self.cmd.value().lines().collect::<Vec<_>>().join(";");
        let lit = LitStr::new(&str, self.cmd.span());
        lit.to_tokens(tokens);
    }
}
