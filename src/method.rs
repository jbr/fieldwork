use proc_macro2::Span;
use syn::{Error, Path, spanned::Spanned};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub(crate) enum Method {
    Get,
    Set,
    With,
    GetMut,
}

impl Method {
    pub(crate) const fn all() -> &'static [Method] {
        &[Self::Get, Self::GetMut, Self::Set, Self::With]
    }

    pub(crate) fn from_str_with_span(s: &str, span: Span) -> syn::Result<Self> {
        match s {
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            "with" => Ok(Self::With),
            "get_mut" => Ok(Self::GetMut),
            _ => Err(Error::new(span, "unrecognized method")),
        }
    }
}

impl TryFrom<&Path> for Method {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_ident("get") {
            Ok(Self::Get)
        } else if path.is_ident("set") {
            Ok(Self::Set)
        } else if path.is_ident("with") {
            Ok(Self::With)
        } else if path.is_ident("get_mut") {
            Ok(Self::GetMut)
        } else {
            Err(Error::new(path.span(), "unrecognized method"))
        }
    }
}
