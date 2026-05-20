# `deprecate` — keep old method names while renaming

The `deprecate` attribute helps you rename fieldwork-generated methods without breaking
downstream callers. Apply it to a field (or to one of a field's method overrides) and
fieldwork will either mark the canonical methods `#[deprecated]` or emit deprecated
alternates next to them.

## Bare deprecate — mark canonical methods

`#[field(deprecate)]` marks every method that fieldwork would generate for this field
with `#[deprecated]`. Use this when a field is on its way out and you have no
replacement to point to.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    #[field(deprecate)]
    legacy_flag: bool,
}
```

```rust
// GENERATED
# struct User { legacy_flag: bool, }
impl User {
    #[deprecated]
    pub fn legacy_flag(&self) -> bool {
        self.legacy_flag
    }
    #[deprecated]
    pub fn set_legacy_flag(&mut self, legacy_flag: bool) -> &mut Self {
        self.legacy_flag = legacy_flag;
        self
    }
}

```

At the method level, `#[field(set(deprecate))]` marks only that one method.

## Renaming — keep old names as deprecated aliases

`#[field(deprecate = "old_name")]` at the field level emits the canonical methods plus
deprecated alternates derived from `old_name`. Templates and method prefixes apply to
the old base in the same way they do to the canonical name, so a `get` + `set` setup
yields `old_name()` and `set_old_name()` next to the canonical pair.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    /// the user's display name
    #[field(deprecate = "name")]
    display_name: String,
}
```

```rust
// GENERATED
# struct User { display_name: String, }
impl User {
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
}

```

At the method level, `#[field(get(deprecate = "literal_name"))]` emits a single
deprecated alternate whose function name is exactly `literal_name` — the method
prefix is *not* applied. This mirrors how `#[field(get = "literal_name")]` already
treats its argument as a literal method name.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    #[field(set(deprecate = "assign_name"))]
    name: String,
}
```

```rust
// GENERATED
# struct User { name: String, }
impl User {
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    #[deprecated(note = "use `set_name` instead")]
    pub fn assign_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

## `since` and `note`

To set `#[deprecated(since = "…")]` or override the default note, use the list form:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct Telemetry {
    #[field(deprecate(was = "old_score", since = "1.2.0", note = "use score() instead"))]
    score: u32,
}
```

```rust
// GENERATED
# struct Telemetry { score: u32, }
impl Telemetry {
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
}

```

Both `since` and `note` work with bare deprecation too:
`#[field(deprecate(since = "1.2.0"))]` marks the canonical methods deprecated with
that `since` clause.

## Cascade

Method-level deprecation wins over field-level for that one method, just like every
other fieldwork setting. For each (field, method) slot, fieldwork emits at most one of:

- the canonical method plus a deprecated alternate (when `was` is set), or
- the canonical method marked `#[deprecated]` (bare deprecate), or
- the canonical method with no deprecation.

Never both for the same slot.

## Enums

For enum virtual fields (the unified field across variants), put `#[field(deprecate
= "old")]` on **one** occurrence — typically the first variant that declares the
field. The deprecation is a property of the virtual field, not the individual variant
occurrence, so a single annotation is all that's needed.

```rust
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
```

```rust
// GENERATED
# enum Packet { Data { id: u32, payload: String }, Heartbeat { id: u32 }, }
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

```
