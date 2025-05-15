#[derive(fieldwork::Fieldwork)]
#[fieldwork(bounds = "T: Clone", get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// must be clone
    generic: T,
}
