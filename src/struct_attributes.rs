use crate::{Method, StructMethodAttributes};
use proc_macro2::Span;
use std::collections::HashSet;
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Lit, LitBool, LitStr, Meta,
    Visibility, WhereClause, WherePredicate,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Where},
};

// this represents the configuration passed to #[fieldwork]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
pub(crate) struct StructAttributes {
    pub(crate) vis: Option<Visibility>,
    pub(crate) methods: Vec<StructMethodAttributes>,
    pub(crate) include: HashSet<Method>,
    pub(crate) where_clause: Option<WhereClause>,
    pub(crate) opt_in: bool,
    pub(crate) option_handling: Option<bool>,
}
impl StructAttributes {
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
                    Expr::Path(ExprPath { path: method, .. }) => {
                        let method = Method::try_from(method)?;
                        self.include.insert(method);
                        self.methods
                            .push(StructMethodAttributes::build(method, args)?);
                    }

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
        match lhs {
            "vis" => self.vis = Some(rhs.parse()?),
            "where_clause" | "bounds" => {
                self.where_clause = Some(WhereClause {
                    predicates: rhs
                        .parse_with(Punctuated::<WherePredicate, Comma>::parse_terminated)?,
                    where_token: Where::default(),
                });
            }
            _ => return Err(Error::new(span, "not recognized")),
        }
        Ok(())
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> Result<(), Error> {
        match lhs {
            "option" => self.option_handling = Some(value),
            "opt_in" => self.opt_in = value,
            other if value => {
                let method = Method::from_str_with_span(other, span)?;
                self.include.insert(method);
                self.methods.push(StructMethodAttributes::new(method));
            }
            _ => return Err(Error::new(span, "not recognized")),
        }
        Ok(())
    }
}
