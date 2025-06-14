use crate::{Method, query::OptionHandling};
use Method::{Get, GetMut, Set, With};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::borrow::Cow;
use syn::{Ident, Type, Visibility};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Resolved<'a> {
    pub(crate) method: Method,
    pub(crate) vis: Cow<'a, Visibility>,
    pub(crate) fn_ident: Cow<'a, Ident>,
    pub(crate) variable_ident: Cow<'a, Ident>,
    pub(crate) argument_ident: Cow<'a, Ident>,
    pub(crate) ty: &'a Type,
    pub(crate) doc: Option<Cow<'a, str>>,
    pub(crate) get_copy: bool,
    pub(crate) chainable_set: bool,
    pub(crate) deref_type: Option<Cow<'a, Type>>,
    pub(crate) option_handling: Option<OptionHandling<'a>>,
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
            option_handling,
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
        } else if let Some(oh) = option_handling {
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
            ty,
            doc,
            chainable_set,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if *chainable_set {
            quote! {
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #ty) -> &mut Self {
                    self.#variable_ident = #argument_ident;
                    self
                }
            }
        } else {
            quote! {
                #doc
                #vis fn #fn_ident(&mut self, #argument_ident: #ty) {
                    self.#variable_ident = #argument_ident;
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
            option_handling,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));

        if let Some(oh) = option_handling {
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
            ty,
            doc,
            ..
        } = self;
        let doc = doc.as_deref().map(|d| quote!(#[doc = #d]));
        quote! {
            #doc
            #[must_use]
            #vis fn #fn_ident(mut self, #argument_ident: #ty) -> Self {
                self.#variable_ident = #argument_ident;
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
