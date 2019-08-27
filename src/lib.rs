/*!
Idiomatic types for locale identifiers.

This crate provides a [`Locale`](locale/enum.Locale.html) enumeration,
[`LocaleIdentifier`](id/trait.LocaleIdentifier.html) trait, and a
[`LocaleString`](string/struct.LocaleString.html) structure are provided that
may be used to parse and construct locale identifiers in a
standards-conformant manner.

## Example

```
use locale_types::LocaleIdentifier;
use locale_strict::StrictLocaleString;

let locale = StrictLocaleString::new("en".to_string()).unwrap()
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


// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod string;
pub use string::StrictLocaleString;
