#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications,
    missing_docs,
    rustdoc::missing_crate_level_docs
)]
#![warn(clippy::pedantic)]
//! `fieldwork` generates field accessor methods for structs via a derive macro.
//!
//! Add [`#[derive(Fieldwork)]`](derive@Fieldwork) to a struct, then declare which methods to
//! generate with `#[fieldwork(...)]` on the struct and refine individual fields with
//! `#[field(...)]`.
//!
//! ## Methods
//!
//! | Module | Generated method | Description |
//! |--------|-----------------|-------------|
//! | [`get`] | `field_name()` | Borrow or copy a field |
//! | [`set`] | `set_field_name()` | Mutating setter, returns `&mut Self` by default |
//! | [`get_mut`] | `field_name_mut()` | Mutably borrow a field |
//! | [`with`] | `with_field_name()` | Owned chainable setter |
//! | [`without`] | `without_field_name()` | Set bool to `false` or Option to `None` |
//! | [`take`] | `take_field_name()` | Take the value out of an `Option` field |
//!
//! ## Configuration
//!
//! Fieldwork has four levels of configuration that cascade from broadest to most specific.
//! See [`configuration`] for a full explanation.
//!
//! Options shared across setter methods ([`into`], [`option_set_some`]) are documented at the
//! top level since they apply equally to [`set`] and [`with`].

pub use fieldwork_derive::Fieldwork;

#[cfg(doc)]
#[doc = include_str!("../docs/get.md")]
pub mod get {
    #[doc = include_str!("../docs/get/copy.md")]
    pub mod copy {}
    #[doc = include_str!("../docs/get/deref.md")]
    pub mod deref {}
    #[doc = include_str!("../docs/get/option_borrow_inner.md")]
    pub mod option_borrow_inner {}
    #[doc = include_str!("../docs/get/rename_predicates.md")]
    pub mod rename_predicates {}
}

#[cfg(doc)]
#[doc = include_str!("../docs/set.md")]
pub mod set {
    #[doc = include_str!("../docs/set/chain.md")]
    pub mod chain {}
    #[doc = include_str!("../docs/into.md")]
    pub mod into {}
    #[doc = include_str!("../docs/option_set_some.md")]
    pub mod option_set_some {}
}

#[cfg(doc)]
#[doc = include_str!("../docs/get_mut.md")]
pub mod get_mut {
    #[doc = include_str!("../docs/get/deref.md")]
    pub mod deref {}
    #[doc = include_str!("../docs/get/option_borrow_inner.md")]
    pub mod option_borrow_inner {}
}

#[cfg(doc)]
#[doc = include_str!("../docs/with.md")]
pub mod with {
    #[doc = include_str!("../docs/into.md")]
    pub mod into {}
    #[doc = include_str!("../docs/option_set_some.md")]
    pub mod option_set_some {}
}

#[cfg(doc)]
#[doc = include_str!("../docs/without.md")]
pub mod without {}

#[cfg(doc)]
#[doc = include_str!("../docs/take.md")]
pub mod take {}

#[cfg(doc)]
#[doc = include_str!("../docs/into.md")]
pub mod into {}

#[cfg(doc)]
#[doc = include_str!("../docs/option_set_some.md")]
pub mod option_set_some {}

#[cfg(doc)]
#[doc = include_str!("../docs/configuration.md")]
pub mod configuration {}
