#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, get, get_mut, option_set_some)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,
    #[fieldwork(option_set_some = false)]
    no_option_detection: Option<bool>,
    option_detection: Option<()>,
    #[fieldwork(set(option_set_some = false, option_borrow_inner = false))]
    nothing_fancy_for_set: Option<String>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(with(option_set_some))]
struct BobTheBuilder {
    string: Option<String>,
    bool: Option<bool>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, get, get_mut, option_set_some)]
struct HandlesNonOptionTypes {
    string: String,
    bool: bool,
}

/// Enum: option_set_some on full-coverage Option fields
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, get, option_set_some)]
enum EnumWithOptions {
    HasBoth {
        name: Option<String>,
        tag: Option<u32>,
    },
    HasName {
        name: Option<String>,
        tag: Option<u32>,
    },
}

/// Enum: option_set_some = false to opt out per field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, option_set_some)]
enum SelectiveOptionSet {
    A {
        #[fieldwork(option_set_some = false)]
        no_some: Option<String>,
        with_some: Option<u32>,
    },
    B {
        no_some: Option<String>,
        with_some: Option<u32>,
    },
}
