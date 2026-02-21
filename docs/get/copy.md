# `copy` — return Copy types by value

By default, fieldwork returns common `Copy` types by value rather than as `&T`. This avoids
callers having to immediately dereference the return value in the overwhelming majority of cases.

The types detected automatically are: `bool`, `char`, all numeric primitives (`u8`–`u128`,
`i8`–`i128`, `f32`, `f64`, `usize`, `isize`), and immutable references (`&T`).

## Opting out

To return references for these types instead, disable `copy` at the item level:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(copy = false))]
struct Counters {
    /// total hits
    hits: usize,

    /// whether counting is active
    active: bool,
}
```

```rust
// GENERATED
# struct Counters { hits: usize, active: bool, }
impl Counters {
    ///Borrows total hits
    pub fn hits(&self) -> &usize {
        &self.hits
    }
    ///Borrows whether counting is active
    pub fn active(&self) -> &bool {
        &self.active
    }
}

```

## Opting in for other types

To return a copy of a field that isn't in the auto-detected list, use `#[field(get(copy))]`:

```rust
#[derive(fieldwork::Fieldwork)]
struct Counters {
    /// total hits
    #[field(get(copy))]
    hits: usize,
}
```

```rust
// GENERATED
# struct Counters { hits: usize, }
impl Counters {
    ///Returns a copy of total hits
    pub fn hits(&self) -> usize {
        self.hits
    }
}

```
