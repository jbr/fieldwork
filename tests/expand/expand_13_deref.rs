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

/// Enum: explicit deref target on full-coverage field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
enum HasDeref {
    First {
        #[fieldwork(deref = DerefType)]
        owned: OwnedType,
    },
    Second {
        #[fieldwork(deref = DerefType)]
        owned: OwnedType,
    },
}

/// Enum: deref for get only, partial coverage
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
enum PartialDeref {
    WithValue {
        #[fieldwork(get(deref = DerefType))]
        field: OwnedType,
    },
    Empty {
        other: i32,
    },
}
