#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,

    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option = false)]
    no_option_detection: Option<String>,

    option_detection: Option<()>,

    #[fieldwork(get_mut(option = false))]
    nothing_fancy_for_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option = false)]
struct OptInOption {
    #[fieldwork(deref = "Option<&str>", option = true)]
    option_deref: Option<String>,

    #[fieldwork(deref = str, option = true)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option)]
    option_detection: Option<String>,

    no_option_detection: Option<()>,

    #[fieldwork(get_mut(option = true))]
    option_detection_only_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(option = true), get_mut, option = false)]
struct OptionOnlyForGet {
    #[fieldwork(get(option = false))]
    no_option_detection: Option<()>,

    #[fieldwork(get(deref = str))]
    option_deref: Option<String>,

    #[fieldwork(option = true)]
    field_overrides: Option<String>,

    option_only_for_get: Option<()>,

    #[fieldwork(get_mut(option), get(option = false))]
    option_detection_only_get_mut: Option<()>,
}
