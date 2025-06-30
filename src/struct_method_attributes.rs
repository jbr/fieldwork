use syn::{
    Error, Expr, ExprAssign, ExprLit, ExprPath, Lit, LitBool, Visibility, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::Method;

// this represents the configuration passed to #[fieldwork] for a particular method

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct StructMethodAttributes {
    pub(crate) method: Method,
    pub(crate) vis: Option<Visibility>,
    pub(crate) template: Option<String>,
    pub(crate) skip: bool,
    pub(crate) doc_template: Option<String>,
    pub(crate) chainable_set: Option<bool>,
    pub(crate) option_handling: Option<bool>,
    pub(crate) auto_deref: Option<bool>,
    pub(crate) auto_copy: Option<bool>,
    pub(crate) rename_predicates: Option<bool>,
    pub(crate) option_set_some: Option<bool>,
}

impl StructMethodAttributes {
    pub(crate) fn new(method: Method) -> Self {
        Self {
            method,
            vis: None,
            template: None,
            skip: false,
            doc_template: None,
            chainable_set: None,
            option_handling: None,
            auto_deref: None,
            auto_copy: None,
            rename_predicates: None,
            option_set_some: None,
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
        match lhs {
            "chain" => self.chainable_set = Some(value),
            "copy" => self.auto_copy = Some(value),
            "skip" => self.skip = value,
            "rename_predicate" | "rename_predicates" => self.rename_predicates = Some(value),
            "deref" => self.auto_deref = Some(value),
            "option_borrow_inner" => self.option_handling = Some(value),
            "option_set_some" => self.option_set_some = Some(value),
            _ => return Err(Error::new(span, "not recognized")),
        }
        Ok(())
    }

    fn handle_assign_str_lit(
        &mut self,
        span: proc_macro2::Span,
        lhs: &str,
        rhs: &syn::LitStr,
    ) -> Result<(), Error> {
        match lhs {
            "vis" => self.vis = Some(rhs.parse()?),
            "template" => self.template = Some(rhs.value()),
            "doc_template" => self.doc_template = Some(rhs.value()),
            _ => return Err(Error::new(span, "not recognized")),
        }
        Ok(())
    }
}
