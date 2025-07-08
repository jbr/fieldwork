#[fieldwork(opt_in, get, set, with, get_mut)]
struct MyStruct<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork]
    enabled: bool,
    /// generated
    #[fieldwork]
    generic: T,
    #[fieldwork(get)]
    only_get: (),
}
impl<T> MyStruct<T> {
    ///Returns a copy of generated
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow generated
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for generated, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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
#[fieldwork(opt_in, get, set, with, get_mut)]
struct FieldworkEqualsTrue<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork = true]
    enabled: bool,
    /// generated
    #[field = true]
    generic: T,
}
impl<T> FieldworkEqualsTrue<T> {
    ///Returns a copy of generated
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow generated
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets generated, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for generated, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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
}
#[fieldwork(opt_in, get)]
struct OptingInPerField<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork(set = true, get(skip))]
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
struct OptingInPerField2<T> {
    /// not generated
    number: usize,
    /// generated
    #[fieldwork(set)]
    enabled: bool,
    /// generated
    #[fieldwork(get, set = true)]
    generic: T,
}
impl<T> OptingInPerField2<T> {
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
struct MyStruct2 {
    number: usize,
    #[fieldwork(opt_in = true, get)]
    only_get: (),
}
impl MyStruct2 {
    pub fn number(&self) -> usize {
        self.number
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
