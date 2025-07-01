#[fieldwork(get, rename_predicates)]
struct Bools {
    enabled: bool,
    active: bool,
}
impl Bools {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
}
#[fieldwork(get(rename_predicates = false), rename_predicates)]
struct NoPredicateRenaming {
    enabled: bool,
    #[fieldwork(get(rename_predicate))]
    active: bool,
}
impl NoPredicateRenaming {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
}
