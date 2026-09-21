# `const_fn` — generate `const fn` accessors

`const_fn` emits the generated methods as `const fn`, so a caller can read a
field while building a constant. It applies to every method type and cascades
like any other setting: set it on the item, on an item-method, on a field, or
on a field-method.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, const_fn)]
struct Limits {
    /// how many frames fit in one block
    block_frames: u32,

    /// whether the limit is enforced
    enforced: bool,
}

const BLOCK: u32 = Limits { block_frames: 512, enforced: true }.block_frames();
```

```rust
// GENERATED
# struct Limits { block_frames: u32, enforced: bool, }
impl Limits {
    ///Returns a copy of how many frames fit in one block
    pub const fn block_frames(&self) -> u32 {
        self.block_frames
    }
    ///Returns a copy of whether the limit is enforced
    pub const fn enforced(&self) -> bool {
        self.enforced
    }
}

```

Because it cascades, one field can be const while its neighbours are not:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Config {
    /// read at runtime like any other borrow
    name: String,

    /// read while building a constant
    #[field(const_fn)]
    port: u16,
}
```

```rust
// GENERATED
# struct Config { name: String, port: u16, }
impl Config {
    ///Borrows read at runtime like any other borrow
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Returns a copy of read while building a constant
    pub const fn port(&self) -> u16 {
        self.port
    }
}

```

## What a `const fn` body can hold

Fieldwork adds the `const`; it does not change what it generates, so the
method still has to be something Rust accepts as a constant function. A
generated body reaches outside the language's const subset when it calls a
trait method — [`deref`](crate::get::deref) calls `Deref::deref`, and
[`into`](crate::into) calls `Into::into` — and when a setter drops the value
it overwrites, which const evaluation does not allow for a type with a
destructor. In those cases the error comes from the compiler, on the
generated method, naming the call it cannot make.

Copy getters, borrowing getters, mutable getters, and setters over `Copy`
fields are all within the subset.
