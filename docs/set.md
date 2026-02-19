# `set` — mutating field setters

The `set` method generates a mutating setter prefixed with `set_`. By default it returns `&mut Self`
for chaining — see [`set::chain`](crate::set::chain) to change this.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct Server {
    /// server hostname
    host: String,

    /// port number
    port: u16,

    /// whether TLS is required
    tls_required: bool,
}
```

```rust
// GENERATED
# struct Server { host: String, port: u16, tls_required: bool, }
impl Server {
    ///Sets server hostname, returning `&mut Self` for chaining
    pub fn set_host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }
    ///Sets port number, returning `&mut Self` for chaining
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    ///Sets whether TLS is required, returning `&mut Self` for chaining
    pub fn set_tls_required(&mut self, tls_required: bool) -> &mut Self {
        self.tls_required = tls_required;
        self
    }
}

```

## Options

| Option | Description |
|--------|-------------|
| [`chain`](crate::set::chain) | Control whether setters return `&mut Self` or `()` |
| [`into`](crate::set::into) | Accept `impl Into<T>` instead of `T` |
| [`option_set_some`](crate::set::option_set_some) | Accept `T` and wrap in `Some` for `Option<T>` fields |

`into` and `option_set_some` apply equally to `set` and `with` and are documented at the crate
level as well: [`into`](crate::into), [`option_set_some`](crate::option_set_some).

See [`configuration`](crate::configuration) for how options cascade across struct, method, field,
and field-method levels.
