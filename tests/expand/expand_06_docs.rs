#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,
    /// it's really whatever you want
    ///
    /// Some more information about this type:
    /// - it could really be anything
    /// - we don't know much more than that
    generic: T,
}

/// Enum: doc comments on variants and fields
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum LogLevel {
    /// A debug-level message
    Debug { message: String },
    /// A warning-level message
    Warn {
        /// The warning message content
        #[fieldwork(
            get(doc = "borrow the warning text"),
            set(doc = "update the warning text")
        )]
        message: String,
    },
}

/// Enum: doc_template applies to all methods
#[derive(fieldwork::Fieldwork)]
#[fieldwork(
    get(doc_template = "# gets {} from this variant"),
    set(doc_template = "# sets {} on this variant")
)]
enum Documented {
    /// the first variant
    First {
        /// the value field
        value: String,
    },
    /// the second variant
    Second { value: String },
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(
    set(doc_template = " # ssssets {}

extra info here"),
    get(doc_template = "# ggggets {}"),
    with(doc_template = "# width {}"),
    get_mut(doc_template = "# gmut {}")
)]
struct DocTemplates<T> {
    /// the cool number
    number: usize,

    #[fieldwork(
        get(doc = "get whether it's enabled"),
        set(doc = "assign enabled"),
        with(doc = "chainable setter for enabled"),
        get_mut(doc = "mutably borrow enabled")
    )]
    enabled: bool,

    /// the generic
    ///
    /// Some further info
    generic: T,
}
