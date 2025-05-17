use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Field as SynField, Ident, Lit,
    Meta, Type, TypePath, Visibility, punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::{FieldMethodAttributes, Method};

// this represents the configuration for the field
#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct FieldAttributes {
    pub(crate) decorated: bool,
    pub(crate) skip: bool,
    pub(crate) fn_ident: Option<Ident>,
    pub(crate) vis: Option<Visibility>,
    pub(crate) argument_ident: Option<Ident>,
    pub(crate) method_attributes: Vec<FieldMethodAttributes>,
    pub(crate) deref: Option<Type>,
    pub(crate) opt_in: bool,
}

#[allow(clippy::too_many_lines, reason = "deferred for a later refactor")]
impl FieldAttributes {
    pub(crate) fn build(attribute: Option<&Attribute>) -> syn::Result<FieldAttributes> {
        let mut field_attributes = FieldAttributes::default();
        match attribute {
            Some(Attribute {
                meta: Meta::Path(_),
                ..
            }) => {
                field_attributes.decorated = true;
                Ok(field_attributes)
            }

            Some(Attribute {
                meta: Meta::List(list),
                ..
            }) => {
                field_attributes.decorated = true;
                for expr in list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)? {
                    match &expr {
                        Expr::Assign(ExprAssign { left, right, .. }) => match (&**left, &**right) {
                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Path(ExprPath { path: rhs, .. }),
                            ) if lhs.is_ident("rename") => {
                                field_attributes.fn_ident = Some(rhs.require_ident().cloned()?);
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Path(ExprPath { path: rhs, .. }),
                            ) if lhs.is_ident("argument") => {
                                field_attributes.argument_ident =
                                    Some(rhs.require_ident().cloned()?);
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Path(ExprPath { path: rhs, .. }),
                            ) if lhs.is_ident("deref") => {
                                field_attributes.deref = Some(Type::Path(TypePath {
                                    qself: None,
                                    path: rhs.clone(),
                                }));
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("vis") => field_attributes.vis = Some(rhs.parse()?),

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("rename") => {
                                field_attributes.fn_ident = Some(rhs.parse()?);
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("deref") => {
                                field_attributes.deref = Some(rhs.parse()?);
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("argument") => {
                                field_attributes.argument_ident = Some(rhs.parse()?);
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Path(ExprPath { path: rhs, .. }),
                            ) => {
                                field_attributes
                                    .method_attributes
                                    .push(FieldMethodAttributes {
                                        method: Method::try_from(lhs)?,
                                        fn_ident: Some(rhs.require_ident().cloned()?),
                                        skip: false,
                                        argument_ident: None,
                                        vis: None,
                                        doc: None,
                                        chainable_set: None,
                                        get_copy: None,
                                        deref: None,
                                    });
                            }

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) => {
                                field_attributes
                                    .method_attributes
                                    .push(FieldMethodAttributes {
                                        method: Method::try_from(lhs)?,
                                        fn_ident: Some(rhs.parse()?),
                                        skip: false,
                                        argument_ident: None,
                                        vis: None,
                                        doc: None,
                                        chainable_set: None,
                                        get_copy: None,
                                        deref: None,
                                    });
                            }
                            (_, _) => {
                                return Err(Error::new(expr.span(), "not recognized assign"));
                            }
                        },

                        Expr::Path(ExprPath { path, .. }) if path.is_ident("skip") => {
                            field_attributes.skip = true;
                        }
                        Expr::Path(ExprPath { path, .. }) if path.is_ident("opt_in") => {
                            field_attributes.opt_in = true;
                        }

                        Expr::Path(ExprPath { path, .. }) => {
                            field_attributes
                                .method_attributes
                                .push(FieldMethodAttributes {
                                    method: Method::try_from(path)?,
                                    fn_ident: None,
                                    skip: false,
                                    argument_ident: None,
                                    vis: None,
                                    doc: None,
                                    chainable_set: None,
                                    get_copy: None,
                                    deref: None,
                                });
                        }

                        Expr::Call(ExprCall { func, args, .. }) => match &**func {
                            Expr::Path(ExprPath { path: method, .. }) => {
                                let method = method.try_into()?;
                                field_attributes
                                    .method_attributes
                                    .push(FieldMethodAttributes::build(method, args)?);
                            }

                            _ => {
                                return Err(Error::new(expr.span(), "not recognized call"));
                            }
                        },

                        expr => {
                            return Err(Error::new(expr.span(), "not recognized"));
                        }
                    }
                }
                Ok(field_attributes)
            }
            None => Ok(field_attributes),
            Some(attribute) => Err(Error::new(attribute.span(), "not recognized")),
        }
    }
}
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
