#[fieldwork(get, set, get_mut, with)]
struct MyStruct<T> {
    /// the number
    #[fieldwork(argument = the_number)]
    number: usize,
    /// whether something is enabled
    #[fieldwork(set(argument = "is_enabled_as_a_boolean"))]
    enabled: bool,
    /// the generic
    #[fieldwork(argument = "the_gen", set(argument = the_generic))]
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
    pub fn set_number(&mut self, the_number: usize) -> &mut Self {
        self.number = the_number;
        self
    }
    ///Owned chainable setter for the number, returning `Self`
    #[must_use]
    pub fn with_number(mut self, the_number: usize) -> Self {
        self.number = the_number;
        self
    }
    ///Returns a copy of whether something is enabled
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow whether something is enabled
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets whether something is enabled, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, is_enabled_as_a_boolean: bool) -> &mut Self {
        self.enabled = is_enabled_as_a_boolean;
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
    pub fn set_generic(&mut self, the_generic: T) -> &mut Self {
        self.generic = the_generic;
        self
    }
    ///Owned chainable setter for the generic, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, the_gen: T) -> Self {
        self.generic = the_gen;
        self
    }
}
