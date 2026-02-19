# `deref` — automatic dereferencing to borrowed types

Fieldwork automatically detects owned types that have a canonical borrowed form and returns the
borrowed form from `get` and `get_mut`. This mirrors how you'd write the getter by hand.

## Built-in type mappings

| Owned type | `get` returns | `get_mut` returns |
|------------|--------------|------------------|
| `String` | `&str` | `&mut str` |
| `OsString` | `&OsStr` | `&mut OsStr` |
| `Vec<T>` | `&[T]` | `&mut [T]` |
| `Box<T>` | `&T` | `&mut T` |
| `Arc<T>` | `&T` | _(no `DerefMut`)_ |
| `Rc<T>` | `&T` | _(no `DerefMut`)_ |
| `PathBuf` | `&Path` | `&mut Path` |
| `Cow<'_, T>` | `&T` | _(no `DerefMut`)_ |
| `[T; N]` | `&[T]` | `&mut [T]` |

## Custom deref target

Specify an explicit target type with `deref = Type`. Types containing generics or special syntax
(like `[u8]`) must be quoted:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct Image {
    /// raw pixel data
    #[field(deref = "[u8]")]
    pixels: Vec<u8>,

    /// image label
    label: String,
}
```

```rust
// GENERATED
# struct Image { pixels: Vec<u8>, label: String, }
impl Image {
    ///Borrows raw pixel data
    pub fn pixels(&self) -> &[u8] {
        &*self.pixels
    }
    ///Mutably borrow raw pixel data
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut *self.pixels
    }
    ///Borrows image label
    pub fn label(&self) -> &str {
        &*self.label
    }
    ///Mutably borrow image label
    pub fn label_mut(&mut self) -> &mut str {
        &mut *self.label
    }
}

```

## Opting out

Disable deref detection for a whole struct, a specific method, or a single field:

```rust
# use std::sync::Arc;
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Cache {
    // deref = false: keep the Arc directly
    #[field(deref = false)]
    handle: Arc<()>,

    name: String,
}
```

```rust
// GENERATED
# use std::sync::Arc;
# struct Cache { handle: Arc<()>, name: String, }
impl Cache {
    pub fn handle(&self) -> &Arc<()> {
        &self.handle
    }
    pub fn name(&self) -> &str {
        &*self.name
    }
}

```

At the struct level: `#[fieldwork(get, deref = false)]` disables deref for all `get` accessors.
Individual fields can opt back in with `#[field(deref = true)]` or a specific type.
