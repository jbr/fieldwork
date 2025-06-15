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
