struct MyStruct<T> {
    number: usize,
    /// generated
    #[fieldwork(get(copy))]
    enabled: bool,
    /// generated
    generic: T,
}
impl<T> MyStruct<T> {
    ///Returns a copy of generated
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
