use crate::{Query, r#enum::arm_pattern, option_handling::extract_option_type};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use syn::{Ident, Type, Visibility, parse_quote_spanned};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct GetMut<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    vis: Cow<'a, Visibility>,
    return_ty: Type,
    /// One pattern per matching variant.
    patterns: Vec<TokenStream>,
    /// Arm body expression, already wrapped in `Some(...)` for partial coverage if needed.
    arm_expr: TokenStream,
    full_coverage: bool,
}

impl<'a> GetMut<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Self {
            doc,
            fn_ident,
            span,
            vis,
            return_ty,
            patterns,
            arm_expr,
            full_coverage,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        let match_body = if *full_coverage {
            quote! { #(#patterns => #arm_expr,)* }
        } else {
            quote! { #(#patterns => #arm_expr,)* _ => None }
        };
        quote_spanned! {*span=>
            #doc
            #vis fn #fn_ident(&mut self) -> #return_ty {
                match self { #match_body }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let virtual_field = query.virtual_field()?;
        let span = query.span();
        let fn_ident = query.fn_ident()?;
        let vis = query.vis();
        let doc = query.docs(false);

        let first_binding = &virtual_field.arms.first()?.binding;
        let base = parse_quote_spanned!(span => *#first_binding);
        let (arm_expr_typed, return_ty) = query.apply_mut_transforms(base);

        let full_coverage = virtual_field.is_full_coverage();
        let (return_ty, arm_expr) = if !full_coverage && extract_option_type(query.ty()).is_none() {
            (
                parse_quote_spanned!(span => Option<#return_ty>),
                quote! { Some(#arm_expr_typed) },
            )
        } else {
            (return_ty, quote! { #arm_expr_typed })
        };

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
            arm_expr,
            full_coverage,
        })
    }
}
