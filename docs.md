# fieldwork

`fieldwork` is a procedural macro crate designed to automate the generation of field accessor methods for named structs. By leveraging Rust's powerful macro system, `fieldwork` reduces boilerplate code, enhances code readability, and ensures consistency across your data structures.

## Overview

In Rust, manually writing getter and setter methods for struct fields can be repetitive and error-prone, especially for large structs. `fieldwork` addresses this by providing a procedural macro that automatically generates these methods based on your struct definitions.

## Methods

Fieldwork supports four distinct methods: `get`, `set`, `get_mut`, and `with`.

### `get`

Borrows the field. This can also be used to return a copy of the field using the `#[fieldwork(get(copy))]` annotation on a field.

#### Example:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String
}
```

generates 

```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn admin(&self) -> &bool {
        &self.admin
    }
    /// Borrows the user's name
    pub fn name(&self) -> &String {
        &self.name
    }
}
```

### `set`

By default, set returns `&mut self` for chainable setters. If you would prefer to return `()`, use `#[fieldwork(set(chain = false))]` on the struct or an individual field.

#### Example

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String
}
```
generates:

```rust
impl User {
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```



### `get_mut`

#### Example

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get_mut)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,
}
```
generates the following impl block

```rust
impl User {
    /// Mutably borrow whether this user is an admin
    pub fn admin_mut(&mut self) -> &mut bool {
        &mut self.admin
    }
    /// Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}
```

### `with`
`with` provides a chainable owned setter, for situations that require returning the struct after modification.

#### Example
```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,
}
```

```rust
impl User {
    /// Owned chainable setter for whether this user is an admin, returning `Self`
    #[must_use]
    pub fn with_admin(mut self, admin: bool) -> Self {
        self.admin = admin;
        self
    }
    /// Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
```


## Combined example

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct User {
    /// whether this user is an admin
    #[fieldwork(argument = is_admin, get(copy, rename = is_admin), get_mut = is_admin_mut)]
    admin: bool,

    /// the user's name
    name: String,
}
```

```rust
impl User {
    /// Returns a copy of whether this user is an admin
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    /// Mutably borrow whether this user is an admin
    pub fn is_admin_mut(&mut self) -> &mut bool {
        &mut self.admin
    }
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, is_admin: bool) -> &mut Self {
        self.admin = is_admin;
        self
    }
    /// Owned chainable setter for whether this user is an admin, returning `Self`
    #[must_use]
    pub fn with_admin(mut self, is_admin: bool) -> Self {
        self.admin = is_admin;
        self
    }
    /// Borrows the user's name
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    /// Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
```

## Configuration

fieldwork supports four layers of configuration, from broadest to most specific: struct level, struct-method level, field level, and field-method level. The most specific configuration always overrides.

### struct level options

<!-- #### `opt_in` -->
#### `vis`
Sets the visibility for all generated functions, unless otherwise overridden.
`#[fieldwork(vis = "pub")]` is the default. For `pub(crate)`, use `#[fieldwork(vis = "pub(crate)")]`. To set private visibility, use `#[fieldwork(vis = "")]`.

#### `where_clause`
This option allows you to specify a where clause for the implementation block, such as:

```rust
#[derive(fieldwork::Fieldwork, Clone)]
#[fieldwork(get, set, get_mut, with, where_clause = "PocketContents: Precious")]
struct Hobbit<PocketContents> {
    /// what the hobbit has in his pocket
    pocket_contents: PocketContents,
}
```

generates
```rust
impl<PocketContents> Hobbit<PocketContents>
where
    PocketContents: Precious,
{
    /// Borrows what the hobbit has in his pocket
    pub fn pocket_contents(&self) -> &PocketContents {
        &self.pocket_contents
    }
    /// Mutably borrow what the hobbit has in his pocket
    pub fn pocket_contents_mut(&mut self) -> &mut PocketContents {
        &mut self.pocket_contents
    }
    /// Sets what the hobbit has in his pocket, returning `&mut Self` for chaining
    pub fn set_pocket_contents(&mut self, pocket_contents: PocketContents) -> &mut Self {
        self.pocket_contents = pocket_contents;
        self
    }
    /// Owned chainable setter for what the hobbit has in his pocket, returning `Self`
    #[must_use]
    pub fn with_pocket_contents(mut self, pocket_contents: PocketContents) -> Self {
        self.pocket_contents = pocket_contents;
        self
    }
}
```

### struct-method level

#### `vis`

Override the struct-level definition for a specific method.
`#[vis = "pub(crate), get(vis = "pub"), set, get_mut]` uses `pub(crate)` for all methods other than get.

#### `doc_template`
Override the default documentation template for the specific method. Let's say we want our documentation to say "assigns" instead of "sets":

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(doc_template = "Assigns {}"))]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,
}
```
```rust
impl User {
    /// Assigns whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    /// Assigns the user's name
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```



#### `template`
Override the method naming for all generated functions of this type. Let's say we want our set signature to be `assign_admin` instead of `set_admin` and `assign_name`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(template = "assign_{}"))]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,
}
```

```rust
impl User {
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn assign_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub fn assign_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```

#### `chain` (`set` only)

As discussed in the [Set](#set) section above, set returns `&mut Self` by default. To disable this, specify `chain = false`:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(chain = false))]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,
}
```

```rust
impl User {
    /// Sets whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
    /// Sets the user's name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
```


### field level options

#### `skip`
Omit this field from all generated functions.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,

    #[fieldwork(skip)]
    private: (),
}
```

```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn admin(&self) -> &bool {
        &self.admin
    }
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    /// Borrows the user's name
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```
#### `rename`
Change the name of this field for all generated methods.
```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    #[fieldwork(rename = admin)]
    /// whether this user is an admin
    superadmin: bool,
}
```

```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn admin(&self) -> &bool {
        &self.superadmin
    }
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.superadmin = admin;
        self
    }
}
```

#### `argument`

Change the name of the argument for `with` and `set`. This is occasionally important for rustdocs and lsp.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, set)]
struct User {
    #[fieldwork(argument = is_admin)]
    /// whether this user is an admin
    admin: bool,
}
```

```rust
impl User {
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, is_admin: bool) -> &mut Self {
        self.admin = is_admin;
        self
    }
    /// Owned chainable setter for whether this user is an admin, returning `Self`
    #[must_use]
    pub fn with_admin(mut self, is_admin: bool) -> Self {
        self.admin = is_admin;
        self
    }
}
```

#### `vis`
Change the visibility for all generated methods for a specific field

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    /// whether this user is an admin
    admin: bool,

    #[fieldwork(vis = "pub(crate)")]
    /// the user's name
    name: String,
}
```
```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn admin(&self) -> &bool {
        &self.admin
    }
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    /// Borrows the user's name
    pub(crate) fn name(&self) -> &String {
        &self.name
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub(crate) fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```

### field-method level options

#### `rename`
Specify the full function name for this particular method. Note that this overrides both `template` and field-level [`rename`](#rename).

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(get(rename = is_an_admin))]
    /// whether this user is an admin
    admin: bool,
}
```
```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn is_an_admin(&self) -> &bool {
        &self.admin
    }
}
```

If there are no other configuration options needed, this can be provided with the following shortcut (generates the same code as the above):

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(get = is_an_admin)]
    /// whether this user is an admin
    admin: bool,
}
```


#### `argument`

Specify the name of the argument for this specific method and field.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(set(argument = is_an_admin))]
    /// whether this user is an admin
    admin: bool,
}
```

```
impl User {
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, is_an_admin: bool) -> &mut Self {
        self.admin = is_an_admin;
        self
    }
}
```

#### `doc`
Override the documentation for this specific method and field.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(set(doc = "Specify whether this user can administer this system"))]
    admin: bool,
}
```

```rust
impl User {
    ///Specify whether this user can administer this system
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
}
```


#### `chain` (`set` only)
To return `()` from this specific `set` method instead of `&mut Self`, provide `chain = false`.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    /// whether this user is an admin
    #[fieldwork(set(chain = false))]
    admin: bool,
}
```

```rust
impl User {
    /// Sets whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
}
```

#### `copy` (`get` only)

Sometimes it is more useful to return a `Copy` of the returned type instead of a borrow. To opt into this behavior for a specific field, use `#[fieldwork(get(copy))]`:

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    /// whether the user is an admin
    #[fieldwork(get(copy))]
    admin: bool,
}
```
```rust
impl User {
    /// Returns a copy of whether the user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
}
```

#### `skip`
Omit this field from the particular method.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct User {
    /// whether this user is an admin
    #[fieldwork(set(skip))]
    admin: bool,

    /// the user's name
    name: String,
}
```


```rust
impl User {
    /// Borrows whether this user is an admin
    pub fn admin(&self) -> &bool {
        &self.admin
    }
    /// Borrows the user's name
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```
