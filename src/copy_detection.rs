use syn::{Path, Type, TypePath};

pub(crate) fn enable_copy_for_type(ty: &Type) -> bool {
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    else {
        return false;
    };

    let Some(last_segment) = segments.last() else {
        return false;
    };

    let ident = &last_segment.ident;

    ident == "bool"
}
