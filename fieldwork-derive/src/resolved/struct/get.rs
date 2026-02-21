use crate::Query;
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Expr, Ident, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Get<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    ty: Type,
    expr: Expr,
    vis: Cow<'a, Visibility>,
}

impl<'a> Get<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Get {
            doc,
            fn_ident,
            span,
            ty,
            vis,
            expr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        quote_spanned! {*span=>
            #doc
            #vis fn #fn_ident(&self) -> #ty {
                #expr
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;

        let (expr, ty, is_get_copy) = query.get_access_expr_type_and_copy();

        let doc = query.docs(is_get_copy);

        Some(Self {
            doc,
            fn_ident,
            span,
            ty,
            expr,
            vis,
        })
    }
}
