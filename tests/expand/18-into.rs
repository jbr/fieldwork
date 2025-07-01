#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct AcceptsAnythingInto {
    #[fieldwork(into)]
    string: String,

    #[fieldwork(option_set_some, into)]
    option_string: Option<String>,
}
