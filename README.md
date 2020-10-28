# Crate locale-types

![mit License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/locale-types.svg)](https://crates.io/crates/locale-types)
[![docs.rs](https://docs.rs/locale-types/badge.svg)](https://docs.rs/locale-types)
![Build](https://github.com/johnstonskj/locale-types/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/locale-types/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/locale-types.svg)](https://github.com/johnstonskj/locale-types/stargazers)

This crate provides a `Locale` enumeration, `LocaleIdentifier` trait, and a 
`LocaleString` structure are provided that may be used to parse and construct 
locale identifiers in a standards-conformant manner.

It is used by the `locale-codes` and `locale-settings` crates.

## Example

```rust
use locale_types::{LocaleIdentifier, LocaleString};

let locale = LocaleString::new("en".to_string()).unwrap()
    .with_territory("US".to_string()).unwrap()
    .with_code_set("UTF-8".to_string()).unwrap()
    .with_modifier("collation=pinyin;currency=CNY".to_string()).unwrap();
println!("{}", locale);
```

## History

* **0.4.0** - updated the interface `LocaleIdentifier` to return LocaleError on constructor errors.
* **0.3.0** - updated module structure.
* **0.1.0** - extracted from [simple-locale](https://github.com/johnstonskj/simple-locale).
