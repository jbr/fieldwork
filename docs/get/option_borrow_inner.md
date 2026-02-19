# `option_borrow_inner` — borrow inside an `Option`

By default, fieldwork detects `Option<T>` fields and applies `as_ref` or `as_deref` so that `get`
returns `Option<&T>` or `Option<&BorrowedT>` instead of `&Option<T>`. This is almost always what
you want.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct User {
    /// nickname, if set
    nickname: Option<String>,

    /// profile image bytes, if uploaded
    avatar: Option<Vec<u8>>,
}
```

```rust
// GENERATED
# struct User { nickname: Option<String>, avatar: Option<Vec<u8>>, }
impl User {
    ///Borrows nickname, if set
    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }
    ///Borrows profile image bytes, if uploaded
    pub fn avatar(&self) -> Option<&[u8]> {
        self.avatar.as_deref()
    }
}

```

## Opting out

When you need direct access to the `Option` itself (to call `Option::insert`, `Option::get_or_insert`,
etc.), disable option borrowing for that field or method:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct User {
    // opt out for get_mut only, to allow Option::insert
    #[field(get_mut(option_borrow_inner = false))]
    nickname: Option<String>,
}
```

```rust
// GENERATED
# struct User { nickname: Option<String>, }
impl User {
    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }
    pub fn nickname_mut(&mut self) -> &mut Option<String> {
        &mut self.nickname
    }
}

```

`option_borrow_inner = false` can be set at any level:

- **Struct**: `#[fieldwork(get, option_borrow_inner = false)]` — disables for all getters
- **Method**: `#[fieldwork(get_mut(option_borrow_inner = false))]` — disables for one method type
- **Field**: `#[field(option_borrow_inner = false)]` — disables for all methods on this field
- **Field+method**: `#[field(get(option_borrow_inner = false))]` — most specific

See [`configuration`](crate::configuration) for how these levels interact.
