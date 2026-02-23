# ⛏️ fieldwork – field accessor generation
[![ci][ci-badge]][ci]
[![codecov](https://codecov.io/gh/jbr/fieldwork/graph/badge.svg?token=tlWtminkYf)](https://codecov.io/gh/jbr/fieldwork)
[![crates.io version badge][version-badge]][crate]

[ci]: https://github.com/jbr/fieldwork/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/fieldwork/workflows/CI/badge.svg
[version-badge]: https://img.shields.io/crates/v/fieldwork.svg?style=flat-square
[crate]: https://crates.io/crates/fieldwork

`fieldwork` is a procedural macro crate designed to automate the generation of field accessor
methods for structs and enums. By leveraging Rust's powerful macro system, `fieldwork` reduces
boilerplate code, enhances code readability, and ensures consistency. Just as importantly,
`fieldwork` intends to be fully customizable and expressive for common access patterns.

Manually writing getters and setters is repetitive and adds to maintenance burden. `fieldwork`
addresses this by providing a procedural macro that automatically generates these methods based on
your type definitions. The intent of this crate, and distinguishing feature, is to be as
customizable and expressive as writing your own getters and setters. The crate succeeds if you are
able to emit exactly the code that you would have manually written, but far more concisely.

## Example to get a sense of the library

### Structs

```rust
use std::path::PathBuf;

#[derive(fieldwork::Fieldwork, Default)]
#[fieldwork(get, set, with, without, get_mut, rename_predicates)]
struct ServerConfig {
    /// server hostname
    #[field(into)] // accept Into<String>
    host: String,

    /// server port
    port: u16,

    /// path to log directory
    #[field(into, option_set_some)] // accept Into<PathBuf>
    log_dir: Option<PathBuf>,

    /// whether TLS is required
    tls_required: bool,

    /// whether verbose logging is enabled
    verbose: bool,
}

let mut config = ServerConfig::default()
    .with_host("LocalHost")  // accepts &str via Into<String>
    .with_port(8080)
    .with_log_dir("/var/log") // accepts &str, wraps in Some automatically
    .with_tls_required();    // sets bool to true

config.host_mut().make_ascii_lowercase();

assert_eq!(config.host(), "localhost"); // String → &str
assert_eq!(config.port(), 8080);
assert_eq!(config.log_dir().unwrap(), PathBuf::from("/var/log").as_path());
assert!(config.is_tls_required()); // rename_predicates: bool getters use is_ prefix
assert!(!config.is_verbose());

// Chainable setters return &mut Self
config.set_port(9090).set_verbose(true);

config = config.without_log_dir(); // clears Option to None
assert!(config.log_dir().is_none());
```

### Enums

Enums are supported with smart handling of field **coverage** across variants — whether a field
appears in all variants or only some:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, into_field)]
enum ServerEvent {
    Started  { host: String, port: u16 },
    Request  { host: String, port: u16, path: String },
    Shutdown { host: String, port: u16 },
}

let event = ServerEvent::Request {
    host: "example.com".to_string(),
    port: 8080,
    path: "/api/health".to_string(),
};

// host and port appear in every variant → full coverage, same smart defaults as structs
assert_eq!(event.host(), "example.com"); // String → &str
assert_eq!(event.port(), 8080);          // Copy types returned by value

// path appears in only one variant → partial coverage, return type wrapped in Option
assert_eq!(event.path(), Some("/api/health"));
assert_eq!(
    ServerEvent::Started { host: "example.com".to_string(), port: 8080 }.path(),
    None,
);

// into_field is generated for full-coverage fields
assert_eq!(event.into_host(), "example.com");
```

## Performance

The compile time cost of using a proc macro crate is always worth considering. All efforts have been
made to keep this crate as lightweight as possible and featureful enough to be worth the tradeoff.

## Testing

This crate has a full suite of macro-expansion tests in
[tests/expand](https://github.com/jbr/fieldwork/tree/main/tests/expand). These tests are also used
for test coverage.

## Documentation

View the docs for main on github at [docs.md](https://github.com/jbr/fieldwork/blob/main/docs.md),
or on github pages in [rustdoc format](https://jbr.github.io/fieldwork/fieldwork/).  The most recent
release can always be viewed at [docs.rs](https://docs.rs/fieldwork).

## Safety
This crate uses `#![deny(unsafe_code)]`.

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
