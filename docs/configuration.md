# Configuration

Fieldwork has four levels of configuration that cascade from broadest to most specific. The most
specific level always wins.

| Level | Syntax | Scope |
|-------|--------|-------|
| Struct | `#[fieldwork(option_borrow_inner = false)]` | All methods on all fields |
| Struct-method | `#[fieldwork(get(copy = false))]` | One method type across all fields |
| Field | `#[field(into)]` | All methods on one field |
| Field-method | `#[field(get(copy = false))]` | One method on one field |

## Selecting which methods to generate

Methods are enabled at the struct level: `#[fieldwork(get, set, with)]`. This generates those
methods for every field unless a field opts out.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    name: String,

    // set = false: no setter generated
    #[field(set = false)]
    id: u64,
}
```

```rust
// GENERATED
# struct User { name: String, id: u64, }
impl User {
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn id(&self) -> &u64 {
        &self.id
    }
}

```

## Opt-in mode

By default fieldwork operates in **opt-out** mode: all fields get the struct-level methods unless
they individually opt out. With `opt_in` on the struct, no field gets any methods unless it
explicitly requests them via `#[field(...)]`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get, set)]
struct User {
    // field = true: inherits struct-level methods (get + set)
    #[field = true]
    name: String,

    // field-level opt-in: set only
    #[field(set)]
    role: String,

    // not opted in: no methods generated
    internal_id: u64,
}
```

```rust
// GENERATED
# struct User { name: String, role: String, internal_id: u64, }
impl User {
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn set_role(&mut self, role: String) -> &mut Self {
        self.role = role;
        self
    }
}

```

## Skipping a field entirely

`#[field = false]` or `#[field(skip)]` excludes a field from all generated methods:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    name: String,

    #[field = false]
    _marker: (),
}
```

```rust
// GENERATED
# struct User { name: String, _marker: (), }
impl User {
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

## Visibility

By default all generated methods are `pub`. Override at any level:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, vis = "pub(crate)")]
struct Internal {
    name: String,

    // vis = "pub" overrides the struct-level pub(crate)
    #[field(vis = "pub")]
    version: u32,
}
```

```rust
// GENERATED
# struct Internal { name: String, version: u32, }
impl Internal {
    pub(crate) fn name(&self) -> &str {
        &*self.name
    }
    pub(crate) fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn set_version(&mut self, version: u32) -> &mut Self {
        self.version = version;
        self
    }
}

```

## `where` clause

Add a `where` clause to the generated `impl` block:

```rust
# trait Validate {}
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, where_clause = "T: Validate")]
struct Wrapper<T> {
    /// the wrapped value
    value: T,
}
```

```rust
// GENERATED
# trait Validate { }
# struct Wrapper<T>{ value: T, }
impl<T> Wrapper<T>
where
    T: Validate,
{
    ///Borrows the wrapped value
    pub fn value(&self) -> &T {
        &self.value
    }
    ///Sets the wrapped value, returning `&mut Self` for chaining
    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
}

```

## Naming

**Rename a field's methods** with `name`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    // name = id: methods are id() and set_id()
    #[field(name = id)]
    internal_id: u64,
}
```

```rust
// GENERATED
# struct User { internal_id: u64, }
impl User {
    pub fn id(&self) -> &u64 {
        &self.internal_id
    }
    pub fn set_id(&mut self, id: u64) -> &mut Self {
        self.internal_id = id;
        self
    }
}

```

**Override one specific method's name** with the shorthand `#[field(get = method_name)]`:

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    // get = is_admin overrides the method name; set uses default
    #[field(get = is_admin, set)]
    admin: bool,
}
```

```rust
// GENERATED
# struct User { admin: bool, }
impl User {
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
}

```

**Rename all methods of a type** with `template`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(template = "assign_{}"))]
struct Form {
    name: String,
    email: String,
}
```

```rust
// GENERATED
# struct Form { name: String, email: String, }
impl Form {
    pub fn assign_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn assign_email(&mut self, email: String) -> &mut Self {
        self.email = email;
        self
    }
}

```

**Rename the setter argument** with `argument`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct Connection {
    #[field(argument = is_secure)]
    secure: bool,
}
```

```rust
// GENERATED
# struct Connection { secure: bool, }
impl Connection {
    pub fn set_secure(&mut self, is_secure: bool) -> &mut Self {
        self.secure = is_secure;
        self
    }
}

```

## Documentation

Fieldwork generates doc comments from the field's `///` doc comment. Override the doc for a
specific generated method with `#[field(set(doc = "..."))]`:

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    /// user's display name
    #[field(set(doc = "Set the name shown in the UI"))]
    name: String,
}
```

```rust
// GENERATED
# struct User { name: String, }
impl User {
    ///Set the name shown in the UI
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

Override the doc template for an entire method type with `doc_template`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(doc_template = "Assigns {}"))]
struct User {
    /// the user's name
    name: String,

    /// the user's role
    role: String,
}
```

```rust
// GENERATED
# struct User { name: String, role: String, }
impl User {
    ///Assigns the user's name
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Assigns the user's role
    pub fn set_role(&mut self, role: String) -> &mut Self {
        self.role = role;
        self
    }
}

```

## Boolean attribute shorthand

Everywhere fieldwork accepts a boolean option, the bare name is equivalent to `= true`:

```text
// These are equivalent:
// #[fieldwork(option_borrow_inner = true)]
// #[fieldwork(option_borrow_inner)]
```

## Type quoting

Types containing lifetimes, angle brackets, or other special syntax must be quoted as strings:

```text
// #[field(deref = "[u8]")]         quote slice types
// #[field(deref = "Arc<str>")]     quote generic types (simple paths don't need quoting)
```
