use std::borrow::Cow;

use syn::{Error, LitStr, Visibility, token::Pub};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
pub(crate) struct CommonSettings {
    pub(crate) auto_deref: Option<bool>,
    pub(crate) chainable_set: Option<bool>,
    pub(crate) get_copy: Option<bool>,
    pub(crate) option_borrow_inner: Option<bool>,
    pub(crate) option_set_some: Option<bool>,
    pub(crate) rename_predicates: Option<bool>,
    pub(crate) opt_in: bool,
    pub(crate) skip: bool,
    pub(crate) vis: Option<Vis>,
    pub(crate) into: Option<bool>,
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Vis {
    Default,
    Visibility(Visibility),
}
impl Vis {
    pub(crate) fn as_visibility(&self) -> Cow<'_, Visibility> {
        match self {
            Vis::Default => Cow::Owned(Visibility::Public(Pub::default())),
            Vis::Visibility(visibility) => Cow::Borrowed(visibility),
        }
    }
}

macro_rules! with_common_settings {
    ($($key:literal,)+) => {
        &[
            "chain",
            "copy",
            "debug",
            "deref",
            "inspect",
            "into",
            "opt_in",
            "option_borrow_inner",
            "option_set_some",
            "rename_predicate",
            "rename_predicates",
            "skip",
            "vis",
            $($key,)+
        ]
    }
}
pub(crate) use with_common_settings;

impl CommonSettings {
    pub const DEFAULTS: &'static Self = &Self {
        chainable_set: Some(true),
        option_borrow_inner: Some(true),
        auto_deref: Some(true),
        get_copy: Some(true),
        rename_predicates: Some(false),
        option_set_some: Some(false),
        opt_in: false,
        skip: false,
        vis: Some(Vis::Default),
        into: Some(false),
    };

    pub(crate) fn handle_assign_str_lit(&mut self, lhs: &str, rhs: &LitStr) -> Result<bool, Error> {
        match lhs {
            "vis" => self.vis = Some(Vis::Visibility(rhs.parse()?)),
            _ => return Ok(false),
        }
        Ok(true)
    }

    pub(crate) fn handle_assign_bool_lit(&mut self, lhs: &str, value: bool) -> bool {
        match lhs {
            "chain" => self.chainable_set = Some(value),
            "copy" => self.get_copy = Some(value),
            "deref" => self.auto_deref = Some(value),
            "into" => self.into = Some(value),
            "opt_in" => self.opt_in = value,
            "option" | "option_borrow_inner" => self.option_borrow_inner = Some(value),
            "option_set_some" => self.option_set_some = Some(value),
            "rename_predicate" | "rename_predicates" => self.rename_predicates = Some(value),
            "skip" => self.skip = value,
            _ => return false,
        }
        true
    }
}
