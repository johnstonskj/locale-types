/*!
Idiomatic, strictly validated, types for locale identifiers.

This crate provides a _strict_ version of the `LocaleString` structure provided
by the [locale-types](https://github.com/johnstonskj/locale-types) crate. For
each of the fields _language code_, _territory_, and _code set_ the values
passed to the constructors will be validated using the
[locale-codes](https://github.com/johnstonskj/locale-codes) crate to ensure they
are valid identifiers according to the corresponding standards.

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
