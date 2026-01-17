use clap::Parser;
use testsmith_nvim::cli::{Cli, StructureType};
use testsmith_nvim::file_ops::FileSystem;
use testsmith_nvim::generator::{generate, GeneratorOptions};
use std::path::Path;
use std::process;

/// Auto-detect the appropriate structure based on file extension
fn auto_detect_structure(source_file: &Path) -> StructureType {
    match source_file.extension().and_then(|e| e.to_str()) {
        Some("rs") => StructureType::SameFile, // Rust files use same-file structure
        _ => StructureType::Maven, // Default to Maven for Java and others
    }
}

fn main() {
    let cli = Cli::parse();

    let fs = FileSystem::new_os();

    // Auto-detect structure if not explicitly provided
    let structure = cli.structure.unwrap_or_else(|| auto_detect_structure(&cli.source_file));

    let options = GeneratorOptions {
        structure,
        language: cli.language,
        framework: cli.framework,
        create: cli.create,
        dry_run: cli.dry_run,
    };

    match generate(&fs, &cli.source_file, options) {
        Ok(result) => {
            if result.dry_run {
                println!("Would create test file: {}", result.test_file_path);
            } else if result.created {
                println!("Created test file: {}", result.test_file_path);
            } else {
                println!("Found test file: {}", result.test_file_path);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
