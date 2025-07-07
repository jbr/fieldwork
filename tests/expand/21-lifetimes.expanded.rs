#[fieldwork(get, get_mut)]
struct MyStruct<'a> {
    borrow: &'a (),
    mut_borrow: &'a mut (),
    cow: Cow<'a, str>,
}
impl<'a> MyStruct<'a> {
    pub fn borrow(&self) -> &'a () {
        self.borrow
    }
    pub fn borrow_mut(&mut self) -> &mut &'a () {
        &mut self.borrow
    }
    pub fn mut_borrow(&self) -> &() {
        &*self.mut_borrow
    }
    pub fn mut_borrow_mut(&mut self) -> &mut () {
        &mut *self.mut_borrow
    }
    pub fn cow(&self) -> &str {
        &*self.cow
    }
    pub fn cow_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.cow
    }
}
