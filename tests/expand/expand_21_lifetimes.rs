use std::borrow::Cow;

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, with, without, set(option_set_some))]
struct MyStruct<'a> {
    borrow: &'a (),
    mut_borrow: &'a mut (),
    #[field(into)]
    cow: Cow<'a, str>,
    box_dyn_trait: Box<dyn std::fmt::Debug + 'a>,
    option_lifetime: Option<&'a str>,
}
