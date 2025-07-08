use proc_macro2::Span;
use syn::{
    GenericArgument, Path, PathArguments, Type, TypeArray, TypePath, TypeReference,
    TypeTraitObject, parse_quote_spanned,
};

use crate::Method;

pub(crate) fn auto_deref(ty: &Type, method: Method, span: Span) -> Option<(Type, usize)> {
    let mut ty = ty.clone();
    let mut count = 0;

    while let Some(next_ty) = auto_deref_inner(&ty, method, span) {
        ty = next_ty;
        count += 1;
    }

    match &ty {
        Type::TraitObject(TypeTraitObject { bounds, .. }) if bounds.len() > 1 => {
            ty = parse_quote_spanned! {span => (#ty)};
        }
        _ => (),
    }

    if count > 0 { Some((ty, count)) } else { None }
}

pub(crate) fn auto_deref_inner(ty: &Type, method: Method, span: Span) -> Option<Type> {
    let segments = match ty {
        Type::Reference(TypeReference {
            mutability: Some(_),
            elem,
            ..
        }) => return Some(*elem.clone()),
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments,
        Type::Array(TypeArray { elem, .. }) => return Some(parse_quote_spanned!(span => [#elem])),
        _ => {
            return None;
        }
    };

    let last_segment = segments.last()?;
    if last_segment.ident == "String" {
        return Some(parse_quote_spanned!(span => str));
    }

    if last_segment.ident == "PathBuf" {
        return Some(parse_quote_spanned!(span => std::path::Path));
    }

    if last_segment.ident == "OsString" {
        return Some(parse_quote_spanned!(span => std::ffi::OsStr));
    }

    if last_segment.ident == "Vec" {
        let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
            return None;
        };

        let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
            return None;
        };

        return Some(parse_quote_spanned!(span => [#inner_type]));
    }

    if last_segment.ident == "Box" {
        let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
            return None;
        };

        let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
            return None;
        };

        return Some(inner_type.clone());
    }

    if method == Method::Get {
        if last_segment.ident == "Arc" || last_segment.ident == "Rc" {
            let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
                return None;
            };

            let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
                return None;
            };

            return Some(inner_type.clone());
        }

        if last_segment.ident == "Cow" {
            let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
                return None;
            };

            let Some(GenericArgument::Lifetime(_)) = bracketed_args.args.first() else {
                return None;
            };

            let Some(GenericArgument::Type(t)) = bracketed_args.args.get(1) else {
                return None;
            };

            return Some(t.clone());
        }
    }

    None
}
