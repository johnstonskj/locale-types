/*!
The `StrictLocaleString` type provides a `LocaleIdentifier` that validates
that language, territory, and code set identifiers are present in the
corresponding standards.

*/
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use locale_codes::{codeset, country, language};
use locale_types::string::ParseError;
use locale_types::{LocaleError, LocaleIdentifier, LocaleResult, LocaleString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A `StringLocaleString` is a wrapper around `LocaleString`.
#[derive(Debug, PartialEq)]
pub struct StrictLocaleString(LocaleString);

// ------------------------------------------------------------------------------------------------
// Implementations - LocaleString
// ------------------------------------------------------------------------------------------------

impl LocaleIdentifier for StrictLocaleString {
    fn new(language_code: String) -> LocaleResult<Self> {
        match language::lookup(&language_code) {
            None => Err(LocaleError::InvalidLanguageCode),
            Some(_) => Ok(StrictLocaleString(LocaleString::new(language_code)?)),
        }
    }

    fn with_language(&self, language_code: String) -> LocaleResult<Self> {
        match language::lookup(&language_code) {
            None => Err(LocaleError::InvalidLanguageCode),
            Some(_) => Ok(StrictLocaleString(self.0.with_language(language_code)?)),
        }
    }

    fn with_territory(&self, territory: String) -> LocaleResult<Self> {
        match country::lookup(&territory) {
            None => Err(LocaleError::InvalidTerritoryCode),
            Some(_) => Ok(StrictLocaleString(self.0.with_territory(territory)?)),
        }
    }

    fn with_code_set(&self, code_set: String) -> LocaleResult<Self> {
        match codeset::lookup(&code_set) {
            None => Err(LocaleError::InvalidCodeSet),
            Some(_) => Ok(StrictLocaleString(self.0.with_code_set(code_set)?)),
        }
    }

    fn with_modifier(&self, modifier: String) -> LocaleResult<Self> {
        Ok(StrictLocaleString(self.0.with_modifier(modifier)?))
    }

    fn with_modifiers<K, V>(&self, modifiers: HashMap<K, V>) -> LocaleResult<Self>
    where
        K: Display,
        V: Display,
    {
        Ok(StrictLocaleString(self.0.with_modifiers(modifiers)?))
    }

    fn language_code(&self) -> String {
        self.0.language_code()
    }

    fn territory(&self) -> Option<String> {
        self.0.territory()
    }

    fn code_set(&self) -> Option<String> {
        self.0.code_set()
    }

    fn modifier(&self) -> Option<String> {
        self.0.modifier()
    }
}

impl Display for StrictLocaleString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StrictLocaleString {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match LocaleString::from_str(s) {
            Err(e) => Err(e),
            Ok(locale) => {
                let mut strict = StrictLocaleString::new(locale.language_code()).unwrap();
                if let Some(territory) = locale.territory() {
                    strict = strict.with_territory(territory).unwrap();
                }
                if let Some(code_set) = locale.code_set() {
                    strict = strict.with_code_set(code_set).unwrap();
                }
                if let Some(modifier) = locale.modifier() {
                    strict = strict.with_modifier(modifier).unwrap();
                }
                Ok(strict)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::StrictLocaleString;
    use locale_types::{LocaleError, LocaleIdentifier};

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_unknown_language() {
        assert_eq!(
            StrictLocaleString::new("xx".to_string()),
            Err(LocaleError::InvalidLanguageCode)
        );
    }

    #[test]
    fn test_unknown_territory() {
        assert_eq!(
            StrictLocaleString::new("en".to_string())
                .unwrap()
                .with_territory("XX".to_string()),
            Err(LocaleError::InvalidTerritoryCode)
        );
    }

    #[test]
    fn test_unknown_code_set() {
        assert_eq!(
            StrictLocaleString::new("en".to_string())
                .unwrap()
                .with_code_set("UNKNOWN".to_string()),
            Err(LocaleError::InvalidCodeSet)
        );
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_constructor() {
        let locale = StrictLocaleString::new("en".to_string()).unwrap();
        assert_eq!(locale.language_code(), "en".to_string());
        assert_eq!(locale.territory(), None);
        assert_eq!(locale.modifier(), None);
    }

    #[test]
    fn test_with_language() {
        let locale = StrictLocaleString::new("en".to_string()).unwrap();
        assert_eq!(
            locale
                .with_language("fr".to_string())
                .unwrap()
                .language_code(),
            "fr".to_string()
        );
    }

    #[test]
    fn test_with_territory() {
        let locale = StrictLocaleString::new("en".to_string()).unwrap();
        assert_eq!(
            locale.with_territory("GB".to_string()).unwrap().territory(),
            Some("GB".to_string())
        );
    }

    #[test]
    fn test_with_code_set() {
        let locale = StrictLocaleString::new("en".to_string()).unwrap();
        assert_eq!(
            locale
                .with_code_set("UTF-8".to_string())
                .unwrap()
                .code_set(),
            Some("UTF-8".to_string())
        );
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_to_string() {
        let locale = StrictLocaleString::new("en".to_string())
            .unwrap()
            .with_territory("US".to_string())
            .unwrap()
            .with_code_set("UTF-8".to_string())
            .unwrap()
            .with_modifier("collation=pinyin;currency=CNY".to_string())
            .unwrap();
        assert_eq!(
            locale.to_string(),
            "en_US.UTF-8@collation=pinyin;currency=CNY".to_string()
        );
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_from_str_1() {
        match StrictLocaleString::from_str("en") {
            Ok(locale) => assert_eq!(locale.language_code(), "en"),
            _ => panic!("LocaleString::from_str failure"),
        }
    }

    #[test]
    fn test_from_str_2() {
        match StrictLocaleString::from_str("en_US") {
            Ok(locale) => {
                assert_eq!(locale.language_code(), "en");
                assert_eq!(locale.territory(), Some("US".to_string()));
            }
            _ => panic!("LocaleString::from_str failure"),
        }
    }

    #[test]
    fn test_from_str_3() {
        match StrictLocaleString::from_str("en_US.UTF-8") {
            Ok(locale) => {
                assert_eq!(locale.language_code(), "en");
                assert_eq!(locale.territory(), Some("US".to_string()));
                assert_eq!(locale.code_set(), Some("UTF-8".to_string()));
            }
            _ => panic!("LocaleString::from_str failure"),
        }
    }

    #[test]
    fn test_from_str_4() {
        match StrictLocaleString::from_str("en_US.UTF-8@Latn") {
            Ok(locale) => {
                assert_eq!(locale.language_code(), "en");
                assert_eq!(locale.territory(), Some("US".to_string()));
                assert_eq!(locale.code_set(), Some("UTF-8".to_string()));
                assert_eq!(locale.modifier(), Some("Latn".to_string()));
            }
            _ => panic!("LocaleString::from_str failure"),
        }
    }
}
