extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::DeriveInput;

mod precond;
mod unit;

#[proc_macro_derive(UnitVal, attributes(unitval))]
pub fn unitval_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = syn::parse2(tokens.into()).unwrap();
    expand_unitval(tokens).unwrap_or_else(|e| e).into()
}

fn expand_unitval(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    let ident = input.ident;
    let enum_data = precond::unit_enum(input.data)?;

    let unit::Tokens { as_tokens, from_tokens } = unit::Tokens::try_from(enum_data)?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::unitval::AsUnitVal for #ident #ty_generics #where_clause {
            fn as_unitval(&self) -> &'static str {
                match self {
                    #as_tokens
                }
            }
        }

        impl #impl_generics ::unitval::FromUnitVal for #ident #ty_generics #where_clause {
            fn from_unitval(value: &str) -> ::std::io::Result<Self> {
                match value {
                    #from_tokens
                    _ => Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidInput,
                        format!("unknown unitval {:?} for {}", value, stringify!(#ident)),
                    ))
                }
            }
        }
    })
}
