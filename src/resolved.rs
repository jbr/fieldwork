use crate::{Method, query::OptionHandling};
use Method::{Get, GetMut, Set, With, Without};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use syn::{Expr, Ident, Member, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Resolved<'a> {
    pub(crate) argument_ident_and_ty: Option<(Cow<'a, Ident>, Cow<'a, Type>)>,
    pub(crate) assigned_value: Expr,
    pub(crate) chainable_set: bool,
    pub(crate) deref_type: Option<Cow<'a, Type>>,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) get_copy: bool,
    pub(crate) method: Method,
    pub(crate) option_borrow_inner: Option<OptionHandling<'a>>,
    pub(crate) span: Span,
    pub(crate) ty: &'a Type,
    pub(crate) variable_ident: &'a Member,
    pub(crate) vis: Cow<'a, Visibility>,
}

impl Resolved<'_> {
    pub(crate) fn build_get(&self) -> proc_macro2::TokenStream {
        let Resolved {
            vis,
            fn_ident,
            variable_ident,
            ty,
            doc,
            get_copy,
            deref_type,
            option_borrow_inner,
            span,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if *get_copy {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&self) -> #ty {
                    self.#variable_ident
                }
            }
        } else if let Some(oh) = option_borrow_inner {
            match oh {
                OptionHandling::Deref(ty) => quote_spanned! {*span=>
                    #doc
                    #vis fn #fn_ident(&self) -> Option<&#ty> {
                        self.#variable_ident.as_deref()
                    }
                },
                OptionHandling::Ref(ty) => quote_spanned! {*span=>
                    #doc
                    #vis fn #fn_ident(&self) -> Option<&#ty> {
                        self.#variable_ident.as_ref()
                    }
                },
            }
        } else if let Some(deref) = deref_type {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&self) -> &#deref {
                    &*self.#variable_ident
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&self) -> &#ty {
                    &self.#variable_ident
                }
            }
        }
    }

    fn build_set(&self) -> TokenStream2 {
        let Resolved {
            vis,
            fn_ident,
            variable_ident,
            doc,
            chainable_set,
            assigned_value,
            argument_ident_and_ty,
            span,
            ..
        } = self;

        let Some((argument_ident, argument_ty)) = argument_ident_and_ty else {
            return quote!();
        };

        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if *chainable_set {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #argument_ty) -> &mut Self {
                    self.#variable_ident = #assigned_value;
                    self
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #argument_ty) {
                    self.#variable_ident = #assigned_value;
                }
            }
        }
    }

    fn build_get_mut(&self) -> TokenStream2 {
        let Resolved {
            vis,
            fn_ident,
            variable_ident,
            ty,
            doc,
            deref_type,
            option_borrow_inner,
            span,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if let Some(oh) = option_borrow_inner {
            match oh {
                OptionHandling::Deref(ty) => quote_spanned! {*span=>
                    #doc
                    #vis fn #fn_ident(&mut self) -> Option<&mut #ty> {
                        self.#variable_ident.as_deref_mut()
                    }
                },
                OptionHandling::Ref(ty) => quote_spanned! {*span=>
                    #doc
                    #vis fn #fn_ident(&mut self) -> Option<&mut #ty> {
                        self.#variable_ident.as_mut()
                    }
                },
            }
        } else if let Some(deref) = deref_type {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self) -> &mut #deref {
                    &mut *self.#variable_ident
                }
            }
        } else {
            quote_spanned! {*span=>
                #doc
                #vis fn #fn_ident(&mut self) -> &mut #ty {
                    &mut self.#variable_ident
                }
            }
        }
    }

    fn build_with(&self) -> TokenStream2 {
        let Resolved {
            vis,
            fn_ident,
            variable_ident,
            argument_ident_and_ty,
            doc,
            assigned_value,
            span,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));
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

    pub(crate) fn build(&self) -> TokenStream2 {
        match self.method {
            Get => self.build_get(),
            Set => self.build_set(),
            GetMut => self.build_get_mut(),
            With => self.build_with(),
            Without => self.build_without(),
        }
    }

    fn build_without(&self) -> TokenStream2 {
        let Resolved {
            vis,
            fn_ident,
            variable_ident,
            doc,
            assigned_value,
            span,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

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
