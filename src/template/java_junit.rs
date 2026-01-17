use crate::cli::{Framework, Language};
use crate::error::TestsmithError;
use crate::template::traits::{TemplateContext, TemplateGenerator};
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct JavaJunitTemplate;

impl JavaJunitTemplate {
    pub fn new() -> Self {
        JavaJunitTemplate
    }

    /// Extract package name from Java source file
    pub fn extract_package_name(source_path: &Path) -> Result<Option<String>, TestsmithError> {
        let content = fs::read_to_string(source_path).map_err(|e| {
            TestsmithError::FileReadError {
                path: source_path.to_path_buf(),
                source: e,
            }
        })?;

        // Look for package declaration: package com.example.foo;
        let package_regex = Regex::new(r"^\s*package\s+([\w\.]+)\s*;").unwrap();

        for line in content.lines() {
            if let Some(caps) = package_regex.captures(line) {
                if let Some(package_name) = caps.get(1) {
                    return Ok(Some(package_name.as_str().to_string()));
                }
            }
        }

        Ok(None)
    }

    /// Extract class name from filename (Foo.java -> Foo)
    pub fn extract_class_name(path: &Path) -> Result<String, TestsmithError> {
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

        // Remove .java extension
        let class_name = if file_name.ends_with("Test.java") {
            // Remove both Test and .java
            file_name.trim_end_matches("Test.java").to_string()
        } else if file_name.ends_with(".java") {
            file_name.trim_end_matches(".java").to_string()
        } else {
            file_name.to_string()
        };

        Ok(class_name)
    }
}

impl Default for JavaJunitTemplate {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateGenerator for JavaJunitTemplate {
    fn generate(&self, context: &TemplateContext) -> Result<String, TestsmithError> {
        let package_part = if let Some(ref package_name) = context.package_name {
            format!("package {};\n\n", package_name)
        } else {
            String::new()
        };

        let class_name = context
            .class_name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Example".to_string());

        let test_class_name = format!("{}Test", class_name);

        let template = format!(
            "{}import org.junit.jupiter.api.Test;\nimport static org.junit.jupiter.api.Assertions.*;\n\nclass {} {{\n    @Test\n    void testExample() {{\n        // TODO: Implement test\n    }}\n}}\n",
            package_part, test_class_name
        );

        Ok(template)
    }

    fn name(&self) -> &'static str {
        "Java JUnit 5"
    }

    fn language(&self) -> Language {
        Language::Java
    }

    fn framework(&self) -> Framework {
        Framework::JUnit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_package_name() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "package com.example.foo;\n\npublic class Foo {}";
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let package_name = JavaJunitTemplate::extract_package_name(temp_file.path()).unwrap();
        assert_eq!(package_name, Some("com.example.foo".to_string()));
    }

    #[test]
    fn test_extract_package_name_with_whitespace() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "   package   com.example.bar   ;   \n\npublic class Bar {}";
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let package_name = JavaJunitTemplate::extract_package_name(temp_file.path()).unwrap();
        assert_eq!(package_name, Some("com.example.bar".to_string()));
    }

    #[test]
    fn test_extract_package_name_none() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "public class Foo {}";
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let package_name = JavaJunitTemplate::extract_package_name(temp_file.path()).unwrap();
        assert_eq!(package_name, None);
    }

    #[test]
    fn test_extract_class_name() {
        let path = Path::new("Foo.java");
        let class_name = JavaJunitTemplate::extract_class_name(path).unwrap();
        assert_eq!(class_name, "Foo");
    }

    #[test]
    fn test_extract_class_name_from_test_file() {
        let path = Path::new("FooTest.java");
        let class_name = JavaJunitTemplate::extract_class_name(path).unwrap();
        assert_eq!(class_name, "Foo");
    }

    #[test]
    fn test_generate_template_with_package() {
        let template = JavaJunitTemplate::new();
        let context = TemplateContext::new(
            "Foo.java".into(),
            "FooTest.java".into(),
            Language::Java,
            Framework::JUnit,
        )
        .with_class_name("Foo".to_string())
        .with_package_name("com.example".to_string());

        let result = template.generate(&context).unwrap();
        assert!(result.contains("package com.example;"));
        assert!(result.contains("class FooTest"));
        assert!(result.contains("@Test"));
    }

    #[test]
    fn test_generate_template_without_package() {
        let template = JavaJunitTemplate::new();
        let context = TemplateContext::new(
            "Foo.java".into(),
            "FooTest.java".into(),
            Language::Java,
            Framework::JUnit,
        )
        .with_class_name("Foo".to_string());

        let result = template.generate(&context).unwrap();
        assert!(!result.contains("package"));
        assert!(result.contains("class FooTest"));
    }
}
