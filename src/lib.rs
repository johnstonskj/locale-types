/*!
Idiomatic types for locale identifiers.

This crate provides a [`Locale`](locale/enum.Locale.html) enumeration,
[`LocaleIdentifier`](id/trait.LocaleIdentifier.html) trait, and a
[`LocaleString`](string/struct.LocaleString.html) structure are provided that
may be used to parse and construct locale identifiers in a
standards-conformant manner.

## Example

```
use locale_types::{LocaleIdentifier, LocaleString};

let locale = LocaleString::new("en".to_string()).unwrap()
    .with_territory("US".to_string()).unwrap()
    .with_code_set("UTF-8".to_string()).unwrap()
    .with_modifier("collation=pinyin;currency=CNY".to_string()).unwrap();
println!("{}", locale);
```

*/

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Common error type for functions in this crate.
#[derive(Debug)]
pub enum LocaleError {
    /// The provided locale string was badly formatted
    InvalidLocaleString,
    /// The provided language code was not valid, or was unknown.
    InvalidLanguageCode,
    /// The provided territory code was not valid, or was unknown.
    InvalidTerritoryCode,
    /// The provided code set name was not valid, or was unknown.
    InvalidCodeSet,
    /// The provided modifier string was not valid, or was unknown.
    InvalidModifier,
    /// The provided locale was unknown
    UnknownLocale,
    /// Locale category not set/or supported
    UnsetCategory,
    /// Operating system could not set the specified locale
    OSError,
    /// The operation you tried to perform was not supported.
    Unsupported,
}

/// Common result type for functions in this crate.
pub type LocaleResult<T> = Result<T, LocaleError>;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod id;
pub use id::LocaleIdentifier;

pub mod string;
pub use string::LocaleString;

pub mod locale;
pub use locale::Locale;
