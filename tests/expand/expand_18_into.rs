#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct AcceptsAnythingInto {
    #[fieldwork(into)]
    string: String,

    #[fieldwork(option_set_some, into)]
    option_string: Option<String>,
}

/// Enum: into on full-coverage field; annotation on one variant applies to the whole field.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with)]
enum IntoEnum {
    Alpha {
        #[fieldwork(into)]
        name: String,
        value: i32,
    },
    Beta {
        name: String,
        value: i32,
    },
}

/// Enum: into on partial-coverage field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
enum PartialInto {
    Full {
        shared: i32,
        #[fieldwork(into)]
        label: String,
    },
    Minimal {
        shared: i32,
    },
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, into, with)]
struct Test {
    field: Option<std::borrow::Cow<'static, str>>,
}
