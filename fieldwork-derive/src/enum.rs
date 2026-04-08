use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Generics, Ident, Member, Variant,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::{Field, ItemAttributes, Method, Query};

/// A parsed `#[derive(Fieldwork)]` enum.
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct Enum {
    pub(crate) ident: Ident,
    pub(crate) variants: Vec<EnumVariant>,
    pub(crate) attributes: ItemAttributes,
    pub(crate) generics: Generics,
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct EnumVariant {
    /// Only fields that have a usable name (natural or via `#[field = name]`).
    pub(crate) fields: Vec<Field>,
}

impl EnumVariant {
    fn build(variant: &Variant) -> syn::Result<Self> {
        let ident = variant.ident.clone();
        // Only keep fields that have a usable name: either a natural ident or
        // an explicit `#[field = name]` override. Set variant_ident on each.
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| Field::build(f, i))
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .filter(|f| f.attributes.fn_ident.is_some() || matches!(f.member, Member::Named(_)))
            .map(|mut f| {
                f.variant_ident = Some(ident.clone());
                f
            })
            .collect();

        Ok(Self { fields })
    }
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = DeriveInput::parse(input)?;
        let Data::Enum(de) = &input.data else {
            return Err(syn::Error::new(input.span(), "expected enum"));
        };

        let ident = input.ident.clone();
        let mut attributes = ItemAttributes::build(&input.attrs)?;
        let variants = de
            .variants
            .iter()
            .map(EnumVariant::build)
            .collect::<syn::Result<Vec<_>>>()?;

        let mut generics = input.generics.clone();
        if let Some(attr_where) = attributes.where_clause.take() {
            generics
                .make_where_clause()
                .predicates
                .extend(attr_where.predicates);
        }

        Ok(Self {
            ident,
            variants,
            attributes,
            generics,
        })
    }
}

impl Enum {
    /// Group fields across variants by binding name. Each group is a `Vec<Field>` where
    /// every element shares the same method name and each field carries its `variant_ident`.
    ///
    /// Returns an error if:
    /// - More than one occurrence in a group has a `#[field]` annotation (`decorated = true`).
    /// - Any occurrence is annotated and the group has inconsistent field types.
    ///
    /// Silently drops groups where types differ but no occurrence is annotated.
    pub(crate) fn named_fields(&self) -> syn::Result<Vec<Vec<Field>>> {
        let mut by_name: BTreeMap<String, Vec<Field>> = BTreeMap::new();
        for variant in &self.variants {
            for field in &variant.fields {
                let key = field.binding().to_string();
                by_name.entry(key).or_default().push(field.clone());
            }
        }

        let mut result = Vec::new();
        for fields in by_name.into_values() {
            let name = fields[0].binding().to_string();

            // At most one occurrence may have a substantive #[field] annotation.
            // Rename-only annotations (#[field(rename = foo)]) may appear on multiple occurrences
            // since they control group membership and carry no other configuration.
            let substantive: Vec<&Field> = fields
                .iter()
                .filter(|f| f.attributes.is_substantive_annotation())
                .collect();
            if substantive.len() > 1 {
                let mut err = syn::Error::new(
                    substantive[1].span,
                    format!(
                        "multiple `#[field]` annotations for virtual field `{name}`; \
                         only one occurrence may be annotated"
                    ),
                );
                err.combine(syn::Error::new(
                    substantive[0].span,
                    "first annotation here",
                ));
                for f in substantive.iter().skip(2) {
                    err.combine(syn::Error::new(f.span, "also annotated here"));
                }
                return Err(err);
            }

            // All occurrences must share the same type.
            let reference = &fields[0];
            if let Some(mismatched) = fields.iter().skip(1).find(|f| f.ty != reference.ty) {
                if fields.iter().any(|f| f.attributes.decorated) {
                    use quote::ToTokens;
                    let ref_ty = reference.ty.to_token_stream().to_string();
                    let mis_ty = mismatched.ty.to_token_stream().to_string();
                    let ref_variant = reference
                        .variant_ident
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    let mis_variant = mismatched
                        .variant_ident
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    let mut err = syn::Error::new(
                        mismatched.span,
                        format!(
                            "field `{name}` has type `{mis_ty}` in variant `{mis_variant}`, \
                             but type `{ref_ty}` in variant `{ref_variant}`; \
                             fieldwork cannot generate a single accessor method"
                        ),
                    );
                    err.combine(syn::Error::new(
                        reference.span,
                        format!("field `{name}` type `{ref_ty}` first seen here"),
                    ));
                    return Err(err);
                }
                // No annotation: silently skip this group.
                continue;
            }

            result.push(fields);
        }
        Ok(result)
    }

    /// Generate all field accessor methods for this enum.
    pub(crate) fn generate_methods(&self) -> syn::Result<TokenStream> {
        let total_variants = self.variants.len();
        let named_fields = self.named_fields()?;
        let methods = named_fields
            .iter()
            .flat_map(|fields| {
                Method::all().iter().filter_map(|method| {
                    Query::new(method, fields, &self.attributes, total_variants).resolve()
                })
            })
            .map(|resolved| resolved.build())
            .collect();
        Ok(methods)
    }
}

/// Generate the match pattern for one arm, binding the field as its binding ident.
/// For set/with/without (needing a mutable rebind that doesn't shadow the
/// argument), pass `Some(override_binding)`.
pub(crate) fn arm_pattern(field: &Field, override_binding: Option<&Ident>) -> TokenStream {
    let variant_ident = field
        .variant_ident
        .as_ref()
        .expect("arm_pattern called on non-enum field");
    let default_binding = field.binding();
    let binding = override_binding.unwrap_or(default_binding);
    match &field.member {
        Member::Named(field_ident) => {
            if field_ident == binding {
                quote! { Self::#variant_ident { #binding, .. } }
            } else {
                // Renamed: `#[field = name]` on a named field
                quote! { Self::#variant_ident { #field_ident: #binding, .. } }
            }
        }
        Member::Unnamed(idx) => {
            let index = idx.index as usize;
            let underscores = std::iter::repeat_n(quote!(_), index);
            quote! { Self::#variant_ident(#(#underscores,)* #binding, ..) }
        }
    }
}
