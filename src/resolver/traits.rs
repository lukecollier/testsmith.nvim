use crate::cli::Language;
use crate::error::TestsmithError;
use crate::file_ops::FileSystem;
use std::path::PathBuf;

/// Trait for resolving test file paths based on project structure
pub trait StructureResolver: Send + Sync {
    /// Given a source file path, determine the corresponding test file path
    fn resolve_test_path(
        &self,
        fs: &FileSystem,
        source_path: &std::path::Path,
        language: Language,
    ) -> Result<PathBuf, TestsmithError>;

    /// Check if a path is a valid source file for this structure
    fn is_source_path(&self, path: &std::path::Path) -> bool;

    /// Check if a path is a valid test file for this structure
    fn is_test_path(&self, path: &std::path::Path) -> bool;

    /// Get the name of this resolver (for debug/display purposes)
    fn name(&self) -> &'static str;
}
