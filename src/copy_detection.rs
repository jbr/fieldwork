use syn::{Path, Type, TypePath, TypeReference};

use crate::Method;

pub(crate) fn enable_copy_for_type(ty: &Type, method: Method) -> bool {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            let Some(last_segment) = segments.last() else {
                return false;
            };

            let ident = &last_segment.ident;

            ident == "char"
                || ident == "f32"
                || ident == "f64"
                || ident == "i128"
                || ident == "i16"
                || ident == "i32"
                || ident == "i8"
                || ident == "isize"
                || ident == "u128"
                || ident == "u16"
                || ident == "u32"
                || ident == "u8"
                || ident == "usize"
                || ident == "bool"
        }

        Type::Reference(TypeReference {
            mutability: None, ..
        }) => true,

        Type::Reference(TypeReference {
            mutability: Some(_),
            ..
        }) => method == Method::GetMut,

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
