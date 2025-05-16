struct MyStruct<T> {
    number: usize,
    /// generated
    #[fieldwork(get(copy))]
    enabled: bool,
    /// generated
    generic: T,
    #[fieldwork(get(copy = true))]
    another: bool,
}
impl<T> MyStruct<T> {
    ///Returns a copy of generated
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn another(&self) -> bool {
        self.another
    }
}
