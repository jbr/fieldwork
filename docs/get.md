# `get` — immutable field accessors

The `get` method generates an immutable accessor for each field. Fieldwork applies several smart
defaults to make getters idiomatic without configuration:

- **Copy types** (`bool`, integers, `char`, `&T`) are returned by value rather than by reference
- **Common owned types** (`String`, `Vec<T>`, `PathBuf`, etc.) return their borrowed form (`&str`,
  `&[T]`, `&Path`, etc.) — see [`get::deref`](crate::get::deref)
- **`Option` fields** return `Option<&T>` or `Option<&BorrowedT>` rather than `&Option<T>` — see
  [`get::option_borrow_inner`](crate::get::option_borrow_inner)

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,

    /// the user's age, if known
    age: Option<u8>,

    /// favorite color, if set
    favorite_color: Option<String>,
}
```

```rust
// GENERATED
# struct User { admin: bool, name: String, age: Option<u8>, favorite_color: Option<String>, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Returns a copy of the user's age, if known
    pub fn age(&self) -> Option<u8> {
        self.age
    }
    ///Borrows favorite color, if set
    pub fn favorite_color(&self) -> Option<&str> {
        self.favorite_color.as_deref()
    }
}

```

## Options

| Option | Description |
|--------|-------------|
| [`copy`](crate::get::copy) | Return by value instead of by reference |
| [`deref`](crate::get::deref) | Control automatic dereferencing to borrowed types |
| [`option_borrow_inner`](crate::get::option_borrow_inner) | Control how `Option` fields are returned |
| [`rename_predicates`](crate::get::rename_predicates) | Prefix `bool`-returning getters with `is_` |

All options can be set at the struct level (`#[fieldwork(get(copy = false))]`), per-field
(`#[field(get(copy))]`), or combined with field-level configuration
(`#[field(copy)]` applies to all methods for that field). See [`configuration`](crate::configuration)
for the full cascade rules.
