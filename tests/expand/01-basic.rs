#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// the number
    number: usize,
    /// whether something is enabled
    enabled: bool,
    /// the generic
    generic: T,
}
