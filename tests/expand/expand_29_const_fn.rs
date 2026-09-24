#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, const_fn)]
struct Counters {
    /// how many times this has happened
    count: usize,
    /// whether the thing is on
    enabled: bool,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Mixed {
    /// borrowed by an ordinary getter
    name: String,
    /// read in a const context
    #[field(const_fn)]
    port: u16,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, const_fn)]
struct Override {
    /// read in a const context
    port: u16,
    /// opted back out of const
    #[field(const_fn = false)]
    name: String,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, const_fn, deref = false)]
struct NoDeref {
    /// borrowed without auto-deref
    bytes: [u8; 4],
    /// borrowed without auto-deref
    label: Option<String>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(const_fn), set, with, without, take)]
struct MethodLevel {
    /// only the getter is const
    enabled: bool,
    /// only the getter is const
    retries: Option<u8>,
}
