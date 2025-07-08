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

    Some(inner_type)
}

pub(crate) fn option_type_mut(ty: &mut Type) -> Option<&mut Type> {
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    else {
        return None;
    };

    let last_segment = segments.last_mut()?;
    if last_segment.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(bracketed_args) = &mut last_segment.arguments else {
        return None;
    };

    let Some(GenericArgument::Type(inner_type)) = bracketed_args.args.first_mut() else {
        return None;
    };

    Some(inner_type)
}

pub(crate) fn strip_ref(ty: &Type) -> &Type {
    ref_inner(ty).unwrap_or(ty)
}

pub(crate) fn ref_inner(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Reference(TypeReference { elem, .. }) => Some(elem),
        _ => None,
    }
}

pub(crate) fn ref_inner_mut(ty: &mut Type) -> Option<(&mut Type, bool)> {
    match ty {
        Type::Reference(TypeReference {
            elem, mutability, ..
        }) => Some((elem, mutability.is_some())),
        _ => None,
    }
}
