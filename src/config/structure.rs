use crate::cli::StructureType;

/// Get metadata about a project structure
pub struct StructureInfo {
    pub name: &'static str,
    pub description: &'static str,
}

pub fn get_structure_info(structure: StructureType) -> StructureInfo {
    match structure {
        StructureType::Maven => StructureInfo {
            name: "Maven",
            description: "Standard Maven structure with src/main and src/test directories",
        },
        StructureType::SameFile => StructureInfo {
            name: "Same File",
            description: "Tests in same file as source (e.g., Rust #[cfg(test)])",
        },
        StructureType::Gradle => StructureInfo {
            name: "Gradle",
            description: "Gradle project structure (similar to Maven)",
        },
        StructureType::Flat => StructureInfo {
            name: "Flat",
            description: "Flat structure with src/ and tests/ directories",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maven_structure_info() {
        let info = get_structure_info(StructureType::Maven);
        assert_eq!(info.name, "Maven");
    }

    #[test]
    fn test_same_file_structure_info() {
        let info = get_structure_info(StructureType::SameFile);
        assert_eq!(info.name, "Same File");
    }
}
