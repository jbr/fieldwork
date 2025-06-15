#[fieldwork(get)]
struct MyStruct<T: Copy> {
    number: usize,
    enabled: bool,
    #[fieldwork(get(copy))]
    generic: T,
    #[fieldwork(get(copy = true))]
    another: usize,
    static_str: &'static str,
}
impl<T: Copy> MyStruct<T> {
    pub fn number(&self) -> usize {
        self.number
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
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
#[fieldwork(get)]
struct HoldsAReference<'a> {
    name: &'a str,
    mut_ref_not_copy: &'a mut (),
}
impl<'a> HoldsAReference<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn mut_ref_not_copy(&self) -> &&'a mut () {
        &self.mut_ref_not_copy
    }
}
