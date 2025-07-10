#[derive(fieldwork::Fieldwork)]
struct MyStruct {
    #[field(take)]
    string: Option<String>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(take)]
struct MyStruct2 {
    /// the users's name
    name: Option<String>,
    #[take = false]
    not_take: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(take(template = "remove_{}"))]
struct MyStruct3 {
    /// the user's name
    name: Option<String>,
    ignored: bool,
}
