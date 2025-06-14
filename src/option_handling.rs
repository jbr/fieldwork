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

pub(crate) fn strip_ref(ty: &Type) -> &Type {
    match ty {
        Type::Reference(TypeReference { elem, .. }) => elem,
        _ => ty,
    }
}
