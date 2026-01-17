use crate::cli::Language;
use crate::error::TestsmithError;
use std::path::Path;

/// Detect language from file extension
pub fn detect_language(path: &Path) -> Result<Language, TestsmithError> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| TestsmithError::InvalidSourceFile {
            reason: "File has no extension".to_string(),
        })?;

    match extension {
        "java" => Ok(Language::Java),
        "rs" => Ok(Language::Rust),
        "py" => Ok(Language::Python),
        "js" => Ok(Language::JavaScript),
        "ts" => Ok(Language::TypeScript),
        _ => Err(TestsmithError::UnsupportedLanguage {
            language: extension.to_string(),
        }),
    }
}

/// Get the default framework for a given language
pub fn default_framework_for_language(language: Language) -> crate::cli::Framework {
    use crate::cli::Framework;

    match language {
        Language::Java => Framework::JUnit,
        Language::Rust => Framework::Native,
        Language::Python => Framework::Pytest,
        Language::JavaScript => Framework::Jest,
        Language::TypeScript => Framework::Jest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_java_language() {
        let path = PathBuf::from("Foo.java");
        assert_eq!(detect_language(&path).unwrap(), Language::Java);
    }

    #[test]
    fn test_detect_rust_language() {
        let path = PathBuf::from("lib.rs");
        assert_eq!(detect_language(&path).unwrap(), Language::Rust);
    }

    #[test]
    fn test_detect_python_language() {
        let path = PathBuf::from("script.py");
        assert_eq!(detect_language(&path).unwrap(), Language::Python);
    }

    #[test]
    fn test_unsupported_extension() {
        let path = PathBuf::from("file.unknown");
        assert!(detect_language(&path).is_err());
    }

    #[test]
    fn test_no_extension() {
        let path = PathBuf::from("Makefile");
        assert!(detect_language(&path).is_err());
    }

    #[test]
    fn test_default_framework_java() {
        let framework = default_framework_for_language(Language::Java);
        assert_eq!(framework, crate::cli::Framework::JUnit);
    }

    #[test]
    fn test_default_framework_rust() {
        let framework = default_framework_for_language(Language::Rust);
        assert_eq!(framework, crate::cli::Framework::Native);
    }
}
