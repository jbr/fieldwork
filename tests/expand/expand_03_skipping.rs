#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(skip)]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(skip = true)]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct AnotherInterface {
    number: usize,
    #[fieldwork = false]
    enabled: bool,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, get)]
struct SetAndGet<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(get(skip = true))]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(skip))]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct SkipWithAssignment<T> {
    #[fieldwork(with = false)]
    no_with: bool,
    #[fieldwork(get = false)]
    no_get: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut = false)] // parses just fine, but noops
struct GetMutEqualsFalseDoesNothing<T> {
    field: T,
}

/// Enum: skip a specific field; it's excluded from all method generation
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum SkipField {
    Alpha {
        name: String,
        #[field(skip)]
        internal: u32,
    },
    Beta {
        name: String,
    },
}

/// Enum: #[variant(skip)] makes all fields in that variant behave as if absent
/// → partial coverage (Option return + `_ => None`) even for fields shared with other variants
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum SkipVariant {
    Active {
        value: i32,
    },
    #[variant(skip)]
    Debug {
        value: i32,
        extra: String,
    },
    Inactive {
        value: i32,
    },
}

/// Enum: per-method field skip
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum PerMethodSkip {
    Foo {
        #[fieldwork(get(skip))]
        write_only: i32,
        #[fieldwork(set(skip))]
        read_only: i32,
    },
    Bar {
        write_only: i32,
        read_only: i32,
    },
}
