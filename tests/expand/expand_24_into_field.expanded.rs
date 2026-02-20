/// Copy fields (u8) are skipped; non-Copy (String) gets into_field
#[fieldwork(into_field)]
struct Mixed {
    red: u8,
    green: u8,
    blue: u8,
    name: String,
}
impl Mixed {
    pub fn into_name(self) -> String {
        self.name
    }
}
/// Newtype with #[field = "inner"] produces the conventional into_inner
#[fieldwork(into_field)]
struct Wrapper(#[field = "inner"] String);
impl Wrapper {
    pub fn into_inner(self) -> String {
        self.0
    }
}
/// Per-field opt-in alongside a copy type that would be skipped anyway
struct OptIn {
    #[field(into_field)]
    name: String,
    id: u32,
}
impl OptIn {
    pub fn into_name(self) -> String {
        self.name
    }
}
/// Explicit copy=true on a non-Copy type: into_field should be skipped
#[fieldwork(into_field)]
struct ExplicitCopyOnString {
    #[field(copy)]
    label: String,
    value: String,
}
impl ExplicitCopyOnString {
    pub fn into_value(self) -> String {
        self.value
    }
}
/// Explicit copy=false on a Copy type: into_field should be generated
#[fieldwork(into_field)]
struct ExplicitNoCopyOnU8 {
    #[field(copy = false)]
    id: u32,
    name: String,
}
impl ExplicitNoCopyOnU8 {
    pub fn into_id(self) -> u32 {
        self.id
    }
    pub fn into_name(self) -> String {
        self.name
    }
}
/// Option<CopyType>: skipped via borrow_inner + copy detection
/// Option<NonCopyType>: generated
#[fieldwork(into_field)]
struct OptionFields {
    opt_copy: Option<u8>,
    opt_string: Option<String>,
}
impl OptionFields {
    pub fn into_opt_string(self) -> Option<String> {
        self.opt_string
    }
}
