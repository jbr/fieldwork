/// Copy fields (u8) are skipped; non-Copy (String) gets into_field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct Mixed {
    red: u8,
    green: u8,
    blue: u8,
    name: String,
}

/// Newtype with #[field = "inner"] produces the conventional into_inner
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct Wrapper(#[field = "inner"] String);

/// Per-field opt-in alongside a copy type that would be skipped anyway
#[derive(fieldwork::Fieldwork)]
struct OptIn {
    #[field(into_field)]
    name: String,
    id: u32,
}

/// Explicit copy=true on a non-Copy type: into_field should be skipped
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct ExplicitCopyOnString {
    #[field(copy)]
    label: String,
    value: String,
}

/// Explicit copy=false on a Copy type: into_field should be generated
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct ExplicitNoCopyOnU8 {
    #[field(copy = false)]
    id: u32,
    name: String,
}

/// Option<CopyType>: skipped via borrow_inner + copy detection
/// Option<NonCopyType>: generated
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct OptionFields {
    opt_copy: Option<u8>,
    opt_string: Option<String>,
}

/// Enum: into_field skips Copy types (same as struct behavior)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum Coords {
    TwoD { x: i32, y: i32 },
    ThreeD { x: i32, y: i32 },
}

/// Enum: into_field with copy = false on Copy type → generates into_field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum ExplicitNoCopy {
    A {
        #[field(copy = false)]
        id: u32,
    },
    B {
        #[field(copy = false)]
        id: u32,
    },
}

/// Enum: into_field on non-Copy full coverage
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum Messages {
    Simple { text: String, code: u32 },
    Detailed { text: String, code: u32 },
}

/// Enum: into_field on partial non-Copy field — nothing generates (same as set)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum PartialOwned {
    Rich { name: String, code: u32 },
    Simple { code: u32 },
}
