#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,
    /// generated
    #[fieldwork(set(chain = false))]
    enabled: bool,
    /// generated
    generic: T,
}
impl<T> MyStruct<T> {
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    /// # Sets generated
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// # Sets generated, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
#[fieldwork(set(chain = false))]
struct MyStruct2<T> {
    number: usize,
    /// generated
    enabled: bool,
    /// generated
    generic: T,
}
impl<T> MyStruct2<T> {
    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }
    /// # Sets generated
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// # Sets generated
    pub fn set_generic(&mut self, generic: T) {
        self.generic = generic;
    }
}
