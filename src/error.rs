use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestsmithError {
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Invalid file path: {path} - {reason}")]
    InvalidPath { path: PathBuf, reason: String },

    #[error("Unsupported language: {language}")]
    UnsupportedLanguage { language: String },

    #[error("Unsupported framework: {framework}")]
    UnsupportedFramework { framework: String },

    #[error("Invalid combination: language '{language}' does not support framework '{framework}'")]
    InvalidCombination { language: String, framework: String },

    #[error("Unsupported project structure: {structure}")]
    UnsupportedStructure { structure: String },

    #[error("Failed to read file {path}: {source}")]
    FileReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file {path}: {source}")]
    FileWriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreateError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Package name not found in source file {path}")]
    PackageNameNotFound { path: PathBuf },

    #[error("Class name extraction failed for {path}: {reason}")]
    ClassNameExtractionError { path: PathBuf, reason: String },

    #[error("Test file already exists: {path}")]
    TestFileAlreadyExists { path: PathBuf },

    #[error("Invalid source file: {reason}")]
    InvalidSourceFile { reason: String },

    #[error("Configuration error: {reason}")]
    ConfigError { reason: String },

    #[error("Cache error: {reason}")]
    CacheError { reason: String },

    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Unknown error: {reason}")]
    Unknown { reason: String },
}
