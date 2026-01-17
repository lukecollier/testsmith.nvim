use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "Testsmith Contributors",
    version,
    about = "Find or create test files for source code",
    long_about = "Testsmith finds or creates test files for Java and other languages. \
                  It supports multiple project structures (Maven, flat, etc.) and \
                  test frameworks (JUnit, native Rust tests, etc.)"
)]
pub struct Cli {
    /// Source file path to find/create test for
    #[arg(value_name = "FILE")]
    pub source_file: PathBuf,

    /// Project structure type (auto-detected if not provided)
    #[arg(short, long, value_enum)]
    pub structure: Option<StructureType>,

    /// Programming language (auto-detected from file extension if not provided)
    #[arg(short, long, value_enum)]
    pub language: Option<Language>,

    /// Test framework (defaults based on language if not provided)
    #[arg(short, long, value_enum)]
    pub framework: Option<Framework>,

    /// Create test file if it doesn't exist
    #[arg(short, long, default_value = "true")]
    pub create: bool,

    /// Show what would be done without creating files
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum StructureType {
    /// Maven structure (src/main/java <-> src/test/java)
    #[value(name = "maven")]
    Maven,

    /// Same file structure (#[cfg(test)] mod tests for Rust)
    #[value(name = "same-file")]
    SameFile,

    /// Gradle structure (similar to Maven)
    #[value(name = "gradle")]
    Gradle,

    /// Flat structure (src/ and tests/ at root)
    #[value(name = "flat")]
    Flat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum Language {
    #[value(name = "java")]
    Java,

    #[value(name = "rust")]
    Rust,

    #[value(name = "python")]
    Python,

    #[value(name = "javascript")]
    JavaScript,

    #[value(name = "typescript")]
    TypeScript,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum Framework {
    #[value(name = "junit")]
    JUnit,

    #[value(name = "junit4")]
    JUnit4,

    #[value(name = "testng")]
    TestNG,

    #[value(name = "native")]
    Native,

    #[value(name = "jest")]
    Jest,

    #[value(name = "pytest")]
    Pytest,
}
