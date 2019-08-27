/*!
Provides a single trait that describes common structure for local identifiers.

A locale identifier is comprised of, at least, the following components:

1. a `language` identifier, ...
2. a `territory`, or `country`, identifier, ...
3. a `code_set`, or `charset`, identifier, ...
4. a `territory`, or `country`, identifier, ...
5. a `modifier` string (commonly used to identify the script) ...

*/
use std::collections::HashMap;
use std::fmt::Display;
use crate::LocaleResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Common trait for different kinds of locale identifier.
pub trait LocaleIdentifier
where
    Self: Sized,
{
    /// Construct a new identifier with the given language code only.
    fn new(language_code: String) -> LocaleResult<Self>;

    /// Return a new identifier based on `self` with a new language code.
    fn with_language(&self, language_code: String) -> LocaleResult<Self>;

    /// Return a new identifier based on `self` with a new territory code.
    fn with_territory(&self, territory: String) -> LocaleResult<Self>;

    /// Return a new identifier based on `self` with a new code_set code.
    fn with_code_set(&self, code_set: String) -> LocaleResult<Self>;

    /// Return a new identifier based on `self` with a new modifier string.
    fn with_modifier(&self, modifier: String) -> LocaleResult<Self>;

    /// Return a new identifier based on `self` with a new modifier string.
    fn with_modifiers<K, V>(&self, modifiers: HashMap<K, V>) -> LocaleResult<Self>
    where
        K: Display,
        V: Display;

    /// Return the current language code.
    fn language_code(&self) -> String;

    /// Return the current territory/country code.
    fn territory(&self) -> Option<String>;

    /// Return the current code set/charset code.
    fn code_set(&self) -> Option<String>;

    /// Return the current modifier string.
    fn modifier(&self) -> Option<String>;
}
