extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

#[proc_macro_derive(UnitVal)]
pub fn unitval_derive(tokens: TokenStream) -> TokenStream {
    let _ast: DeriveInput = parse2(tokens.into()).unwrap();
    TokenStream::from(quote! {})
}
