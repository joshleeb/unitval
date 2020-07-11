use proc_macro2::TokenStream;
use quote::ToTokens;
use std::convert::TryFrom;
use syn::DataEnum;

use crate::unit::Unit;

pub struct Tokens {
    pub as_tokens: AsUnitValTokens,
    pub from_tokens: FromUnitValTokens,
}

impl TryFrom<DataEnum> for Tokens {
    type Error = TokenStream;

    fn try_from(value: DataEnum) -> Result<Self, Self::Error> {
        let mut as_tokens = vec![];
        let mut from_tokens = vec![];

        for variant in value.variants {
            let unit = Unit::try_from(variant)?;
            as_tokens.push(unit.as_unitval_tokens());
            from_tokens.push(unit.from_unitval_tokens());
        }
        Ok(Tokens {
            as_tokens: AsUnitValTokens(as_tokens),
            from_tokens: FromUnitValTokens(from_tokens),
        })
    }
}

pub struct AsUnitValTokens(Vec<TokenStream>);

impl ToTokens for AsUnitValTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.clone().into_iter().for_each(|t| tokens.extend(t))
    }
}

pub struct FromUnitValTokens(Vec<TokenStream>);

impl ToTokens for FromUnitValTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.clone().into_iter().for_each(|t| tokens.extend(t))
    }
}
