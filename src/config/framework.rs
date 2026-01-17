use crate::cli::{Framework, Language};
use crate::error::TestsmithError;

/// Validate that a language/framework combination is valid
pub fn is_valid_combination(language: Language, framework: Framework) -> bool {
    match language {
        Language::Java => matches!(framework, Framework::JUnit | Framework::JUnit4 | Framework::TestNG),
        Language::Rust => matches!(framework, Framework::Native),
        Language::Python => matches!(framework, Framework::Pytest),
        Language::JavaScript => matches!(framework, Framework::Jest),
        Language::TypeScript => matches!(framework, Framework::Jest),
    }
}

/// Validate a language/framework combination and return an error if invalid
pub fn validate_combination(
    language: Language,
    framework: Framework,
) -> Result<(), TestsmithError> {
    if is_valid_combination(language, framework) {
        Ok(())
    } else {
        Err(TestsmithError::InvalidCombination {
            language: format!("{:?}", language),
            framework: format!("{:?}", framework),
        })
    }
}

/// Get the supported frameworks for a given language
pub fn supported_frameworks_for_language(language: Language) -> Vec<Framework> {
    match language {
        Language::Java => vec![Framework::JUnit, Framework::JUnit4, Framework::TestNG],
        Language::Rust => vec![Framework::Native],
        Language::Python => vec![Framework::Pytest],
        Language::JavaScript => vec![Framework::Jest],
        Language::TypeScript => vec![Framework::Jest],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_junit_valid() {
        assert!(is_valid_combination(Language::Java, Framework::JUnit));
    }

    #[test]
    fn test_java_testng_valid() {
        assert!(is_valid_combination(Language::Java, Framework::TestNG));
    }

    #[test]
    fn test_java_pytest_invalid() {
        assert!(!is_valid_combination(Language::Java, Framework::Pytest));
    }

    #[test]
    fn test_rust_native_valid() {
        assert!(is_valid_combination(Language::Rust, Framework::Native));
    }

    #[test]
    fn test_rust_junit_invalid() {
        assert!(!is_valid_combination(Language::Rust, Framework::JUnit));
    }

    #[test]
    fn test_validate_valid_combination() {
        assert!(validate_combination(Language::Java, Framework::JUnit).is_ok());
    }

    #[test]
    fn test_validate_invalid_combination() {
        assert!(validate_combination(Language::Java, Framework::Jest).is_err());
    }

    #[test]
    fn test_java_junit4_valid() {
        assert!(is_valid_combination(Language::Java, Framework::JUnit4));
    }

    #[test]
    fn test_supported_frameworks_java() {
        let frameworks = supported_frameworks_for_language(Language::Java);
        assert_eq!(frameworks.len(), 3);
        assert!(frameworks.contains(&Framework::JUnit));
        assert!(frameworks.contains(&Framework::JUnit4));
        assert!(frameworks.contains(&Framework::TestNG));
    }
}
