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

/// Enum: generic with explicit bounds
#[derive(fieldwork::Fieldwork)]
#[fieldwork(bounds = "T: Clone", get, set)]
enum Container<T> {
    Filled { value: T, label: String },
    Empty { label: String },
}
