use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Generics, Ident, Member, Variant,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::{Field, ItemAttributes, Method, Query, VariantAttributes};

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
    pub(crate) ident: Ident,
    /// Only fields that have a usable name (natural or via `#[field = name]`).
    pub(crate) fields: Vec<Field>,
    pub(crate) attributes: VariantAttributes,
}

/// One variant's participation in a virtual field's match arm.
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct VariantArm {
    /// The variant name, e.g. `Click`.
    pub(crate) variant_ident: Ident,
    /// The binding name used in the match pattern (== the field's fn_ident).
    pub(crate) binding: Ident,
    /// The original member in the variant (Named or Unnamed index), used to
    /// produce the correct struct/tuple pattern syntax.
    pub(crate) member: Member,
}

/// A field name that appears in at least one variant, treated as a single
/// virtual field for method generation purposes.
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct VirtualField {
    /// A representative `Field` used to construct a `Query`.
    pub(crate) field: Field,
    /// Variants that contain this field (already filtered for `#[variant(skip)]`).
    pub(crate) arms: Vec<VariantArm>,
    /// Total non-skipped variants in the enum (denominator for coverage).
    pub(crate) total_variants: usize,
}

impl VirtualField {
    pub(crate) fn is_full_coverage(&self) -> bool {
        self.arms.len() == self.total_variants
    }
}

impl EnumVariant {
    fn build(variant: &Variant) -> syn::Result<Self> {
        let ident = variant.ident.clone();
        let attributes = VariantAttributes::build(&variant.attrs)?;
        // Only keep fields that have a usable name: either a natural ident or
        // an explicit `#[field = name]` override.
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| Field::build(f, i))
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .filter(|f| f.attributes.fn_ident.is_some() || matches!(f.member, Member::Named(_)))
            .collect();

        Ok(Self {
            ident,
            fields,
            attributes,
        })
    }

    /// The binding ident to use in match patterns for a given field.
    fn binding_for(&self, field: &Field) -> Ident {
        field
            .attributes
            .fn_ident
            .clone()
            .unwrap_or_else(|| match &field.member {
                Member::Named(ident) => ident.clone(),
                Member::Unnamed(_) => unreachable!("unnamed field without fn_ident was filtered"),
            })
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
        generics.where_clause = attributes.where_clause.take();

        Ok(Self {
            ident,
            variants,
            attributes,
            generics,
        })
    }
}

impl Enum {
    /// Collect all virtual fields across variants, with coverage information.
    /// Variants marked `#[variant(skip)]` are excluded from coverage.
    pub(crate) fn virtual_fields(&self) -> Vec<VirtualField> {
        let active_variants: Vec<&EnumVariant> = self
            .variants
            .iter()
            .filter(|v| !v.attributes.skip)
            .collect();
        // Use the total enum variant count (including skipped) as the denominator.
        // A skipped variant is treated as "not having any field", so any field
        // that would otherwise be full-coverage becomes partial when variants are
        // skipped — forcing Option return types and a `_ => None` wildcard arm
        // instead of an exhaustive or-pattern that would miss skipped variants.
        let total_variants = self.variants.len();

        // Map field name → (first seen Field, Vec<VariantArm>)
        let mut by_name: BTreeMap<String, (Field, Vec<VariantArm>)> = BTreeMap::new();

        for variant in &active_variants {
            for field in &variant.fields {
                let binding = variant.binding_for(field);
                let key = binding.to_string();
                let arm = VariantArm {
                    variant_ident: variant.ident.clone(),
                    binding: binding.clone(),
                    member: field.member.clone(),
                };
                by_name
                    .entry(key)
                    .or_insert_with(|| (field.clone(), Vec::new()))
                    .1
                    .push(arm);
            }
        }

        by_name
            .into_values()
            .map(|(field, arms)| VirtualField {
                field,
                arms,
                total_variants,
            })
            .collect()
    }

    /// Generate all field accessor methods for this enum.
    pub(crate) fn generate_methods(&self) -> TokenStream {
        let virtual_fields = self.virtual_fields();
        virtual_fields
            .iter()
            .flat_map(|virtual_field| {
                Method::all().iter().filter_map(|method| {
                    Query::new(method, &virtual_field.field, &self.attributes)
                        .with_virtual_field(virtual_field)
                        .resolve()
                })
            })
            .map(|resolved| resolved.build())
            .collect()
    }
}

/// Generate the match pattern for one arm, binding the field as `binding`.
/// For set/with/without (needing a mutable rebind that doesn't shadow the
/// argument), pass `Some(override_binding)`.
pub(crate) fn arm_pattern(
    variant_ident: &Ident,
    arm: &VariantArm,
    override_binding: Option<&Ident>,
) -> TokenStream {
    let binding = override_binding.unwrap_or(&arm.binding);
    match &arm.member {
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
