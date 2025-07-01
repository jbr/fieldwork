#[fieldwork(set, with, get, get_mut, option_set_some)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,
    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,
    #[fieldwork(option_set_some = false)]
    no_option_detection: Option<bool>,
    option_detection: Option<()>,
    #[fieldwork(set(option_set_some = false, option = false))]
    nothing_fancy_for_set: Option<String>,
}
