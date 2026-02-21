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

/// Enum: lifetime parameters on fields (same type across variants → full coverage)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with)]
enum Borrowed<'a> {
    Short { content: &'a str, id: u32 },
    Long { content: &'a str, id: u32 },
}

/// Enum: Option with lifetime
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum WithOptionalBorrow<'a> {
    Named { name: &'a str, tag: Option<&'a str> },
    Anonymous { tag: Option<&'a str> },
}
