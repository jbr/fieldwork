use crate::{
    CommonSettings, ItemMethodAttributes, Method, MethodSettings, errors::invalid_key,
    is_fieldwork_attr, with_common_settings, with_methods,
};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprCall, ExprLit, ExprPath, Lit, LitBool, LitStr, Meta,
    WhereClause, WherePredicate,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Where},
};

// this represents the configuration passed to #[fieldwork]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
pub(crate) struct ItemAttributes {
    pub(crate) methods: MethodSettings<ItemMethodAttributes>,
    pub(crate) include: MethodSettings<bool>,
    pub(crate) where_clause: Option<WhereClause>,

    pub(crate) common_settings: CommonSettings,
}

impl ItemAttributes {
    const VALID_KEYS: &[&str] = with_methods!(with_common_settings!("where_clause", "bounds",));

    pub(crate) fn build(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut item_attributes = Self::default();
        let Some(attr) = attributes.iter().find(|x| is_fieldwork_attr(x)) else {
            return Ok(item_attributes);
        };
        let Meta::List(list) = &attr.meta else {
            return Err(Error::new(attr.span(), "unexpected attribute format"));
        };

        item_attributes
            .handle_list(&list.parse_args_with(Punctuated::<Expr, Comma>::parse_terminated)?)?;

        Ok(item_attributes)
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
                    Expr::Path(ExprPath { path: method, .. }) => match Method::try_from(method) {
                        Ok(method) => {
                            self.include.insert(method, true);
                            self.methods
                                .insert(method, ItemMethodAttributes::build(args)?);
                        }

                        Err(_) => {
                            return Err(invalid_key(
                                func.span(),
                                &method.require_ident()?.to_string(),
                                Self::VALID_KEYS,
                            ));
                        }
                    },

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

            //            Expr::Path(ExprPath { path: rhs, .. }) => self.handle_assign_path(span, &lhs, rhs),
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, .. }),
                ..
            }) => self.handle_assign_bool_lit(span, &lhs, *value),
            _ => Err(Error::new(span, "not recognized")),
        }
    }

    fn handle_assign_str_lit(&mut self, span: Span, lhs: &str, rhs: &LitStr) -> Result<(), Error> {
        if !self.common_settings.handle_assign_str_lit(lhs, rhs)? {
            match lhs {
                "where_clause" | "bounds" => {
                    self.where_clause = Some(WhereClause {
                        predicates: rhs
                            .parse_with(Punctuated::<WherePredicate, Comma>::parse_terminated)?,
                        where_token: Where::default(),
                    });
                }
                _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
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

            if value {
                // if they said `get = false`, we do nothing currently becuase it's opt in
                self.include.insert(method, true);
                self.methods.insert(method, ItemMethodAttributes::default());
            }
            Ok(())
        }
    }
}
