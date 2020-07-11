pub use tokens::{AsUnitValTokens, FromUnitValTokens, Tokens};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::convert::TryFrom;
use syn::{Lit, Meta, MetaNameValue, Variant};

mod tokens;

const ATTR_PATH: &str = "unitval";

struct Unit {
    variant: Variant,
    unitval: String,
}

impl Unit {
    pub fn as_unitval_tokens(&self) -> TokenStream {
        let ident = &self.variant.ident;
        let unitval = &self.unitval;
        quote! { Self::#ident => #unitval }
    }

    pub fn from_unitval_tokens(&self) -> TokenStream {
        let ident = &self.variant.ident;
        let unitval = &self.unitval;
        quote! { #unitval => Ok(Self::#ident) }
    }
}

impl TryFrom<Variant> for Unit {
    type Error = TokenStream;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        for attr in value.attrs.iter() {
            let meta = attr.parse_meta().map_err(|e| e.to_compile_error())?;
            let attr_ident = meta.path().get_ident().map(|i| i.to_string());
            if attr_ident.unwrap_or_default() == ATTR_PATH {
                if let Meta::NameValue(MetaNameValue { lit: Lit::Str(lit), .. }) = meta {
                    return Ok(Self { variant: value, unitval: lit.value() });
                }
            }
        }
        Err(quote_spanned! {
            value.ident.span() =>
            compile_error!("enum deriving UnitVal has variants without a specified `unitval`");
        })
    }
}
