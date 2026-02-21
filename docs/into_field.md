# `into_field` — consuming field accessors

The `into_field` method consumes `self` and returns the owned field value directly. It is the
natural complement to `get` for non-Copy types: where `get` borrows a field, `into_field`
transfers ownership out of the struct.

Copy types are silently skipped — they are already returned by value from `get`, so a consuming
accessor would be strictly worse.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct Config {
    /// the server address
    address: String,

    /// maximum number of connections
    max_connections: u32,

    /// optional TLS certificate path
    cert_path: Option<String>,
}
```

```rust
// GENERATED
# struct Config { address: String, max_connections: u32, cert_path: Option<String>, }
impl Config {
    ///Consumes self, returning the server address
    pub fn into_address(self) -> String {
        self.address
    }
    ///Consumes self, returning optional TLS certificate path
    pub fn into_cert_path(self) -> Option<String> {
        self.cert_path
    }
}

```

`max_connections` is `u32` (Copy) so no `into_max_connections` is generated.

## Enums

On enums, `into_field` is only generated for full-coverage fields. The method
consumes `self` and returns the field value via an exhaustive or-pattern match.
Partial-coverage fields are not generated. See [`enums`](crate::enums).

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum Status {
    Active { name: String },
    Inactive { name: String },
}
```

```rust
// GENERATED
# enum Status { Active { name: String }, Inactive { name: String }, }
impl Status {
    pub fn into_name(self) -> String {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => name,
        }
    }
}

```

## Newtype pattern

The most common use of `into_field` is the conventional `into_inner` for newtype wrappers.
Use `#[field = "inner"]` on the single tuple field:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
struct Token(#[field = "inner"] String);
```

```rust
// GENERATED
# struct Token (String);
impl Token {
    pub fn into_inner(self) -> String {
        self.0
    }
}

```
