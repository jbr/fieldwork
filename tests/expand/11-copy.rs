#[derive(fieldwork::Fieldwork)]
struct MyStruct<T: Copy> {
    number: usize,

    /// generated
    enabled: bool,

    #[fieldwork(get(copy))]
    /// generated
    generic: T,

    #[fieldwork(get(copy = true))]
    another: usize,
}
