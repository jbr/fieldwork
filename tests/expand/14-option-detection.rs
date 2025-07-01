#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,

    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option_borrow_inner = false)]
    no_option_detection: Option<String>,

    option_detection: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner = false))]
    nothing_fancy_for_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option_borrow_inner = false)]
struct OptInOption {
    #[fieldwork(deref = "Option<&str>", option_borrow_inner = true)]
    option_deref: Option<String>,

    #[fieldwork(deref = str, option_borrow_inner = true)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option_borrow_inner)]
    option_detection: Option<String>,

    no_option_detection: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner = true))]
    option_detection_only_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(option_borrow_inner = true), get_mut, option_borrow_inner = false)]
struct OptionOnlyForGet {
    #[fieldwork(get(option_borrow_inner = false))]
    no_option_detection: Option<()>,

    #[fieldwork(get(deref = str))]
    option_deref: Option<String>,

    #[fieldwork(option_borrow_inner = true)]
    field_overrides: Option<String>,

    option_only_for_get: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner), get(option_borrow_inner = false))]
    option_detection_only_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option_borrow_inner = false, deref = false)]
struct OptionAndDerefInteraction {
    a: Option<String>,
    #[fieldwork(option_borrow_inner)]
    b: Option<String>,
    #[fieldwork(option_borrow_inner, deref)]
    c: Option<String>,
    #[fieldwork(deref)]
    d: Option<String>, // remains Option<String>
    #[fieldwork(option_borrow_inner, deref = "Option<&CustomDeref>")]
    e: Option<CustomOwned>,
}
