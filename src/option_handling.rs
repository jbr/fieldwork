use std::borrow::Cow;

use syn::{GenericArgument, Path, PathArguments, Type, TypePath, TypeReference};

pub(crate) fn extract_option_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            let last_segment = segments.last()?;
            if last_segment.ident != "Option" {
                return None;
            }

            let PathArguments::AngleBracketed(ref bracketed_args) = last_segment.arguments else {
                return None;
            };

            let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first() else {
                return None;
            };
            let Cow::Borrowed(b) = strip_ref(Cow::Borrowed(inner_type)) else {
                return None;
            };

            return Some(b);
        }

        _ => None,
    }
}

pub(crate) fn strip_ref(ty: Cow<'_, Type>) -> Cow<'_, Type> {
    match ty {
        Cow::Borrowed(Type::Reference(TypeReference { elem, .. })) => Cow::Borrowed(elem),
        Cow::Owned(Type::Reference(TypeReference { elem, .. })) => Cow::Owned(*elem),
        _ => ty,
    }
}
