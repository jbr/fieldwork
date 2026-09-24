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
trait method — [`deref`](crate::get::deref) calls `Deref::deref` for types
like `String`, `Vec`, and `PathBuf`, borrowing an array as a slice calls
`Index`, and [`into`](crate::into) calls `Into::into` — and when a setter
drops the value it overwrites, which const evaluation does not allow for a
type with a destructor. The compiler reports these errors on the field in
your struct, naming the call it cannot make.

Dereferencing a `Box` or a `&mut` reference is built into the language, so
those fields are fine as they are. For the rest, turn off auto-deref
alongside `const_fn`, at whichever level you set it:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, const_fn, deref = false)]
struct Header {
    /// the magic bytes
    magic: [u8; 4],

    /// the optional label
    label: Option<String>,
}
```

```rust
// GENERATED
# struct Header { magic: [u8; 4], label: Option<String>, }
impl Header {
    ///Borrows the magic bytes
    pub const fn magic(&self) -> &[u8; 4] {
        &self.magic
    }
    ///Mutably borrow the magic bytes
    pub const fn magic_mut(&mut self) -> &mut [u8; 4] {
        &mut self.magic
    }
    ///Borrows the optional label
    pub const fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
    ///Mutably borrow the optional label
    pub const fn label_mut(&mut self) -> Option<&mut String> {
        self.label.as_mut()
    }
}

```

Every method type — `get`, `get_mut`, `set`, `with`, `without`, `take`, and
`into_field` — is within the subset for `Copy` fields and `Option`s of them.
