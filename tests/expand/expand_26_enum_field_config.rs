/// #[field = false] on one occurrence vetoes the virtual field for all methods.
/// value is in all three variants, but Debug's annotation vetoes it — no value() or set_value().
/// extra only exists in Debug and is also vetoed — no extra() either.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum GlobalVeto {
    Active {
        value: i32,
    },
    Debug {
        #[field = false]
        value: i32,
        #[field = false]
        extra: String,
    },
    Inactive {
        value: i32,
    },
}

/// #[field(get = false)] on one occurrence vetoes only the get method.
/// No x() getter, but set_x() is generated with full coverage across all three variants.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum GetVeto {
    A {
        x: i32,
    },
    B {
        #[field(get = false)]
        x: i32,
    },
    C {
        x: i32,
    },
}

/// Order independence: annotated variant is last, not first.
/// Should behave identically to PerMethodSkip in expand_03 regardless of variant order.
/// No write_only() getter (vetoed by Foo), no set_read_only() (vetoed by Foo).
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum PerMethodSkipAnnotatedLast {
    Bar {
        write_only: i32,
        read_only: i32,
    },
    Foo {
        #[field(get = false)]
        write_only: i32,
        #[field(set = false)]
        read_only: i32,
    },
}

/// Field-level opt-in with no item-level #[fieldwork].
/// B's annotation opts the x virtual field into get; all occurrences (A, B, C) participate.
/// Full coverage (3/3) → x() -> i32.
#[derive(fieldwork::Fieldwork)]
enum OptInFull {
    A {
        x: i32,
    },
    B {
        #[field(get)]
        x: i32,
    },
    C {
        x: i32,
    },
}

/// Field-level opt-in where the field is structurally absent from some variants.
/// B opts in `data`; A doesn't have `data`. Coverage: 2 arms / 3 variants → Option<&str>.
#[derive(fieldwork::Fieldwork)]
enum OptInPartial {
    A {
        id: u32,
    },
    B {
        id: u32,
        #[field(get)]
        data: String,
    },
    C {
        id: u32,
        data: String,
    },
}

/// Type mismatch with no field-level annotation: silently generates nothing for `x`.
/// The `id` field is consistent and generates normally.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum TypeMismatchSilent {
    A { id: u32, x: i32 },
    B { id: u32, x: String },
}
