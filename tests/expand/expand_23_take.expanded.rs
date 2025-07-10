struct MyStruct {
    #[field(take)]
    string: Option<String>,
}
impl MyStruct {
    pub fn take_string(&mut self) -> Option<String> {
        self.string.take()
    }
}
#[fieldwork(take)]
struct MyStruct2 {
    /// the users's name
    name: Option<String>,
    #[take = false]
    not_take: Option<()>,
}
impl MyStruct2 {
    ///Takes the users's name, leaving a None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }
    pub fn take_not_take(&mut self) -> Option<()> {
        self.not_take.take()
    }
}
#[fieldwork(take(template = "remove_{}"))]
struct MyStruct3 {
    /// the user's name
    name: Option<String>,
    ignored: bool,
}
impl MyStruct3 {
    ///Takes the user's name, leaving a None in its place
    pub fn remove_name(&mut self) -> Option<String> {
        self.name.take()
    }
}
