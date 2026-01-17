use crate::error::TestsmithError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

/// Abstraction over file system operations
/// Supports both OS filesystem and in-memory filesystem for testing
pub enum FileSystemBackend {
    /// Use the real OS filesystem
    Os,
    /// Use an in-memory filesystem
    Memory(Mutex<MemoryFileSystem>),
}

/// Simple in-memory file system for testing
#[derive(Default)]
pub struct MemoryFileSystem {
    files: HashMap<String, String>,
}

impl MemoryFileSystem {
    fn new() -> Self {
        MemoryFileSystem {
            files: HashMap::new(),
        }
    }

    fn normalize_path(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }

    fn write_file(&mut self, path: &Path, content: &str) -> Result<(), String> {
        let path_str = Self::normalize_path(path);
        self.files.insert(path_str, content.to_string());
        Ok(())
    }

    fn read_file(&self, path: &Path) -> Result<String, String> {
        let path_str = Self::normalize_path(path);
        self.files
            .get(&path_str)
            .cloned()
            .ok_or_else(|| format!("File not found: {}", path_str))
    }

    fn file_exists(&self, path: &Path) -> bool {
        let path_str = Self::normalize_path(path);
        self.files.contains_key(&path_str)
    }

    fn append_to_file(&mut self, path: &Path, content: &str) -> Result<(), String> {
        let path_str = Self::normalize_path(path);
        if let Some(existing) = self.files.get_mut(&path_str) {
            existing.push('\n');
            existing.push_str(content);
        } else {
            return Err(format!("File not found: {}", path_str));
        }
        Ok(())
    }
}

/// Wrapper around file system operations
pub struct FileSystem {
    backend: FileSystemBackend,
}

impl FileSystem {
    /// Create a FileSystem using the OS filesystem
    pub fn new_os() -> Self {
        FileSystem {
            backend: FileSystemBackend::Os,
        }
    }

    /// Create an in-memory FileSystem for testing
    pub fn new_memory() -> Self {
        FileSystem {
            backend: FileSystemBackend::Memory(Mutex::new(MemoryFileSystem::new())),
        }
    }

    /// Create all parent directories for a given path
    pub fn create_parent_directories(&self, path: &Path) -> Result<(), TestsmithError> {
        match &self.backend {
            FileSystemBackend::Os => {
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| {
                            TestsmithError::DirectoryCreateError {
                                path: parent.to_path_buf(),
                                source: e,
                            }
                        })?;
                    }
                }
                Ok(())
            }
            FileSystemBackend::Memory(_) => {
                // In-memory FS doesn't need directory creation
                Ok(())
            }
        }
    }

    /// Check if a file exists
    pub fn file_exists(&self, path: &Path) -> bool {
        match &self.backend {
            FileSystemBackend::Os => path.exists() && path.is_file(),
            FileSystemBackend::Memory(mem_fs) => {
                mem_fs.lock().unwrap().file_exists(path)
            }
        }
    }

    /// Read a file to string
    pub fn read_file(&self, path: &Path) -> Result<String, TestsmithError> {
        match &self.backend {
            FileSystemBackend::Os => {
                fs::read_to_string(path).map_err(|e| TestsmithError::FileReadError {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
            FileSystemBackend::Memory(mem_fs) => {
                mem_fs
                    .lock()
                    .unwrap()
                    .read_file(path)
                    .map_err(|e| TestsmithError::FileReadError {
                        path: path.to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::NotFound, e),
                    })
            }
        }
    }

    /// Write content to a file (creates new or overwrites existing)
    pub fn write_file_new(&self, path: &Path, content: &str) -> Result<(), TestsmithError> {
        // Ensure parent directories exist
        self.create_parent_directories(path)?;

        match &self.backend {
            FileSystemBackend::Os => {
                fs::write(path, content).map_err(|e| TestsmithError::FileWriteError {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
            FileSystemBackend::Memory(mem_fs) => {
                mem_fs
                    .lock()
                    .unwrap()
                    .write_file(path, content)
                    .map_err(|e| TestsmithError::FileWriteError {
                        path: path.to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::Other, e),
                    })
            }
        }
    }

    /// Append content to a file
    pub fn append_to_file(&self, path: &Path, content: &str) -> Result<(), TestsmithError> {
        match &self.backend {
            FileSystemBackend::Os => {
                use std::fs::OpenOptions;
                use std::io::Write;

                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path)
                    .map_err(|e| TestsmithError::FileWriteError {
                        path: path.to_path_buf(),
                        source: e,
                    })?;

                writeln!(file, "{}", content).map_err(|e| TestsmithError::FileWriteError {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
            FileSystemBackend::Memory(mem_fs) => {
                mem_fs
                    .lock()
                    .unwrap()
                    .append_to_file(path, content)
                    .map_err(|e| TestsmithError::FileWriteError {
                        path: path.to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::Other, e),
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_parent_directories() {
        let fs = FileSystem::new_memory();
        let nested_path = PathBuf::from("/a/b/c/test.txt");

        fs.create_parent_directories(&nested_path).unwrap();

        // For in-memory FS, directory creation is a no-op, so we just verify no error
        // The real test is that file creation works
    }

    #[test]
    fn test_file_exists_true() {
        let fs = FileSystem::new_memory();
        let file_path = PathBuf::from("/test.txt");

        fs.write_file_new(&file_path, "content").unwrap();

        assert!(fs.file_exists(&file_path));
    }

    #[test]
    fn test_file_exists_false() {
        let fs = FileSystem::new_memory();
        let file_path = PathBuf::from("/nonexistent.txt");

        assert!(!fs.file_exists(&file_path));
    }

    #[test]
    fn test_read_file() {
        let fs = FileSystem::new_memory();
        let file_path = PathBuf::from("/test.txt");

        fs.write_file_new(&file_path, "hello world").unwrap();

        let content = fs.read_file(&file_path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_write_file_new() {
        let fs = FileSystem::new_memory();
        let file_path = PathBuf::from("/subdir/test.txt");

        fs.write_file_new(&file_path, "test content").unwrap();

        assert!(fs.file_exists(&file_path));
        let content = fs.read_file(&file_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_append_to_file() {
        let fs = FileSystem::new_memory();
        let file_path = PathBuf::from("/test.txt");

        fs.write_file_new(&file_path, "line 1\n").unwrap();
        fs.append_to_file(&file_path, "line 2").unwrap();

        let content = fs.read_file(&file_path).unwrap();
        assert!(content.contains("line 1"));
        assert!(content.contains("line 2"));
    }
}
