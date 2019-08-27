# Crate locale-strict

[![travis.ci](https://travis-ci.org/johnstonskj/locale-strict.svg?branch=master)](https://travis-ci.org/johnstonskj/locale-strict)
[![crates.io](https://img.shields.io/crates/v/locale-strict.svg)](https://crates.io/crates/locale-strict)
[![docs.rs](https://docs.rs/locale-strict/badge.svg)](https://docs.rs/locale-strict)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
![mit License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/locale-strict.svg)](https://github.com/johnstonskj/locale-strict/stargazers)

This crate provides a _strict_ version of the `LocaleString` structure provided
by the [locale-types](https://github.com/johnstonskj/locale-types) crate. For 
each of the fields _language code_, _territory_, and _code set_ the values 
passed to the constructors will be validated using the 
[locale-codes](https://github.com/johnstonskj/locale-codes) crate to ensure they
are valid identifiers according to the corresponding standards.

## History

* **0.1.0** - Initial release.
