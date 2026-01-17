use crate::cache;
use crate::cli::{Framework, Language, StructureType};
use crate::config::{framework as config_framework, language as config_language, framework_detector, project_root as config_project_root, structure_detector};
use crate::error::TestsmithError;
use crate::file_ops::FileSystem;
use crate::resolver::maven::MavenResolver;
use crate::resolver::same_file::SameFileResolver;
use crate::resolver::traits::StructureResolver;
use crate::template::java_junit::JavaJunitTemplate;
use crate::template::registry::TemplateRegistry;
use crate::template::traits::TemplateContext;
use std::path::Path;

pub struct GeneratorOptions {
    pub structure: StructureType,
    pub language: Option<Language>,
    pub framework: Option<Framework>,
    pub create: bool,
    pub dry_run: bool,
}

pub struct GeneratorResult {
    pub test_file_path: String,
    pub created: bool,
    pub dry_run: bool,
    pub line_number: i32,
}

/// Generate or find test files based on source files
pub fn generate(
    fs: &FileSystem,
    source_path: &Path,
    options: GeneratorOptions,
) -> Result<GeneratorResult, TestsmithError> {
    // Note: We don't validate source file existence here because:
    // 1. For OS filesystem, the resolver will handle path validation
    // 2. For memory filesystem, the file must be created by the test
    // The actual validation happens during resolver.resolve_test_path()

    // Detect language if not provided
    let language = if let Some(lang) = options.language {
        lang
    } else {
        config_language::detect_language(source_path)?
    };

    // Load cache (don't fail if unavailable - it's optional)
    let mut cache = cache::load_cache().unwrap_or_default();

    // Find project root (language-specific)
    let project_root = config_project_root::find_project_root(source_path, language);
    let language_str = format!("{:?}", language);

    // Determine framework
    let framework = if let Some(fw) = options.framework {
        // Explicit framework provided - use it
        config_framework::validate_combination(language, fw)?;
        fw
    } else {
        // Try to use cache if we have a project root
        let mut cached_framework = None;

        if let Some(ref root) = project_root {
            if let Some(cached_entry) = cache::get_cache_entry(&cache, root, &language_str) {
                let config_files = config_project_root::config_files_for_language(language);

                // Check if cache is stale
                if !cache::is_cache_stale(root, cached_entry.last_used, &config_files) {
                    // Cache is valid, parse the framework string
                    cached_framework = match cached_entry.framework.as_str() {
                        "JUnit" => Some(Framework::JUnit),
                        "JUnit4" => Some(Framework::JUnit4),
                        "TestNG" => Some(Framework::TestNG),
                        "Native" => Some(Framework::Native),
                        "Jest" => Some(Framework::Jest),
                        "Pytest" => Some(Framework::Pytest),
                        _ => None,
                    };
                }
            }
        }

        // If we have valid cached framework, use it
        if let Some(fw) = cached_framework {
            config_framework::validate_combination(language, fw)?;
            fw
        } else {
            // Try to auto-detect framework from project config files
            let detected = framework_detector::detect_framework(source_path, language)?;

            if let Some(fw) = detected {
                // Validate the detected combination
                config_framework::validate_combination(language, fw)?;
                fw
            } else {
                // Fall back to default framework for language
                config_language::default_framework_for_language(language)
            }
        }
    };

    // Determine structure
    let structure = if options.structure == StructureType::Maven {
        // If explicitly provided (Maven is default), check if we should auto-detect instead
        if let Some(ref root) = project_root {
            if let Some(cached_entry) = cache::get_cache_entry(&cache, root, &language_str) {
                // Parse the cached structure
                match cached_entry.structure.as_str() {
                    "Maven" => StructureType::Maven,
                    "Gradle" => StructureType::Gradle,
                    "SameFile" => StructureType::SameFile,
                    "Flat" => StructureType::Flat,
                    _ => options.structure,
                }
            } else {
                // Not in cache, try to auto-detect
                structure_detector::detect_structure(root, language).unwrap_or(options.structure)
            }
        } else {
            options.structure
        }
    } else {
        // Non-Maven structure explicitly specified
        options.structure
    };

    // Update cache with current values
    if let Some(ref root) = project_root {
        let _ = cache::update_cache_entry(&mut cache, root, &language_str, &framework, &structure);
        let _ = cache::save_cache(&cache);
    }

    // Get the appropriate resolver
    let resolver: Box<dyn StructureResolver> = match structure {
        StructureType::Maven | StructureType::Gradle => Box::new(MavenResolver::new()),
        StructureType::SameFile => Box::new(SameFileResolver::new()),
        StructureType::Flat => Box::new(MavenResolver::new()), // Use Maven as placeholder for flat
    };

    // Resolve test file path
    let test_file_path = resolver.resolve_test_path(fs, source_path, language)?;

    // Check if test file exists (different logic for same-file vs separate files)
    let mut test_exists = false;
    let mut has_test_module = false;

    if structure == StructureType::SameFile {
        // For same-file: check if a test module already exists within the file
        if let Ok(content) = fs.read_file(&test_file_path) {
            test_exists = true;
            has_test_module = content.contains("#[cfg(test)]");
        }
    } else {
        // For separate files: just check if file exists
        test_exists = fs.file_exists(&test_file_path);
    }

    // If tests already exist, just position cursor and return
    if test_exists && has_test_module {
        let line_number = if let Ok(content) = fs.read_file(&test_file_path) {
            // Look for first #[test] function
            content
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains("#[test]"))
                .map(|(idx, _)| (idx + 1) as i32)
                .unwrap_or_else(|| {
                    // Fall back to TODO comment
                    content
                        .lines()
                        .enumerate()
                        .find(|(_, line)| line.contains("// TODO"))
                        .map(|(idx, _)| (idx + 1) as i32)
                        .unwrap_or(1)
                })
        } else {
            1
        };

        return Ok(GeneratorResult {
            test_file_path: test_file_path.to_string_lossy().to_string(),
            created: false,
            dry_run: false,
            line_number,
        });
    } else if test_exists && !has_test_module && structure != StructureType::SameFile {
        // For non-same-file structures, if file exists but has no tests, return error
        let line_number = if let Ok(content) = fs.read_file(&test_file_path) {
            content
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains("// TODO") || line.contains("TODO:"))
                .map(|(idx, _)| (idx + 1) as i32)
                .unwrap_or(1)
        } else {
            1
        };

        return Ok(GeneratorResult {
            test_file_path: test_file_path.to_string_lossy().to_string(),
            created: false,
            dry_run: false,
            line_number,
        });
    }

    // Test file doesn't exist
    if !options.create {
        return Err(TestsmithError::FileNotFound {
            path: test_file_path,
        });
    }

    // Generate test file
    let registry = TemplateRegistry::new();
    let generator = registry.get_generator(language, framework)?;

    // Extract metadata from source file
    let mut context = TemplateContext::new(
        source_path.to_path_buf(),
        test_file_path.clone(),
        language,
        framework,
    );

    // For Java, extract package and class names
    if language == Language::Java {
        if let Ok(package_name) = JavaJunitTemplate::extract_package_name(source_path) {
            if let Some(pkg) = package_name {
                context = context.with_package_name(pkg);
            }
        }

        if let Ok(class_name) = JavaJunitTemplate::extract_class_name(source_path) {
            context = context.with_class_name(class_name);
        }
    }

    // Generate content
    let content = generator.generate(&context)?;

    // Calculate line number of TODO comment for cursor positioning
    let line_number = if structure == StructureType::SameFile {
        // For same-file: calculate where the test module will be in the existing file
        if let Ok(existing_content) = fs.read_file(&test_file_path) {
            let existing_lines = existing_content.lines().count() as i32;
            // Find the TODO line in the new content to add to existing line count
            let todo_offset = content
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains("// TODO"))
                .map(|(idx, _)| (idx + 1) as i32)
                .unwrap_or(1);
            existing_lines + todo_offset
        } else {
            // If can't read existing file, default to 1
            1
        }
    } else {
        // For separate files: TODO is relative to start of new file
        content
            .lines()
            .enumerate()
            .find(|(_, line)| line.contains("// TODO"))
            .map(|(idx, _)| (idx + 1) as i32)
            .unwrap_or(1)
    };

    // Write file (unless dry run)
    if !options.dry_run {
        if structure == StructureType::SameFile {
            // For same-file structure, append to existing file
            fs.append_to_file(&test_file_path, &content)?;
        } else {
            // For other structures, create new test file
            fs.write_file_new(&test_file_path, &content)?;
        }
    }

    Ok(GeneratorResult {
        test_file_path: test_file_path.to_string_lossy().to_string(),
        created: true,
        dry_run: options.dry_run,
        line_number,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_nonexistent_source_file() {
        let fs = FileSystem::new_memory();
        let options = GeneratorOptions {
            structure: StructureType::Maven,
            language: Some(Language::Java),
            framework: Some(Framework::JUnit),
            create: true,
            dry_run: false,
        };

        let result = generate(&fs, Path::new("nonexistent.java"), options);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_language_from_java_file() {
        let fs = FileSystem::new_memory();
        let java_file = PathBuf::from("/src/main/java/Foo.java");

        // Create the source file
        fs.write_file_new(&java_file, "public class Foo {}").unwrap();

        let options = GeneratorOptions {
            structure: StructureType::Maven,
            language: None, // Auto-detect
            framework: None,
            create: false, // Don't create yet
            dry_run: false,
        };

        // Should fail because test file doesn't exist and create=false
        let result = generate(&fs, &java_file, options);
        assert!(result.is_err());
    }

    #[test]
    fn test_dry_run_does_not_create_file() {
        let fs = FileSystem::new_memory();
        let java_file = PathBuf::from("/src/main/java/Foo.java");

        fs.write_file_new(&java_file, "package com.example;\n\npublic class Foo {}").unwrap();

        let options = GeneratorOptions {
            structure: StructureType::Maven,
            language: Some(Language::Java),
            framework: Some(Framework::JUnit),
            create: true,
            dry_run: true, // Dry run
        };

        let result = generate(&fs, &java_file, options);
        assert!(result.is_ok());

        let test_file_path_str = result.unwrap().test_file_path;
        let test_file_path = PathBuf::from(&test_file_path_str);
        assert!(!fs.file_exists(&test_file_path));
    }
}
