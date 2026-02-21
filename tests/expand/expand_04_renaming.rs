#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(argument = tee)]
    generic: T,

    #[fieldwork(get = "get_another")]
    another: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(template = "get_{}"), set(template = "put_{}"))]
struct WithTemplate<T> {
    /// this number is cool
    #[fieldwork(rename = "number_in_seconds")]
    number: usize,
    /// is this struct on or not
    #[fieldwork(get = is_it_enabled)]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(rename = "put_the_generic"))]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut, without)]
struct RenameWithEquals {
    /// this number is cool
    #[fieldwork = "number_in_seconds"]
    number: usize,
    /// is this struct on or not
    #[field = "setting_enabled"]
    enabled: bool,
}

/// Enum: field rename with various syntaxes (same field name across variants = full coverage)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum RenameEnum {
    Metric {
        #[fieldwork(rename = radius_px)]
        radius: f64,
        #[field = "label"]
        name: String,
    },
    Imperial {
        #[fieldwork(rename = radius_px)]
        radius: f64,
        #[field = "label"]
        name: String,
    },
}
