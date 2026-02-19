# `into` — accept `impl Into<T>` in setters

When `into` is enabled, [`set`](crate::set) and [`with`](crate::with) methods accept
`impl Into<FieldType>` instead of the field type directly. This lets callers pass any compatible
type without explicit conversion — most commonly passing `&str` to a `String` field.

```rust
# use std::path::PathBuf;
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, into)]
struct Server {
    /// server hostname
    host: String,

    /// path to config file
    config_path: PathBuf,

    // Into not useful for Copy primitives; disabled per-field
    #[field(into = false)]
    port: u16,
}
```

```rust
// GENERATED
# use std::path::PathBuf;
# struct Server { host: String, config_path: PathBuf, port: u16, }
impl Server {
    ///Sets server hostname, returning `&mut Self` for chaining
    pub fn set_host(&mut self, host: impl Into<String>) -> &mut Self {
        self.host = host.into();
        self
    }
    ///Owned chainable setter for server hostname, returning `Self`
    #[must_use]
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }
    ///Sets path to config file, returning `&mut Self` for chaining
    pub fn set_config_path(&mut self, config_path: impl Into<PathBuf>) -> &mut Self {
        self.config_path = config_path.into();
        self
    }
    ///Owned chainable setter for path to config file, returning `Self`
    #[must_use]
    pub fn with_config_path(mut self, config_path: impl Into<PathBuf>) -> Self {
        self.config_path = config_path.into();
        self
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    #[must_use]
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

```

## Combining with `option_set_some`

`into` and [`option_set_some`](crate::option_set_some) compose: when both are enabled for an
`Option<T>` field, the setter accepts `impl Into<T>` and wraps the result in `Some`.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, into, option_set_some)]
struct User {
    /// display name
    display_name: Option<String>,
}
```

```rust
// GENERATED
# struct User { display_name: Option<String>, }
impl User {
    ///Sets display name, returning `&mut Self` for chaining
    pub fn set_display_name(&mut self, display_name: impl Into<String>) -> &mut Self {
        self.display_name = Some(display_name.into());
        self
    }
    ///Owned chainable setter for display name, returning `Self`
    #[must_use]
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }
}

```

## Configuration levels

`into` can be set at any level:

- **Struct**: `#[fieldwork(into)]` — enables for all setters on all fields
- **Method**: `#[fieldwork(set(into))]` — enables only for `set` or only for `with`
- **Field**: `#[field(into)]` — enables for all setter methods on this field
- **Field+method**: `#[field(set(into))]` — most specific

See [`configuration`](crate::configuration) for how these levels interact.
