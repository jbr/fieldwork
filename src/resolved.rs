use crate::{Method, query::OptionHandling};
use Method::{Get, GetMut, Set, With};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::borrow::Cow;
use syn::{Expr, Ident, Member, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Resolved<'a> {
    pub(crate) method: Method,
    pub(crate) vis: Cow<'a, Visibility>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) variable_ident: &'a Member,
    pub(crate) argument_ident: Cow<'a, Ident>,
    pub(crate) ty: &'a Type,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) get_copy: bool,
    pub(crate) chainable_set: bool,
    pub(crate) deref_type: Option<Cow<'a, Type>>,
    pub(crate) option_borrow_inner: Option<OptionHandling<'a>>,
    pub(crate) assigned_value: Expr,
    pub(crate) argument_ty: Cow<'a, Type>,
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
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if *get_copy {
            quote! {
                #doc
                #vis fn #fn_ident(&self) -> #ty {
                    self.#variable_ident
                }
            }
        } else if let Some(oh) = option_borrow_inner {
            match oh {
                OptionHandling::Deref(ty) => quote! {
                    #doc
                    #vis fn #fn_ident(&self) -> Option<&#ty> {
                        self.#variable_ident.as_deref()
                    }
                },
                OptionHandling::Ref(ty) => quote! {
                    #doc
                    #vis fn #fn_ident(&self) -> Option<&#ty> {
                        self.#variable_ident.as_ref()
                    }
                },
            }
        } else if let Some(deref) = deref_type {
            quote! {
                #doc
                #vis fn #fn_ident(&self) -> &#deref {
                    &*self.#variable_ident
                }
            }
        } else {
            quote! {
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
            argument_ident,
            doc,
            chainable_set,
            argument_ty,
            assigned_value,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if *chainable_set {
            quote! {
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #argument_ty) -> &mut Self {
                    self.#variable_ident = #assigned_value;
                    self
                }
            }
        } else {
            quote! {
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
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if let Some(oh) = option_borrow_inner {
            match oh {
                OptionHandling::Deref(ty) => quote! {
                    #doc
                    #vis fn #fn_ident(&mut self) -> Option<&mut #ty> {
                        self.#variable_ident.as_deref_mut()
                    }
                },
                OptionHandling::Ref(ty) => quote! {
                    #doc
                    #vis fn #fn_ident(&mut self) -> Option<&mut #ty> {
                        self.#variable_ident.as_mut()
                    }
                },
            }
        } else if let Some(deref) = deref_type {
            quote! {
                #doc
                #vis fn #fn_ident(&mut self) -> &mut #deref {
                    &mut *self.#variable_ident
                }
            }
        } else {
            quote! {
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
            argument_ident,
            doc,
            argument_ty,
            assigned_value,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));
        quote! {
            #doc
            #[must_use]
            #vis fn #fn_ident(mut self, #argument_ident: #argument_ty) -> Self {
                self.#variable_ident = #assigned_value;
                self
            }
        }
    }

    pub(crate) fn build(&self) -> TokenStream2 {
        match self.method {
            Get => self.build_get(),
            Set => self.build_set(),
            GetMut => self.build_get_mut(),
            With => self.build_with(),
        }
    }
}
