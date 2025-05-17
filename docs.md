# fieldwork

`fieldwork` is a procedural macro crate designed to automate the generation of field accessor
methods for named structs. By leveraging Rust's powerful macro system, `fieldwork` reduces
boilerplate code, enhances code readability, and ensures consistency. Just as importantly,
`fieldwork` intends to be fully customizable and expressive for common access patterns.

Manually writing getters and setters for struct fields is repetitive, and adds to maintenance
burden. `fieldwork` addresses this by providing a procedural macro that automatically generates
these methods based on your struct definitions. The intent of this crate, and distinguishing
feature, is to be as customizable and expressive as writing your own getters and setters. The crate
succeeds if you are able to emit exactly the code that you would have manually written, but far more
concisely.

## Performance

The compile time cost of using a proc macro crate is always worth considering. All efforts have been
made to keep this crate as lightweight as possible and featureful enough to be worth the tradeoff.

## Testing

This crate has a full suite of macro-expansion tests in
[tests/expand](https://github.com/jbr/fieldwork/tree/main/tests/expand). These tests are also used
for test coverage.

## Configuration

`fieldwork` supports four layers of configuration, from broadest to most specific: [struct
configuration](#struct-configuration), [struct method configuration](#struct-method-configuration),
[field configuration](#field-configuration) and [field method
configuration](#field-method-configuration). The most specific configuration always has precedence.


## Quick Links (TOC)


[**Methods**](#methods): [`get`](#get), [`set`](#set), [`get_mut`](#get_mut), [`with`](#with)

[**Struct configuration**](#struct-configuration): [`vis`](#struct-vis),
[`where_clause`](#struct-where-clause)

[**Struct method configuration**](#struct-method-configuration):
[`doc_template`](#struct-method-doc-template), [`template`](#struct-method-template),
[`chain`](#struct-method-chain)

[**Field configuration**](#field-configuration): [`skip`](#field-skip), [`rename`](#field-rename),
[`argument`](#field-argument), [`vis`](#field-vis), [`deref`](#field-deref)

[**Field method configuration**](#field-method-configuration): [`rename`](#field-method-rename),
[`argument`](#field-method-rename), [`doc`](#field-method-doc), [`chain`](#field-method-chain),
[`copy`](#field-method-copy), [`skip`](#field-method-skip), [`deref`](#field-method-deref)

[How fieldwork selects which methods to generate for which
fields](#how-fieldwork-selects-which-methods-to-generate-for-which-fields)


## Example to get a sense of the library


This contrived example intentionally exercises a number of configuration options in order to
demonstrate capabilities. Most real-world usage will not require this much configuration.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct User {
    /// whether this user is an admin
    ///
    /// Note that this is distinct from the notion of group administration,
    /// for historical reasons
    #[fieldwork(argument = is_admin, get(copy, rename = is_admin), get_mut = is_admin_mut)]
    admin: bool,

    /// the user's name
    #[fieldwork(deref = str)]
    name: String,

    #[fieldwork(skip)]
    private: (),

    /// read-only unique identifier
    #[fieldwork(deref = "[u8]", opt_in, get)]
    id: Vec<u8>,
}
```

This generates all of the following code:

```rust
// GENERATED
# struct User { admin: bool, name: String, private: (), id: Vec<u8> }
impl User {
    /**Returns a copy of whether this user is an admin

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    /**Mutably borrow whether this user is an admin

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn is_admin_mut(&mut self) -> &mut bool {
        &mut self.admin
    }
    /**Sets whether this user is an admin, returning `&mut Self` for chaining

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn set_admin(&mut self, is_admin: bool) -> &mut Self {
        self.admin = is_admin;
        self
    }
    /**Owned chainable setter for whether this user is an admin, returning `Self`

Note that this is distinct from the notion of group administration,
for historical reasons*/
    #[must_use]
    pub fn with_admin(mut self, is_admin: bool) -> Self {
        self.admin = is_admin;
        self
    }
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    ///Borrows read-only unique identifier
    pub fn id(&self) -> &[u8] {
        &self.id
    }
}
```

<br/><hr/><br/>

## Methods

Fieldwork supports four distinct methods: `get`, `set`, `get_mut`, and `with`.

### `get`

Borrows the field. This can also be used to return a copy of the field using the
`#[fieldwork(get(copy))]` annotation on a field.

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
// GENERATED
# struct User { admin: bool, name: String }
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

By default, set returns `&mut self` for chainable setters. If you would prefer to return `()`, use
`#[fieldwork(set(chain = false))]` on the struct or an individual field.

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
// GENERATED
# struct User { admin: bool, name: String }
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
// GENERATED
# struct User { admin: bool, name: String }
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

The `with` method provides a chainable owned setter, for situations that require returning the
struct after modification.

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
// GENERATED
# struct User { admin: bool, name: String }
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

<br/><hr/><br/>

### Struct Configuration

<h4 id="struct-vis"> <code>vis</code> </h4> Sets the visibility for all generated functions, unless
otherwise overridden.  `#[fieldwork(vis = "pub")]` is the default. For `pub(crate)`, use
`#[fieldwork(vis = "pub(crate)")]`. To set private visibility, use `#[fieldwork(vis = "")]`.

<h4 id="struct-where-clause">
<code>where_clause</code>
</h4>

This option allows you to specify a where clause for the implementation block, such as:

```rust
# trait Precious {}
#[derive(fieldwork::Fieldwork, Clone)]
#[fieldwork(get, set, get_mut, with, where_clause = "PocketContents: Precious")]
struct Hobbit<PocketContents> {
    /// what the hobbit has in his pocket
    pocket_contents: PocketContents,
}
```
```rust
// GENERATED
# trait Precious {}; struct Hobbit<PocketContents> { pocket_contents: PocketContents }
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

<br/><hr/><br/>

### Struct Method Configuration

<h4 id="struct-method-vis"><code>vis</code></h4>

Override the struct-level definition for a specific method.  `#[vis = "pub(crate)", get(vis =
"pub"), set, get_mut]` uses `pub(crate)` for all methods other than get, which uses "pub".

<h4 id="struct-method-doc-template"><code>doc_template</code></h4>

Override the default documentation template for the specific method. Let's say we want our
documentation to say "assigns" instead of "sets":

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
// GENERATED
# struct User { admin: bool, name: String }
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



<h4 id="struct-method-template"><code>template</code></h4>

Override the method naming for all generated functions of this type. Let's say we want our set
signature to be `assign_admin` instead of `set_admin` and `assign_name`:

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
// GENERATED
# struct User { admin: bool, name: String }
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

<h4 id="struct-method-chain"><code>chain</code> (<code>set</code> only)</h4>

As discussed in the [Set](#set) section above, set returns `&mut Self` by default. To disable this,
specify `chain = false`:

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
// GENERATED
# struct User { admin: bool, name: String }
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

<br/><hr/><br/>

### Field Configuration

<h4 id="field-skip"><code>skip</code></h4>

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
// GENERATED
# struct User { admin: bool, name: String, private: () }
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

<h4 id="field-rename"><code>rename</code></h4>

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
// GENERATED
# struct User { superadmin: bool }
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

<h4 id="field-argument"><code>argument</code></h4>

Change the name of the argument for `with` and `set`. This is occasionally important for rustdocs
and lsp.

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
// GENERATED
# struct User { admin: bool }
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

<h4 id="field-vis"><code>vis</code></h4>

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
// GENERATED
# struct User { admin: bool, name: String }
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

<h4 id="field-deref"><code>deref</code></h4>

For `get` and `get_mut`, return this derefenced type. Some types such as `[u8]` will require
quoting.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut)]
struct User {
    /// the user's name
    #[fieldwork(deref = str)]
    name: String,

    /// a small image in jpg format
    #[fieldwork(deref = "[u8]")]
    profile_thumbnail: Vec<u8>,
}
```

```rust
// GENERATED
# struct User { name: String, profile_thumbnail: Vec<u8> }
impl User {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Borrows a small image in jpg format
    pub fn profile_thumbnail(&self) -> &[u8] {
        &self.profile_thumbnail
    }
    ///Mutably borrow a small image in jpg format
    pub fn profile_thumbnail_mut(&mut self) -> &mut [u8] {
        &mut self.profile_thumbnail
    }
    ///Sets a small image in jpg format, returning `&mut Self` for chaining
    pub fn set_profile_thumbnail(&mut self, profile_thumbnail: Vec<u8>) -> &mut Self {
        self.profile_thumbnail = profile_thumbnail;
        self
    }
}
```

<br/><hr/><br/>

### Field Method Configuration

<h4 id="field-method-rename"><code>rename</code></h4>

Specify the full function name for this particular method. Note that this overrides both `template`
and field-level [`rename`](#rename).

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(get(rename = is_an_admin))]
    /// whether this user is an admin
    admin: bool,
}
```
```rust
// GENERATED
# struct User { admin: bool }
impl User {
    /// Borrows whether this user is an admin
    pub fn is_an_admin(&self) -> &bool {
        &self.admin
    }
}
```

If there are no other configuration options needed, this can be provided with the following shortcut
(generates the same code as the above):

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(get = is_an_admin)]
    /// whether this user is an admin
    admin: bool,
}
```


<h4 id="field-method-argument"><code>argument</code></h4>

Specify the name of the argument for this specific method and field.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(set(argument = is_an_admin))]
    /// whether this user is an admin
    admin: bool,
}
```

```rust
// GENERATED
# struct User { admin: bool }
impl User {
    /// Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, is_an_admin: bool) -> &mut Self {
        self.admin = is_an_admin;
        self
    }
}
```

<h4 id="field-method-doc"><code>doc</code></h4>

Override the documentation for this specific method and field.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(set(doc = "Specify whether this user can administer this system"))]
    admin: bool,
}
```

```rust
// GENERATED
# struct User { admin: bool }
impl User {
    ///Specify whether this user can administer this system
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
}
```

<h4 id="field-method-chain"><code>chain</code> (<code>set</code> only)</h4>

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
// GENERATED
# struct User { admin: bool }
impl User {
    /// Sets whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
}
```

<h4 id="field-method-copy"><code>copy</code> (<code>get</code> only)</h4>

Sometimes it is more useful to return a `Copy` of the returned type instead of a borrow. To opt into
this behavior for a specific field, use `#[fieldwork(get(copy))]`:

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    /// whether the user is an admin
    #[fieldwork(get(copy))]
    admin: bool,
}
```
```rust
// GENERATED
# struct User { admin: bool }
impl User {
    /// Returns a copy of whether the user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
}
```

<h4 id="field-method-skip"><code>skip</code></h4>

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
// GENERATED
# struct User { admin: bool, name: String }
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

<h4 id="field-method-deref"><code>deref</code></h4>

For `get` and `get_mut`, return this derefenced type for this specific method and field. Some types
such as `[u8]` will require quoting.

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    /// the user's name
    #[fieldwork(get(deref = str), set, get_mut)]
    name: String,

    /// a small image in jpg format
    #[fieldwork(get_mut(deref = "[u8]"), get, set)]
    profile_thumbnail: Vec<u8>,
}
```

```rust
// GENERATED
# struct User { name: String, profile_thumbnail: Vec<u8> }
impl User {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Borrows a small image in jpg format
    pub fn profile_thumbnail(&self) -> &Vec<u8> {
        &self.profile_thumbnail
    }
    ///Mutably borrow a small image in jpg format
    pub fn profile_thumbnail_mut(&mut self) -> &mut [u8] {
        &mut self.profile_thumbnail
    }
    ///Sets a small image in jpg format, returning `&mut Self` for chaining
    pub fn set_profile_thumbnail(&mut self, profile_thumbnail: Vec<u8>) -> &mut Self {
        self.profile_thumbnail = profile_thumbnail;
        self
    }
}
```

<br/><hr/><br/>


## How fieldwork selects which methods to generate for which fields

In order to be maximally expressive, fieldwork can operate in both opt-in and opt-out
mode. `#[derive(fieldwork::Fieldwork)]` does nothing without at least one `#[fieldwork]`
attribute.

### Opt-out

If a `#[fieldwork(get, set, with, get_mut)]` attribute is applied to the struct, it applies those
methods to all fields that don't have `#[fieldwork(skip)]` (to skip the entire field) or, using get
as an example, `#[fieldwork(get(skip)]` to skip just the get method for the particular field.

### Opt-in

It is also possible to omit the struct-level attribute and opt individual fields in with eg
`#[fieldwork(get, set)]`.

If you need to specify struct-level configuration in order to reduce repetition but still want to
operate in an opt-in mode instead of using `skip`, fieldwork supports `opt_in` as a top level
argument.  It is also possible to specify `opt_in` at a field level, which will only include the
methods specified on that field.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(opt_in, get(template = "get_{}"))]
struct User {
    /// whether this user is an admin
    #[fieldwork(get)]
    admin: bool,

    /// the user's name
    #[fieldwork(set)]
    name: String,

    private: ()
}
```
```rust
// GENERATED
# struct User { admin: bool, name: String, private: () }
impl User {
    ///Borrows whether this user is an admin
    pub fn get_admin(&self) -> &bool {
        &self.admin
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
```
