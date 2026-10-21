# `delegate` — emit a struct's methods on a type that holds it

Mark a struct `#[fieldwork(delegatable)]` and a field that holds it `#[field(delegate)]`, and
fieldwork emits the delegatable struct's methods on the holding type as well, reaching each
field through the delegate field. The methods are the delegatable struct's, as configured there
— the same names, signatures, docs, and deprecations — with `Self` now meaning the holding type.

This keeps an accessor API intact when a type's fields move into a struct of their own, for
example behind a `Box` to make the type cheap to move:

```rust
#[derive(fieldwork::Fieldwork)]
struct Request {
    #[field(delegate)]
    head: Box<Head>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, delegatable)]
struct Head {
    /// the request path
    path: String,

    /// the response status
    status: u16,
}
```

```rust
// GENERATED
# struct Request { head: Box<Head>, }
# struct Head { path: String, status: u16, }
impl Request {}

impl Request {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.head.path
    }
    ///Sets the request path, returning `&mut Self` for chaining
    #[inline]
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.head.path = path;
        self
    }
    ///Returns a copy of the response status
    #[inline]
    pub fn status(&self) -> u16 {
        self.head.status
    }
    ///Sets the response status, returning `&mut Self` for chaining
    #[inline]
    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.head.status = status;
        self
    }
}

impl Head {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.path
    }
    ///Sets the request path, returning `&mut Self` for chaining
    #[inline]
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    ///Returns a copy of the response status
    #[inline]
    pub fn status(&self) -> u16 {
        self.status
    }
    ///Sets the response status, returning `&mut Self` for chaining
    #[inline]
    pub fn set_status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }
}

```

The delegatable struct keeps its own methods, as `Head` does here.

## Delegation is additive

`delegate` adds the delegated methods and changes nothing else: the delegate field gets exactly
the methods it would get without it, alongside methods for its holder's other fields. With
methods enabled for every field, that includes accessors for the delegate field itself, which
can be skipped like those of any other field:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct Request {
    /// the request body
    body: Vec<u8>,

    #[field(delegate, skip)]
    head: Box<Head>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
struct Head {
    /// the request path
    path: String,
}
```

```rust
// GENERATED
# struct Request { body: Vec<u8>, head: Box<Head>, }
# struct Head { path: String, }
impl Request {
    ///Borrows the request body
    #[inline]
    pub fn body(&self) -> &[u8] {
        &*self.body
    }
    ///Sets the request body, returning `&mut Self` for chaining
    #[inline]
    pub fn set_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = body;
        self
    }
}

impl Request {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.head.path
    }
}

impl Head {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.path
    }
}

```

## Naming the delegatable struct

Fieldwork finds the delegatable struct through the delegate field's type, looking through a
`Box`, and a tuple field works the same way as a named one. When the type doesn't name the
struct — a type alias, say, or a pointer other than `Box` — name it with `delegate = path`:

```rust
type BoxedHead = Box<Head>;

#[derive(fieldwork::Fieldwork)]
struct Request(#[field(delegate = Head)] BoxedHead);

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
struct Head {
    /// the request path
    path: String,
}
```

```rust
// GENERATED
# type BoxedHead = Box<Head>;
# struct Request (BoxedHead);
# struct Head { path: String, }
impl Request {}

impl Request {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.0.path
    }
}

impl Head {
    ///Borrows the request path
    #[inline]
    pub fn path(&self) -> &str {
        &*self.path
    }
}

```

## How it works, and what follows from that

A derive macro can't see other items, so the two derives meet through name resolution:
`delegatable` emits a hidden macro holding the struct's methods, available under the struct's
own name, and `delegate` invokes it through the path that names the field's type. Some
consequences:

- Both types have to be in the same crate.
- The delegated methods are expanded where the delegating type is defined. The delegatable
  struct's fields must be visible from there, as they are from the same module or one nested
  inside it; the types those fields are declared with must be nameable there; and intra-doc
  links in their docs resolve there.
- The delegated methods are emitted in an `impl` block with the delegating type's generics and
  `where` clause, including any [`bounds`](crate::configuration#where-clause). Generic
  parameters in the delegatable struct's field types must use the same names.
- `set`, `get_mut`, `with`, `without`, and `take` write through the delegate field, and
  `into_field` moves out of it, so those need the struct held by value or in a `Box`.
- A module that glob-imports one delegatable struct can't delegate to a different one of the
  same name defined in the module itself, because the compiler reports the name as ambiguous.
  Import what the module needs from the glob explicitly instead.
- Only structs take part. Enums can't be `delegatable` or have `delegate` fields, and a
  delegatable struct's own `delegate` fields aren't passed along to types that delegate to it.
