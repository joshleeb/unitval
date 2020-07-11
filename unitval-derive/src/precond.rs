use proc_macro2::TokenStream;
use quote::quote_spanned;

pub fn unit_enum(ident: syn::Ident, data: syn::Data) -> Result<syn::DataEnum, TokenStream> {
    if let syn::Data::Enum(enum_data) = data {
        all_unit_variants(&enum_data)?;
        Ok(enum_data)
    } else {
        Err(quote_spanned! {
            ident.span() => compile_error!("#[derive(UnitVal)] is only defined for enums");
        })
    }
}

fn all_unit_variants(data: &syn::DataEnum) -> Result<(), TokenStream> {
    data.variants.iter().try_for_each(|variant| {
        if let syn::Fields::Unit = variant.fields {
            Ok(())
        } else {
            Err(quote_spanned! {
                variant.ident.span() =>
                compile_error!("#[derive(UnitVal)] on enums must have all unit variants");
            })
        }
    })
}
