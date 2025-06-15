#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(template = "get_{}"), set(template = "assign_{}"), with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    generic: T,
}
