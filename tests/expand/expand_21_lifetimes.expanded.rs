use std::borrow::Cow;
#[fieldwork(get, get_mut, with, without, set(option_set_some))]
struct MyStruct<'a> {
    borrow: &'a (),
    mut_borrow: &'a mut (),
    #[field(into)]
    cow: Cow<'a, str>,
    box_dyn_trait: Box<dyn std::fmt::Debug + 'a>,
    option_lifetime: Option<&'a str>,
}
impl<'a> MyStruct<'a> {
    pub fn borrow(&self) -> &'a () {
        self.borrow
    }
    pub fn borrow_mut(&mut self) -> &mut &'a () {
        &mut self.borrow
    }
    pub fn set_borrow(&mut self, borrow: &'a ()) -> &mut Self {
        self.borrow = borrow;
        self
    }
    #[must_use]
    pub fn with_borrow(mut self, borrow: &'a ()) -> Self {
        self.borrow = borrow;
        self
    }
    pub fn mut_borrow(&self) -> &() {
        &*self.mut_borrow
    }
    pub fn mut_borrow_mut(&mut self) -> &mut () {
        &mut *self.mut_borrow
    }
    pub fn set_mut_borrow(&mut self, mut_borrow: &'a mut ()) -> &mut Self {
        self.mut_borrow = mut_borrow;
        self
    }
    #[must_use]
    pub fn with_mut_borrow(mut self, mut_borrow: &'a mut ()) -> Self {
        self.mut_borrow = mut_borrow;
        self
    }
    pub fn cow(&self) -> &str {
        &*self.cow
    }
    pub fn cow_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.cow
    }
    pub fn set_cow(&mut self, cow: impl Into<Cow<'a, str>>) -> &mut Self {
        self.cow = cow.into();
        self
    }
    #[must_use]
    pub fn with_cow(mut self, cow: impl Into<Cow<'a, str>>) -> Self {
        self.cow = cow.into();
        self
    }
    pub fn box_dyn_trait(&self) -> &(dyn std::fmt::Debug + 'a) {
        &*self.box_dyn_trait
    }
    pub fn box_dyn_trait_mut(&mut self) -> &mut (dyn std::fmt::Debug + 'a) {
        &mut *self.box_dyn_trait
    }
    pub fn set_box_dyn_trait(
        &mut self,
        box_dyn_trait: Box<dyn std::fmt::Debug + 'a>,
    ) -> &mut Self {
        self.box_dyn_trait = box_dyn_trait;
        self
    }
    #[must_use]
    pub fn with_box_dyn_trait(
        mut self,
        box_dyn_trait: Box<dyn std::fmt::Debug + 'a>,
    ) -> Self {
        self.box_dyn_trait = box_dyn_trait;
        self
    }
    pub fn option_lifetime(&self) -> Option<&'a str> {
        self.option_lifetime
    }
    pub fn option_lifetime_mut(&mut self) -> Option<&'a str> {
        self.option_lifetime
    }
    pub fn set_option_lifetime(&mut self, option_lifetime: &'a str) -> &mut Self {
        self.option_lifetime = Some(option_lifetime);
        self
    }
    #[must_use]
    pub fn with_option_lifetime(mut self, option_lifetime: &'a str) -> Self {
        self.option_lifetime = Some(option_lifetime);
        self
    }
    #[must_use]
    pub fn without_option_lifetime(mut self) -> Self {
        self.option_lifetime = None;
        self
    }
}
