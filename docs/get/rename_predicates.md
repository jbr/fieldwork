# `rename_predicates` — `is_` prefix for boolean getters

When `rename_predicates` is enabled, fieldwork prefixes boolean-returning `get` accessors with
`is_` to match Rust naming conventions. This only affects `get` — `get_mut` retains the `_mut`
suffix without the `is_` prefix.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, rename_predicates)]
struct Connection {
    active: bool,
    verified: bool,
    name: String,
}
```

```rust
// GENERATED
# struct Connection { active: bool, verified: bool, name: String, }
impl Connection {
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn active_mut(&mut self) -> &mut bool {
        &mut self.active
    }
    pub fn is_verified(&self) -> bool {
        self.verified
    }
    pub fn verified_mut(&mut self) -> &mut bool {
        &mut self.verified
    }
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
    }
}

```

## Enabling per-field

`rename_predicates` can also be set at the field level to selectively rename individual bool
getters when it isn't enabled struct-wide:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Connection {
    #[field(rename_predicates)]
    active: bool,

    // no rename_predicates: keeps default name `verified()`
    verified: bool,
}
```

```rust
// GENERATED
# struct Connection { active: bool, verified: bool, }
impl Connection {
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn verified(&self) -> bool {
        self.verified
    }
}

```
