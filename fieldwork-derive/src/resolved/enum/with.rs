use crate::{Query, r#enum::arm_pattern};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use syn::{Expr, Ident, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct With<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    vis: Cow<'a, Visibility>,
    argument_ident_and_ty: Option<(Cow<'a, Ident>, Cow<'a, Type>)>,
    assigned_value: Expr,
    /// Binding ident used in arm patterns and body — derived from the field name,
    /// suffixed with `_binding` when the field name would otherwise shadow the
    /// argument ident (e.g. field `x` with argument also named `x`).
    field_binding: Ident,
    patterns: Vec<TokenStream>,
}

impl<'a> With<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Self {
            doc,
            fn_ident,
            span,
            vis,
            argument_ident_and_ty,
            assigned_value,
            field_binding,
            patterns,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        let match_body = quote! { #(#patterns => { *#field_binding = #assigned_value; })* };
        if let Some((argument_ident, argument_ty)) = argument_ident_and_ty {
            quote_spanned! {*span=>
                #doc
                #[must_use]
                #vis fn #fn_ident(mut self, #argument_ident: #argument_ty) -> Self {
                    match &mut self { #match_body }
                    self
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #[must_use]
                #vis fn #fn_ident(mut self) -> Self {
                    match &mut self { #match_body }
                    self
                }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let virtual_field = query.virtual_field()?;
        if !virtual_field.is_full_coverage() {
            return None;
        }
        let span = query.span();
        let fn_ident = query.fn_ident()?;
        let vis = query.vis();
        let doc = query.docs(false);
        let argument_ident = query.argument_ident()?;
        let (argument_ty, assigned_value) =
            query.determine_argument_ty_and_assigned_value(&argument_ident)?;
        let argument_ident_and_ty = argument_ty.map(|ty| (argument_ident.clone(), ty));

        let arm_binding = &virtual_field.arms.first()?.binding;
        // Only suffix when there's an actual function argument that could clash.
        let field_binding: Ident =
            if argument_ident_and_ty.is_some() && arm_binding == &*argument_ident {
                Ident::new(&format!("{}_binding", arm_binding), span)
            } else {
                arm_binding.clone()
            };
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
            argument_ident_and_ty,
            assigned_value,
            field_binding,
            patterns,
        })
    }
}
