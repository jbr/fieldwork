use proc_macro2::Span;
use std::string::ToString;
use syn::{
    Error, Expr, ExprAssign, ExprLit, ExprPath, Ident, Lit, LitBool, LitStr, Path, Type, TypePath,
    Visibility, punctuated::Punctuated, spanned::Spanned, token::Comma,
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
    pub(crate) option_handling: Option<bool>,
    pub(crate) auto_deref: Option<bool>,
    pub(crate) rename_predicates: Option<bool>,
}

impl FieldMethodAttributes {
    pub(crate) fn new(method: Method, fn_ident: Option<Ident>) -> Self {
        Self {
            method,
            fn_ident,
            argument_ident: None,
            doc: None,
            vis: None,
            chainable_set: None,
            get_copy: None,
            deref: None,
            skip: false,
            option_handling: None,
            auto_deref: None,
            rename_predicates: None,
        }
    }

    pub(crate) fn build(
        method: Method,
        exprs: &Punctuated<Expr, Comma>,
    ) -> syn::Result<FieldMethodAttributes> {
        let mut field_method_attributes = Self::new(method, None);
        field_method_attributes.handle_exprs(exprs)?;
        Ok(field_method_attributes)
    }

    fn handle_exprs(&mut self, exprs: &Punctuated<Expr, Comma>) -> syn::Result<()> {
        for expr in exprs {
            match expr {
                Expr::Assign(assign) => self.handle_assign(assign)?,
                Expr::Path(ExprPath { path, .. }) => self.handle_assign_bool_lit(
                    path.span(),
                    &path.require_ident()?.to_string(),
                    true,
                )?,
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

            Expr::Path(ExprPath { path: rhs, .. }) => self.handle_assign_path(span, &lhs, rhs),

            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),

            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_str_lit(&mut self, span: Span, lhs: &str, rhs: &LitStr) -> syn::Result<()> {
        match lhs {
            "vis" => self.vis = Some(rhs.parse()?),
            "rename" => self.fn_ident = Some(rhs.parse()?),
            "argument" => self.argument_ident = Some(rhs.parse()?),
            "deref" => self.deref = Some(rhs.parse()?),
            "doc" => self.doc = Some(rhs.value()),
            _ => return Err(Error::new(span, "not recognized")),
        }
        Ok(())
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> syn::Result<()> {
        match (lhs, self.method) {
            ("chain", Method::Set) => self.chainable_set = Some(value),
            ("copy", Method::Get) => self.get_copy = Some(value),
            ("skip", _) => self.skip = value,
            ("option", _) => self.option_handling = Some(value),
            ("deref", _) => self.auto_deref = Some(value),
            ("rename_predicate" | "rename_predicates", _) => self.rename_predicates = Some(value),
            _ => return Err(Error::new(span, "not recognized")),
        }

        Ok(())
    }

    fn handle_assign_path(&mut self, span: Span, lhs: &str, rhs: &Path) -> syn::Result<()> {
        match lhs {
            "rename" => self.fn_ident = Some(rhs.require_ident().cloned()?),
            "argument" => self.argument_ident = Some(rhs.require_ident().cloned()?),
            "deref" => {
                self.deref = Some(Type::Path(TypePath {
                    qself: None,
                    path: rhs.clone(),
                }));
            }
            _ => return Err(Error::new(span, "not recognized")),
        }

        Ok(())
    }
}
