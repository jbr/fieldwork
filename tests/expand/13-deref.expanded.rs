struct User {
    /// the user's name
    #[fieldwork(deref = str, get, set, get_mut)]
    name: String,
    #[fieldwork(deref = "[u8]", get, set, get_mut)]
    arr: Vec<u8>,
}
impl User {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn arr(&self) -> &[u8] {
        &self.arr
    }
    pub fn arr_mut(&mut self) -> &mut [u8] {
        &mut self.arr
    }
    pub fn set_arr(&mut self, arr: Vec<u8>) -> &mut Self {
        self.arr = arr;
        self
    }
}
struct OnlyDerefForMethods {
    /// the user's name
    #[fieldwork(get(deref = str), set, get_mut)]
    name: String,
    #[fieldwork(get, set, get_mut(deref = "[u8]"))]
    arr: Vec<u8>,
}
impl OnlyDerefForMethods {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn arr(&self) -> &Vec<u8> {
        &self.arr
    }
    pub fn arr_mut(&mut self) -> &mut [u8] {
        &mut self.arr
    }
    pub fn set_arr(&mut self, arr: Vec<u8>) -> &mut Self {
        self.arr = arr;
        self
    }
}
