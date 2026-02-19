# `chain` — control the return type of `set`

By default, `set` methods return `&mut Self` so that multiple setters can be chained in sequence.
Set `chain = false` to return `()` instead.

## Disabling chaining struct-wide

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(chain = false))]
struct Config {
    /// server hostname
    host: String,

    /// port number
    port: u16,
}
```

```rust
// GENERATED
# struct Config { host: String, port: u16, }
impl Config {
    ///Sets server hostname
    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }
    ///Sets port number
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

```

## Disabling chaining for one field

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct Config {
    // chain = false: returns ()
    #[field(set(chain = false))]
    host: String,

    // chain = true (default): returns &mut Self
    port: u16,
}
```

```rust
// GENERATED
# struct Config { host: String, port: u16, }
impl Config {
    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
}

```
