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

    #[fieldwork(get)]
    only_get: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get)]
struct OptingInPerField<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork(set = true, get(skip))]
    enabled: bool,

    /// generated
    #[fieldwork]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get(template = "get_{}"))]
struct OptingInPerField<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork(set)]
    enabled: bool,

    /// generated
    #[fieldwork(get, set = true)]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct {
    number: usize,
    #[fieldwork(opt_in = true, get)]
    only_get: (),
}
