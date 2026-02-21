#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set, with, get_mut)]
struct MyStruct<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork]
    enabled: bool,

    /// generated
    #[fieldwork]
    generic: T,

    #[fieldwork(get)]
    only_get: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set, with, get_mut)]
struct FieldworkEqualsTrue<T> {
    /// not generated
    number: usize,

    /// generated
    #[fieldwork = true]
    enabled: bool,

    /// generated
    #[field = true]
    generic: T,
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
struct OptingInPerField2<T> {
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
struct MyStruct2 {
    number: usize,
    #[fieldwork(opt_in = true, get)]
    only_get: (),
}

/// Enum: opt_in — only annotated fields get methods
#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set)]
enum OptInEnum {
    Foo {
        /// generated
        #[fieldwork]
        name: String,
        /// not generated
        internal: u32,
    },
    Bar {
        /// also generated (using #[field])
        #[field]
        name: String,
        /// not generated
        data: Vec<u8>,
    },
}

/// Enum: opt_in per-field with method-level opt-in
#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set)]
enum SelectiveMethods {
    Alpha {
        #[fieldwork(get)]
        read_only: String,
        #[fieldwork(set)]
        write_only: String,
    },
    Beta {
        #[fieldwork]
        read_only: String,
        #[fieldwork]
        write_only: String,
    },
}
