use proc_macro2::TokenStream;

mod r#enum;
mod r#struct;

use crate::{Method, Query};

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) enum Resolved<'a> {
    StructGet(r#struct::Get<'a>),
    StructSet(r#struct::Set<'a>),
    StructWith(r#struct::With<'a>),
    StructWithout(r#struct::Without<'a>),
    StructGetMut(r#struct::GetMut<'a>),
    StructTake(r#struct::Take<'a>),
    StructIntoField(r#struct::IntoField<'a>),
    EnumGet(r#enum::Get<'a>),
    EnumSet(r#enum::Set<'a>),
    EnumWith(r#enum::With<'a>),
    EnumWithout(r#enum::Without<'a>),
    EnumGetMut(r#enum::GetMut<'a>),
    EnumTake(r#enum::Take<'a>),
    EnumIntoField(r#enum::IntoField<'a>),
}

impl<'a> Resolved<'a> {
    pub(crate) fn build(&self) -> TokenStream {
        match self {
            Resolved::StructGet(r) => r.build(),
            Resolved::StructSet(r) => r.build(),
            Resolved::StructWith(r) => r.build(),
            Resolved::StructWithout(r) => r.build(),
            Resolved::StructGetMut(r) => r.build(),
            Resolved::StructTake(r) => r.build(),
            Resolved::StructIntoField(r) => r.build(),
            Resolved::EnumGet(r) => r.build(),
            Resolved::EnumSet(r) => r.build(),
            Resolved::EnumWith(r) => r.build(),
            Resolved::EnumWithout(r) => r.build(),
            Resolved::EnumGetMut(r) => r.build(),
            Resolved::EnumTake(r) => r.build(),
            Resolved::EnumIntoField(r) => r.build(),
        }
    }

    pub(crate) fn from_query(query: &Query<'a>) -> Option<Self> {
        if !query.enabled() {
            return None;
        }
        if query.virtual_field().is_some() {
            match query.method() {
                Method::Get => r#enum::Get::from_query(query).map(Self::EnumGet),
                Method::Set => r#enum::Set::from_query(query).map(Self::EnumSet),
                Method::With => r#enum::With::from_query(query).map(Self::EnumWith),
                Method::GetMut => r#enum::GetMut::from_query(query).map(Self::EnumGetMut),
                Method::Without => r#enum::Without::from_query(query).map(Self::EnumWithout),
                Method::Take => r#enum::Take::from_query(query).map(Self::EnumTake),
                Method::IntoField => r#enum::IntoField::from_query(query).map(Self::EnumIntoField),
            }
        } else {
            match query.method() {
                Method::Get => r#struct::Get::from_query(query).map(Self::StructGet),
                Method::Set => r#struct::Set::from_query(query).map(Self::StructSet),
                Method::With => r#struct::With::from_query(query).map(Self::StructWith),
                Method::GetMut => r#struct::GetMut::from_query(query).map(Self::StructGetMut),
                Method::Without => r#struct::Without::from_query(query).map(Self::StructWithout),
                Method::Take => r#struct::Take::from_query(query).map(Self::StructTake),
                Method::IntoField => {
                    r#struct::IntoField::from_query(query).map(Self::StructIntoField)
                }
            }
        }
    }
}
