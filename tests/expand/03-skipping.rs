#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(skip)]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, get)]
struct SetAndGet<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(get(skip))]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(skip))]
    generic: T,
}
