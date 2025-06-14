use syn::{GenericArgument, Path, PathArguments, Type, TypePath, TypeReference};

pub(crate) fn extract_option_type(ty: &Type) -> Option<&Type> {
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    else {
        return None;
    };

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
    Some(strip_ref(inner_type))
}

pub(crate) fn strip_option_and_ref(ty: &Type) -> &Type {
    strip_ref(extract_option_type(ty).unwrap_or(ty))
}

pub(crate) fn strip_ref(ty: &Type) -> &Type {
    if let Type::Reference(TypeReference { elem, .. }) = &ty {
        elem
    } else {
        ty
    }
}
