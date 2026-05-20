use crate::Query;
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::borrow::Cow;
use syn::{Attribute, Ident, Member, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct IntoField<'a> {
    doc: Option<Cow<'a, str>>,
    fn_ident: Cow<'a, Ident>,
    span: Span,
    ty: Type,
    member: &'a Member,
    vis: Cow<'a, Visibility>,
    deprecation_attr: Option<Attribute>,
}

impl<'a> IntoField<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        let IntoField {
            doc,
            fn_ident,
            span,
            ty,
            member,
            vis,
            deprecation_attr,
        } = self;
        let doc = doc.as_deref().map(|d| quote_spanned!(*span => #[doc = #d]));

        quote_spanned! {*span=>
            #doc
            #deprecation_attr
            #vis fn #fn_ident(self) -> #ty {
                self.#member
            }
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        if query.is_get_copy(query.ty()) {
            return None;
        }

        let span = query.span();
        let vis = query.vis();
        let fn_ident = query.fn_ident()?;
        let member = query.member();
        let ty = query.ty().clone();
        let doc = query.docs(false);
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
