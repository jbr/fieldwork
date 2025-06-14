#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(argument = tee)]
    generic: T,
    #[fieldwork(get = "get_another")]
    another: (),
}
impl<T> MyStruct<T> {
    ///Borrows this number is cool
    pub fn number_in_seconds(&self) -> &usize {
        &self.number
    }
    ///Mutably borrow this number is cool
    pub fn number_in_seconds_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn set_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
        self.number = number_in_seconds;
        self
    }
    ///Owned chainable setter for this number is cool, returning `Self`
    #[must_use]
    pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
        self.number = number_in_seconds;
        self
    }
    ///Returns a copy of is this struct on or not
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow is this struct on or not
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for is this struct on or not, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    ///Borrows it's really whatever you want
    pub fn generic(&self) -> &T {
        &self.generic
    }
    ///Mutably borrow it's really whatever you want
    pub fn generic_mut(&mut self) -> &mut T {
        &mut self.generic
    }
    ///Sets it's really whatever you want, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, tee: T) -> &mut Self {
        self.generic = tee;
        self
    }
    ///Owned chainable setter for it's really whatever you want, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, tee: T) -> Self {
        self.generic = tee;
        self
    }
    pub fn get_another(&self) -> &() {
        &self.another
    }
    pub fn another_mut(&mut self) -> &mut () {
        &mut self.another
    }
    pub fn set_another(&mut self, another: ()) -> &mut Self {
        self.another = another;
        self
    }
    #[must_use]
    pub fn with_another(mut self, another: ()) -> Self {
        self.another = another;
        self
    }
}
#[fieldwork(get(template = "get_{}"), set(template = "put_{}"))]
struct WithTemplate<T> {
    /// this number is cool
    #[fieldwork(rename = "number_in_seconds")]
    number: usize,
    /// is this struct on or not
    #[fieldwork(get = is_it_enabled)]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(rename = "put_the_generic"))]
    generic: T,
}
impl<T> WithTemplate<T> {
    ///Borrows this number is cool
    pub fn get_number_in_seconds(&self) -> &usize {
        &self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn put_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
        self.number = number_in_seconds;
        self
    }
    ///Returns a copy of is this struct on or not
    pub fn is_it_enabled(&self) -> bool {
        self.enabled
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn put_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Borrows it's really whatever you want
    pub fn get_generic(&self) -> &T {
        &self.generic
    }
    ///Sets it's really whatever you want, returning `&mut Self` for chaining
    pub fn put_the_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
