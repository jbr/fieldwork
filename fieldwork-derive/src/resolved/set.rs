use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Expr, Ident, Member, Type, Visibility};

use crate::Query;

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct ResolvedSet<'a> {
    pub(crate) argument_ident: Cow<'a, Ident>,
    pub(crate) argument_ty: Cow<'a, Type>,
    pub(crate) assigned_value: Expr,
    pub(crate) chainable_set: bool,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) span: Span,
    pub(crate) member: &'a Member,
    pub(crate) vis: Cow<'a, Visibility>,
}
impl<'a> ResolvedSet<'a> {
    pub(crate) fn build(&self) -> TokenStream2 {
        let ResolvedSet {
            argument_ident,
            argument_ty,
            assigned_value,
            chainable_set,
            doc,
            fn_ident,
            span,
            member,
            vis,
        } = self;

        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        if *chainable_set {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #argument_ty) -> &mut Self {
                    self.#member = #assigned_value;
                    self
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #argument_ty) {
                    self.#member = #assigned_value;
                }
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let member = query.member();
        let argument_ident = query.argument_ident()?;
        let chainable_set = query.chainable_set();
        let (argument_ty, assigned_value) =
            query.determine_argument_ty_and_assigned_value(&argument_ident)?;
        let argument_ty = argument_ty?;
        let doc = query.docs(false);

        Some(Self {
            argument_ident,
            argument_ty,
            assigned_value,
            chainable_set,
            doc,
            fn_ident,
            span,
            member,
            vis,
        })
    }
}
