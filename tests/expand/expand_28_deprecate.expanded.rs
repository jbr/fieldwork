//! Field- and method-level deprecation: bare-mark, rename-from, since/note.
#![allow(deprecated)]
/// Field-level rename: emits canonical methods plus deprecated alternates
/// derived from the old binding base.
#[fieldwork(get, set, with)]
struct RenamedField {
    /// the user's display name
    #[field(deprecate = "name")]
    display_name: String,
}
impl RenamedField {
    ///Borrows the user's display name
    pub fn display_name(&self) -> &str {
        &*self.display_name
    }
    ///Borrows the user's display name
    #[deprecated(note = "use `display_name` instead")]
    pub fn name(&self) -> &str {
        &*self.display_name
    }
    ///Sets the user's display name, returning `&mut Self` for chaining
    pub fn set_display_name(&mut self, display_name: String) -> &mut Self {
        self.display_name = display_name;
        self
    }
    ///Sets the user's display name, returning `&mut Self` for chaining
    #[deprecated(note = "use `set_display_name` instead")]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.display_name = name;
        self
    }
    ///Owned chainable setter for the user's display name, returning `Self`
    #[must_use]
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = display_name;
        self
    }
    ///Owned chainable setter for the user's display name, returning `Self`
    #[deprecated(note = "use `with_display_name` instead")]
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.display_name = name;
        self
    }
}
/// Bare deprecate marks every canonical method as `#[deprecated]`.
#[fieldwork(get, set)]
struct BareMark {
    #[field(deprecate)]
    legacy: bool,
}
impl BareMark {
    #[deprecated]
    pub fn legacy(&self) -> bool {
        self.legacy
    }
    #[deprecated]
    pub fn set_legacy(&mut self, legacy: bool) -> &mut Self {
        self.legacy = legacy;
        self
    }
}
/// Method-level deprecate uses the old name literally for that method only.
#[fieldwork(get, set)]
struct MethodLevel {
    #[field(get(deprecate = "is_admin"))]
    admin: bool,
}
impl MethodLevel {
    pub fn admin(&self) -> bool {
        self.admin
    }
    #[deprecated(note = "use `admin` instead")]
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
}
/// `since` and `note` populate the deprecated attribute. Field-level rename
/// applies the same attribute to every deprecated alternate.
#[fieldwork(get, set, with)]
struct WithSinceAndNote {
    #[field(deprecate(was = "old_score", since = "1.2.0", note = "use score() instead"))]
    score: u32,
}
impl WithSinceAndNote {
    pub fn score(&self) -> u32 {
        self.score
    }
    #[deprecated(since = "1.2.0", note = "use score() instead")]
    pub fn old_score(&self) -> u32 {
        self.score
    }
    pub fn set_score(&mut self, score: u32) -> &mut Self {
        self.score = score;
        self
    }
    #[deprecated(since = "1.2.0", note = "use score() instead")]
    pub fn set_old_score(&mut self, old_score: u32) -> &mut Self {
        self.score = old_score;
        self
    }
    #[must_use]
    pub fn with_score(mut self, score: u32) -> Self {
        self.score = score;
        self
    }
    #[deprecated(since = "1.2.0", note = "use score() instead")]
    #[must_use]
    pub fn with_old_score(mut self, old_score: u32) -> Self {
        self.score = old_score;
        self
    }
}
/// Method-level bare deprecate marks only that one method as deprecated.
#[fieldwork(get, set)]
struct MethodLevelBare {
    #[field(set(deprecate))]
    counter: u32,
}
impl MethodLevelBare {
    pub fn counter(&self) -> u32 {
        self.counter
    }
    #[deprecated]
    pub fn set_counter(&mut self, counter: u32) -> &mut Self {
        self.counter = counter;
        self
    }
}
/// Rename combined with deprecate: methods derive from the new name and the
/// deprecated alternates derive from the supplied old base.
#[fieldwork(get, set)]
struct RenameAndDeprecate {
    /// the field's stored value
    #[field(rename = new_name, deprecate = "old_name")]
    stored: String,
}
impl RenameAndDeprecate {
    ///Borrows the field's stored value
    pub fn new_name(&self) -> &str {
        &*self.stored
    }
    ///Borrows the field's stored value
    #[deprecated(note = "use `new_name` instead")]
    pub fn old_name(&self) -> &str {
        &*self.stored
    }
    ///Sets the field's stored value, returning `&mut Self` for chaining
    pub fn set_new_name(&mut self, new_name: String) -> &mut Self {
        self.stored = new_name;
        self
    }
    ///Sets the field's stored value, returning `&mut Self` for chaining
    #[deprecated(note = "use `set_new_name` instead")]
    pub fn set_old_name(&mut self, old_name: String) -> &mut Self {
        self.stored = old_name;
        self
    }
}
/// Enum: putting `deprecate` on one variant covers the unified virtual field.
#[fieldwork(get, set)]
enum Packet {
    Data { #[field(deprecate = "ident")] id: u32, payload: String },
    Heartbeat { id: u32 },
}
impl Packet {
    pub fn id(&self) -> u32 {
        match self {
            Self::Data { id, .. } | Self::Heartbeat { id, .. } => *id,
        }
    }
    #[deprecated(note = "use `id` instead")]
    pub fn ident(&self) -> u32 {
        match self {
            Self::Data { id, .. } | Self::Heartbeat { id, .. } => *id,
        }
    }
    pub fn set_id(&mut self, id: u32) -> &mut Self {
        match self {
            Self::Data { id: id_binding, .. } => {
                *id_binding = id;
            }
            Self::Heartbeat { id: id_binding, .. } => {
                *id_binding = id;
            }
        }
        self
    }
    #[deprecated(note = "use `set_id` instead")]
    pub fn set_ident(&mut self, ident: u32) -> &mut Self {
        match self {
            Self::Data { id, .. } => {
                *id = ident;
            }
            Self::Heartbeat { id, .. } => {
                *id = ident;
            }
        }
        self
    }
    pub fn payload(&self) -> Option<&str> {
        match self {
            Self::Data { payload, .. } => Some(&**payload),
            _ => None,
        }
    }
}
