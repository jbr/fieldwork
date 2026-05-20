use crate::errors::invalid_key;
use proc_macro2::Span;
use quote::{ToTokens, quote_spanned};
use syn::{
    Attribute, Error, Expr, ExprAssign, ExprLit, ExprPath, Ident, Lit, LitStr, parse::Parser,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

/// Deprecation configuration for a field or one of its method-level overrides.
///
/// `was` carries the old name. At the field level, the old name is treated as the
/// field's old binding-base (templates and method prefixes are applied to it). At the
/// method level, the old name is the literal old method name.
///
/// `since` and `note` populate the corresponding `#[deprecated(...)]` arguments.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct Deprecation {
    pub(crate) was: Option<Ident>,
    pub(crate) since: Option<String>,
    pub(crate) note: Option<String>,
}

impl Deprecation {
    pub(crate) const VALID_KEYS: &[&str] = &["was", "since", "note"];

    /// Parse the contents of `deprecate(...)` from its argument list.
    pub(crate) fn parse_list(args: &Punctuated<Expr, Comma>) -> syn::Result<Self> {
        let mut deprecation = Self::default();
        for expr in args {
            match expr {
                Expr::Assign(ExprAssign { left, right, .. }) => {
                    let lhs = match &**left {
                        Expr::Path(ExprPath { path, .. }) => {
                            path.get_ident().map(ToString::to_string).ok_or_else(|| {
                                invalid_key(
                                    left.span(),
                                    &left.to_token_stream().to_string(),
                                    Self::VALID_KEYS,
                                )
                            })?
                        }
                        _ => {
                            return Err(invalid_key(
                                left.span(),
                                &left.to_token_stream().to_string(),
                                Self::VALID_KEYS,
                            ));
                        }
                    };
                    deprecation.handle_assign(expr.span(), &lhs, right)?;
                }
                _ => return Err(Error::new(expr.span(), "not recognized")),
            }
        }
        Ok(deprecation)
    }

    fn handle_assign(&mut self, span: Span, lhs: &str, rhs: &Expr) -> syn::Result<()> {
        match (lhs, rhs) {
            (
                "was",
                Expr::Lit(ExprLit {
                    lit: Lit::Str(rhs), ..
                }),
            ) => {
                self.was = Some(rhs.parse()?);
            }
            ("was", Expr::Path(ExprPath { path, .. })) => {
                self.was = Some(path.require_ident().cloned()?);
            }
            (
                "since",
                Expr::Lit(ExprLit {
                    lit: Lit::Str(rhs), ..
                }),
            ) => {
                self.since = Some(rhs.value());
            }
            (
                "note",
                Expr::Lit(ExprLit {
                    lit: Lit::Str(rhs), ..
                }),
            ) => {
                self.note = Some(rhs.value());
            }
            ("was" | "since" | "note", _) => {
                return Err(Error::new(span, "expected a string literal"));
            }
            _ => return Err(invalid_key(span, lhs, Self::VALID_KEYS)),
        }
        Ok(())
    }

    /// Render this Deprecation as a `#[deprecated(...)]` attribute, supplying a default
    /// note pointing to `default_replacement` when no explicit note is set and one is
    /// provided. Returns the bare `#[deprecated]` when nothing else is configured.
    pub(crate) fn to_attribute(
        &self,
        span: Span,
        default_replacement: Option<&Ident>,
    ) -> Attribute {
        let note = self
            .note
            .clone()
            .or_else(|| default_replacement.map(|ident| format!("use `{ident}` instead")));
        let since = self.since.as_deref();

        let tokens = match (note, since) {
            (None, None) => quote_spanned!(span => #[deprecated]),
            (Some(note), None) => quote_spanned!(span => #[deprecated(note = #note)]),
            (None, Some(since)) => quote_spanned!(span => #[deprecated(since = #since)]),
            (Some(note), Some(since)) => {
                quote_spanned!(span => #[deprecated(since = #since, note = #note)])
            }
        };

        Attribute::parse_outer
            .parse2(tokens)
            .expect("generated #[deprecated(...)] failed to parse")
            .into_iter()
            .next()
            .expect("Attribute::parse_outer produced no attributes")
    }
}

/// Build a `Deprecation` from a bare ident (`deprecate`).
pub(crate) fn from_bare() -> Deprecation {
    Deprecation::default()
}

/// Build a `Deprecation` from a string-literal assignment (`deprecate = "old"`).
pub(crate) fn from_str_lit(rhs: &LitStr) -> syn::Result<Deprecation> {
    Ok(Deprecation {
        was: Some(rhs.parse()?),
        ..Deprecation::default()
    })
}

/// Build a `Deprecation` from a path assignment (`deprecate = old`).
pub(crate) fn from_path(rhs: &syn::Path) -> syn::Result<Deprecation> {
    Ok(Deprecation {
        was: Some(rhs.require_ident().cloned()?),
        ..Deprecation::default()
    })
}
