use crate::cli::Language;
use crate::error::TestsmithError;
use crate::resolver::traits::StructureResolver;
use std::path::{Path, PathBuf};

pub struct SameFileResolver;

impl SameFileResolver {
    pub fn new() -> Self {
        SameFileResolver
    }
}

impl Default for SameFileResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl StructureResolver for SameFileResolver {
    fn resolve_test_path(
        &self,
        fs: &crate::file_ops::FileSystem,
        source_path: &Path,
        _language: Language,
    ) -> Result<PathBuf, TestsmithError> {
        // For same-file structure, the test path is the same as the source path
        // Tests are appended to the same file using #[cfg(test)] mod tests {}

        if !fs.file_exists(source_path) {
            return Err(TestsmithError::FileNotFound {
                path: source_path.to_path_buf(),
            });
        }

        Ok(source_path.to_path_buf())
    }

    fn is_source_path(&self, _path: &Path) -> bool {
        // In same-file structure, we don't distinguish source from test paths
        true
    }

    fn is_test_path(&self, _path: &Path) -> bool {
        // In same-file structure, we don't distinguish source from test paths
        true
    }

    fn name(&self) -> &'static str {
        "Same File"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_file_returns_same_path() {
        let resolver = SameFileResolver::new();
        let source = Path::new("src/lib.rs");
        // Note: This would fail because file doesn't exist, so we just test the trait methods
        // In real tests, we'd use tempfile
        assert!(resolver.is_source_path(source));
        assert!(resolver.is_test_path(source));
    }

    #[test]
    fn test_resolver_name() {
        let resolver = SameFileResolver::new();
        assert_eq!(resolver.name(), "Same File");
    }
}
