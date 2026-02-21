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

/// Enum: vis override at the enum level
#[derive(fieldwork::Fieldwork)]
#[fieldwork(vis = "pub(crate)", get, set)]
enum State {
    Active { value: i32 },
    Inactive { value: i32 },
}

/// Item-method-level vis: get(vis = ...) in the item attribute restricts all getters
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(vis = "pub(crate)"), set)]
struct MethodVis {
    value: i32,
    name: String,
}

/// Enum: field-level vis override
#[derive(fieldwork::Fieldwork)]
#[fieldwork(vis = "pub(crate)", get, set)]
enum Mixed {
    Alpha {
        /// pub(crate) by default
        shared: i32,
        /// this field overrides to pub
        #[fieldwork(get(vis = "pub"))]
        prominent: String,
    },
    Beta {
        shared: i32,
        #[fieldwork(vis = "pub")]
        prominent: String,
    },
}
