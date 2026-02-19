# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.7](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.6...fieldwork-v0.4.7) - 2026-02-19

### Other

- restructure into a façade crate so docs can be split up

## [0.4.6](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.5...fieldwork-v0.4.6) - 2025-07-10

### Added

- take

### Fixed

- option_set_some with borrows

### Other

- remove duplication of valid keys
- fix take test
- remove mod.expanded.rs
- rename variable_ident to member for accuracy

## [0.4.5](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.4...fieldwork-v0.4.5) - 2025-07-08

### Fixed

- include the correct number of dereferences inside options
- correctly coerce arrays to slices
- handle `dyn Trait + 'lifetime` by wrapping with parens
- allow specifying `copy` at the field configuration level

### Other

- *(deps)* update rust crate trybuild to v1.0.106
- actually compile expansion tests
- gitignore debug.rs
- add a missing backtick
- *(deps)* update swatinem/rust-cache action to v2.8.0
- use `field = ""` syntax for tuple example

## [0.4.4](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.3...fieldwork-v0.4.4) - 2025-07-07

### Added

- add `#[field = false]`, `#[field = true]`, and `#[field = "renamed"]`
- add support for a `#[field]` attribute synonym

### Other

- update README to use `#[field]`

## [0.4.3](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.2...fieldwork-v0.4.3) - 2025-07-07

### Fixed

- address poorly-handled nested type detection
- Do not auto-deref Rc, Cow, or Arc

### Other

- Merge pull request #60 from jbr/fix-deref-mut
- move example higher in readme

## [0.4.2](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.1...fieldwork-v0.4.2) - 2025-07-03

### Fixed

- accidentally added a dev-dep as a primary dep

### Other

- Merge pull request #55 from jbr/fix-rustversion-dev-dep

## [0.4.1](https://github.com/jbr/fieldwork/compare/fieldwork-v0.4.0...fieldwork-v0.4.1) - 2025-07-03

### Added

- add `without` method
- improved error messages
- support tuple structs

### Other

- add documentation for `without`
- improve how per-method settings are stored
- improve coverage
- only run ui tests on stable
- add ui tests to coverage

## [0.4.0](https://github.com/jbr/fieldwork/compare/fieldwork-v0.3.0...fieldwork-v0.4.0) - 2025-07-01

### Added

- add auto-deref for PathBuf, OsString, and Array
- add more primitive types to copy detection
- add backwards-compatibility acceptance of `option`
- accept impl Into<T> for set and with
- option set some
- [**breaking**] rename option to option_borrow_inner for specificity of configuration
- [**breaking**] rename boolean-returning predicate getters to is_{}

### Fixed

- option-set-some should no-op for non-option types

### Other

- improve main example
- document `into`
- extract common settings
- improve coverage
- document option_set_some
- rename internal `option_handling` to `option_borrow_inner`
- run docs-gen --verify in ci
- docs-generator

## [0.3.0](https://github.com/jbr/fieldwork/compare/v0.2.0...v0.3.0) - 2025-06-15

### Added

- [**breaking**] add usize and u8 to copy detection
- [**breaking**] copy type detection now includes references
- [**breaking**] auto enable copy for common types (currently just bool)
- [**breaking**] Deref detection for common types

### Fixed

- add missing support for specifying a custom deref type within an option.
- remove Cow handling in extract_option_type

## [0.2.0](https://github.com/jbr/fieldwork/compare/v0.1.5...v0.2.0) - 2025-06-14

### Added

- [**breaking**] add Option detection

### Other

- update readme
- *(deps)* update swatinem/rust-cache action to v2.7.8
- add example to readme

## [0.1.5](https://github.com/jbr/fieldwork/compare/v0.1.4...v0.1.5) - 2025-05-17

### Other

- better emoji
- make readme links less ugly
- improve caching of convco bin
- improve readme, fix link to tests in docs

## [0.1.4](https://github.com/jbr/fieldwork/compare/v0.1.3...v0.1.4) - 2025-05-17

### Added

- don't require syn/extra-traits or syn/visit

## [0.1.3](https://github.com/jbr/fieldwork/compare/v0.1.2...v0.1.3) - 2025-05-16

### Added

- add field-level opt_in
- multi-line docs

### Other

- improve intro paragraphs

## [0.1.2](https://github.com/jbr/fieldwork/compare/v0.1.1...v0.1.2) - 2025-05-16

### Added

- add the ability to specify a deref type for get and get_mut

### Fixed

- refine opt-in behavior

### Other

- one last coverage improvement
- improve test coverage
- add in coverage using the same examples as macroexpand
- add a // GENERATED comment to all generated blocks
- use deref in the combined example

## [0.1.1](https://github.com/jbr/fieldwork/compare/v0.1.0...v0.1.1) - 2025-05-16

### Added

- add documentation

### Fixed

- doctests
