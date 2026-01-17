use crate::cli::Language;
use crate::error::TestsmithError;
use crate::resolver::traits::StructureResolver;
use path_clean::PathClean;
use std::path::{Path, PathBuf};

pub struct MavenResolver;

impl MavenResolver {
    pub fn new() -> Self {
        MavenResolver
    }

    /// Transform a source path to test path by replacing src/main with src/test
    /// and adding "Test" suffix to the filename
    fn transform_path(source_path: &Path, _language: Language) -> Result<PathBuf, TestsmithError> {
        let normalized = source_path.clean();
        let path_str = normalized
            .to_str()
            .ok_or_else(|| TestsmithError::InvalidPath {
                path: source_path.to_path_buf(),
                reason: "Path contains invalid UTF-8".to_string(),
            })?;

        // Check if path contains src/main
        if !path_str.contains("src/main") && !path_str.contains("src\\main") {
            return Err(TestsmithError::InvalidPath {
                path: source_path.to_path_buf(),
                reason: "Path does not contain 'src/main' directory".to_string(),
            });
        }

        // Replace src/main with src/test
        let test_path_str = path_str
            .replace("src/main", "src/test")
            .replace("src\\main", "src\\test");

        // Add "Test" suffix before the extension
        let path = Path::new(&test_path_str);
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let file_name = path
            .file_name()
            .ok_or_else(|| TestsmithError::InvalidPath {
                path: source_path.to_path_buf(),
                reason: "File has no name".to_string(),
            })?;

        let file_name_str = file_name.to_str().ok_or_else(|| TestsmithError::InvalidPath {
            path: source_path.to_path_buf(),
            reason: "Filename contains invalid UTF-8".to_string(),
        })?;

        // Extract extension
        let (base_name, extension) = if let Some(dot_idx) = file_name_str.rfind('.') {
            (
                &file_name_str[..dot_idx],
                &file_name_str[dot_idx..],
            )
        } else {
            (file_name_str, "")
        };

        let test_file_name = format!("{}Test{}", base_name, extension);
        let mut result = parent.to_path_buf();
        result.push(test_file_name);

        Ok(result.clean())
    }
}

impl Default for MavenResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl StructureResolver for MavenResolver {
    fn resolve_test_path(
        &self,
        fs: &crate::file_ops::FileSystem,
        source_path: &Path,
        language: Language,
    ) -> Result<PathBuf, TestsmithError> {
        // Validate file exists
        if !fs.file_exists(source_path) {
            return Err(TestsmithError::FileNotFound {
                path: source_path.to_path_buf(),
            });
        }

        Self::transform_path(source_path, language)
    }

    fn is_source_path(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            path_str.contains("src/main") || path_str.contains("src\\main")
        } else {
            false
        }
    }

    fn is_test_path(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            (path_str.contains("src/test") || path_str.contains("src\\test"))
                && path_str.ends_with("Test.java")
        } else {
            false
        }
    }

    fn name(&self) -> &'static str {
        "Maven"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_java_path() {
        let source = Path::new("src/main/java/com/example/Foo.java");
        let result = MavenResolver::transform_path(source, Language::Java);
        assert!(result.is_ok());
        let test_path = result.unwrap();
        assert!(test_path.to_str().unwrap().contains("src/test"));
        assert!(test_path.to_str().unwrap().ends_with("FooTest.java"));
    }

    #[test]
    fn test_transform_path_preserves_package() {
        let source = Path::new("src/main/java/com/example/nested/Foo.java");
        let result = MavenResolver::transform_path(source, Language::Java);
        assert!(result.is_ok());
        let test_path = result.unwrap();
        let path_str = test_path.to_str().unwrap();
        assert!(path_str.contains("com/example/nested"));
        assert!(path_str.ends_with("FooTest.java"));
    }

    #[test]
    fn test_transform_invalid_path_no_src_main() {
        let source = Path::new("src/Foo.java");
        let result = MavenResolver::transform_path(source, Language::Java);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_source_path() {
        let resolver = MavenResolver::new();
        assert!(resolver.is_source_path(Path::new("src/main/java/Foo.java")));
        assert!(!resolver.is_source_path(Path::new("src/test/java/Foo.java")));
    }

    #[test]
    fn test_is_test_path() {
        let resolver = MavenResolver::new();
        assert!(resolver.is_test_path(Path::new("src/test/java/FooTest.java")));
        assert!(!resolver.is_test_path(Path::new("src/main/java/Foo.java")));
        assert!(!resolver.is_test_path(Path::new("src/test/java/Foo.java")));
    }

    #[test]
    fn test_resolver_name() {
        let resolver = MavenResolver::new();
        assert_eq!(resolver.name(), "Maven");
    }
}
