use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Ident, Lit, LitBool, LitStr,
    Meta, MetaList, Path, Type, TypePath, punctuated::Punctuated, spanned::Spanned, token::Comma,
};

use crate::{
    CommonSettings, FieldMethodAttributes, Method, errors::invalid_key, method::MethodSettings,
};

// this represents the configuration for the field
#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct FieldAttributes {
    pub(crate) decorated: bool,
    pub(crate) fn_ident: Option<Ident>,
    pub(crate) argument_ident: Option<Ident>,
    pub(crate) method_attributes: MethodSettings<(Span, FieldMethodAttributes)>,
    pub(crate) deref: Option<Type>,

    pub(crate) common_settings: CommonSettings,
}

impl FieldAttributes {
    const VALID_KEYS: &[&str] = &[
        "argument",
        "chain",
        "copy",
        "deref",
        "get",
        "get_mut",
        "into",
        "name",
        "opt_in",
        "option",
        "option_borrow_inner",
        "option_set_some",
        "rename",
        "rename_predicate",
        "set",
        "skip",
        "vis",
        "with",
    ];

    fn handle_assign(&mut self, assign: &ExprAssign) -> syn::Result<()> {
        let ExprAssign { left, right, .. } = assign;
        let span = assign.span();
        let lhs = if let Expr::Path(ExprPath { path: lhs, .. }) = &**left {
            lhs.get_ident().map(ToString::to_string)
        } else {
            None
        }
        .ok_or_else(|| {
            invalid_key(
                left.span(),
                &left.to_token_stream().to_string(),
                Self::VALID_KEYS,
            )
        })?;
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

    fn handle_list(&mut self, list: &MetaList) -> syn::Result<()> {
        self.decorated = true;
        for expr in list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)? {
            match &expr {
                Expr::Assign(assign) => self.handle_assign(assign)?,

                Expr::Path(ExprPath { path, .. }) => self.handle_assign_bool_lit(
                    path.span(),
                    &path.require_ident()?.to_string(),
                    true,
                )?,

                Expr::Call(ExprCall { func, args, .. }) => match &**func {
                    Expr::Path(ExprPath { path: method, .. }) => {
                        let method = method.try_into().map_err(|_| {
                            invalid_key(
                                method.span(),
                                &method.to_token_stream().to_string(),
                                Self::VALID_KEYS,
                            )
                        })?;
                        self.method_attributes
                            .insert(method, (expr.span(), FieldMethodAttributes::build(args)?));
                    }

                    func => {
                        return Err(invalid_key(
                            func.span(),
                            &func.to_token_stream().to_string(),
                            Self::VALID_KEYS,
                        ));
                    }
                },

                expr => {
                    return Err(Error::new(expr.span(), "not recognized"));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn build(attribute: Option<&Attribute>) -> syn::Result<Self> {
        let mut field_attributes = Self::default();
        match attribute {
            Some(Attribute {
                meta: Meta::Path(_),
                ..
            }) => field_attributes.decorated = true,

            Some(Attribute {
                meta: Meta::List(list),
                ..
            }) => field_attributes.handle_list(list)?,

            None => {}

            Some(attribute) => return Err(Error::new(attribute.span(), "not recognized")),
        }

        Ok(field_attributes)
    }

    fn handle_assign_str_lit(&mut self, span: Span, lhs: &str, rhs: &LitStr) -> Result<(), Error> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "name" | "rename" => self.fn_ident = Some(rhs.parse()?),
                "deref" => self.deref = Some(rhs.parse()?),
                "argument" => self.argument_ident = Some(rhs.parse()?),
                _ => {
                    let method = Method::from_str_with_span(lhs, span)
                        .map_err(|_| invalid_key(span, lhs, Self::VALID_KEYS))?;

                    let fma = FieldMethodAttributes {
                        fn_ident: Some(rhs.parse()?),
                        ..FieldMethodAttributes::default()
                    };

                    self.method_attributes.insert(method, (span, fma));
                }
            }
        }
        Ok(())
    }

    fn handle_assign_path(&mut self, span: Span, lhs: &str, rhs: &Path) -> Result<(), Error> {
        match lhs {
            "name" | "rename" => self.fn_ident = Some(rhs.require_ident().cloned()?),
            "argument" => self.argument_ident = Some(rhs.require_ident().cloned()?),
            "deref" => {
                self.deref = Some(Type::Path(TypePath {
                    qself: None,
                    path: rhs.clone(),
                }));
            }

            _ => {
                let method = Method::from_str_with_span(lhs, span)
                    .map_err(|_| invalid_key(span, lhs, Self::VALID_KEYS))?;
                let fma = FieldMethodAttributes {
                    fn_ident: Some(rhs.require_ident().cloned()?),
                    ..FieldMethodAttributes::default()
                };

                self.method_attributes.insert(method, (span, fma));
            }
        }
        Ok(())
    }

    fn handle_assign_bool_lit(&mut self, span: Span, lhs: &str, value: bool) -> Result<(), Error> {
        if self.common_settings.handle_assign_bool_lit(lhs, value) {
            Ok(())
        } else {
            let method = Method::from_str_with_span(lhs, span)
                .map_err(|_| invalid_key(span, lhs, Self::VALID_KEYS))?;
            let fma = FieldMethodAttributes {
                common_settings: CommonSettings {
                    skip: !value,
                    ..CommonSettings::default()
                },
                ..FieldMethodAttributes::default()
            };

            self.method_attributes.insert(method, (span, fma));
            Ok(())
        }
    }
}
