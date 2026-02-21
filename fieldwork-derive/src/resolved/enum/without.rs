use crate::{Query, r#enum::arm_pattern};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use syn::{Expr, Ident, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Without<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    vis: Cow<'a, Visibility>,
    assigned_value: Expr,
    /// Binding ident derived from the field name; no suffix needed since `without`
    /// takes no function argument, so there is nothing to clash with.
    field_binding: Ident,
    patterns: Vec<TokenStream>,
    full_coverage: bool,
}

impl<'a> Without<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Self {
            doc,
            fn_ident,
            span,
            vis,
            assigned_value,
            field_binding,
            patterns,
            full_coverage,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        let fallthrough = if *full_coverage {
            quote! {}
        } else {
            quote! { _ => {} }
        };
        quote_spanned! {*span=>
            #doc
            #[must_use]
            #vis fn #fn_ident(mut self) -> Self {
                match &mut self {
                    #(#patterns => { *#field_binding = #assigned_value; })*
                    #fallthrough
                }
                self
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let virtual_field = query.virtual_field()?;
        let span = query.span();
        let fn_ident = query.fn_ident()?;
        let vis = query.vis();
        let doc = query.docs(false);
        let argument_ident = query.argument_ident()?;
        let (_, assigned_value) =
            query.determine_argument_ty_and_assigned_value(&argument_ident)?;

        // `without` takes no function argument, so the field binding can always
        // be the field name itself with no risk of shadowing.
        let field_binding = virtual_field.arms.first()?.binding.clone();
        let patterns = virtual_field
            .arms
            .iter()
            .map(|arm| arm_pattern(&arm.variant_ident, arm, Some(&field_binding)))
            .collect();

        Some(Self {
            doc,
            fn_ident,
            span,
            vis,
            assigned_value,
            field_binding,
            patterns,
            full_coverage: virtual_field.is_full_coverage(),
        })
    }
}
