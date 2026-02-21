# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.7](https://github.com/jbr/fieldwork/compare/fieldwork-derive-v0.4.6...fieldwork-derive-v0.4.7) - 2026-02-21

### Added

- Initial enum support

### Other

- improve test coverage
- s/struct/item/g towards enums
- make base_expr an arg (towards enums)

## [0.4.6](https://github.com/jbr/fieldwork/releases/tag/fieldwork-derive-v0.4.6) - 2026-02-19

### Added

- take
- add `#[field = false]`, `#[field = true]`, and `#[field = "renamed"]`
- [**breaking**] rename boolean-returning predicate getters to is_{}
- [**breaking**] auto enable copy for common types (currently just bool)
- [**breaking**] Deref detection for common types

### Other

- restructure into a façade crate so docs can be split up
- update README to use `#[field]`
- move example higher in readme
- add documentation for `without`
- improve main example
- fix readme codecov badge
- update readme
- add example to readme
- better emoji
- make readme links less ugly
- improve readme, fix link to tests in docs
- standard repo config and clippy pass
