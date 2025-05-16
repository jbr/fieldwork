#[derive(fieldwork::Fieldwork)]
struct User {
    /// the user's name
    #[fieldwork(deref = str, get, set, get_mut)]
    name: String,

    #[fieldwork(deref = "[u8]", get, set, get_mut)]
    arr: Vec<u8>,
}

#[derive(fieldwork::Fieldwork)]
struct OnlyDerefForMethods {
    /// the user's name
    #[fieldwork(get(deref = str), set, get_mut)]
    name: String,

    #[fieldwork(get, set, get_mut(deref = "[u8]"))]
    arr: Vec<u8>,
}
