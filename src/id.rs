/*!
Provides a single trait that describes common structure for local identifiers.
*/
use std::collections::HashMap;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait LocaleIdentifier
where
    Self: Sized,
{
    fn new(language_code: String) -> Self;

    fn with_language(&self, language_code: String) -> Self;

    fn with_territory(&self, territory: String) -> Self;

    fn with_code_set(&self, code_set: String) -> Self;

    fn with_modifier(&self, modifier: String) -> Self;

    fn with_modifiers<K, V>(&self, modifiers: HashMap<K, V>) -> Self
    where
        K: Display,
        V: Display;

    fn language_code(&self) -> String;

    fn territory(&self) -> Option<String>;

    fn code_set(&self) -> Option<String>;

    fn modifier(&self) -> Option<String>;
}
