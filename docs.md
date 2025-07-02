# ⛏️ fieldwork - field accessor generation

`fieldwork` is a procedural macro crate designed to automate the generation of field accessor
methods for structs. By leveraging Rust's powerful macro system, `fieldwork` reduces
boilerplate code, enhances code readability, and ensures consistency. Just as importantly,
`fieldwork` intends to be fully customizable and expressive for common access patterns.

Manually writing getters and setters for struct fields is repetitive, and adds to maintenance
burden. `fieldwork` addresses this by providing a procedural macro that automatically generates
these methods based on your struct definitions. The intent of this crate, and distinguishing
feature, is to be as customizable and expressive as writing your own getters and setters. The crate
succeeds if you are able to emit exactly the code that you would have manually written, but far more
concisely.

Although this crate is fully configurable and these docs carefully describe the many configuration
settings, wherever possible, fieldwork tries to express common patterns as the default.

## Performance

The compile time cost of using a proc macro crate is always worth considering. All efforts have been
made to keep this crate as lightweight as possible and featureful enough to be worth the tradeoff.

## Testing

[![ci][ci-badge]][ci] [![codecov][codecov-badge]][codecov]

[ci]: https://github.com/jbr/fieldwork/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/fieldwork/workflows/CI/badge.svg
[codecov-badge]: https://codecov.io/gh/jbr/fieldwork/graph/badge.svg?token=tlWtminkYf
[codecov]: https://codecov.io/gh/jbr/fieldwork


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
[`where_clause`](#struct-where-clause), [`option_borrow_inner`](#struct-option),
[`deref`](#struct-deref), [`option_set_some`](#struct-option_set_some), [`into`](#struct-into),
[`rename_predicates`](#struct-rename_predicates)

[**Struct method configuration**](#struct-method-configuration): [`vis`](#struct-method-vis),
[`doc_template`](#struct-method-doc-template), [`template`](#struct-method-template),
[`chain`](#struct-method-chain), [`option_borrow_inner`](#struct-method-option),
[`deref`](#struct-method-deref), [`option_set_some`](#struct-method-option_set_some),
[`into`](#struct-method-into), [`copy`](#struct-method-copy)

[**Field configuration**](#field-configuration): [`skip`](#field-skip), [`rename`](#field-rename),
[`argument`](#field-argument), [`vis`](#field-vis), [`deref`](#field-deref),
[`option_set_some`](#field-option_set_some), [`into`](#field-into),
[`option_borrow_inner`](#field-option)

[**Field method configuration**](#field-method-configuration): [`rename`](#field-method-rename),
[`argument`](#field-method-argument), [`doc`](#field-method-doc), [`chain`](#field-method-chain),
[`copy`](#field-method-copy), [`skip`](#field-method-skip), [`deref`](#field-method-deref),
[`option_set_some`](#field-method-option_set_some), [`into`](#field-method-into),
[`option_borrow_inner`](#field-method-option)

[How fieldwork selects which methods to generate for which
fields](#how-fieldwork-selects-which-methods-to-generate-for-which-fields)


## Example to get a sense of the library

```rust
# // docgen-skip
use std::path::PathBuf;

#[derive(fieldwork::Fieldwork, Default)]
#[fieldwork(get, set, with, get_mut, into, option_set_some, rename_predicates)]
struct ServerConfig {
    /// server hostname
    host: String,

    /// server port
    #[fieldwork(into = false)]
    port: u16,

    /// path to SSL certificate file
    #[fieldwork(option_borrow_inner = false)]
    ssl_cert: Option<PathBuf>,

    /// path to log directory  
    log_dir: Option<PathBuf>,

    /// whether TLS is required
    tls_required: bool,

    /// whether verbose logging is enabled
    verbose: bool,

    #[fieldwork(skip)]
    _runtime_data: (),
}

// Usage examples:
let mut config = ServerConfig::default()
    .with_host("LocalHost") // accepts &str via Into<String>
    .with_port(8080)
    .with_log_dir("/var/log") // accepts &str, wraps in Some() automatically
    .with_tls_required(true)
    .with_verbose(false);

config.host_mut().make_ascii_lowercase();

// Getters use idiomatic naming
assert_eq!(config.host(), "localhost");
assert_eq!(config.port(), 8080);
assert_eq!(config.log_dir().unwrap(), PathBuf::from("/var/log"));
assert!(config.is_tls_required()); // boolean getters use is_ prefix because of `rename_predicates`
assert!(!config.is_verbose());

// Chainable setters return &mut Self
config
    .set_ssl_cert(PathBuf::from("/etc/ssl/cert.pem"))
    .set_port(9090)
    .set_verbose(true);

let cert = config.ssl_cert_mut().take();
assert_eq!(cert, Some(PathBuf::from("/etc/ssl/cert.pem")));
```

<br/><hr/><br/>

## General notes and configuration

### Fieldwork has four configuration levels that cascade

Configuration at each field method inherits from the field's configuration, the struct configuration
for that method, and the struct's top level configuration. The most specific configuration always
takes precedence. The intent of this approach is to avoid duplication and do what you intend.

### Boolean handling

`#[fieldwork(option_borrow_inner = true)]` is the same as `#[fieldwork(option_borrow_inner)]` and
this is the case anywhere booleans are accepted.

### Type quoting

Some types will need to be quoted if they contain lifetimes, brackets, or generics. Simple path
types like `std::sync::Arc` do not need to be quoted.

### Common dereference types are detected by default

Cwned types that `Deref` to commonly used borrowed types will automatically get detected and
dereferenced as such. So for example, a field that contains a `String` will return `&str` from `get`
or `&mut str` from `get_mut` by default. This behavior can be opted out of, at any configuration
level.

| Owned Type | Borrowed Type |
|------------|---------------|
| `String`   | `&str`        |
| `OsString  | `&OsStr`      |
| `Vec<T>`   | `&[T]`        |
| `Box<T>`   | `&T`          |
| `Arc<T>`   | `&T`          |
| `Rc<T>`    | `&T`          |
| `PathBuf`  | `&Path`       |
| `Cow<T>`   | `&T`          |
| `[T; N]`   | `&[T]`        |

### Common copy types are detected by default

The current list of types that are detected are: `bool`, `char`, numeric primitives, and immutable
references (`&T`). Almost always, if a getter returns a `&bool`, the caller will want to dereference
that immediately, so by default `get` returns those types by copy. This behavior can be opted out of
at the struct method level with `#[fieldwork(get(copy = false))]` or at the field method level with
the same invocation.

### Options are returned `as_ref` or `as_deref` by default

By default, fieldwork detects Options and calls `as_deref` or `as_ref` on them, so instead of getting
`&Option<String>`, you get `Option<&str>` by default. It is possible to opt out of the option
detection behavior and the deref detection behavior distinctly, so you can have it return
`Option<&String>` or `&Option<String>`, at any configuration level.

### Option setters can automatically wrap values in Some

When `option_set_some` is enabled, `set` and `with` methods for `Option<T>` fields will accept `T`
as input and automatically wrap it in `Some(T)`. This is useful for builder patterns and situations
where `None` represents an unset default value that is only ever replaced with populated
values. Instead of calling `user.set_name(Some("Alice".to_string()))`, you can simply call
`user.set_name("Alice".to_string())`. This feature can be enabled at any configuration level and
only affects setter methods - getters remain unchanged.

### Setters can accept impl Into<T> for ergonomic APIs

When `into` is enabled, `set` and `with` methods will accept `impl Into<T>` instead of `T` as their
parameter. This allows callers to pass any type that can be converted into the field type, making
APIs more ergonomic. For example, a `String` field can accept `&str`, `String`, `Cow<str>`, or any
other type implementing `Into<String>`. This feature works seamlessly with `option_set_some` - when
both are enabled, the setter accepts `impl Into<T>` and wraps the result in `Some()`. This feature
can be enabled at any configuration level and only affects setter methods.

### Tuple struct support

Fieldwork supports both named structs and tuple structs. For tuple structs, you must provide a
`name` attribute for each field you want to generate methods for. Fields without a `name` attribute
are ignored.

```rust
# // docgen-skip
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct Color(
    #[fieldwork(name = red)] u8,
    #[fieldwork(name = green)] u8,
    #[fieldwork(name = blue)] u8,
);

// Usage
let color = Color(255, 128, 0)
    .with_red(200)
    .with_blue(100);

assert_eq!(color.red(), 200);
assert_eq!(color.green(), 128);
assert_eq!(color.blue(), 100);
```

<br/><hr/><br/>

## Methods

Fieldwork supports four distinct methods: `get`, `set`, `get_mut`, and `with`.

### `get`

Borrows the field. This can also be used to return a copy of the field using the
`#[fieldwork(get(copy))]` annotation on a field, or when common copy types are detected (see above).

#### Example:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct User {
    /// whether this user is an admin
    admin: bool,

    /// the user's name
    name: String,

    /// the user's age, if set
    age: Option<u8>,

    /// favorite color, if set
    favorite_color: Option<String>
}
```

generates 

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
    ///Borrows the user's age, if set
    pub fn age(&self) -> Option<&u8> {
        self.age.as_ref()
    }
    ///Borrows favorite color, if set
    pub fn favorite_color(&self) -> Option<&str> {
        self.favorite_color.as_deref()
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
# struct User { admin: bool, name: String, }
impl User {
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    ///Sets the user's name, returning `&mut Self` for chaining
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

    /// the user's age, if set
    age: Option<u8>,

    /// favorite color, if set
    favorite_color: Option<String>
}
```
generates the following impl block

```rust
// GENERATED
# struct User { admin: bool, name: String, age: Option<u8>, favorite_color: Option<String>, }
impl User {
    ///Mutably borrow whether this user is an admin
    pub fn admin_mut(&mut self) -> &mut bool {
        &mut self.admin
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
    }
    ///Mutably borrow the user's age, if set
    pub fn age_mut(&mut self) -> Option<&mut u8> {
        self.age.as_mut()
    }
    ///Mutably borrow favorite color, if set
    pub fn favorite_color_mut(&mut self) -> Option<&mut str> {
        self.favorite_color.as_deref_mut()
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
# struct User { admin: bool, name: String, }
impl User {
    ///Owned chainable setter for whether this user is an admin, returning `Self`
    #[must_use]
    pub fn with_admin(mut self, admin: bool) -> Self {
        self.admin = admin;
        self
    }
    ///Owned chainable setter for the user's name, returning `Self`
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
# trait Precious { }
# struct Hobbit<PocketContents>{ pocket_contents: PocketContents, }
impl<PocketContents> Hobbit<PocketContents>
where
    PocketContents: Precious,
{
    ///Borrows what the hobbit has in his pocket
    pub fn pocket_contents(&self) -> &PocketContents {
        &self.pocket_contents
    }
    ///Mutably borrow what the hobbit has in his pocket
    pub fn pocket_contents_mut(&mut self) -> &mut PocketContents {
        &mut self.pocket_contents
    }
    ///Sets what the hobbit has in his pocket, returning `&mut Self` for chaining
    pub fn set_pocket_contents(&mut self, pocket_contents: PocketContents) -> &mut Self {
        self.pocket_contents = pocket_contents;
        self
    }
    ///Owned chainable setter for what the hobbit has in his pocket, returning `Self`
    #[must_use]
    pub fn with_pocket_contents(mut self, pocket_contents: PocketContents) -> Self {
        self.pocket_contents = pocket_contents;
        self
    }
}

```

<h4 id="struct-option">
<code>option_borrow_inner</code>
</h4>

Opt out of Option detection with `option_borrow_inner = false`. Instead of `get` returning
`Option<&T>` and `get_mut` returning `Option<&mut T>`, `get` returns `&Option<T>` and `get_mut`
returns `&mut Option<T>`. Default behavior is for Option detection to be enabled at the struct
level.

```rust
#[derive(fieldwork::Fieldwork, Clone)]
#[fieldwork(get, get_mut, option_borrow_inner = false)]
struct User {
    // the user's name
    name: Option<String>
}
```
```rust
// GENERATED
# struct User { name: Option<String>, }
impl User {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut Option<String> {
        &mut self.name
    }
}

```

<h4 id="struct-deref">
<code>deref</code>
</h4>

Opt out of auto-deref at the struct level with `deref = false`. See the Deref section for more
information.

```rust
#[derive(fieldwork::Fieldwork, Clone)]
#[fieldwork(get, get_mut, deref = false)]
struct User {
    // the user's name
    name: String
}
```
```rust
// GENERATED
# struct User { name: String, }
impl User {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

```

<h4 id="struct-option_set_some">
<code>option_set_some</code>
</h4>

Enable automatic wrapping of setter values in `Some()` for `Option<T>` fields at the struct level
with `option_set_some` or `option_set_some = true`. When enabled, `set` and `with` methods for
Option fields will accept the inner type `T` instead of `Option<T>` and automatically wrap the
value. This is particularly useful for builder patterns where `None` represents an unset default
value.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, option_set_some)]
struct User {
    /// the user's nickname, if provided
    nickname: Option<String>,
}
```
```rust
// GENERATED
# struct User { nickname: Option<String>, }
impl User {
    ///Sets the user's nickname, if provided, returning `&mut Self` for chaining
    pub fn set_nickname(&mut self, nickname: String) -> &mut Self {
        self.nickname = Some(nickname);
        self
    }
    ///Owned chainable setter for the user's nickname, if provided, returning `Self`
    #[must_use]
    pub fn with_nickname(mut self, nickname: String) -> Self {
        self.nickname = Some(nickname);
        self
    }
}

```

<h4 id="struct-into">
<code>into</code>
</h4>

Enable `impl Into<T>` parameters for setter methods at the struct level with `into` or `into =
true`. When enabled, `set` and `with` methods will accept `impl Into<T>` instead of `T`, allowing
callers to pass any type that can be converted into the field type.

```rust
# use std::path::PathBuf;
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with, into)]
struct User {
    /// the user's name
    name: String,
    
    /// the user's home directory
    home_dir: PathBuf,
}
```
```rust
// GENERATED
# use std::path::PathBuf;
# struct User { name: String, home_dir: PathBuf, }
impl User {
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
    ///Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    ///Sets the user's home directory, returning `&mut Self` for chaining
    pub fn set_home_dir(&mut self, home_dir: impl Into<PathBuf>) -> &mut Self {
        self.home_dir = home_dir.into();
        self
    }
    ///Owned chainable setter for the user's home directory, returning `Self`
    #[must_use]
    pub fn with_home_dir(mut self, home_dir: impl Into<PathBuf>) -> Self {
        self.home_dir = home_dir.into();
        self
    }
}

```

<h4 id="struct-rename_predicates">
<code>rename_predicates</code>
</h4>

Rename all bool-returning methods to `is_{}` at the struct level with `rename_predicates` or
`rename_predicates = true`.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, rename_predicates)]
struct User {
    admin: bool
}
```
```rust
// GENERATED
# struct User { admin: bool, }
impl User {
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    pub fn admin_mut(&mut self) -> &mut bool {
        &mut self.admin
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
# struct User { admin: bool, name: String, }
impl User {
    ///Assigns whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    ///Assigns the user's name
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
# struct User { admin: bool, name: String, }
impl User {
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn assign_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    ///Sets the user's name, returning `&mut Self` for chaining
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
# struct User { admin: bool, name: String, }
impl User {
    ///Sets whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
    ///Sets the user's name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

```

<h4 id="struct-method-option"><code>option_borrow_inner</code></h4>

Opt out of Option detection with `option_borrow_inner = false`, or if it has been opted out at the
struct level, opt back in with `option_borrow_inner` or `option_borrow_inner = true` for a single
method, as in `get(option_borrow_inner)` or `get_mut(option_borrow_inner = true)`. See
[option_borrow_inner](#struct-option) above for more information.


<h4 id="struct-method-deref">
<code>deref</code>
</h4>

Opt out of auto-deref at the struct method level with `deref = false`. See the Deref section for more
information.

```rust
#[derive(fieldwork::Fieldwork, Clone)]
#[fieldwork(get, get_mut(deref = false))]
struct User {
    // the user's name
    name: String
}
```
```rust
// GENERATED
# struct User { name: String, }
impl User {
    pub fn name(&self) -> &str {
        &*self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

```


<h4 id="struct-method-option_set_some"><code>option_set_some</code> (<code>set</code> and <code>with</code> only)</h4>

Enable automatic wrapping of setter values in `Some()` for `Option<T>` fields for a specific
method. This can be used to enable the feature for just `set` or just `with`, or to override the
struct-level setting.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(option_set_some), with)]
struct User {
    /// the user's nickname, if provided
    nickname: Option<String>,
}
```
```rust
// GENERATED
# struct User { nickname: Option<String>, }
impl User {
    ///Sets the user's nickname, if provided, returning `&mut Self` for chaining
    pub fn set_nickname(&mut self, nickname: String) -> &mut Self {
        self.nickname = Some(nickname);
        self
    }
    ///Owned chainable setter for the user's nickname, if provided, returning `Self`
    #[must_use]
    pub fn with_nickname(mut self, nickname: Option<String>) -> Self {
        self.nickname = nickname;
        self
    }
}

```

<h4 id="struct-method-into"><code>into</code> (<code>set</code> and <code>with</code> only)</h4>

Enable `impl Into<T>` parameters for setter methods for a specific method. This can be used to
enable the feature for just `set` or just `with`, or to override the struct-level setting.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(into), with)]
struct User {
    /// the user's name
    name: String,
}
```
```rust
// GENERATED
# struct User { name: String, }
impl User {
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
    ///Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

```

<h4 id="struct-method-copy"><code>copy</code> (<code>get</code> only)</h4>

By default, common Copy types such as bool will be returned by copy instead of by reference. To opt
out of this behavior for a whole struct, use `#[fieldwork(get(copy = false))]`.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(copy = false))]
struct Collection {
    /// length
    len: usize,

    /// enabled
    enabled: bool,
}
```
```rust
// GENERATED
# struct Collection { len: usize, enabled: bool, }
impl Collection {
    ///Borrows length
    pub fn len(&self) -> &usize {
        &self.len
    }
    ///Borrows enabled
    pub fn enabled(&self) -> &bool {
        &self.enabled
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
# struct User { admin: bool, name: String, private: (), }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
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
# struct User { superadmin: bool, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn admin(&self) -> bool {
        self.superadmin
    }
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
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
# struct User { admin: bool, }
impl User {
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, is_admin: bool) -> &mut Self {
        self.admin = is_admin;
        self
    }
    ///Owned chainable setter for whether this user is an admin, returning `Self`
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
# struct User { admin: bool, name: String, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
    pub fn set_admin(&mut self, admin: bool) -> &mut Self {
        self.admin = admin;
        self
    }
    ///Borrows the user's name
    pub(crate) fn name(&self) -> &str {
        &*self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub(crate) fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

<h4 id="field-deref"><code>deref</code></h4>

If set to `true`, this opts the field into deref detection for common types if the struct or struct-method have turned `deref = false`.
If set to `false`, this opts the specific field out of deref detection for common types, borrowing the owned type.
If set to a specific type, dereference to the specific type. Some types such as `[u8]` will require
quoting.

```rust
# use std::sync::Arc;
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct User {
    /// the user's name
    name: String,

    /// a small image in jpg format
    #[fieldwork(deref = "[u8]")]
    profile_thumbnail: Vec<u8>,

    // opt out of deref detection so we can use the arc directly
    #[fieldwork(deref = false)]
    an_arc: Arc<()>,
}
```

```rust
// GENERATED
# use std::sync::Arc;
# struct User { name: String, profile_thumbnail: Vec<u8>, an_arc: Arc<()>, }
impl User {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
    }
    ///Borrows a small image in jpg format
    pub fn profile_thumbnail(&self) -> &[u8] {
        &*self.profile_thumbnail
    }
    ///Mutably borrow a small image in jpg format
    pub fn profile_thumbnail_mut(&mut self) -> &mut [u8] {
        &mut *self.profile_thumbnail
    }
    pub fn an_arc(&self) -> &Arc<()> {
        &self.an_arc
    }
    pub fn an_arc_mut(&mut self) -> &mut Arc<()> {
        &mut self.an_arc
    }
}

```


<h4 id="field-option_set_some"><code>option_set_some</code></h4>

Enable or disable automatic wrapping of setter values in `Some()` for this specific Option field
with `option_set_some = true` or `option_set_some = false`. This allows you to override struct or
struct-method level settings for individual fields.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with)]
struct User {
    /// Nickname - uses regular Option setter
    #[fieldwork(option_set_some = false)]
    nickname: Option<String>,
    
    /// Display name - uses automatic Some() wrapping
    #[fieldwork(option_set_some = true)]
    display_name: Option<String>,
}
```

```rust
// GENERATED
# struct User { nickname: Option<String>, display_name: Option<String>, }
impl User {
    ///Sets Nickname - uses regular Option setter, returning `&mut Self` for chaining
    pub fn set_nickname(&mut self, nickname: Option<String>) -> &mut Self {
        self.nickname = nickname;
        self
    }
    ///Owned chainable setter for Nickname - uses regular Option setter, returning `Self`
    #[must_use]
    pub fn with_nickname(mut self, nickname: Option<String>) -> Self {
        self.nickname = nickname;
        self
    }
    ///Sets Display name - uses automatic Some() wrapping, returning `&mut Self` for chaining
    pub fn set_display_name(&mut self, display_name: String) -> &mut Self {
        self.display_name = Some(display_name);
        self
    }
    ///Owned chainable setter for Display name - uses automatic Some() wrapping, returning `Self`
    #[must_use]
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }
}

```

<h4 id="field-into"><code>into</code></h4>

Enable or disable `impl Into<T>` parameters for setter methods for this specific field with
`into`/`into = true` or `into = false`. This allows you to override struct or struct-method level
settings for individual fields.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with)]
struct User {
    name: String,
    
    /// Display name - uses impl Into<String> parameter
    #[fieldwork(into)]
    display_name: String,
}
```

```rust
// GENERATED
# struct User { name: String, display_name: String, }
impl User {
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    ///Sets Display name - uses impl Into<String> parameter, returning `&mut Self` for chaining
    pub fn set_display_name(&mut self, display_name: impl Into<String>) -> &mut Self {
        self.display_name = display_name.into();
        self
    }
    ///Owned chainable setter for Display name - uses impl Into<String> parameter, returning `Self`
    #[must_use]
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }
}

```

<h4 id="field-option"><code>option_borrow_inner</code></h4>

Opt out of Option detection for this field with `option_borrow_inner = false`, or if it has been
opted out at the struct or struct method level, opt back in with `option_borrow_inner` or
`option_borrow_inner = true` for a single field, as in `#[fieldwork(option_borrow_inner)]` or
`#[fieldwork(option_borrow_inner = true)]`. See [option_borrow_inner](#struct-option) above for more
information.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct User {
    profile_thumbnail: Option<Vec<u8>>,

    #[fieldwork(option_borrow_inner = false)] // opt out of option_borrow_inner
    nickname: Option<String>,
}
```

```rust
// GENERATED
# struct User { profile_thumbnail: Option<Vec<u8>>, nickname: Option<String>, }
impl User {
    pub fn profile_thumbnail(&self) -> Option<&[u8]> {
        self.profile_thumbnail.as_deref()
    }
    pub fn profile_thumbnail_mut(&mut self) -> Option<&mut [u8]> {
        self.profile_thumbnail.as_deref_mut()
    }
    pub fn nickname(&self) -> &Option<String> {
        &self.nickname
    }
    pub fn nickname_mut(&mut self) -> &mut Option<String> {
        &mut self.nickname
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
# struct User { admin: bool, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn is_an_admin(&self) -> bool {
        self.admin
    }
}

```

If there are no other configuration options needed, this can be provided with the following shortcut:

```rust
#[derive(fieldwork::Fieldwork)]
struct User {
    #[fieldwork(get = is_an_admin)]
    /// whether this user is an admin
    admin: bool,
}
```
```rust
// GENERATED
# struct User { admin: bool, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn is_an_admin(&self) -> bool {
        self.admin
    }
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
# struct User { admin: bool, }
impl User {
    ///Sets whether this user is an admin, returning `&mut Self` for chaining
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
# struct User { admin: bool, }
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
# struct User { admin: bool, }
impl User {
    ///Sets whether this user is an admin
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
    }
}

```

<h4 id="field-method-copy"><code>copy</code> (<code>get</code> only)</h4>

Sometimes it is more useful to return a `Copy` of the returned type instead of a borrow. To opt into
this behavior for a specific field, use `#[fieldwork(get(copy))]`. To opt out of default copy
behavior for common types such as `bool`, use `#[fieldwork(get(copy = false))]`.

```rust
#[derive(fieldwork::Fieldwork)]
struct Collection {
    /// length
    #[fieldwork(get(copy))]
    len: usize,
}
```
```rust
// GENERATED
# struct Collection { len: usize, }
impl Collection {
    ///Returns a copy of length
    pub fn len(&self) -> usize {
        self.len
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
# struct User { admin: bool, name: String, }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn admin(&self) -> bool {
        self.admin
    }
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

<h4 id="field-method-deref"><code>deref</code></h4>

For `get` and `get_mut`, return this derefenced type for this specific method and field. Some types
such as `[u8]` will require quoting. This can also be set to true or false to opt in or out of deref
detection for common types.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(deref = false)]
struct User {
    /// the user's name
    #[fieldwork(get(deref = str), set, get_mut)]
    name: String,

    /// a small image in jpg format
    #[fieldwork(get_mut(deref = true), get, set)]
    profile_thumbnail: Vec<u8>,
}
```

```rust
// GENERATED
# struct User { name: String, profile_thumbnail: Vec<u8>, }
impl User {
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
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
        &mut *self.profile_thumbnail
    }
    ///Sets a small image in jpg format, returning `&mut Self` for chaining
    pub fn set_profile_thumbnail(&mut self, profile_thumbnail: Vec<u8>) -> &mut Self {
        self.profile_thumbnail = profile_thumbnail;
        self
    }
}

```


<h4 id="field-method-option_set_some"><code>option_set_some</code> (<code>set</code> and
<code>with</code> only)</h4>

Enable or disable automatic wrapping of setter values in `Some()` for a specific method and
field. This provides the most granular control, allowing you to enable the feature for just the
`set` method but not `with`, or vice versa.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with)]
struct User {
    /// Enable automatic Some() wrapping only for the set method
    #[fieldwork(set(option_set_some = true))]
    nickname: Option<String>,
}
```

```rust
// GENERATED
# struct User { nickname: Option<String>, }
impl User {
    ///Sets Enable automatic Some() wrapping only for the set method, returning `&mut Self` for chaining
    pub fn set_nickname(&mut self, nickname: String) -> &mut Self {
        self.nickname = Some(nickname);
        self
    }
    ///Owned chainable setter for Enable automatic Some() wrapping only for the set method, returning `Self`
    #[must_use]
    pub fn with_nickname(mut self, nickname: Option<String>) -> Self {
        self.nickname = nickname;
        self
    }
}

```

<h4 id="field-method-into"><code>into</code> (<code>set</code> and <code>with</code> only)</h4>

Enable or disable `impl Into<T>` parameters for a specific method and field. This provides the most
granular control, allowing you to enable the feature for just the `set` method but not `with`, or
vice versa.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, with)]
struct User {
    /// Enable impl Into<String> only for the set method
    #[fieldwork(set(into))]
    name: String,
}
```

```rust
// GENERATED
# struct User { name: String, }
impl User {
    ///Sets Enable impl Into<String> only for the set method, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
    ///Owned chainable setter for Enable impl Into<String> only for the set method, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

```

<h4 id="field-method-option"><code>option_borrow_inner</code></h4>

Opt out of Option detection for this field and method with `#[fieldwork(option_borrow_inner =
false)]`, or if it has been opted out at the struct, struct method, or field level, opt back in with
`option_borrow_inner` or `option_borrow_inner = true` for a single field and method, as in
`#[fieldwork(get(option_borrow_inner))]` or `#[fieldwork(get_mut(option_borrow_inner = true))]`. See
[option_borrow_inner](#struct-option) above for more information.


```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct User {
    profile_thumbnail: Option<Vec<u8>>,

    // opt out of option_borrow_inner just for get_mut, so we can use Option::insert or similar
    #[fieldwork(get_mut(option_borrow_inner = false))]
    nickname: Option<String>,
}
```

```rust
// GENERATED
# struct User { profile_thumbnail: Option<Vec<u8>>, nickname: Option<String>, }
impl User {
    pub fn profile_thumbnail(&self) -> Option<&[u8]> {
        self.profile_thumbnail.as_deref()
    }
    pub fn profile_thumbnail_mut(&mut self) -> Option<&mut [u8]> {
        self.profile_thumbnail.as_deref_mut()
    }
    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }
    pub fn nickname_mut(&mut self) -> &mut Option<String> {
        &mut self.nickname
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
# struct User { admin: bool, name: String, private: (), }
impl User {
    ///Returns a copy of whether this user is an admin
    pub fn get_admin(&self) -> bool {
        self.admin
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}

```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

---

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
