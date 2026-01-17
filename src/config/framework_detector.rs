use crate::cli::{Framework, Language};
use crate::error::TestsmithError;
use std::fs;
use std::path::{Path, PathBuf};

/// Find the project root by searching for config files
fn find_project_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        start_path.parent()?.to_path_buf()
    };

    loop {
        // Check for common config files
        if current.join("Cargo.toml").exists()
            || current.join("pom.xml").exists()
            || current.join("build.gradle").exists()
            || current.join("build.gradle.kts").exists()
            || current.join("package.json").exists()
        {
            return Some(current);
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

/// Detect test framework from Cargo.toml for Rust projects
fn detect_rust_framework(cargo_toml: &Path) -> Option<Framework> {
    if fs::read_to_string(cargo_toml).is_ok() {
        // Check if any test framework dependencies are listed
        // Rust's native test framework is built-in, so just return Native
        // unless we find alternative test frameworks (rarely used)
        Some(Framework::Native)
    } else {
        None
    }
}

/// Detect test framework from pom.xml for Java Maven projects
fn detect_java_maven_framework(pom_xml: &Path) -> Option<Framework> {
    if let Ok(content) = fs::read_to_string(pom_xml) {
        // Look for JUnit 5 (jupiter)
        if content.contains("junit-jupiter") || content.contains("org.junit.jupiter") {
            return Some(Framework::JUnit);
        }

        // Look for JUnit 4
        if content.contains("junit:junit") || content.contains("junit</artifactId>") {
            return Some(Framework::JUnit4);
        }

        // Look for TestNG
        if content.contains("testng") || content.contains("org.testng") {
            return Some(Framework::TestNG);
        }
    }

    None
}

/// Detect test framework from build.gradle for Java Gradle projects
fn detect_java_gradle_framework(build_gradle: &Path) -> Option<Framework> {
    if let Ok(content) = fs::read_to_string(build_gradle) {
        // Look for JUnit 5
        if content.contains("org.junit.jupiter") || content.contains("junit-jupiter") {
            return Some(Framework::JUnit);
        }

        // Look for JUnit 4 (new test suite API: useJUnit('4.x'))
        if content.contains("useJUnit('4") || content.contains("useJUnit(\"4") {
            return Some(Framework::JUnit4);
        }

        // Look for JUnit 4 (old dependency style: junit:junit)
        if content.contains("junit:junit") && !content.contains("org.junit.jupiter") {
            return Some(Framework::JUnit4);
        }

        // Look for TestNG
        if content.contains("org.testng") || content.contains("testng") {
            return Some(Framework::TestNG);
        }
    }

    None
}

/// Detect test framework from package.json for JavaScript/TypeScript projects
fn detect_js_framework(package_json: &Path) -> Option<Framework> {
    if let Ok(content) = fs::read_to_string(package_json) {
        // Look for Jest
        if content.contains("jest") {
            return Some(Framework::Jest);
        }

        // Could add Mocha, Vitest, etc. here if needed
    }

    None
}

/// Detect test framework from project configuration files
pub fn detect_framework(
    source_path: &Path,
    language: Language,
) -> Result<Option<Framework>, TestsmithError> {
    // Find project root
    let project_root = match find_project_root(source_path) {
        Some(root) => root,
        None => return Ok(None),
    };

    match language {
        Language::Rust => {
            let cargo_toml = project_root.join("Cargo.toml");
            Ok(detect_rust_framework(&cargo_toml))
        }
        Language::Java => {
            // Try Maven first
            let pom_xml = project_root.join("pom.xml");
            if pom_xml.exists() {
                if let Some(framework) = detect_java_maven_framework(&pom_xml) {
                    return Ok(Some(framework));
                }
            }

            // Try Gradle
            let build_gradle = project_root.join("build.gradle");
            if build_gradle.exists() {
                if let Some(framework) = detect_java_gradle_framework(&build_gradle) {
                    return Ok(Some(framework));
                }
            }

            let build_gradle_kts = project_root.join("build.gradle.kts");
            if build_gradle_kts.exists() {
                if let Some(framework) = detect_java_gradle_framework(&build_gradle_kts) {
                    return Ok(Some(framework));
                }
            }

            Ok(None)
        }
        Language::JavaScript | Language::TypeScript => {
            let package_json = project_root.join("package.json");
            Ok(detect_js_framework(&package_json))
        }
        Language::Python => {
            // Could implement Python framework detection here
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_detect_rust_native() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        let mut file = fs::File::create(&cargo_toml).unwrap();
        writeln!(file, "[package]\nname = \"test\"").unwrap();

        let framework =
            detect_rust_framework(&cargo_toml).expect("Should detect Rust native framework");
        assert_eq!(framework, Framework::Native);
    }

    #[test]
    fn test_detect_java_junit5_maven() {
        let temp_dir = TempDir::new().unwrap();
        let pom_xml = temp_dir.path().join("pom.xml");
        let mut file = fs::File::create(&pom_xml).unwrap();
        writeln!(
            file,
            r#"<project>
            <dependency>
                <groupId>org.junit.jupiter</groupId>
                <artifactId>junit-jupiter</artifactId>
            </dependency>
        </project>"#
        )
        .unwrap();

        let framework =
            detect_java_maven_framework(&pom_xml).expect("Should detect JUnit 5");
        assert_eq!(framework, Framework::JUnit);
    }

    #[test]
    fn test_detect_java_junit4_maven() {
        let temp_dir = TempDir::new().unwrap();
        let pom_xml = temp_dir.path().join("pom.xml");
        let mut file = fs::File::create(&pom_xml).unwrap();
        writeln!(
            file,
            r#"<project>
            <dependency>
                <groupId>junit</groupId>
                <artifactId>junit</artifactId>
            </dependency>
        </project>"#
        )
        .unwrap();

        let framework =
            detect_java_maven_framework(&pom_xml).expect("Should detect JUnit 4");
        assert_eq!(framework, Framework::JUnit4);
    }

    #[test]
    fn test_detect_java_testng_maven() {
        let temp_dir = TempDir::new().unwrap();
        let pom_xml = temp_dir.path().join("pom.xml");
        let mut file = fs::File::create(&pom_xml).unwrap();
        writeln!(
            file,
            r#"<project>
            <dependency>
                <groupId>org.testng</groupId>
                <artifactId>testng</artifactId>
            </dependency>
        </project>"#
        )
        .unwrap();

        let framework =
            detect_java_maven_framework(&pom_xml).expect("Should detect TestNG");
        assert_eq!(framework, Framework::TestNG);
    }

    #[test]
    fn test_detect_java_junit_gradle() {
        let temp_dir = TempDir::new().unwrap();
        let build_gradle = temp_dir.path().join("build.gradle");
        let mut file = fs::File::create(&build_gradle).unwrap();
        writeln!(
            file,
            r#"dependencies {{
            testImplementation 'org.junit.jupiter:junit-jupiter'
        }}"#
        )
        .unwrap();

        let framework = detect_java_gradle_framework(&build_gradle).expect("Should detect JUnit");
        assert_eq!(framework, Framework::JUnit);
    }

    #[test]
    fn test_detect_junit4_gradle_test_suite_api() {
        let temp_dir = TempDir::new().unwrap();
        let build_gradle = temp_dir.path().join("build.gradle");
        let mut file = fs::File::create(&build_gradle).unwrap();
        writeln!(
            file,
            r#"testing {{
            suites {{
                test {{
                    useJUnit('4.13.2')
                }}
            }}
        }}"#
        )
        .unwrap();

        let framework = detect_java_gradle_framework(&build_gradle).expect("Should detect JUnit 4");
        assert_eq!(framework, Framework::JUnit4);
    }

    #[test]
    fn test_detect_jest() {
        let temp_dir = TempDir::new().unwrap();
        let package_json = temp_dir.path().join("package.json");
        let mut file = fs::File::create(&package_json).unwrap();
        writeln!(
            file,
            r#"{{
            "devDependencies": {{
                "jest": "^29.0.0"
            }}
        }}"#
        )
        .unwrap();

        let framework = detect_js_framework(&package_json).expect("Should detect Jest");
        assert_eq!(framework, Framework::Jest);
    }
}
