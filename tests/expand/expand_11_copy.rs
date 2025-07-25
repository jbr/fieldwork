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
#[fieldwork(get)]
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

mod x {
    use std::sync::Arc;
    #[derive(fieldwork::Fieldwork)]
    #[fieldwork(get)]
    struct CopyInteractsWithDeref {
        box_bool: Box<bool>,
        arc_usize: Arc<usize>,
    }
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct AllowSpecifyingCopyAtFieldAttribute<T: Copy> {
    #[fieldwork(copy)]
    generic: T,

    #[fieldwork(copy = true)]
    another: (T, T),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct AllowSpecifyingCopyFalseAtFieldAttribute {
    #[fieldwork(copy = false)]
    usize: usize,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, copy = false)]
struct AllowOptingBackInAtFieldAttribute<T: Copy> {
    #[fieldwork(copy = true)]
    usize: usize,

    #[field(copy)]
    generic: T,
}
