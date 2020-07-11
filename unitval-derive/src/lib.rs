extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;

mod precond;

#[proc_macro_derive(UnitVal, attributes(unitval))]
pub fn unitval_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = syn::parse2(tokens.into()).unwrap();
    expand_unitval(tokens).unwrap_or_else(|e| e).into()
}

fn expand_unitval(input: syn::DeriveInput) -> Result<TokenStream, TokenStream> {
    let _enum_data = precond::unit_enum(&input.ident, input.data)?;

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics AsUnitVal for #ident #ty_generics #where_clause {
            fn as_unitval(&self) -> &'static str {
                todo!()
            }
        }

        impl #impl_generics FromUnitVal for #ident #ty_generics #where_clause {
            fn from_unitval(value: &str) -> ::std::io::Result<Self> {
                todo!()
            }
        }
    })
}
