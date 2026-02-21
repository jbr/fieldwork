use syn::{
    Attribute, Error, Expr, ExprAssign, ExprLit, ExprPath, Lit, Meta, MetaNameValue,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

/// Configuration from `#[variant(...)]` on an enum variant.
#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct VariantAttributes {
    /// Skip this variant from all field method coverage.
    pub(crate) skip: bool,
}

impl VariantAttributes {
    pub(crate) fn build(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut va = Self::default();
        let Some(attr) = attributes.iter().find(|a| a.path().is_ident("variant")) else {
            return Ok(va);
        };

        match &attr.meta {
            Meta::Path(_) => {}

            Meta::NameValue(MetaNameValue { value, .. }) => match value {
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(b), ..
                }) => {
                    va.skip = !b.value;
                }
                _ => return Err(Error::new(value.span(), "expected bool")),
            },

            Meta::List(list) => {
                for expr in list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)? {
                    match &expr {
                        Expr::Path(ExprPath { path, .. }) => {
                            match path.require_ident()?.to_string().as_str() {
                                "skip" => va.skip = true,
                                key => {
                                    return Err(Error::new(
                                        path.span(),
                                        format!("unrecognized variant option `{key}`"),
                                    ));
                                }
                            }
                        }

                        Expr::Assign(ExprAssign { left, right, .. }) => {
                            let lhs = if let Expr::Path(ExprPath { path, .. }) = &**left {
                                path.require_ident()?.to_string()
                            } else {
                                return Err(Error::new(left.span(), "expected identifier"));
                            };

                            match lhs.as_str() {
                                "skip" => match &**right {
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Bool(b), ..
                                    }) => {
                                        va.skip = b.value;
                                    }
                                    _ => return Err(Error::new(right.span(), "expected bool")),
                                },
                                key => {
                                    return Err(Error::new(
                                        left.span(),
                                        format!("unrecognized variant option `{key}`"),
                                    ));
                                }
                            }
                        }

                        _ => return Err(Error::new(expr.span(), "not recognized")),
                    }
                }
            }
        }

        Ok(va)
    }
}
