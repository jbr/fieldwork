use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Expr, Ident, Member, Type, Visibility};

use crate::Query;

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct ResolvedWith<'a> {
    pub(crate) argument_ident_and_ty: Option<(Cow<'a, Ident>, Cow<'a, Type>)>,
    pub(crate) assigned_value: Expr,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) span: Span,
    pub(crate) variable_ident: &'a Member,
    pub(crate) vis: Cow<'a, Visibility>,
}

impl<'a> ResolvedWith<'a> {
    pub(crate) fn build(&self) -> TokenStream2 {
        let ResolvedWith {
            vis,
            fn_ident,
            variable_ident,
            argument_ident_and_ty,
            doc,
            assigned_value,
            span,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        if let Some((argument_ident, argument_ty)) = argument_ident_and_ty {
            quote_spanned! {*span=>
                #doc
                #[must_use]
                #vis fn #fn_ident(mut self, #argument_ident: #argument_ty) -> Self {
                    self.#variable_ident = #assigned_value;
                    self
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #[must_use]
                #vis fn #fn_ident(mut self) -> Self {
                    self.#variable_ident = #assigned_value;
                    self
                }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let variable_ident = query.variable_ident();
        let argument_ident = query.argument_ident()?;
        let (argument_ty, assigned_value) =
            query.determine_argument_ty_and_assigned_value(&argument_ident)?;
        let doc = query.docs(false);

        let argument_ident_and_ty = argument_ty.map(|ty| (argument_ident, ty));

        Some(Self {
            argument_ident_and_ty,
            assigned_value,
            doc,
            fn_ident,
            span,
            variable_ident,
            vis,
        })
    }
}
