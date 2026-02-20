use proc_macro2::TokenStream;

mod get;
mod get_mut;
mod into_field;
mod set;
mod take;
mod with;
mod without;

use get::ResolvedGet;
use get_mut::ResolvedGetMut;
use into_field::ResolvedIntoField;
use set::ResolvedSet;
use take::ResolvedTake;
use with::ResolvedWith;
use without::ResolvedWithout;

use crate::{Method, Query};

impl<'a> Resolved<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        match self {
            Resolved::Get(resolved) => resolved.build(),
            Resolved::Set(resolved) => resolved.build(),
            Resolved::With(resolved) => resolved.build(),
            Resolved::Without(resolved) => resolved.build(),
            Resolved::GetMut(resolved) => resolved.build(),
            Resolved::Take(resolved) => resolved.build(),
            Resolved::IntoField(resolved) => resolved.build(),
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        if !query.enabled() {
            return None;
        }
        match query.method() {
            Method::Get => ResolvedGet::from_query(query).map(Self::Get),
            Method::Set => ResolvedSet::from_query(query).map(Self::Set),
            Method::With => ResolvedWith::from_query(query).map(Self::With),
            Method::GetMut => ResolvedGetMut::from_query(query).map(Self::GetMut),
            Method::Without => ResolvedWithout::from_query(query).map(Self::Without),
            Method::Take => ResolvedTake::from_query(query).map(Self::Take),
            Method::IntoField => ResolvedIntoField::from_query(query).map(Self::IntoField),
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) enum Resolved<'a> {
    Get(ResolvedGet<'a>),
    Set(ResolvedSet<'a>),
    With(ResolvedWith<'a>),
    Without(ResolvedWithout<'a>),
    GetMut(ResolvedGetMut<'a>),
    Take(ResolvedTake<'a>),
    IntoField(ResolvedIntoField<'a>),
}
