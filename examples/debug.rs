#![allow(dead_code)]

#[derive(fieldwork::Fieldwork, Default)]
#[fieldwork(get, get_mut, option = false, deref = false)]
struct User {
    a: Option<String>,
    #[fieldwork(option)]
    b: Option<String>,
    #[fieldwork(option, deref)]
    c: Option<String>,
    #[fieldwork(deref)]
    d: Option<String>,
}

fn main() {}
