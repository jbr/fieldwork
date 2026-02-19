# `with` — owned chainable setters

The `with` method generates an owned setter prefixed with `with_`, consuming and returning `Self`.
This is the idiomatic pattern for builder-style initialization. Generated methods are marked
`#[must_use]` to catch accidentally discarding the returned value.

When [`without`](crate::without) is also enabled, `with` changes behavior for `bool` and `Option`
fields: `with_field()` sets to `true`/`Some(value)` and `without_field()` sets to `false`/`None`.
See [`without`](crate::without) for details.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with)]
struct Request {
    /// request URL
    url: String,

    /// optional auth token
    token: Option<String>,

    /// whether to follow redirects
    follow_redirects: bool,
}
```

```rust
// GENERATED
# struct Request { url: String, token: Option<String>, follow_redirects: bool, }
impl Request {
    ///Owned chainable setter for request URL, returning `Self`
    #[must_use]
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
    ///Owned chainable setter for optional auth token, returning `Self`
    #[must_use]
    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.token = token;
        self
    }
    ///Owned chainable setter for whether to follow redirects, returning `Self`
    #[must_use]
    pub fn with_follow_redirects(mut self, follow_redirects: bool) -> Self {
        self.follow_redirects = follow_redirects;
        self
    }
}

```

## Options

| Option | Description |
|--------|-------------|
| [`into`](crate::with::into) | Accept `impl Into<T>` instead of `T` |
| [`option_set_some`](crate::with::option_set_some) | Accept `T` and wrap in `Some` for `Option<T>` fields |

Both options are shared with [`set`](crate::set) and documented at the crate level:
[`into`](crate::into), [`option_set_some`](crate::option_set_some).
