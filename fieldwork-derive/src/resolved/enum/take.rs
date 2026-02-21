use crate::{Query, r#enum::arm_pattern, option_handling::extract_option_type};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use syn::{Ident, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Take<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    vis: Cow<'a, Visibility>,
    return_ty: &'a Type,
    /// One pattern per matching variant.
    patterns: Vec<TokenStream>,
    /// The binding name shared across all arms (used to call `.take()`).
    binding: Ident,
    full_coverage: bool,
}

impl<'a> Take<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Self {
            doc,
            fn_ident,
            span,
            vis,
            return_ty,
            patterns,
            binding,
            full_coverage,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        let match_body = if *full_coverage {
            quote! { #(#patterns)|* => #binding.take() }
        } else {
            quote! { #(#patterns => #binding.take(),)* _ => None }
        };
        quote_spanned! {*span=>
            #doc
            #vis fn #fn_ident(&mut self) -> #return_ty {
                match self { #match_body }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        extract_option_type(query.ty())?;

        let virtual_field = query.virtual_field()?;
        let span = query.span();
        let fn_ident = query.fn_ident()?;
        let vis = query.vis();
        let doc = query.docs(false);
        let return_ty = query.ty();
        let binding = virtual_field.arms.first()?.binding.clone();

        let patterns = virtual_field
            .arms
            .iter()
            .map(|arm| arm_pattern(&arm.variant_ident, arm, None))
            .collect();

        Some(Self {
            doc,
            fn_ident,
            span,
            vis,
            return_ty,
            patterns,
            binding,
            full_coverage: virtual_field.is_full_coverage(),
        })
    }
}
