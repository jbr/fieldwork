struct DerefType;
struct OwnedType(DerefType);
impl std::ops::Deref for OwnedType {
    type Target = DerefType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for OwnedType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
