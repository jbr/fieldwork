#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct MyStruct<T: Copy> {
    number: usize,

    enabled: bool,

    #[fieldwork(get(copy))]
    generic: T,

    #[fieldwork(get(copy = true))]
    another: usize,

    static_str: &'static str,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct HoldsAReference<'a> {
    name: &'a str,

    mut_ref_not_copy: &'a mut (),
}
