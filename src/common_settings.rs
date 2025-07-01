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
            "copy" => self.get_copy = Some(value),
            "chain" => self.chainable_set = Some(value),
            "option_borrow_inner" => self.option_borrow_inner = Some(value),
            "opt_in" => self.opt_in = value,
            "skip" => self.skip = value,
            "deref" => self.auto_deref = Some(value),
            "rename_predicate" | "rename_predicates" => self.rename_predicates = Some(value),
            "option_set_some" => self.option_set_some = Some(value),
            _ => return false,
        }
        true
    }
}
