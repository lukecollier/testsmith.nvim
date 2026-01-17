use crate::cli::{Language, StructureType};
use crate::error::TestsmithError;
use std::path::Path;

/// Auto-detect the structure type for a given language in a project root
pub fn detect_structure(
    project_root: &Path,
    language: Language,
) -> Result<StructureType, TestsmithError> {
    match language {
        Language::Java => detect_java_structure(project_root),
        Language::Rust => detect_rust_structure(project_root),
        Language::JavaScript | Language::TypeScript => detect_js_structure(project_root),
        Language::Python => detect_python_structure(project_root),
    }
}

/// Detect Java project structure
/// Priority: Maven > Gradle > Flat
fn detect_java_structure(project_root: &Path) -> Result<StructureType, TestsmithError> {
    // Check for Maven structure: src/main/java and src/test/java
    if project_root.join("src/main/java").exists() && project_root.join("src/test/java").exists()
    {
        return Ok(StructureType::Maven);
    }

    // Check for Gradle with build.gradle (which is Maven-like structure)
    if project_root.join("build.gradle").exists() || project_root.join("build.gradle.kts").exists()
    {
        // Gradle can use Maven structure or custom structure
        // For now, treat it as Maven-like since the resolver handles both
        return Ok(StructureType::Gradle);
    }

    // Default to Maven for Java (most common)
    Ok(StructureType::Maven)
}

/// Detect Rust project structure
/// Priority: separate tests/ directory > same-file #[cfg(test)]
fn detect_rust_structure(project_root: &Path) -> Result<StructureType, TestsmithError> {
    // Check for tests/ directory
    if project_root.join("tests").is_dir() {
        return Ok(StructureType::SameFile); // For now, tests/ is treated like same-file
    }

    // Default to same-file for Rust (idiomatic)
    Ok(StructureType::SameFile)
}

/// Detect JavaScript/TypeScript project structure
/// Priority: __tests__/ > tests/ > test/ > same-file (.test.js/.spec.js)
fn detect_js_structure(project_root: &Path) -> Result<StructureType, TestsmithError> {
    // Check for __tests__ directory (Jest default)
    if project_root.join("__tests__").is_dir() {
        return Ok(StructureType::Flat); // Use Flat to indicate subdirectory strategy
    }

    // Check for tests/ directory
    if project_root.join("tests").is_dir() {
        return Ok(StructureType::Flat);
    }

    // Check for test/ directory
    if project_root.join("test").is_dir() {
        return Ok(StructureType::Flat);
    }

    // Default to same-file (tests co-located with source)
    Ok(StructureType::SameFile)
}

/// Detect Python project structure
/// Priority: tests/ > test/ > same-file (test_*.py)
fn detect_python_structure(project_root: &Path) -> Result<StructureType, TestsmithError> {
    // Check for tests/ directory
    if project_root.join("tests").is_dir() {
        return Ok(StructureType::Flat);
    }

    // Check for test/ directory
    if project_root.join("test").is_dir() {
        return Ok(StructureType::Flat);
    }

    // Default to same-file for Python
    Ok(StructureType::SameFile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_java_maven_structure() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir_all(temp_dir.path().join("src/main/java")).unwrap();
        fs::create_dir_all(temp_dir.path().join("src/test/java")).unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Java).unwrap();
        assert_eq!(structure, StructureType::Maven);
    }

    #[test]
    fn test_detect_java_gradle_structure() {
        let temp_dir = TempDir::new().unwrap();
        fs::File::create(temp_dir.path().join("build.gradle")).unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Java).unwrap();
        assert_eq!(structure, StructureType::Gradle);
    }

    #[test]
    fn test_detect_java_default_maven() {
        let temp_dir = TempDir::new().unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Java).unwrap();
        assert_eq!(structure, StructureType::Maven);
    }

    #[test]
    fn test_detect_rust_same_file() {
        let temp_dir = TempDir::new().unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Rust).unwrap();
        assert_eq!(structure, StructureType::SameFile);
    }

    #[test]
    fn test_detect_js_tests_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("__tests__")).unwrap();

        let structure = detect_structure(temp_dir.path(), Language::JavaScript).unwrap();
        assert_eq!(structure, StructureType::Flat);
    }

    #[test]
    fn test_detect_js_test_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("test")).unwrap();

        let structure = detect_structure(temp_dir.path(), Language::JavaScript).unwrap();
        assert_eq!(structure, StructureType::Flat);
    }

    #[test]
    fn test_detect_js_same_file_default() {
        let temp_dir = TempDir::new().unwrap();

        let structure = detect_structure(temp_dir.path(), Language::JavaScript).unwrap();
        assert_eq!(structure, StructureType::SameFile);
    }

    #[test]
    fn test_detect_python_tests_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("tests")).unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Python).unwrap();
        assert_eq!(structure, StructureType::Flat);
    }

    #[test]
    fn test_detect_python_same_file_default() {
        let temp_dir = TempDir::new().unwrap();

        let structure = detect_structure(temp_dir.path(), Language::Python).unwrap();
        assert_eq!(structure, StructureType::SameFile);
    }
}
