# `option_set_some` — accept `T` instead of `Option<T>` in setters

When `option_set_some` is enabled for an `Option<T>` field, [`set`](crate::set) and
[`with`](crate::with) accept `T` directly and wrap it in `Some` automatically. This is useful in
builder patterns where `None` represents "not yet set" and you never call the setter to assign
`None` — you use [`without`](crate::without) or simply omit the call for that.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, option_set_some)]
struct User {
    /// display name, if provided
    display_name: Option<String>,

    /// profile photo URL, if uploaded
    avatar_url: Option<String>,
}
```

```rust
// GENERATED
# struct User { display_name: Option<String>, avatar_url: Option<String>, }
impl User {
    ///Sets display name, if provided, returning `&mut Self` for chaining
    pub fn set_display_name(&mut self, display_name: String) -> &mut Self {
        self.display_name = Some(display_name);
        self
    }
    ///Owned chainable setter for display name, if provided, returning `Self`
    #[must_use]
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }
    ///Sets profile photo URL, if uploaded, returning `&mut Self` for chaining
    pub fn set_avatar_url(&mut self, avatar_url: String) -> &mut Self {
        self.avatar_url = Some(avatar_url);
        self
    }
    ///Owned chainable setter for profile photo URL, if uploaded, returning `Self`
    #[must_use]
    pub fn with_avatar_url(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }
}

```

## Combining with `into`

`option_set_some` and [`into`](crate::into) compose: when both are enabled, the setter accepts
`impl Into<T>` and wraps the result in `Some`. See [`into`](crate::into) for an example.

## Enabling for one method but not the other

Because `option_set_some` is set-and-with only, you can enable it for just `set` or just `with`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(option_set_some), with)]
struct User {
    /// nickname, if provided
    nickname: Option<String>,
}
```

```rust
// GENERATED
# struct User { nickname: Option<String>, }
impl User {
    ///Sets nickname, if provided, returning `&mut Self` for chaining
    pub fn set_nickname(&mut self, nickname: String) -> &mut Self {
        self.nickname = Some(nickname);
        self
    }
    ///Owned chainable setter for nickname, if provided, returning `Self`
    #[must_use]
    pub fn with_nickname(mut self, nickname: Option<String>) -> Self {
        self.nickname = nickname;
        self
    }
}

```

## Configuration levels

`option_set_some` can be set at any level:

- **Struct**: `#[fieldwork(option_set_some)]` — enables for all setters on all `Option` fields
- **Method**: `#[fieldwork(set(option_set_some))]` — enables only for `set` or only for `with`
- **Field**: `#[field(option_set_some)]` — enables for all setter methods on this `Option` field
- **Field+method**: `#[field(set(option_set_some))]` — most specific

See [`configuration`](crate::configuration) for how these levels interact.
