#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(deref = DerefType, get, set, get_mut)]
    deref_both: OwnedType,
}

#[derive(fieldwork::Fieldwork)]
struct OnlyDerefForMethods {
    #[fieldwork(get(deref = DerefType), set, get_mut)]
    deref_for_get_only: OwnedType,

    #[fieldwork(get, set, get_mut(deref = "&DerefType"))]
    deref_for_get_mut_only: OwnedType,
}
