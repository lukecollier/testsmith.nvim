use crate::cli::{Framework, Language};
use crate::error::TestsmithError;
use crate::template::traits::{TemplateContext, TemplateGenerator};

pub struct RustNativeTemplate;

impl RustNativeTemplate {
    pub fn new() -> Self {
        RustNativeTemplate
    }

    /// Extract module name from filename (lib.rs -> lib, main.rs -> main, Foo.rs -> foo)
    pub fn extract_module_name(path: &std::path::Path) -> Result<String, TestsmithError> {
        let file_name = path
            .file_name()
            .ok_or_else(|| TestsmithError::ClassNameExtractionError {
                path: path.to_path_buf(),
                reason: "No filename found".to_string(),
            })?
            .to_str()
            .ok_or_else(|| TestsmithError::ClassNameExtractionError {
                path: path.to_path_buf(),
                reason: "Filename contains invalid UTF-8".to_string(),
            })?;

        // Remove .rs extension
        let module_name = if file_name.ends_with(".rs") {
            file_name.trim_end_matches(".rs")
        } else {
            file_name
        };

        Ok(module_name.to_lowercase())
    }
}

impl Default for RustNativeTemplate {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateGenerator for RustNativeTemplate {
    fn generate(&self, _context: &TemplateContext) -> Result<String, TestsmithError> {
        // For Rust, we generate a test module to be appended to the source file
        let template = r#"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // TODO: Implement test
    }
}
"#;

        Ok(template.trim_start().to_string())
    }

    fn name(&self) -> &'static str {
        "Rust Native"
    }

    fn language(&self) -> Language {
        Language::Rust
    }

    fn framework(&self) -> Framework {
        Framework::Native
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_extract_module_name_lib() {
        let path = Path::new("lib.rs");
        let module_name = RustNativeTemplate::extract_module_name(path).unwrap();
        assert_eq!(module_name, "lib");
    }

    #[test]
    fn test_extract_module_name_main() {
        let path = Path::new("main.rs");
        let module_name = RustNativeTemplate::extract_module_name(path).unwrap();
        assert_eq!(module_name, "main");
    }

    #[test]
    fn test_extract_module_name_custom() {
        let path = Path::new("Foo.rs");
        let module_name = RustNativeTemplate::extract_module_name(path).unwrap();
        assert_eq!(module_name, "foo");
    }

    #[test]
    fn test_generate_template() {
        let template = RustNativeTemplate::new();
        let context = TemplateContext::new(
            "lib.rs".into(),
            "lib.rs".into(),
            Language::Rust,
            Framework::Native,
        );

        let result = template.generate(&context).unwrap();
        assert!(result.contains("#[cfg(test)]"));
        assert!(result.contains("mod tests"));
        assert!(result.contains("#[test]"));
    }
}
