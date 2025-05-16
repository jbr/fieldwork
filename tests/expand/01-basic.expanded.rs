#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// the number
    number: usize,
    /// whether something is enabled
    enabled: bool,
    /// the generic
    generic: T,
}
impl<T> MyStruct<T> {
    ///Borrows the number
    pub fn number(&self) -> &usize {
        &self.number
    }
    ///Mutably borrow the number
    pub fn number_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    ///Sets the number, returning `&mut Self` for chaining
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Owned chainable setter for the number, returning `Self`
    #[must_use]
    pub fn with_number(mut self, number: usize) -> Self {
        self.number = number;
        self
    }
    ///Borrows whether something is enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
    ///Mutably borrow whether something is enabled
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets whether something is enabled, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for whether something is enabled, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    ///Borrows the generic
    pub fn generic(&self) -> &T {
        &self.generic
    }
    ///Mutably borrow the generic
    pub fn generic_mut(&mut self) -> &mut T {
        &mut self.generic
    }
    ///Sets the generic, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
    ///Owned chainable setter for the generic, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, generic: T) -> Self {
        self.generic = generic;
        self
    }
}
