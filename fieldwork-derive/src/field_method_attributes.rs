use crate::{CommonSettings, Deprecation, deprecation, errors::invalid_key, with_common_settings};
use proc_macro2::Span;
use quote::ToTokens;
use std::string::ToString;
use syn::{
    Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Ident, Lit, LitBool, LitStr, Path, Type,
    TypePath, punctuated::Punctuated, spanned::Spanned, token::Comma,
};

// this represents the configuration for the field, for a particular method
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct FieldMethodAttributes {
    pub(crate) fn_ident: Option<Ident>,
    pub(crate) argument_ident: Option<Ident>,
    pub(crate) doc: Option<String>,
    pub(crate) deref: Option<Type>,
    pub(crate) deprecate: Option<Deprecation>,

    pub(crate) common_settings: CommonSettings,
}

impl FieldMethodAttributes {
    pub(crate) const VALID_KEYS: &[&str] =
        with_common_settings!("argument", "deprecate", "doc", "name", "rename",);

    pub(crate) fn build(exprs: &Punctuated<Expr, Comma>) -> syn::Result<FieldMethodAttributes> {
        let mut field_method_attributes = Self::default();
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
                Expr::Call(ExprCall { func, args, .. }) => match &**func {
                    Expr::Path(ExprPath { path, .. }) if path.is_ident("deprecate") => {
                        self.deprecate = Some(Deprecation::parse_list(args)?);
                    }
                    func => {
                        return Err(invalid_key(
                            func.span(),
                            &func.to_token_stream().to_string(),
                            Self::VALID_KEYS,
                        ));
                    }
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

            Expr::Path(ExprPath { path: rhs, .. }) => self.handle_assign_path(span, &lhs, rhs),

            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),

            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_str_lit(&mut self, span: Span, lhs: &str, rhs: &LitStr) -> syn::Result<()> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "name" | "rename" => self.fn_ident = Some(rhs.parse()?),
                "argument" => self.argument_ident = Some(rhs.parse()?),
                "deref" => self.deref = Some(rhs.parse()?),
                "doc" => self.doc = Some(rhs.value()),
                "deprecate" => self.deprecate = Some(deprecation::from_str_lit(rhs)?),
                _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
            }
        }
        Ok(())
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> syn::Result<()> {
        if self.common_settings.handle_assign_bool_lit(lhs, value) {
            Ok(())
        } else if lhs == "deprecate" {
            self.deprecate = value.then(deprecation::from_bare);
            Ok(())
        } else {
            Err(invalid_key(span, lhs, Self::VALID_KEYS))
        }
    }

    fn handle_assign_path(&mut self, span: Span, lhs: &str, rhs: &Path) -> syn::Result<()> {
        match lhs {
            "name" | "rename" => self.fn_ident = Some(rhs.require_ident().cloned()?),
            "argument" => self.argument_ident = Some(rhs.require_ident().cloned()?),
            "deref" => {
                self.deref = Some(Type::Path(TypePath {
                    qself: None,
                    path: rhs.clone(),
                }));
            }
            "deprecate" => self.deprecate = Some(deprecation::from_path(rhs)?),
            _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
        }

        Ok(())
    }
}
