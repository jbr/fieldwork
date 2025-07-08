#[fieldwork(set, get = true)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    generic: T,
}
impl<T> MyStruct<T> {
    ///Returns a copy of this number is cool
    pub fn number(&self) -> usize {
        self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Returns a copy of is this struct on or not
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Borrows it's really whatever you want
    pub fn generic(&self) -> &T {
        &self.generic
    }
    ///Sets it's really whatever you want, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
