#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, rename_predicates)]
struct Bools {
    enabled: bool,
    active: bool,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(rename_predicates = false), rename_predicates)]
struct NoPredicateRenaming {
    enabled: bool,
    #[field(get(rename_predicate))]
    active: bool,
}

/// Enum: bool fields with rename_predicates — full coverage → is_enabled() -> bool
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, rename_predicates)]
enum FeatureFlags {
    Enabled { active: bool, debug: bool },
    Disabled { active: bool, debug: bool },
}

/// Enum: partial-coverage bool field with rename_predicates
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, rename_predicates)]
enum MixedFlags {
    Full { active: bool, verbose: bool },
    Partial { active: bool },
}
