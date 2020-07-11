use crate::unit;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_as_unit_val(input: &DeriveInput, tokens: &unit::Tokens) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let as_tokens = &tokens.as_tokens;
    quote! {
        impl #impl_generics ::unitval::AsUnitVal for #ident #ty_generics #where_clause {
            fn as_unitval(&self) -> &'static str {
                match self {
                    #as_tokens
                }
            }
        }
    }
}

pub fn impl_from_unit_val(input: &DeriveInput, tokens: &unit::Tokens) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let from_tokens = &tokens.from_tokens;
    quote! {
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
    }
}
