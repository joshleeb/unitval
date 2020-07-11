extern crate proc_macro;

use proc_macro2::TokenStream;
use std::convert::TryFrom;
use syn::DeriveInput;

mod ast;
mod precond;
mod unit;

#[proc_macro_derive(UnitVal, attributes(unitval))]
pub fn unitval_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = syn::parse2(tokens.into()).unwrap();
    expand_unitval(tokens).unwrap_or_else(|e| e).into()
}

fn expand_unitval(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    let enum_data = precond::unit_enum(&input.data)?;
    let tokens = unit::Tokens::try_from(enum_data)?;

    let mut ast = ast::impl_as_unit_val(&input, &tokens);
    ast.extend(ast::impl_from_unit_val(&input, &tokens));
    Ok(ast)
}
