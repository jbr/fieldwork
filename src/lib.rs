#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications,
    missing_docs,
    rustdoc::missing_crate_level_docs
)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../docs.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

mod copy_detection;
mod deref_handling;
mod field;
mod field_attributes;
mod field_method_attributes;
mod method;
mod option_handling;
mod query;
mod resolved;
mod r#struct;
mod struct_attributes;
mod struct_method_attributes;

#[cfg(test)]
mod coverage_tests;

pub(crate) use field::Field;
pub(crate) use field_attributes::FieldAttributes;
pub(crate) use field_method_attributes::FieldMethodAttributes;
pub(crate) use method::Method;
pub(crate) use query::Query;
pub(crate) use resolved::Resolved;
pub(crate) use r#struct::Struct;
pub(crate) use struct_attributes::StructAttributes;
pub(crate) use struct_method_attributes::StructMethodAttributes;

const DEFAULT_CHAINABLE_SET: bool = true;
const DEFAULT_OPTION_HANDLING: bool = true;
const DEFAULT_AUTO_DEREF: bool = true;
const DEFAULT_AUTO_COPY: bool = true;

/// see crate-level documentation
#[proc_macro_derive(Fieldwork, attributes(fieldwork))]
pub fn derive_fieldwork(input: TokenStream) -> TokenStream {
    derive_fieldwork_internal(input.into()).into()
}
fn derive_fieldwork_internal(input: TokenStream2) -> TokenStream2 {
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
