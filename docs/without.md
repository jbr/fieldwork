# `without` — negative owned setters

The `without` method generates owned setters that represent the "negative" state of a field.
It is designed to be used alongside [`with`](crate::with) and changes how `with` works for
`bool` and `Option` fields:

- **`bool` fields**: `with_field()` sets to `true`, `without_field()` sets to `false`
- **`Option<T>` fields**: `with_field(value)` sets to `Some(value)`, `without_field()` sets to `None`
- **Other fields**: `without` methods are not generated; `with` works normally

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)]
struct Config {
    /// whether debug mode is on
    debug: bool,

    /// optional log file path
    log_file: Option<String>,

    /// application name (no without generated)
    name: String,
}
```

```rust
// GENERATED
# struct Config { debug: bool, log_file: Option<String>, name: String, }
impl Config {
    ///Owned chainable setter for whether debug mode is on, returning `Self`
    #[must_use]
    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }
    ///Owned chainable setter for whether debug mode is on, returning `Self`
    #[must_use]
    pub fn without_debug(mut self) -> Self {
        self.debug = false;
        self
    }
    ///Owned chainable setter for optional log file path, returning `Self`
    #[must_use]
    pub fn with_log_file(mut self, log_file: String) -> Self {
        self.log_file = Some(log_file);
        self
    }
    ///Owned chainable setter for optional log file path, returning `Self`
    #[must_use]
    pub fn without_log_file(mut self) -> Self {
        self.log_file = None;
        self
    }
    ///Owned chainable setter for application name (no without generated), returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

```

## Skipping `without` for a field

Use `#[field(without = false)]` or `#[field(without(skip))]` to suppress the `without` method
for a specific field while keeping it enabled struct-wide.
