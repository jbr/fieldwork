use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Expr, ExprAssign, ExprLit,
    ExprPath, Lit, LitBool, Visibility,
};
use Method::Set;

use crate::Method;

// this represents the configuration passed to #[fieldwork] for a particular method
#[derive(Debug)]
pub(crate) struct StructMethodAttributes {
    pub(crate) method: Method,
    pub(crate) vis: Option<Visibility>,
    pub(crate) template: Option<String>,
    pub(crate) skip: bool,
    pub(crate) doc_template: Option<String>,
    pub(crate) chainable_set: Option<bool>,
}

impl StructMethodAttributes {
    pub(crate) fn build(
        method: Method,
        exprs: &Punctuated<Expr, Comma>,
    ) -> syn::Result<StructMethodAttributes> {
        let mut vis = None;
        let mut template = None;
        let mut doc_template = None;
        let mut skip = false;
        let mut chainable_set = None;

        for expr in exprs {
            match expr {
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
                    ) if lhs.is_ident("template") => template = Some(rhs.value()),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("doc_template") => doc_template = Some(rhs.value()),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Bool(LitBool { value, .. }),
                            ..
                        }),
                    ) if lhs.is_ident("chain") => match &method {
                        Set => chainable_set = Some(*value),
                        _ => {
                            return Err(Error::new(
                                expr.span(),
                                "`chain` can only be used with setters",
                            ))
                        }
                    },

                    _ => return Err(Error::new(expr.span(), "not recognized")),
                },

                Expr::Path(ExprPath { path, .. }) if path.is_ident("skip") => skip = true,

                expr => {
                    return Err(Error::new(expr.span(), "not recognized"));
                }
            }
        }

        Ok(StructMethodAttributes {
            method,
            vis,
            template,
            skip,
            doc_template,
            chainable_set,
        })
    }
}
