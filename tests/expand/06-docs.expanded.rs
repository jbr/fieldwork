#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,
    /// it's really whatever you want
    generic: T,
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
    ///get whether it's enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
    ///mutably borrow enabled
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///assign enabled
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///chainable setter for enabled
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
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
    ///Owned chainable setter for it's really whatever you want, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, generic: T) -> Self {
        self.generic = generic;
        self
    }
}
#[fieldwork(
    set(doc_template = " # ssssets {}"),
    get(doc_template = "# ggggets {}"),
    with(doc_template = "# width {}"),
    update(doc_template = "# updates {}"),
    get_mut(doc_template = "# gmut {}")
)]
struct DocTemplates<T> {
    /// the cool number
    number: usize,
    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,
    /// the generic
    generic: T,
}
