use crate::cli::Language;
use std::path::{Path, PathBuf};

/// Config files that identify a project root for each language
pub fn config_files_for_language(language: Language) -> Vec<&'static str> {
    match language {
        Language::Java => vec!["pom.xml", "build.gradle", "build.gradle.kts", "build.sbt"],
        Language::Rust => vec!["Cargo.toml"],
        Language::JavaScript | Language::TypeScript => vec!["package.json"],
        Language::Python => vec!["pyproject.toml", "setup.py", "requirements.txt"],
    }
}

/// Find the closest project root by walking up from the given path
/// looking for language-specific config files
///
/// For example, for Java starting from `src/main/java/com/example/Foo.java`:
/// - Checks: `src/main/java/` for pom.xml, build.gradle, etc.
/// - Then: `src/main/`
/// - Then: `src/`
/// - Then: `/` (project root)
/// Returns the first match (closest to the source file)
///
/// Handles both absolute and relative paths by canonicalizing them first.
pub fn find_project_root(start_path: &Path, language: Language) -> Option<PathBuf> {
    let config_files = config_files_for_language(language);

    // Canonicalize the path to handle relative paths correctly
    // If canonicalize fails (e.g., file doesn't exist), just use the path as-is
    let canonical_path = start_path.canonicalize().unwrap_or_else(|_| {
        // Fallback: manually resolve relative to current directory
        if start_path.is_absolute() {
            start_path.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(start_path)
        }
    });

    let mut current = if canonical_path.is_dir() {
        canonical_path
    } else {
        canonical_path.parent()?.to_path_buf()
    };

    loop {
        // Check if any config file for this language exists in current directory
        for config_file in &config_files {
            if current.join(config_file).exists() {
                return Some(current);
            }
        }

        // Move to parent directory
        match current.parent() {
            Some(parent) => {
                if parent == current {
                    // Reached filesystem root
                    break;
                }
                current = parent.to_path_buf();
            }
            None => break,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_java_project_root_pom_xml() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src/main/java/com/example");
        fs::create_dir_all(&src_dir).unwrap();

        // Create pom.xml in root
        fs::File::create(temp_dir.path().join("pom.xml")).unwrap();

        // Start from deep in the source tree
        let root = find_project_root(&src_dir, Language::Java).unwrap();
        assert_eq!(root.canonicalize().unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_java_project_root_build_gradle() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src/main/java/com/example");
        fs::create_dir_all(&src_dir).unwrap();

        // Create build.gradle in root
        fs::File::create(temp_dir.path().join("build.gradle")).unwrap();

        let root = find_project_root(&src_dir, Language::Java).unwrap();
        assert_eq!(root.canonicalize().unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_rust_project_root() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();

        fs::File::create(temp_dir.path().join("Cargo.toml")).unwrap();

        let root = find_project_root(&src_dir, Language::Rust).unwrap();
        assert_eq!(root.canonicalize().unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_js_project_root() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();

        fs::File::create(temp_dir.path().join("package.json")).unwrap();

        let root = find_project_root(&src_dir, Language::JavaScript).unwrap();
        assert_eq!(root.canonicalize().unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_no_project_root_found() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src/main/java");
        fs::create_dir_all(&src_dir).unwrap();

        // Don't create any config files
        let root = find_project_root(&src_dir, Language::Java);
        assert!(root.is_none());
    }

    #[test]
    fn test_closest_project_root_wins() {
        let temp_dir = TempDir::new().unwrap();
        let subproject = temp_dir.path().join("subproject/src/main/java");
        fs::create_dir_all(&subproject).unwrap();

        // Create pom.xml in root
        fs::File::create(temp_dir.path().join("pom.xml")).unwrap();

        // Create pom.xml in subproject (closer match)
        fs::File::create(temp_dir.path().join("subproject/pom.xml")).unwrap();

        // Starting from subproject source, should find subproject's pom.xml
        let root = find_project_root(&subproject, Language::Java).unwrap();
        assert_eq!(root.canonicalize().unwrap(), temp_dir.path().join("subproject").canonicalize().unwrap());
    }

    #[test]
    fn test_config_files_for_java() {
        let files = config_files_for_language(Language::Java);
        assert!(files.contains(&"pom.xml"));
        assert!(files.contains(&"build.gradle"));
        assert!(files.contains(&"build.gradle.kts"));
        assert!(files.contains(&"build.sbt"));
    }

    #[test]
    fn test_config_files_for_rust() {
        let files = config_files_for_language(Language::Rust);
        assert_eq!(files.len(), 1);
        assert!(files.contains(&"Cargo.toml"));
    }
}
