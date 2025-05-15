#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,

    /// generated
    #[fieldwork(set(chain = false))]
    enabled: bool,

    /// generated
    generic: T,
}
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(chain = false))]
struct MyStruct2<T> {
    number: usize,

    /// generated
    enabled: bool,

    /// generated
    generic: T,
}
