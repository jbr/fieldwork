/// `#[variant(skip)]` excludes a variant: only active variants contribute fields
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Message {
    Text {
        content: String,
    },
    Image {
        url: String,
    },
    #[variant(skip)]
    Hidden {
        data: String,
    },
}

/// `#[variant = false]` is an alternative skip syntax
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Status {
    Active {
        value: i32,
    },
    #[variant = false]
    Deprecated {
        value: i32,
    },
    Inactive {
        value: i32,
    },
}

/// `#[variant(skip = true)]` explicitly skips a variant
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Command {
    Write {
        data: String,
    },
    #[variant(skip = true)]
    Noop {},
    Read {
        data: String,
    },
}

/// `#[variant(skip = false)]` explicitly opts in (same as no attribute)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Priority {
    High {
        level: u8,
    },
    #[variant(skip = false)]
    Low {
        level: u8,
    },
}

/// bare `#[variant]` is a no-op
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Mode {
    #[variant]
    Normal {
        speed: u32,
    },
    Fast {
        speed: u32,
    },
}
