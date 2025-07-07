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
impl<T: Copy> MyStruct<T> {
    pub fn number(&self) -> usize {
        self.number
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn generic(&self) -> T {
        self.generic
    }
    pub fn another(&self) -> usize {
        self.another
    }
    pub fn static_str(&self) -> &'static str {
        self.static_str
    }
}
#[fieldwork(get)]
struct HoldsAReference<'a> {
    name: &'a str,
    mut_ref_not_copy: &'a mut (),
}
impl<'a> HoldsAReference<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn mut_ref_not_copy(&self) -> &() {
        &*self.mut_ref_not_copy
    }
}
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
impl AllCopyTypes {
    pub fn char(&self) -> char {
        self.char
    }
    pub fn f32(&self) -> f32 {
        self.f32
    }
    pub fn f64(&self) -> f64 {
        self.f64
    }
    pub fn i128(&self) -> i128 {
        self.i128
    }
    pub fn i16(&self) -> i16 {
        self.i16
    }
    pub fn i32(&self) -> i32 {
        self.i32
    }
    pub fn i8(&self) -> i8 {
        self.i8
    }
    pub fn isize(&self) -> isize {
        self.isize
    }
    pub fn u128(&self) -> u128 {
        self.u128
    }
    pub fn u16(&self) -> u16 {
        self.u16
    }
    pub fn u32(&self) -> u32 {
        self.u32
    }
    pub fn u8(&self) -> u8 {
        self.u8
    }
    pub fn usize(&self) -> usize {
        self.usize
    }
    pub fn bool(&self) -> bool {
        self.bool
    }
}
#[fieldwork(get)]
struct CopyInteractsWithDeref {
    box_bool: Box<bool>,
    arc_usize: Arc<usize>,
}
impl CopyInteractsWithDeref {
    pub fn box_bool(&self) -> bool {
        *self.box_bool
    }
    pub fn arc_usize(&self) -> usize {
        *self.arc_usize
    }
}
#[fieldwork(get)]
struct AllowSpecifyingCopyAtFieldAttribute<T: Copy> {
    #[fieldwork(copy)]
    generic: T,
    #[fieldwork(copy = true)]
    another: (T, T),
}
impl<T: Copy> AllowSpecifyingCopyAtFieldAttribute<T> {
    pub fn generic(&self) -> T {
        self.generic
    }
    pub fn another(&self) -> (T, T) {
        self.another
    }
}
#[fieldwork(get)]
struct AllowSpecifyingCopyFalseAtFieldAttribute<T: Copy> {
    #[fieldwork(copy = false)]
    usize: usize,
}
impl<T: Copy> AllowSpecifyingCopyFalseAtFieldAttribute<T> {
    pub fn usize(&self) -> &usize {
        &self.usize
    }
}
#[fieldwork(get, copy = false)]
struct AllowOptingBackInAtFieldAttribute<T: Copy> {
    #[fieldwork(copy = true)]
    usize: usize,
    #[field(copy)]
    generic: T,
}
impl<T: Copy> AllowOptingBackInAtFieldAttribute<T> {
    pub fn usize(&self) -> usize {
        self.usize
    }
    pub fn generic(&self) -> T {
        self.generic
    }
}
