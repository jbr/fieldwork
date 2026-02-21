# `take` — take the value out of an `Option` field

The `take` method is only generated for `Option<T>` fields. It calls `Option::take` on the field,
returning `Option<T>` and leaving `None` in its place. Non-option fields are silently skipped.

On enums, `take` is only generated for full-coverage `Option<T>` fields. See
[`enums`](crate::enums).

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(take)]
struct Session {
    /// auth token, consumed on first use
    token: Option<String>,

    /// cached response, cleared after reading
    cached_response: Option<Vec<u8>>,
}
```

```rust
// GENERATED
# struct Session { token: Option<String>, cached_response: Option<Vec<u8>>, }
impl Session {
    ///Takes auth token, consumed on first use, leaving a None in its place
    pub fn take_token(&mut self) -> Option<String> {
        self.token.take()
    }
    ///Takes cached response, cleared after reading, leaving a None in its place
    pub fn take_cached_response(&mut self) -> Option<Vec<u8>> {
        self.cached_response.take()
    }
}

```
