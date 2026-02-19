use crate::{CommonSettings, errors::invalid_key, with_common_settings};
use proc_macro2::Span;
use syn::{
    Error, Expr, ExprAssign, ExprLit, ExprPath, Lit, LitBool, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

// this represents the configuration passed to #[fieldwork] for a particular method

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
pub(crate) struct StructMethodAttributes {
    pub(crate) template: Option<String>,
    pub(crate) doc_template: Option<String>,

    pub(crate) common_settings: CommonSettings,
}

impl StructMethodAttributes {
    const VALID_KEYS: &[&str] = with_common_settings!("doc_template", "template",);

    pub(crate) fn build(exprs: &Punctuated<Expr, Comma>) -> syn::Result<Self> {
        let mut struct_method_attributes = Self::default();
        struct_method_attributes.handle_exprs(exprs)?;
        Ok(struct_method_attributes)
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

            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),

            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> syn::Result<()> {
        if self.common_settings.handle_assign_bool_lit(lhs, value) {
            Ok(())
        } else {
            Err(invalid_key(span, lhs, Self::VALID_KEYS))
        }
    }

    fn handle_assign_str_lit(
        &mut self,
        span: Span,
        lhs: &str,
        rhs: &syn::LitStr,
    ) -> Result<(), Error> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "template" => self.template = Some(rhs.value()),
                "doc_template" => self.doc_template = Some(rhs.value()),
                _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
            }
        }
        Ok(())
    }
}
