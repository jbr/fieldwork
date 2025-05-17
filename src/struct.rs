use crate::{Method, StructMethodAttributes};
use std::collections::HashSet;
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Lit, Meta, Visibility,
    WhereClause, WherePredicate,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Where},
};

// this represents the configuration passed to #[fieldwork]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct StructAttributes {
    pub(crate) vis: Option<Visibility>,
    pub(crate) methods: Vec<StructMethodAttributes>,
    pub(crate) include: Option<HashSet<Method>>,
    pub(crate) where_clause: Option<WhereClause>,
    pub(crate) opt_in: bool,
}
impl StructAttributes {
    pub(crate) fn build(attributes: &[Attribute]) -> syn::Result<StructAttributes> {
        let mut vis = None;
        let mut methods = Vec::new();
        let mut include = Some(HashSet::new());
        let mut where_clause = None;
        let mut opt_in = false;
        if let Some(attr) = attributes.iter().find(|x| x.path().is_ident("fieldwork")) {
            if let Meta::List(list) = &attr.meta {
                for expr in list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)? {
                    match &expr {
                        Expr::Assign(ExprAssign { left, right, .. }) => match (&**left, &**right) {
                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("vis") => vis = Some(rhs.parse()?),

                            (
                                Expr::Path(ExprPath { path: lhs, .. }),
                                Expr::Lit(ExprLit {
                                    lit: Lit::Str(rhs), ..
                                }),
                            ) if lhs.is_ident("where_clause") || lhs.is_ident("bounds") => {
                                where_clause = Some(WhereClause {
                                    predicates: rhs.parse_with(
                                        Punctuated::<WherePredicate, Comma>::parse_terminated,
                                    )?,
                                    where_token: Where::default(),
                                });
                            }

                            _ => return Err(Error::new(expr.span(), "not recognized")),
                        },

                        Expr::Call(ExprCall { func, args, .. }) => match &**func {
                            Expr::Path(p) => {
                                let method = Method::try_from(&p.path)?;
                                include.get_or_insert_with(HashSet::default).insert(method);
                                methods.push(StructMethodAttributes::build(method, args)?);
                            }
                            _ => {
                                return Err(Error::new(expr.span(), "not recognized"));
                            }
                        },

                        Expr::Path(ExprPath { path, .. }) if path.is_ident("opt_in") => {
                            opt_in = true;
                        }

                        Expr::Path(ExprPath { path, .. }) => {
                            let method = Method::try_from(path)?;
                            include.get_or_insert_with(HashSet::default).insert(method);
                        }

                        expr => {
                            return Err(Error::new(expr.span(), "not recognized"));
                        }
                    }
                }
            } else {
                return Err(Error::new(attr.span(), "unexpected attribute format"));
            }
        }

        Ok(StructAttributes {
            vis,
            methods,
            include,
            where_clause,
            opt_in,
        })
    }
}
