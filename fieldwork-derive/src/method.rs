use proc_macro2::Span;
use syn::{Error, Path, spanned::Spanned};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[repr(u8)]
pub(crate) enum Method {
    Get,
    Set,
    With,
    GetMut,
    Without,
    Take,
    IntoField,
}

macro_rules! with_methods {
    ($macro:ident!($($inner:literal,)*)) => {
        $macro!($($inner,)* "get", "set", "with", "get_mut", "without", "take", "into_field", )
    };
}
pub(crate) use with_methods;

impl Method {
    pub(crate) const fn all() -> &'static [Method] {
        &[
            Self::Get,
            Self::GetMut,
            Self::Set,
            Self::With,
            Self::Without,
            Self::Take,
            Self::IntoField,
        ]
    }

    pub(crate) fn from_str_with_span(s: &str, span: Span) -> syn::Result<Self> {
        match s {
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            "with" => Ok(Self::With),
            "get_mut" => Ok(Self::GetMut),
            "without" => Ok(Self::Without),
            "take" => Ok(Self::Take),
            "into_field" => Ok(Self::IntoField),
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
        } else if path.is_ident("without") {
            Ok(Self::Without)
        } else if path.is_ident("take") {
            Ok(Self::Take)
        } else if path.is_ident("into_field") {
            Ok(Self::IntoField)
        } else {
            Err(Error::new(path.span(), "unrecognized method"))
        }
    }
}

#[derive(Debug)]
pub(crate) struct MethodSettings<T>([Option<T>; 7]);

impl<T> Default for MethodSettings<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> MethodSettings<T> {
    fn iter(&self) -> impl Iterator<Item = Option<&T>> {
        self.0.iter().map(|x| x.as_ref())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.iter().all(|x| x.is_none())
    }

    pub(crate) fn contains(&self, method: Method) -> bool {
        self.0[method as usize].is_some()
    }

    pub(crate) fn insert(&mut self, method: Method, setting: T) {
        self.0[method as usize] = Some(setting);
    }

    pub(crate) fn retrieve(&self, method: Method) -> Option<&T> {
        self.0[method as usize].as_ref()
    }
}
