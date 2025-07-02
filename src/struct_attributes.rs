use crate::{CommonSettings, Method, StructMethodAttributes, errors::invalid_key};
use proc_macro2::Span;
use std::collections::HashSet;
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Lit, LitBool, LitStr, Meta,
    WhereClause, WherePredicate,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Where},
};

// this represents the configuration passed to #[fieldwork]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
pub(crate) struct StructAttributes {
    pub(crate) methods: Vec<StructMethodAttributes>,
    pub(crate) include: HashSet<Method>,
    pub(crate) where_clause: Option<WhereClause>,

    pub(crate) common_settings: CommonSettings,
}

impl StructAttributes {
    const VALID_KEYS: &[&str] = &[
        "chain",
        "copy",
        "deref",
        "into",
        "opt_in",
        "option",
        "option_borrow_inner",
        "option_set_some",
        "rename_predicate",
        "rename_predicates",
        "vis",
        "where_clause",
        "bounds",
        "get",
        "set",
        "with",
        "get_mut",
    ];

    pub(crate) fn build(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut struct_attributes = Self::default();
        let Some(attr) = attributes.iter().find(|x| x.path().is_ident("fieldwork")) else {
            return Ok(struct_attributes);
        };
        let Meta::List(list) = &attr.meta else {
            return Err(Error::new(attr.span(), "unexpected attribute format"));
        };

        struct_attributes
            .handle_list(&list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)?)?;

        Ok(struct_attributes)
    }

    fn handle_list(&mut self, list: &Punctuated<Expr, Comma>) -> syn::Result<()> {
        for expr in list {
            match expr {
                Expr::Assign(assign) => self.handle_assign(assign)?,

                Expr::Path(ExprPath { path, .. }) => self.handle_assign_bool_lit(
                    path.span(),
                    &path.require_ident()?.to_string(),
                    true,
                )?,

                Expr::Call(ExprCall { func, args, .. }) => match &**func {
                    Expr::Path(ExprPath { path: method, .. }) => match Method::try_from(method) {
                        Ok(method) => {
                            self.include.insert(method);
                            self.methods
                                .push(StructMethodAttributes::build(method, args)?);
                        }

                        Err(_) => {
                            return Err(invalid_key(
                                func.span(),
                                &method.require_ident()?.to_string(),
                                Self::VALID_KEYS,
                            ));
                        }
                    },

                    _ => return Err(Error::new(expr.span(), "not recognized call")),
                },

                expr => return Err(Error::new(expr.span(), "not recognized")),
            }
        }

        Ok(())
    }

    fn handle_assign(&mut self, assign: &ExprAssign) -> syn::Result<()> {
        let ExprAssign { left, right, .. } = assign;
        let span = assign.span();
        let lhs = if let Expr::Path(ExprPath { path: lhs, .. }) = &**left {
            lhs.get_ident().map(ToString::to_string)
        } else {
            None
        }
        .ok_or_else(|| Error::new(span, "not recognized"))?;
        match &**right {
            Expr::Lit(ExprLit {
                lit: Lit::Str(rhs), ..
            }) => self.handle_assign_str_lit(span, &lhs, rhs),

            //            Expr::Path(ExprPath { path: rhs, .. }) => self.handle_assign_path(span, &lhs, rhs),
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),
            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_str_lit(&mut self, span: Span, lhs: &str, rhs: &LitStr) -> Result<(), Error> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "where_clause" | "bounds" => {
                    self.where_clause = Some(WhereClause {
                        predicates: rhs
                            .parse_with(Punctuated::<WherePredicate, Comma>::parse_terminated)?,
                        where_token: Where::default(),
                    });
                }
                _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
            }
        }
        Ok(())
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> Result<(), Error> {
        if self.common_settings.handle_assign_bool_lit(lhs, value) {
            Ok(())
        } else if value {
            let method = Method::from_str_with_span(lhs, span)
                .map_err(|_| invalid_key(span, lhs, Self::VALID_KEYS))?;
            self.include.insert(method);
            self.methods.push(StructMethodAttributes::new(method));
            Ok(())
        } else {
            Err(invalid_key(span, lhs, Self::VALID_KEYS))
        }
    }
}
