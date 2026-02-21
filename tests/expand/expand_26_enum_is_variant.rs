/// Basic is_variant: struct, tuple, and unit variants
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle,
}

/// Variant name override: #[variant = "name"] changes the method name
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant)]
enum Protocol {
    #[variant = "http"]
    HTTP,
    #[variant = "https"]
    HTTPS,
    FTP,
}

/// Template drops the is_ prefix entirely
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant(template = "{}"))]
enum Status {
    Active,
    Inactive,
    Pending,
}

/// Template uses a different prefix
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant(template = "check_{}"))]
enum Connection {
    Open,
    Closed,
    Connecting,
}

/// Skip a specific variant's predicate via is_variant = false
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant)]
enum Event {
    Click,
    #[variant(is_variant = false)]
    Internal,
    Move,
}

/// #[variant(skip)] skips all generated methods for that variant
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant, get)]
enum Message {
    Text { content: String },
    Image { url: String },
    #[variant(skip)]
    Hidden { data: String },
}

/// is_variant combined with field accessors
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant, get, set)]
enum State {
    Active { value: i32, label: String },
    Inactive { value: i32, label: String },
}

/// Multi-word variant names convert to snake_case
#[derive(fieldwork::Fieldwork)]
#[fieldwork(is_variant)]
enum HttpStatus {
    NotFound,
    InternalServerError,
    Ok,
}
