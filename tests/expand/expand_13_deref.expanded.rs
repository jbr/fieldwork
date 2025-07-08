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
