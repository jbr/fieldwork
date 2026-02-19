use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Expr, Ident, Type, Visibility};

use crate::Query;

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct ResolvedGetMut<'a> {
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) span: Span,
    pub(crate) ty: Type,
    pub(crate) vis: Cow<'a, Visibility>,
    pub(crate) access_expr: Expr,
}

impl<'a> ResolvedGetMut<'a> {
    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let doc = query.docs(false);

        let (access_expr, ty) = query.mut_access_expr_and_type();

        Some(Self {
            doc,
            fn_ident,
            span,
            ty,
            vis,
            access_expr,
        })
    }

    pub(crate) fn build(&self) -> TokenStream {
        let ResolvedGetMut {
            doc,
            fn_ident,
            span,
            ty,
            vis,
            access_expr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        quote_spanned! {*span=>
            #doc
            #vis fn #fn_ident(&mut self) -> #ty {
                #access_expr
            }
        }
    }
}
