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
    #[fieldwork(get(rename_predicate))]
    active: bool,
}
