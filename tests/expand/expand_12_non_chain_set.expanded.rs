#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,
    /// opting out
    #[fieldwork(set(chain = false))]
    enabled: bool,
    generic: T,
}
impl<T> MyStruct<T> {
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Sets opting out
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
#[fieldwork(set(chain = false))]
struct MyStruct2<T> {
    /// opted out at struct-method level
    number: usize,
    #[fieldwork(set(chain = true))]
    /// opting back in
    enabled: bool,
    #[fieldwork(set(chain))]
    /// opting back in
    generic: T,
}
impl<T> MyStruct2<T> {
    ///Sets opted out at struct-method level
    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }
    ///Sets opting back in, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Sets opting back in, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
