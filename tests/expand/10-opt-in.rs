#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set, with, get_mut)]
struct MyStruct<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork(rename = is_enabled)]
    enabled: bool,

    /// generated
    #[fieldwork]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get)]
struct OptingInPerField<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork(set, get(skip))]
    enabled: bool,

    /// generated
    #[fieldwork]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in)]
struct OptingInPerField<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork(set)]
    enabled: bool,

    /// generated
    #[fieldwork(get, set)]
    generic: T,
}
