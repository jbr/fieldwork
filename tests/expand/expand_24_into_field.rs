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
