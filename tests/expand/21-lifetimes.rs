#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct MyStruct<'a> {
    borrow: &'a (),
    mut_borrow: &'a mut (),
    cow: Cow<'a, str>,
    box_dyn_trait: Box<dyn Clone + 'a>,
}
