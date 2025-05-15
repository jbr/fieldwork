#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,
    /// it's really whatever you want
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(
    set(doc_template = " # ssssets {}"),
    get(doc_template = "# ggggets {}"),
    with(doc_template = "# width {}"),
    update(doc_template = "# updates {}"),
    get_mut(doc_template = "# gmut {}")
)]
struct DocTemplates<T> {
    /// the cool number
    number: usize,

    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,

    /// the generic
    generic: T,
}
