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

/// Enum: custom method templates on get/set
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(template = "fetch_{}"), set(template = "assign_{}"))]
enum Config {
    Dev { host: String, port: u16 },
    Prod { host: String, port: u16 },
}
