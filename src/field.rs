use syn::{Error, Expr, ExprLit, Field as SynField, Ident, Lit, Type, spanned::Spanned};

use crate::FieldAttributes;

// this represents a field within a struct that Access has been derived for
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Field {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) attributes: FieldAttributes,
    pub(crate) doc: Vec<String>,
}

impl Field {
    pub(crate) fn build(field: &SynField) -> syn::Result<Field> {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| Error::new(field.span(), "can only be used with named fields"))?;
        let ty = field.ty.clone();

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

        let attrs = FieldAttributes::build(
            field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("fieldwork")),
        )?;

        Ok(Field {
            ident,
            ty,
            doc,
            attributes: attrs,
        })
    }
}
