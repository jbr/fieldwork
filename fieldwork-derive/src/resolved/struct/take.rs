use crate::{Query, option_handling::extract_option_type};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Attribute, Ident, Member, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Take<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    ty: &'a Type,
    member: &'a Member,
    vis: Cow<'a, Visibility>,
    deprecation_attr: Option<Attribute>,
}

impl<'a> Take<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let Take {
            doc,
            fn_ident,
            span,
            ty,
            vis,
            member,
            deprecation_attr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        quote_spanned! {*span=>
            #doc
            #deprecation_attr
            #vis fn #fn_ident(&mut self) -> #ty {
                self.#member.take()
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        let ty = query.ty();
        extract_option_type(ty)?;

        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let doc = query.docs(false);
        let member = query.member();
        let deprecation_attr = query.deprecation_attr();

        Some(Self {
            doc,
            fn_ident,
            span,
            ty,
            member,
            vis,
            deprecation_attr,
        })
    }
}
