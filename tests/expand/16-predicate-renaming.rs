#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Bools {
    enabled: bool,
    active: bool,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(rename_predicates = false))]
struct NoPredicateRenaming {
    enabled: bool,
    #[fieldwork(get(rename_predicate))]
    active: bool,
}
