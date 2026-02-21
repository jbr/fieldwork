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
    #[field(get(rename_predicate))]
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
/// Enum: bool fields with rename_predicates — full coverage → is_enabled() -> bool
#[fieldwork(get, rename_predicates)]
enum FeatureFlags {
    Enabled { active: bool, debug: bool },
    Disabled { active: bool, debug: bool },
}
impl FeatureFlags {
    pub fn is_active(&self) -> bool {
        match self {
            Self::Enabled { active, .. } | Self::Disabled { active, .. } => *active,
        }
    }
    pub fn is_debug(&self) -> bool {
        match self {
            Self::Enabled { debug, .. } | Self::Disabled { debug, .. } => *debug,
        }
    }
}
/// Enum: partial-coverage bool field with rename_predicates
#[fieldwork(get, rename_predicates)]
enum MixedFlags {
    Full { active: bool, verbose: bool },
    Partial { active: bool },
}
impl MixedFlags {
    pub fn is_active(&self) -> bool {
        match self {
            Self::Full { active, .. } | Self::Partial { active, .. } => *active,
        }
    }
    pub fn is_verbose(&self) -> Option<bool> {
        match self {
            Self::Full { verbose, .. } => Some(*verbose),
            _ => None,
        }
    }
}
