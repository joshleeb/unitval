extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;

mod precond;

#[proc_macro_derive(UnitVal)]
pub fn unitval_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = syn::parse2(tokens.into()).unwrap();
    expand_unitval(tokens).unwrap_or_else(|e| e).into()
}

fn expand_unitval(input: syn::DeriveInput) -> Result<TokenStream, TokenStream> {
    let _enum_data = precond::unit_enum(input.ident, input.data)?;

    Ok(quote! {})
}
