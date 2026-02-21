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

/// Enum: Copy types auto-detected; full coverage returns T, partial returns Option<T>
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Coord {
    TwoD { x: f64, y: f64 },
    ThreeD { x: f64, y: f64, z: f64 },
}

/// Enum: copy = false on a Copy type → returns &T even for Copy types
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum NoCopyEnum {
    A {
        #[fieldwork(copy = false)]
        code: u32,
    },
    B {
        #[fieldwork(copy = false)]
        code: u32,
    },
}

/// Enum: copy = false at enum level, opting back in per-field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, copy = false)]
enum CopyOverride {
    A {
        #[fieldwork(copy = true)]
        id: u32,
        name: String,
    },
    B {
        #[fieldwork(copy = true)]
        id: u32,
        name: String,
    },
}
