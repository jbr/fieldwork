#[derive(fieldwork::Fieldwork)]
#[fieldwork(vis = "pub(crate)", get, set)]
struct MyStruct {
    /// default visibility (should be `pub(crate)`)
    number: usize,

    /// this one overrides only the getter to be fully pub
    #[fieldwork(get(vis = "pub"))]
    enabled: bool,

    /// this one overrides only the getter to be private (no `pub`)
    #[fieldwork(get(vis = ""))]
    active: bool,

    #[fieldwork(vis = "pub")]
    other: (),

    #[fieldwork(vis = "pub", get(vis = ""))]
    double_override: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct MyStruct2 {
    /// default visibility
    number: usize,

    /// this one overrides only the getter to be fully pub
    #[fieldwork(get(vis = "pub(crate)"))]
    enabled: bool,

    /// this one overrides only the getter to be private (no `pub`)
    #[fieldwork(get(vis = ""))]
    active: bool,

    #[fieldwork(vis = "pub(crate)")]
    other: (),

    #[fieldwork(vis = "pub(crate)", get(vis = ""))]
    method_override_field: (),
}
