/// C FFI bindings for calling Rust code from Lua
///
/// This module provides C-compatible functions that can be called via Lua FFI
/// All memory is managed by the caller to ensure safety and compatibility

use crate::cli::{Framework, StructureType};
use crate::file_ops::FileSystem;
use crate::generator::{generate, GeneratorOptions};
use crate::config::language as config_language;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

/// Result type for FFI operations
#[repr(C)]
pub struct TestsmithResult {
    /// Success flag (0 = error, 1 = success)
    pub success: i32,
    /// Test file path or error message (caller must free)
    pub message: *mut c_char,
    /// Whether a file was created (0 = no, 1 = yes)
    pub created: i32,
    /// Line number where cursor should be positioned (1-indexed)
    pub line_number: i32,
}

impl TestsmithResult {
    fn success(message: &str, created: bool, line_number: i32) -> Self {
        let c_string = CString::new(message).unwrap_or_else(|_| CString::new("").unwrap());
        TestsmithResult {
            success: 1,
            message: c_string.into_raw(),
            created: if created { 1 } else { 0 },
            line_number,
        }
    }

    fn error(message: &str) -> Self {
        let c_string = CString::new(message).unwrap_or_else(|_| CString::new("Unknown error").unwrap());
        TestsmithResult {
            success: 0,
            message: c_string.into_raw(),
            created: 0,
            line_number: 0,
        }
    }
}

/// Free a TestsmithResult's allocated memory
/// IMPORTANT: This must be called after reading the result to avoid memory leaks
#[unsafe(no_mangle)]
pub extern "C" fn testsmith_result_free(result: *mut TestsmithResult) {
    if !result.is_null() {
        unsafe {
            if !(*result).message.is_null() {
                let _ = CString::from_raw((*result).message);
            }
            let _ = Box::from_raw(result);
        }
    }
}

/// Find or create test file
///
/// # Arguments
/// * `source_path` - Null-terminated C string path to source file (used to auto-detect language)
/// * `structure` - Project structure type: "maven", "gradle", "flat", "same-file"
/// * `framework` - Test framework: "auto" (auto-detect), "junit", "junit4", "testng", "native", "jest", "pytest"
/// * `create` - Whether to create the test file (1 = yes, 0 = no)
/// * `dry_run` - Dry run mode (1 = yes, 0 = no)
///
/// # Returns
/// TestsmithResult containing status and message
///
/// # Safety
/// The caller is responsible for:
/// 1. Ensuring source_path is a valid null-terminated C string
/// 2. Freeing the returned TestsmithResult using testsmith_result_free
#[unsafe(no_mangle)]
pub extern "C" fn testsmith_find_or_create(
    source_path: *const c_char,
    structure: *const c_char,
    framework: *const c_char,
    create: i32,
    dry_run: i32,
) -> *mut TestsmithResult {
    // Convert C strings to Rust strings
    let source_path_str = match unsafe { CStr::from_ptr(source_path).to_str() } {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(TestsmithResult::error("Invalid source path encoding"))),
    };

    let structure_str = match unsafe { CStr::from_ptr(structure).to_str() } {
        Ok(s) => s,
        Err(_) => return Box::into_raw(Box::new(TestsmithResult::error("Invalid structure encoding"))),
    };

    // Parse structure type
    let structure_type = match structure_str {
        "maven" => StructureType::Maven,
        "gradle" => StructureType::Gradle,
        "flat" => StructureType::Flat,
        "same-file" => StructureType::SameFile,
        _ => return Box::into_raw(Box::new(TestsmithResult::error("Invalid structure type"))),
    };

    // Auto-detect language from source path
    let source_path_obj = Path::new(source_path_str);
    let parsed_language = match config_language::detect_language(source_path_obj) {
        Ok(lang) => Some(lang),
        Err(_) => None,
    };

    // Parse optional framework ("auto" or explicit framework name)
    let parsed_framework = if !framework.is_null() {
        match unsafe { CStr::from_ptr(framework).to_str() } {
            Ok(s) => match s {
                "auto" => None,  // Auto-detect in generator
                "junit" => Some(Framework::JUnit),
                "junit4" => Some(Framework::JUnit4),
                "testng" => Some(Framework::TestNG),
                "native" => Some(Framework::Native),
                "jest" => Some(Framework::Jest),
                "pytest" => Some(Framework::Pytest),
                _ => return Box::into_raw(Box::new(TestsmithResult::error("Invalid framework type"))),
            },
            Err(_) => return Box::into_raw(Box::new(TestsmithResult::error("Invalid framework encoding"))),
        }
    } else {
        None
    };

    let fs = FileSystem::new_os();

    let options = GeneratorOptions {
        structure: structure_type,
        language: parsed_language,
        framework: parsed_framework,
        create: create != 0,
        dry_run: dry_run != 0,
    };

    match generate(&fs, source_path_obj, options) {
        Ok(result) => {
            let message = format!("{}", result.test_file_path);
            Box::into_raw(Box::new(TestsmithResult::success(
                &message,
                result.created,
                result.line_number,
            )))
        }
        Err(e) => {
            let error_msg = format!("Error: {}", e);
            Box::into_raw(Box::new(TestsmithResult::error(&error_msg)))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_success() {
        let result = TestsmithResult::success("test message", true, 9);
        assert_eq!(result.success, 1);
        assert_eq!(result.created, 1);
        assert_eq!(result.line_number, 9);
        assert!(!result.message.is_null());

        unsafe {
            let msg = CStr::from_ptr(result.message).to_str().unwrap();
            assert_eq!(msg, "test message");
            testsmith_result_free(Box::into_raw(Box::new(result)));
        }
    }

    #[test]
    fn test_result_error() {
        let result = TestsmithResult::error("error message");
        assert_eq!(result.success, 0);
        assert_eq!(result.created, 0);
        assert_eq!(result.line_number, 0);
        assert!(!result.message.is_null());

        unsafe {
            let msg = CStr::from_ptr(result.message).to_str().unwrap();
            assert_eq!(msg, "error message");
            testsmith_result_free(Box::into_raw(Box::new(result)));
        }
    }
}
