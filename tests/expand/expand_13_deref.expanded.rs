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
struct User {
    #[fieldwork(deref = DerefType, get, set, get_mut)]
    deref_both: OwnedType,
}
impl User {
    pub fn deref_both(&self) -> &DerefType {
        &*self.deref_both
    }
    pub fn deref_both_mut(&mut self) -> &mut DerefType {
        &mut *self.deref_both
    }
    pub fn set_deref_both(&mut self, deref_both: OwnedType) -> &mut Self {
        self.deref_both = deref_both;
        self
    }
}
struct OnlyDerefForMethods {
    #[fieldwork(get(deref = DerefType), set, get_mut)]
    deref_for_get_only: OwnedType,
    #[fieldwork(get, set, get_mut(deref = "&DerefType"))]
    deref_for_get_mut_only: OwnedType,
}
impl OnlyDerefForMethods {
    pub fn deref_for_get_only(&self) -> &DerefType {
        &*self.deref_for_get_only
    }
    pub fn deref_for_get_only_mut(&mut self) -> &mut OwnedType {
        &mut self.deref_for_get_only
    }
    pub fn set_deref_for_get_only(
        &mut self,
        deref_for_get_only: OwnedType,
    ) -> &mut Self {
        self.deref_for_get_only = deref_for_get_only;
        self
    }
    pub fn deref_for_get_mut_only(&self) -> &OwnedType {
        &self.deref_for_get_mut_only
    }
    pub fn deref_for_get_mut_only_mut(&mut self) -> &mut DerefType {
        &mut *self.deref_for_get_mut_only
    }
    pub fn set_deref_for_get_mut_only(
        &mut self,
        deref_for_get_mut_only: OwnedType,
    ) -> &mut Self {
        self.deref_for_get_mut_only = deref_for_get_mut_only;
        self
    }
}
/// Enum: explicit deref target on full-coverage field
#[fieldwork(get, get_mut)]
enum HasDeref {
    First { #[fieldwork(deref = DerefType)] owned: OwnedType },
    Second { #[fieldwork(deref = DerefType)] owned: OwnedType },
}
impl HasDeref {
    pub fn owned(&self) -> &DerefType {
        match self {
            Self::First { owned, .. } | Self::Second { owned, .. } => &**owned,
        }
    }
    pub fn owned_mut(&mut self) -> &mut DerefType {
        match self {
            Self::First { owned, .. } => &mut **owned,
            Self::Second { owned, .. } => &mut **owned,
        }
    }
}
/// Enum: deref for get only, partial coverage
#[fieldwork(get, get_mut)]
enum PartialDeref {
    WithValue { #[fieldwork(get(deref = DerefType))] field: OwnedType },
    Empty { other: i32 },
}
impl PartialDeref {
    pub fn field(&self) -> Option<&DerefType> {
        match self {
            Self::WithValue { field, .. } => Some(&**field),
            _ => None,
        }
    }
    pub fn field_mut(&mut self) -> Option<&mut OwnedType> {
        match self {
            Self::WithValue { field, .. } => Some(field),
            _ => None,
        }
    }
    pub fn other(&self) -> Option<i32> {
        match self {
            Self::Empty { other, .. } => Some(*other),
            _ => None,
        }
    }
    pub fn other_mut(&mut self) -> Option<&mut i32> {
        match self {
            Self::Empty { other, .. } => Some(other),
            _ => None,
        }
    }
}
