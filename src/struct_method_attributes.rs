use syn::{
    Error, Expr, ExprAssign, ExprLit, ExprPath, Lit, LitBool, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::{CommonSettings, Method};

// this represents the configuration passed to #[fieldwork] for a particular method

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct StructMethodAttributes {
    pub(crate) method: Method,
    pub(crate) template: Option<String>,
    pub(crate) doc_template: Option<String>,

    pub(crate) common_settings: CommonSettings,
}

impl StructMethodAttributes {
    pub(crate) fn new(method: Method) -> Self {
        Self {
            method,
            template: None,
            doc_template: None,
            common_settings: CommonSettings::default(),
        }
    }

    pub(crate) fn build(method: Method, exprs: &Punctuated<Expr, Comma>) -> syn::Result<Self> {
        let mut struct_method_attributes = Self::new(method);
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

            //            Expr::Path(ExprPath { path: rhs, .. }) => self.handle_assign_path(span, &lhs, rhs),
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),

            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_bool_lit(
        &mut self,
        span: proc_macro2::Span,
        lhs: &str,
        value: bool,
    ) -> syn::Result<()> {
        if self.common_settings.handle_assign_bool_lit(lhs, value) {
            Ok(())
        } else {
            Err(Error::new(span, "not recognized"))
        }
    }

    fn handle_assign_str_lit(
        &mut self,
        span: proc_macro2::Span,
        lhs: &str,
        rhs: &syn::LitStr,
    ) -> Result<(), Error> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "template" => self.template = Some(rhs.value()),
                "doc_template" => self.doc_template = Some(rhs.value()),
                _ => return Err(Error::new(span, "not recognized")),
            }
        }
        Ok(())
    }
}
