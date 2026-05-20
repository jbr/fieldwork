//! Field- and method-level deprecation: bare-mark, rename-from, since/note.

#![allow(deprecated)]

/// Field-level rename: emits canonical methods plus deprecated alternates
/// derived from the old binding base.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with)]
struct RenamedField {
    /// the user's display name
    #[field(deprecate = "name")]
    display_name: String,
}

/// Bare deprecate marks every canonical method as `#[deprecated]`.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct BareMark {
    #[field(deprecate)]
    legacy: bool,
}

/// Method-level deprecate uses the old name literally for that method only.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct MethodLevel {
    #[field(get(deprecate = "is_admin"))]
    admin: bool,
}

/// `since` and `note` populate the deprecated attribute. Field-level rename
/// applies the same attribute to every deprecated alternate.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with)]
struct WithSinceAndNote {
    #[field(deprecate(was = "old_score", since = "1.2.0", note = "use score() instead"))]
    score: u32,
}

/// Method-level bare deprecate marks only that one method as deprecated.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct MethodLevelBare {
    #[field(set(deprecate))]
    counter: u32,
}

/// Rename combined with deprecate: methods derive from the new name and the
/// deprecated alternates derive from the supplied old base.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct RenameAndDeprecate {
    /// the field's stored value
    #[field(rename = new_name, deprecate = "old_name")]
    stored: String,
}

/// Enum: putting `deprecate` on one variant covers the unified virtual field.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Packet {
    Data {
        #[field(deprecate = "ident")]
        id: u32,
        payload: String,
    },
    Heartbeat {
        id: u32,
    },
}
