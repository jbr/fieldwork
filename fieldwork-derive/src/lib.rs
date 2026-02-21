#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications,
    rustdoc::missing_crate_level_docs
)]
#![warn(clippy::pedantic)]
//! Procedural macro implementation for [fieldwork](https://docs.rs/fieldwork).
//!
//! This crate is the proc-macro backend. For documentation, configuration options,
//! and examples, see the [`fieldwork`](https://docs.rs/fieldwork) crate.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput};

mod common_settings;
mod copy_detection;
mod deref_handling;
mod r#enum;
mod errors;
mod field;
mod field_attributes;
mod field_method_attributes;
mod item_attributes;
mod item_method_attributes;
mod method;
mod option_handling;
mod query;
mod resolved;
mod r#struct;
mod variant_attributes;

#[cfg(test)]
mod coverage_tests;

pub(crate) use common_settings::{CommonSettings, with_common_settings};
pub(crate) use r#enum::Enum;
pub(crate) use field::Field;
pub(crate) use field_attributes::FieldAttributes;
pub(crate) use field_method_attributes::FieldMethodAttributes;
pub(crate) use item_attributes::ItemAttributes;
pub(crate) use item_method_attributes::ItemMethodAttributes;
pub(crate) use method::{Method, MethodSettings, with_methods};
pub(crate) use query::Query;
pub(crate) use resolved::Resolved;
pub(crate) use r#struct::Struct;
pub(crate) use variant_attributes::VariantAttributes;

/// Derive field accessor methods for a struct or enum. See
/// [`fieldwork`](https://docs.rs/fieldwork) for full documentation.
#[proc_macro_derive(Fieldwork, attributes(fieldwork, field, variant))]
pub fn derive_fieldwork(input: TokenStream) -> TokenStream {
    derive_fieldwork_internal(input.into()).into()
}

pub(crate) fn derive_fieldwork_internal(input: TokenStream2) -> TokenStream2 {
    let peek = match syn::parse2::<DeriveInput>(input.clone()) {
        Ok(ok) => ok,
        Err(e) => return e.to_compile_error(),
    };

    match &peek.data {
        Data::Struct(_) => derive_struct(input),
        Data::Enum(_) => derive_enum(input),
        Data::Union(_) => {
            syn::Error::new_spanned(peek, "fieldwork does not support unions").to_compile_error()
        }
    }
}

fn derive_struct(input: TokenStream2) -> TokenStream2 {
    let Struct {
        ident,
        fields,
        attributes,
        generics,
    } = match syn::parse2(input) {
        Ok(ok) => ok,
        Err(e) => return e.to_compile_error(),
    };

    let impls = fields
        .iter()
        .flat_map(|field| {
            Method::all()
                .iter()
                .filter_map(|method| Query::new(method, field, &attributes).resolve())
        })
        .map(|resolved| resolved.build())
        .collect::<TokenStream2>();

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #ident #type_generics #where_clause {
            #impls
        }
    }
}

fn derive_enum(input: TokenStream2) -> TokenStream2 {
    let enum_item: Enum = match syn::parse2(input) {
        Ok(ok) => ok,
        Err(e) => return e.to_compile_error(),
    };

    let methods = enum_item.generate_methods();
    let ident = &enum_item.ident;
    let (impl_generics, type_generics, where_clause) = enum_item.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #type_generics #where_clause {
            #methods
        }
    }
}

pub(crate) fn is_fieldwork_attr(attr: &Attribute) -> bool {
    let path = attr.path();
    path.is_ident("fieldwork") || path.is_ident("field")
}
