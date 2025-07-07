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

## Example to get a sense of the library

```rust
use std::path::PathBuf;

#[derive(fieldwork::Fieldwork, Default)]
#[fieldwork(get, set, with, without, get_mut, into, rename_predicates)]
struct ServerConfig {
    /// server hostname
    host: String,

    /// server port
    #[field(into = false)]
    port: u16,

    /// path to SSL certificate file
    #[field(option_borrow_inner = false)]
    ssl_cert: Option<PathBuf>,

    /// path to log directory  
    log_dir: Option<PathBuf>,

    /// whether TLS is required
    tls_required: bool,

    /// whether verbose logging is enabled
    verbose: bool,

    #[field = false]
    _runtime_data: (),
}

// Usage examples:
let mut config = ServerConfig::default()
    .with_host("LocalHost") // accepts &str via Into<String>
    .with_port(8080)
    .with_log_dir("/var/log") // accepts &str, wraps in Some() automatically
    .with_tls_required() // sets to true
    .without_verbose(); // sets to false

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

// Without methods provide convenient clearing
config = config.without_log_dir(); // Sets log_dir to None
assert!(config.log_dir().is_none());
```

## Performance

The compile time cost of using a proc macro crate is always worth considering. All efforts have been
made to keep this crate as lightweight as possible and featureful enough to be worth the tradeoff.

## Testing

[![ci][ci-badge]][ci]
[![codecov](https://codecov.io/gh/jbr/fieldwork/graph/badge.svg?token=tlWtminkYf)](https://codecov.io/gh/jbr/fieldwork)

[ci]: https://github.com/jbr/fieldwork/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/fieldwork/workflows/CI/badge.svg

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
