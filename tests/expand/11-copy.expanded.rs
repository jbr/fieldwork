struct MyStruct<T: Copy> {
    number: usize,
    /// generated
    enabled: bool,
    #[fieldwork(get(copy))]
    /// generated
    generic: T,
    #[fieldwork(get(copy = true))]
    another: usize,
}
impl<T: Copy> MyStruct<T> {
    ///Returns a copy of generated
    pub fn generic(&self) -> T {
        self.generic
    }
    pub fn another(&self) -> usize {
        self.another
    }
}
