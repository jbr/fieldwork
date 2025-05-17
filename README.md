# 📚 fieldwork – field accessor generation

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
