#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, get = true)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    generic: T,
}
