# ⛏️ fieldwork – field accessor generation

[![crates.io version badge][version-badge]][crate]

[version-badge]: https://img.shields.io/crates/v/fieldwork.svg?style=flat-square
[crate]: https://crates.io/crates/fieldwork

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

[![ci][ci-badge]][ci] [![codecov][codecov-badge]][codecov]

[ci]: https://github.com/jbr/fieldwork/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/fieldwork/workflows/CI/badge.svg
[codecov-badge]: https://codecov.io/gh/jbr/fieldwork/graph/badge.svg?token=tlWtminkYf
[codecov]: https://codecov.io/gh/jbr/fieldwork


This crate has a full suite of macro-expansion tests in
[tests/expand](https://github.com/jbr/fieldwork/tree/main/tests/expand). These tests are also used
for test coverage.

## Documentation

View the docs for main on github at [docs.md](https://github.com/jbr/fieldwork/blob/main/docs.md),
or on github pages in [rustdoc format](https://jbr.github.io/fieldwork/fieldwork/).  The most recent
release can always be viewed at [docs.rs](https://docs.rs/fieldwork).

## Safety
This crate uses `#![deny(unsafe_code)]`.

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
    name: String,

    /// the user's favorite color, if set
    favorite_color: Option<String>,

    #[fieldwork(skip)]
    private: (),

    /// read-only unique identifier
    #[fieldwork(opt_in, get)]
    id: Vec<u8>,
}
```

This generates all of the following code:

```rust
// GENERATED

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
        &*self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
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
    ///Borrows the user's favorite color, if set
    pub fn favorite_color(&self) -> Option<&str> {
        self.favorite_color.as_deref()
    }
    ///Mutably borrow the user's favorite color, if set
    pub fn favorite_color_mut(&mut self) -> Option<&mut str> {
        self.favorite_color.as_deref_mut()
    }
    ///Sets the user's favorite color, if set, returning `&mut Self` for chaining
    pub fn set_favorite_color(&mut self, favorite_color: Option<String>) -> &mut Self {
        self.favorite_color = favorite_color;
        self
    }
    ///Owned chainable setter for the user's favorite color, if set, returning `Self`
    #[must_use]
    pub fn with_favorite_color(mut self, favorite_color: Option<String>) -> Self {
        self.favorite_color = favorite_color;
        self
    }
    ///Borrows read-only unique identifier
    pub fn id(&self) -> &[u8] {
        &*self.id
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
