#[fieldwork(opt_in, get, set, with, get_mut)]
struct MyStruct<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork(rename = is_enabled)]
    enabled: bool,
    /// generated
    #[fieldwork]
    generic: T,
    #[fieldwork(get)]
    only_get: (),
}
impl<T> MyStruct<T> {
    ///Borrows generated
    pub fn is_enabled(&self) -> &bool {
        &self.enabled
    }
    ///Mutably borrow generated
    pub fn is_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_is_enabled(&mut self, is_enabled: bool) -> &mut Self {
        self.enabled = is_enabled;
        self
    }
    ///Owned chainable setter for generated, returning `Self`
    #[must_use]
    pub fn with_is_enabled(mut self, is_enabled: bool) -> Self {
        self.enabled = is_enabled;
        self
    }
    ///Borrows generated
    pub fn generic(&self) -> &T {
        &self.generic
    }
    ///Mutably borrow generated
    pub fn generic_mut(&mut self) -> &mut T {
        &mut self.generic
    }
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
    ///Owned chainable setter for generated, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, generic: T) -> Self {
        self.generic = generic;
        self
    }
    pub fn only_get(&self) -> &() {
        &self.only_get
    }
}
#[fieldwork(opt_in, get)]
struct OptingInPerField<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork(set, get(skip))]
    enabled: bool,
    /// generated
    #[fieldwork]
    generic: T,
}
impl<T> OptingInPerField<T> {
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Borrows generated
    pub fn generic(&self) -> &T {
        &self.generic
    }
}
#[fieldwork(opt_in, get(template = "get_{}"))]
struct OptingInPerField<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork(set)]
    enabled: bool,
    /// generated
    #[fieldwork(get, set)]
    generic: T,
}
impl<T> OptingInPerField<T> {
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Borrows generated
    pub fn get_generic(&self) -> &T {
        &self.generic
    }
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
#[fieldwork(get, set, with, get_mut)]
struct MyStruct {
    number: usize,
    #[fieldwork(opt_in, get)]
    only_get: (),
}
impl MyStruct {
    pub fn number(&self) -> &usize {
        &self.number
    }
    pub fn number_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    #[must_use]
    pub fn with_number(mut self, number: usize) -> Self {
        self.number = number;
        self
    }
    pub fn only_get(&self) -> &() {
        &self.only_get
    }
}
