use proc_macro2::Span;
use syn::{Expr, ExprLit, Field as SynField, Ident, Index, Lit, Member, Type, spanned::Spanned};

use crate::{FieldAttributes, is_fieldwork_attr};

// this represents a field within a struct that Access has been derived for
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct Field {
    pub(crate) member: Member,
    pub(crate) span: Span,
    pub(crate) ty: Type,
    pub(crate) attributes: FieldAttributes,
    pub(crate) doc: Vec<String>,
    /// The variant this field belongs to, for enum fields. `None` for struct fields.
    pub(crate) variant_ident: Option<Ident>,
}

impl Field {
    /// The binding name used in match patterns for this field: the explicit rename if set,
    /// otherwise the field's natural ident. Panics for unnamed fields without a rename.
    pub(crate) fn binding(&self) -> &Ident {
        self.attributes
            .fn_ident
            .as_ref()
            .unwrap_or_else(|| match &self.member {
                Member::Named(ident) => ident,
                Member::Unnamed(_) => unreachable!("unnamed field without fn_ident"),
            })
    }

    pub(crate) fn build(field: &SynField, index: usize) -> syn::Result<Field> {
        let member = match field.ident.clone() {
            Some(ident) => Member::Named(ident),
            None => Member::Unnamed(Index {
                index: u32::try_from(index)
                    .map_err(|err| syn::Error::new(field.span(), err.to_string()))?,
                span: field.span(),
            }),
        };

        let ty = field.ty.clone();

        let span = member.span().join(ty.span()).unwrap_or(ty.span());

        let doc = field
            .attrs
            .iter()
            .filter(|doc| doc.path().is_ident("doc"))
            .filter_map(|doc| match &doc.meta.require_name_value().unwrap().value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => Some(s.value().trim().to_string()),
                _ => None,
            })
            .collect();

        let attributes =
            FieldAttributes::build(field.attrs.iter().find(|attr| is_fieldwork_attr(attr)))?;

        Ok(Field {
            member,
            span,
            ty,
            attributes,
            doc,
            variant_ident: None,
        })
    }
}
