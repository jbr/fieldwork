#[derive(fieldwork::Fieldwork)]
struct MyStruct<T> {
    number: usize,

    /// generated
    #[fieldwork(get(copy))]
    enabled: bool,

    /// generated
    generic: T,

    #[fieldwork(get(copy = true))]
    another: bool,
}
