use proc_macro2::TokenStream;
use quote::quote_spanned;

pub fn unit_enum(data: syn::Data) -> Result<syn::DataEnum, TokenStream> {
    match data {
        syn::Data::Enum(enum_data) => {
            all_unit_variants(&enum_data)?;
            Ok(enum_data)
        }
        syn::Data::Struct(struct_data) => Err(quote_spanned! {
            struct_data.struct_token.span =>
            compile_error!("UnitVal cannot be derived for structs");
        }),
        syn::Data::Union(union_data) => Err(quote_spanned! {
            union_data.union_token.span =>
            compile_error!("UnitVal cannot be derived for unions");
        }),
    }
}

fn all_unit_variants(data: &syn::DataEnum) -> Result<(), TokenStream> {
    data.variants.iter().try_for_each(|variant| {
        if let syn::Fields::Unit = variant.fields {
            Ok(())
        } else {
            Err(quote_spanned! {
                data.enum_token.span =>
                compile_error!("UnitVal can only be derived on enums with all unit variants");
            })
        }
    })
}
