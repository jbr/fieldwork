/// Struct with a where clause (the basic case from issue #105)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with)]
struct MyStruct<T>
where
    T: Clone,
{
    field: T,
}

/// Enum with a where clause
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Container<T>
where
    T: Clone,
{
    Filled { value: T, label: String },
    Empty { label: String },
}

/// Struct with both inline bounds and a where clause
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, with)]
struct Mixed<T: std::fmt::Debug, U>
where
    U: Clone,
{
    debug_field: T,
    clone_field: U,
}

/// Struct with both #[fieldwork(bounds = "...")] and a struct-level where clause — predicates merged
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, bounds = "U: std::fmt::Display")]
struct WithExplicitBounds<T, U>
where
    T: Clone,
{
    cloneable: T,
    displayable: U,
}
