use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

use crate::Query;

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct GetMut<'a> {
    constness: Option<Token![const]>,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) span: Span,
    pub(crate) ty: Type,
    pub(crate) vis: Cow<'a, Visibility>,
    pub(crate) access_expr: Expr,
    pub(crate) deprecation_attr: Option<Attribute>,
}

impl<'a> GetMut<'a> {
    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let span = query.span();
        let constness = query.constness();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let doc = query.docs(false);

        let (access_expr, ty) = query.mut_access_expr_and_type();
        let deprecation_attr = query.deprecation_attr();

        Some(Self {
            constness,
            doc,
            fn_ident,
            span,
            ty,
            vis,
            access_expr,
            deprecation_attr,
        })
    }

    pub(crate) fn build(&self) -> TokenStream {
        let GetMut {
            constness,
            doc,
            fn_ident,
            span,
            ty,
            vis,
            access_expr,
            deprecation_attr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));
        quote_spanned! {*span=>
            #doc
            #deprecation_attr
            #vis #constness fn #fn_ident(&mut self) -> #ty {
                #access_expr
            }
        }
    }
}
