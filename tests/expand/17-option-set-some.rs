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
