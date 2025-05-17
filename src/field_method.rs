use syn::{
    Error, Expr, ExprAssign, ExprLit, ExprPath, Ident, Lit, LitBool, Type, TypePath, Visibility,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::Method;

// this represents the configuration for the field, for a particular method
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct FieldMethodAttributes {
    pub(crate) method: Method,
    pub(crate) skip: bool,
    pub(crate) fn_ident: Option<Ident>,
    pub(crate) argument_ident: Option<Ident>,
    pub(crate) doc: Option<String>,
    pub(crate) vis: Option<Visibility>,
    pub(crate) chainable_set: Option<bool>,
    pub(crate) get_copy: Option<bool>,
    pub(crate) deref: Option<Type>,
}
impl FieldMethodAttributes {
    pub(crate) fn build(
        method: Method,
        exprs: &Punctuated<Expr, Comma>,
    ) -> syn::Result<FieldMethodAttributes> {
        let mut vis = None;
        let mut skip = false;
        let mut fn_ident = None;
        let mut argument_ident = None;
        let mut doc = None;
        let mut chainable_set = None;
        let mut get_copy = None;
        let mut deref = None;
        for expr in exprs {
            match expr {
                Expr::Assign(ExprAssign { left, right, .. }) => match (&**left, &**right) {
                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("vis") => {
                        vis = Some(rhs.parse()?);
                    }

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("rename") => fn_ident = Some(rhs.parse()?),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Path(ExprPath { path: rhs, .. }),
                    ) if lhs.is_ident("rename") => fn_ident = Some(rhs.require_ident().cloned()?),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("argument") => argument_ident = Some(rhs.parse()?),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Path(ExprPath { path: rhs, .. }),
                    ) if lhs.is_ident("argument") => {
                        argument_ident = Some(rhs.require_ident().cloned()?);
                    }

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Path(ExprPath { path: rhs, .. }),
                    ) if lhs.is_ident("deref") => {
                        deref = Some(Type::Path(TypePath {
                            qself: None,
                            path: rhs.clone(),
                        }));
                    }

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("deref") => {
                        deref = Some(rhs.parse()?);
                    }

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(rhs), ..
                        }),
                    ) if lhs.is_ident("doc") => doc = Some(rhs.value()),

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Bool(LitBool { value, .. }),
                            ..
                        }),
                    ) if lhs.is_ident("chain") && method == Method::Set => {
                        chainable_set = Some(*value);
                    }

                    (
                        Expr::Path(ExprPath { path: lhs, .. }),
                        Expr::Lit(ExprLit {
                            lit: Lit::Bool(LitBool { value, .. }),
                            ..
                        }),
                    ) if lhs.is_ident("copy") && method == Method::Get => get_copy = Some(*value),

                    _ => return Err(Error::new(expr.span(), "not recognized")),
                },

                Expr::Path(ExprPath { path, .. }) if path.is_ident("skip") => skip = true,
                Expr::Path(ExprPath { path, .. })
                    if path.is_ident("copy") && method == Method::Get =>
                {
                    get_copy = Some(true);
                }

                expr => {
                    return Err(Error::new(expr.span(), "not recognized"));
                }
            }
        }

        Ok(FieldMethodAttributes {
            method,
            skip,
            fn_ident,
            argument_ident,
            doc,
            vis,
            chainable_set,
            get_copy,
            deref,
        })
    }
}
