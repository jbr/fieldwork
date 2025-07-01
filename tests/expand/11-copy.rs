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

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(copy))]
struct AllCopyTypes {
    char: char,
    f32: f32,
    f64: f64,
    i128: i128,
    i16: i16,
    i32: i32,
    i8: i8,
    isize: isize,
    u128: u128,
    u16: u16,
    u32: u32,
    u8: u8,
    usize: usize,
    bool: bool,
}
