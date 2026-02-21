use crate::{Query, r#enum::arm_pattern};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Ident, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct IntoField<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    vis: Cow<'a, Visibility>,
    return_ty: Type,
    /// One pattern per variant (full coverage only).
    patterns: Vec<TokenStream>,
    /// The binding name shared across all arms.
    binding: Ident,
}

impl<'a> IntoField<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Self {
            doc,
            fn_ident,
            span,
            vis,
            return_ty,
            patterns,
            binding,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        quote_spanned! {*span=>
            #doc
            #vis fn #fn_ident(self) -> #return_ty {
                match self { #(#patterns)|* => #binding }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        if query.is_get_copy(query.ty()) {
            return None;
        }
        let virtual_field = query.virtual_field()?;
        if !virtual_field.is_full_coverage() {
            return None;
        }
        let span = query.span();
        let fn_ident = query.fn_ident()?;
        let vis = query.vis();
        let doc = query.docs(false);
        let return_ty = query.ty().clone();
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
        })
    }
}
