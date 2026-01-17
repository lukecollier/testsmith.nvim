use crate::cli::{Framework, Language};
use crate::error::TestsmithError;
use std::path::PathBuf;

/// Context information needed to generate a test file
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Path to the source file
    pub source_file_path: PathBuf,
    /// Path where the test file should be created
    pub test_file_path: PathBuf,
    /// Programming language
    pub language: Language,
    /// Test framework to use
    pub framework: Framework,
    /// Class/module name (extracted from filename)
    pub class_name: Option<String>,
    /// Package name (for Java)
    pub package_name: Option<String>,
    /// Module path (for Rust)
    pub module_path: Option<String>,
}

impl TemplateContext {
    pub fn new(
        source_file_path: PathBuf,
        test_file_path: PathBuf,
        language: Language,
        framework: Framework,
    ) -> Self {
        TemplateContext {
            source_file_path,
            test_file_path,
            language,
            framework,
            class_name: None,
            package_name: None,
            module_path: None,
        }
    }

    pub fn with_class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn with_package_name(mut self, package_name: String) -> Self {
        self.package_name = Some(package_name);
        self
    }

    pub fn with_module_path(mut self, module_path: String) -> Self {
        self.module_path = Some(module_path);
        self
    }
}

/// Trait for generating test file content
pub trait TemplateGenerator: Send + Sync {
    /// Generate test file content based on the context
    fn generate(&self, context: &TemplateContext) -> Result<String, TestsmithError>;

    /// Get the name of this template generator
    fn name(&self) -> &'static str;

    /// Get the language this generator targets
    fn language(&self) -> Language;

    /// Get the framework this generator targets
    fn framework(&self) -> Framework;
}
