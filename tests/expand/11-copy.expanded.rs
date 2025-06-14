#[fieldwork(get)]
struct MyStruct<T: Copy> {
    number: usize,
    /// generated
    enabled: bool,
    #[fieldwork(get(copy))]
    /// generated
    generic: T,
    #[fieldwork(get(copy = true))]
    another: usize,
    static_str: &'static str,
}
impl<T: Copy> MyStruct<T> {
    pub fn number(&self) -> &usize {
        &self.number
    }
    ///Returns a copy of generated
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Returns a copy of generated
    pub fn generic(&self) -> T {
        self.generic
    }
    pub fn another(&self) -> usize {
        self.another
    }
    pub fn static_str(&self) -> &'static str {
        self.static_str
    }
}
struct HoldsAReference<'a> {
    #[fieldwork(get)]
    name: &'a str,
}
impl<'a> HoldsAReference<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }
}
