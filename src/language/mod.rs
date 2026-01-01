//! Language Support Module
//! 
//! Provides comprehensive multi-language code search support with 48 languages.

pub mod types;
pub mod definitions;
pub mod utilities;

pub use types::LanguageInfo;
pub use definitions::get_supported_languages;
pub use utilities::{get_language_by_extension, get_all_supported_extensions, get_language_name};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language_by_extension() {
        let lang = get_language_by_extension("rs");
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Rust");
    }

    #[test]
    fn test_get_language_by_extension_case_insensitive() {
        let lang = get_language_by_extension("RS");
        assert!(lang.is_some());
        assert_eq!(lang.unwrap().name, "Rust");
    }

    #[test]
    fn test_get_language_name() {
        let name = get_language_name("test.rs");
        assert_eq!(name, "Rust");
        
        let name = get_language_name("test.py");
        assert_eq!(name, "Python");
        
        let name = get_language_name("test.unknown");
        assert_eq!(name, "Unknown");
    }

    #[test]
    fn test_supported_languages_count() {
        let langs = get_supported_languages();
        assert!(langs.len() >= 48);
    }

    #[test]
    fn test_all_extensions() {
        let exts = get_all_supported_extensions();
        assert!(exts.len() > 100);
        assert!(exts.contains(&"rs"));
        assert!(exts.contains(&"py"));
        assert!(exts.contains(&"js"));
    }
}
