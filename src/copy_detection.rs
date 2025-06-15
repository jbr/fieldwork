use syn::{Path, Type, TypePath, TypeReference};

pub(crate) fn enable_copy_for_type(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            let Some(last_segment) = segments.last() else {
                return false;
            };

            let ident = &last_segment.ident;

            ident == "bool" || ident == "usize" || ident == "u8"
        }

        Type::Reference(TypeReference {
            mutability: None, ..
        }) => true,

        _ => false,
    }
}

pub(crate) fn is_type(ty: &Type, type_ident: &'static str) -> bool {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        let Some(last_segment) = segments.last() else {
            return false;
        };

        let ident = &last_segment.ident;

        ident == type_ident
    } else {
        false
    }
}
