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
