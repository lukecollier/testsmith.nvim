use crate::cli::{Framework, Language};
use crate::error::TestsmithError;
use crate::template::java_junit::JavaJunitTemplate;
use crate::template::java_junit4::JavaJunit4Template;
use crate::template::rust_native::RustNativeTemplate;
use crate::template::traits::TemplateGenerator;
use std::collections::HashMap;

pub struct TemplateRegistry {
    generators: HashMap<(Language, Framework), Box<dyn TemplateGenerator>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut generators: HashMap<(Language, Framework), Box<dyn TemplateGenerator>> =
            HashMap::new();

        // Register Java/JUnit template
        generators.insert(
            (Language::Java, Framework::JUnit),
            Box::new(JavaJunitTemplate::new()) as Box<dyn TemplateGenerator>,
        );

        // Register Java/JUnit4 template
        generators.insert(
            (Language::Java, Framework::JUnit4),
            Box::new(JavaJunit4Template::new()) as Box<dyn TemplateGenerator>,
        );

        // Register Rust/Native template
        generators.insert(
            (Language::Rust, Framework::Native),
            Box::new(RustNativeTemplate::new()) as Box<dyn TemplateGenerator>,
        );

        TemplateRegistry { generators }
    }

    /// Get a template generator for the given language and framework
    pub fn get_generator(
        &self,
        language: Language,
        framework: Framework,
    ) -> Result<&dyn TemplateGenerator, TestsmithError> {
        self.generators
            .get(&(language, framework))
            .map(|b| b.as_ref())
            .ok_or_else(|| TestsmithError::InvalidCombination {
                language: format!("{:?}", language),
                framework: format!("{:?}", framework),
            })
    }

    /// Check if a language/framework combination is supported
    pub fn is_supported(&self, language: Language, framework: Framework) -> bool {
        self.generators.contains_key(&(language, framework))
    }

    /// Register a new template generator
    pub fn register(
        &mut self,
        language: Language,
        framework: Framework,
        generator: Box<dyn TemplateGenerator>,
    ) {
        self.generators.insert((language, framework), generator);
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_contains_java_junit() {
        let registry = TemplateRegistry::new();
        assert!(registry.is_supported(Language::Java, Framework::JUnit));
    }

    #[test]
    fn test_registry_contains_rust_native() {
        let registry = TemplateRegistry::new();
        assert!(registry.is_supported(Language::Rust, Framework::Native));
    }

    #[test]
    fn test_registry_contains_java_junit4() {
        let registry = TemplateRegistry::new();
        assert!(registry.is_supported(Language::Java, Framework::JUnit4));
    }

    #[test]
    fn test_registry_get_java_junit() {
        let registry = TemplateRegistry::new();
        let generator = registry.get_generator(Language::Java, Framework::JUnit);
        assert!(generator.is_ok());
        assert_eq!(generator.unwrap().name(), "Java JUnit 5");
    }

    #[test]
    fn test_registry_get_java_junit4() {
        let registry = TemplateRegistry::new();
        let generator = registry.get_generator(Language::Java, Framework::JUnit4);
        assert!(generator.is_ok());
        assert_eq!(generator.unwrap().name(), "Java JUnit 4");
    }

    #[test]
    fn test_registry_unsupported_combination() {
        let registry = TemplateRegistry::new();
        let result = registry.get_generator(Language::Java, Framework::Native);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_does_not_contain_unsupported() {
        let registry = TemplateRegistry::new();
        assert!(!registry.is_supported(Language::Python, Framework::Pytest));
    }
}
