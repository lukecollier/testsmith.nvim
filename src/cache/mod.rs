use crate::cli::{Framework, StructureType};
use crate::error::TestsmithError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Cached data for a specific language in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageCache {
    pub framework: String,
    pub structure: String,
    pub last_used: u64,
}

/// The complete cache structure: project_root -> language -> cache data
pub type ProjectCache = HashMap<String, HashMap<String, LanguageCache>>;

/// Get the cache file path: ~/.local/share/nvim/testsmith/testsmith.projects.json
fn get_cache_file_path() -> Result<PathBuf, TestsmithError> {
    let data_dir = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        // Use XDG_DATA_HOME or default to ~/.local/share
        if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg_data)
        } else {
            let home = std::env::var("HOME").map_err(|_| TestsmithError::CacheError {
                reason: "Could not determine home directory".to_string(),
            })?;
            PathBuf::from(home).join(".local/share")
        }
    };

    let cache_dir = data_dir.join("nvim/testsmith");

    // Create directory if it doesn't exist
    fs::create_dir_all(&cache_dir).map_err(|e| TestsmithError::CacheError {
        reason: format!("Failed to create cache directory: {}", e),
    })?;

    Ok(cache_dir.join("testsmith.projects.json"))
}

/// Load the cache from disk
pub fn load_cache() -> Result<ProjectCache, TestsmithError> {
    let cache_file = get_cache_file_path()?;

    if !cache_file.exists() {
        return Ok(HashMap::new());
    }

    let content = fs::read_to_string(&cache_file).map_err(|e| TestsmithError::CacheError {
        reason: format!("Failed to read cache file: {}", e),
    })?;

    serde_json::from_str(&content).map_err(|e| TestsmithError::CacheError {
        reason: format!("Failed to parse cache JSON: {}", e),
    })
}

/// Save the cache to disk
pub fn save_cache(cache: &ProjectCache) -> Result<(), TestsmithError> {
    let cache_file = get_cache_file_path()?;

    let json = serde_json::to_string_pretty(cache).map_err(|e| TestsmithError::CacheError {
        reason: format!("Failed to serialize cache: {}", e),
    })?;

    fs::write(&cache_file, json).map_err(|e| TestsmithError::CacheError {
        reason: format!("Failed to write cache file: {}", e),
    })?;

    Ok(())
}

/// Get current Unix timestamp
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Update or insert a cache entry for a project
pub fn update_cache_entry(
    cache: &mut ProjectCache,
    project_root: &Path,
    language: &str,
    framework: &Framework,
    structure: &StructureType,
) -> Result<(), TestsmithError> {
    let root_str = project_root
        .to_str()
        .ok_or_else(|| TestsmithError::CacheError {
            reason: "Invalid project root path".to_string(),
        })?
        .to_string();

    let lang_cache = LanguageCache {
        framework: format!("{:?}", framework),
        structure: format!("{:?}", structure),
        last_used: current_timestamp(),
    };

    cache
        .entry(root_str)
        .or_insert_with(HashMap::new)
        .insert(language.to_string(), lang_cache);

    Ok(())
}

/// Get a cache entry for a project and language
pub fn get_cache_entry(
    cache: &ProjectCache,
    project_root: &Path,
    language: &str,
) -> Option<LanguageCache> {
    let root_str = project_root.to_str()?;
    cache.get(root_str)?.get(language).cloned()
}

/// Check if a cache entry is stale by comparing modification times of config files
/// Returns true if any config file is newer than the cached `last_used` time
pub fn is_cache_stale(
    project_root: &Path,
    last_used: u64,
    config_files: &[&str],
) -> bool {
    for config_file in config_files {
        let path = project_root.join(config_file);
        if path.exists() {
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                        let mod_time = duration.as_secs();
                        if mod_time > last_used {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_update_and_get_cache_entry() {
        let mut cache = ProjectCache::new();
        let root = Path::new("/project/root");

        update_cache_entry(&mut cache, root, "java", &Framework::JUnit4, &StructureType::Gradle)
            .unwrap();

        let entry = get_cache_entry(&cache, root, "java").unwrap();
        assert_eq!(entry.framework, "JUnit4");
        assert_eq!(entry.structure, "Gradle");
    }

    #[test]
    fn test_multiple_languages_same_project() {
        let mut cache = ProjectCache::new();
        let root = Path::new("/project/root");

        update_cache_entry(&mut cache, root, "java", &Framework::JUnit4, &StructureType::Gradle)
            .unwrap();
        update_cache_entry(
            &mut cache,
            root,
            "rust",
            &Framework::Native,
            &StructureType::SameFile,
        )
        .unwrap();

        assert!(get_cache_entry(&cache, root, "java").is_some());
        assert!(get_cache_entry(&cache, root, "rust").is_some());
    }

    #[test]
    fn test_is_cache_stale() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("build.gradle");

        // Create config file
        let mut file = fs::File::create(&config_file).unwrap();
        file.write_all(b"test").unwrap();

        // Get modification time (current time)
        let metadata = fs::metadata(&config_file).unwrap();
        let mod_time = metadata
            .modified()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Cache with timestamp before file creation should be stale
        assert!(is_cache_stale(
            temp_dir.path(),
            mod_time - 10,
            &["build.gradle"]
        ));

        // Cache with timestamp after file creation should not be stale
        assert!(!is_cache_stale(
            temp_dir.path(),
            mod_time + 10,
            &["build.gradle"]
        ));
    }

    #[test]
    fn test_cache_serialization() {
        let mut cache = ProjectCache::new();
        let root = Path::new("/project/root");

        update_cache_entry(&mut cache, root, "java", &Framework::JUnit4, &StructureType::Gradle)
            .unwrap();

        let json = serde_json::to_string(&cache).unwrap();
        let deserialized: ProjectCache = serde_json::from_str(&json).unwrap();

        assert_eq!(
            get_cache_entry(&deserialized, root, "java").unwrap().framework,
            "JUnit4"
        );
    }
}
