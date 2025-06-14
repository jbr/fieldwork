#[derive(fieldwork::Fieldwork)]
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

#[derive(fieldwork::Fieldwork)]
struct HoldsAReference<'a> {
    #[fieldwork(get)]
    name: &'a str,
}
