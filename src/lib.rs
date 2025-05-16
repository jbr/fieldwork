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

use Method::{Get, GetMut, Set, With};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DeriveInput, Error, Generics, Ident, Path,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
};

mod field;
mod field_method;
mod query;
mod r#struct;
mod struct_method;

pub(crate) use field::{Field, FieldAttributes};
pub(crate) use field_method::FieldMethodAttributes;
pub(crate) use query::{Resolved, resolve};
pub(crate) use r#struct::StructAttributes;
pub(crate) use struct_method::StructMethodAttributes;

const ALL_VARIANTS: &[Method] = &[Get, GetMut, Set, With];
const DEFAULT_CHAINABLE_SET: bool = true;

impl Method {
    fn build(self, field: &Field, struct_attributes: &StructAttributes) -> TokenStream2 {
        let Some(Resolved {
            vis,
            fn_ident,
            variable_ident,
            argument_ident,
            ty,
            doc,
            get_copy,
            chainable_set,
        }) = resolve(&self, field, struct_attributes)
        else {
            return quote!();
        };

        let doc = doc.map(|d| quote!(#[doc = #d]));

        match self {
            Get if get_copy => {
                quote! {
                    #doc
                    #vis fn #fn_ident(&self) -> #ty {
                        self.#variable_ident
                    }
                }
            }

            Get => {
                quote! {
                    #doc
                    #vis fn #fn_ident(&self) -> &#ty {
                        &self.#variable_ident
                    }
                }
            }

            Set if chainable_set => quote! {
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #ty) -> &mut Self {
                    self.#variable_ident = #argument_ident;
                    self
                }
            },

            Set => quote! {
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #ty) {
                    self.#variable_ident = #argument_ident;
                }
            },

            With => quote! {
                #doc
                #[must_use]
                #vis fn #fn_ident(mut self, #argument_ident: #ty) -> Self {
                    self.#variable_ident = #argument_ident;
                    self
                }
            },

            GetMut => quote! {
                #doc
                #vis fn #fn_ident(&mut self) -> &mut #ty {
                    &mut self.#variable_ident
                }
            },
        }
    }
}

/// see crate-level documentation
#[proc_macro_derive(Fieldwork, attributes(fieldwork))]
pub fn derive_fieldwork(input: TokenStream) -> TokenStream {
    let Struct {
        ident,
        fields,
        attributes: attrs,
        generics,
    } = parse_macro_input!(input as Struct);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let impls = fields
        .iter()
        .map(|field| {
            ALL_VARIANTS
                .iter()
                .map(|method| method.build(field, &attrs))
                .collect::<TokenStream2>()
        })
        .collect::<TokenStream2>();

    quote! {
        impl #impl_generics #ident #type_generics #where_clause {
            #impls
        }
    }
    .into()
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = DeriveInput::parse(input)?;
        let Data::Struct(ds) = &input.data else {
            return Err(Error::new(
                input.span(),
                "fieldwork currently only works on named structs",
            ));
        };
        let ident = input.ident;
        let mut attributes = StructAttributes::build(&input.attrs)?;
        let fields = ds
            .fields
            .iter()
            .map(Field::build)
            .collect::<syn::Result<Vec<_>>>()?;

        let mut generics = input.generics.clone();
        generics.where_clause = attributes.where_clause.take();

        Ok(Self {
            ident,
            fields,
            attributes,
            generics,
        })
    }
}

// this represents the struct that we called derive on
#[derive(Debug)]
pub(crate) struct Struct {
    pub(crate) ident: Ident,
    pub(crate) fields: Vec<Field>,
    pub(crate) attributes: StructAttributes,
    pub(crate) generics: Generics,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) enum Method {
    Get,
    Set,
    With,
    GetMut,
}

impl TryFrom<&Path> for Method {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_ident("get") {
            Ok(Get)
        } else if path.is_ident("set") {
            Ok(Set)
        } else if path.is_ident("with") {
            Ok(With)
        } else if path.is_ident("get_mut") {
            Ok(GetMut)
        } else {
            Err(Error::new(path.span(), "unrecognized method"))
        }
    }
}
