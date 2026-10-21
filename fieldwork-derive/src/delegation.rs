//! Re-emitting a `#[fieldwork(delegatable)]` struct's methods on a type that holds it in a
//! `#[field(delegate)]` field.
//!
//! Proc macros can't see other items, so the two derives communicate through name resolution.
//! The delegatable struct's derive emits a hidden `macro_rules!` holding its methods, with the
//! delegate field left as a macro parameter, and re-exports it in the macro namespace under the
//! struct's own name. The delegating type's derive invokes that macro through the same path that
//! names the field's type, passing its impl header and the delegate field. Expansion order
//! doesn't matter, because rustc retries unresolved macro invocations as other expansions
//! define new names.

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use syn::{Error, GenericArgument, Generics, Ident, Path, PathArguments, Type, TypePath};

use crate::Field;

/// A `#[field(delegate)]` or `#[field(delegate = path::Inner)]` annotation.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct Delegate {
    /// The delegatable struct, when named explicitly. Otherwise inferred from the field's type.
    pub(crate) path: Option<Path>,
    pub(crate) span: Span,
}

impl Delegate {
    /// The path of the delegatable struct's macro, which shares the struct's name.
    fn macro_path(&self, ty: &Type) -> syn::Result<Path> {
        let mut path = match &self.path {
            Some(path) => path.clone(),
            None => inferred_path(ty).cloned().ok_or_else(|| {
                Error::new_spanned(
                    ty,
                    "cannot determine which struct this field delegates to; \
                     name it with `#[field(delegate = path::to::Struct)]`",
                )
            })?,
        };
        for segment in &mut path.segments {
            segment.arguments = PathArguments::None;
        }
        Ok(path)
    }
}

/// The path of the struct held in a field of type `ty`, looking through `Box`.
fn inferred_path(ty: &Type) -> Option<&Path> {
    let Type::Path(TypePath {
        qself: None, path, ..
    }) = ty
    else {
        return None;
    };
    let last = path.segments.last()?;
    if last.ident == "Box"
        && let PathArguments::AngleBracketed(args) = &last.arguments
        && let Some(GenericArgument::Type(inner)) = args.args.first()
    {
        return inferred_path(inner);
    }
    Some(path)
}

/// The hidden delegation macro for the `#[fieldwork(delegatable)]` struct `ident`, and its
/// re-export under the struct's name.
///
/// `render` produces the struct's methods given an ident to reach the struct through from the
/// delegating type's `self`. Method bodies are built as `syn::Expr`s, which can't contain the
/// macro's `$via` metavariable, so `render` is handed a placeholder that is substituted afterward.
pub(crate) fn delegation_macro(
    ident: &Ident,
    render: impl FnOnce(&Ident) -> TokenStream,
) -> TokenStream {
    let placeholder = Ident::new("__fieldwork_via", Span::call_site());
    let methods = substitute(render(&placeholder), &placeholder);
    let macro_ident = format_ident!("__fieldwork_delegate_{}", ident);
    quote! {
        #[doc(hidden)]
        macro_rules! #macro_ident {
            ([$($impl_generics:tt)*] [$($self_ty:tt)*] [$($where_clause:tt)*] [$($via:tt)*]) => {
                impl $($impl_generics)* $($self_ty)* $($where_clause)* {
                    #methods
                }
            };
        }
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub(crate) use #macro_ident as #ident;
    }
}

/// Replaces each `placeholder` ident in `tokens` with `$($via)*`.
fn substitute(tokens: TokenStream, placeholder: &Ident) -> TokenStream {
    tokens
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Ident(ident) if ident == *placeholder => quote!($($via)*),
            TokenTree::Group(group) => {
                let mut substituted =
                    Group::new(group.delimiter(), substitute(group.stream(), placeholder));
                substituted.set_span(group.span());
                TokenTree::Group(substituted).into()
            }
            tree => tree.into(),
        })
        .collect()
}

/// Invocations of the delegation macro for each `#[field(delegate)]` field of the struct `ident`.
pub(crate) fn invocations(
    ident: &Ident,
    generics: &Generics,
    fields: &[Field],
) -> syn::Result<TokenStream> {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    fields
        .iter()
        .filter_map(|field| Some((field, field.attributes.delegate.as_ref()?)))
        .map(|(field, delegate)| {
            let path = delegate.macro_path(&field.ty)?;
            let member = &field.member;
            Ok(quote_spanned! {delegate.span=>
                #path! { [#impl_generics] [#ident #type_generics] [#where_clause] [#member] }
            })
        })
        .collect()
}
