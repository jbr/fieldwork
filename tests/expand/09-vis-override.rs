#[derive(fieldwork::Fieldwork)]
#[fieldwork(vis = "pub(crate)", get, set, update)]
struct MyStruct {
    /// default visibility (should be `pub(crate)`)
    number: usize,

    /// this one overrides only the getter to be fully pub
    #[fieldwork(get(vis = "pub"))]
    enabled: bool,

    /// this one overrides only the getter to be private (no `pub`)
    #[fieldwork(get(vis = ""))]
    active: bool,
}
