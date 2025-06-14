use std::borrow::Cow;

use syn::{GenericArgument, Path, PathArguments, Type, TypePath};

pub(crate) fn auto_deref(ty: &Type) -> Option<Cow<'_, Type>> {
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    else {
        return None;
    };

    let last_segment = segments.last()?;
    if last_segment.ident == "String" {
        return Some(Cow::Owned(syn::parse_quote!(str)));
    }

    if last_segment.ident == "Vec" {
        let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
            return None;
        };

        let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
            return None;
        };

        return Some(Cow::Owned(syn::parse_quote!([#inner_type])));
    }

    if last_segment.ident == "Box" || last_segment.ident == "Arc" || last_segment.ident == "Rc" {
        let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
            return None;
        };

        let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
            return None;
        };

        return Some(Cow::Borrowed(inner_type));
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

        return Some(Cow::Borrowed(t));
    }

    None
}
