# `get_mut` — mutable field accessors

The `get_mut` method generates a mutable accessor named `field_name_mut`. It follows the same
dereferencing and option-unwrapping rules as [`get`](crate::get), returning `&mut BorrowedType`
for owned types and `Option<&mut T>` for option fields.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get_mut)]
struct User {
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
# struct User { name: String, age: Option<u8>, favorite_color: Option<String>, }
impl User {
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
    }
    ///Mutably borrow the user's age, if known
    pub fn age_mut(&mut self) -> Option<&mut u8> {
        self.age.as_mut()
    }
    ///Mutably borrow favorite color, if set
    pub fn favorite_color_mut(&mut self) -> Option<&mut str> {
        self.favorite_color.as_deref_mut()
    }
}

```

## Options

`get_mut` shares its configuration options with `get`:

| Option | Description |
|--------|-------------|
| [`deref`](crate::get_mut::deref) | Control automatic dereferencing |
| [`option_borrow_inner`](crate::get_mut::option_borrow_inner) | Control how `Option` fields are returned |

Note that `copy` and `rename_predicates` do not apply to `get_mut` — mutable accessors always
return references, and the `_mut` suffix already distinguishes them from `get`.
