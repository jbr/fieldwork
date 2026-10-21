use crate::{Query, option_handling::extract_option_type};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Take<'a> {
    constness: Option<Token![const]>,
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    ty: &'a Type,
    place: Expr,
    vis: Cow<'a, Visibility>,
    deprecation_attr: Option<Attribute>,
}

impl<'a> Take<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Take {
            constness,
            doc,
            fn_ident,
            span,
            ty,
            vis,
            place,
            deprecation_attr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        quote_spanned! {*span=>
            #doc
            #deprecation_attr
            #[inline]
            #vis #constness fn #fn_ident(&mut self) -> #ty {
                #place.take()
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let ty = query.ty();
        extract_option_type(ty)?;

        let span = query.span();
        let constness = query.constness();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let doc = query.docs(false);
        let place = query.place();
        let deprecation_attr = query.deprecation_attr();

        Some(Self {
            constness,
            doc,
            fn_ident,
            span,
            ty,
            place,
            vis,
            deprecation_attr,
        })
    }
}
