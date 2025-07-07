use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Expr, Ident, Member, Visibility};

use crate::Query;

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct ResolvedWithout<'a> {
    pub(crate) assigned_value: Expr,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) span: Span,
    pub(crate) variable_ident: &'a Member,
    pub(crate) vis: Cow<'a, Visibility>,
}

impl<'a> ResolvedWithout<'a> {
    pub(crate) fn build(&self) -> TokenStream2 {
        let ResolvedWithout {
            vis,
            fn_ident,
            variable_ident,
            doc,
            assigned_value,
            span,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        quote_spanned! {*span=>
            #doc
            #[must_use]
            #vis fn #fn_ident(mut self) -> Self {
                self.#variable_ident = #assigned_value;
                self
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let variable_ident = query.variable_ident();
        let argument_ident = query.argument_ident()?;
        let (_, assigned_value) =
            query.determine_argument_ty_and_assigned_value(&argument_ident)?;
        let doc = query.docs(false);

        Some(Self {
            assigned_value,
            doc,
            fn_ident,
            span,
            variable_ident,
            vis,
        })
    }
}
