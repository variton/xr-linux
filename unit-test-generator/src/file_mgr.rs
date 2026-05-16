use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum SourceFileType {
    Cpp,
    Python,
    Rust,
}

pub fn detect_source_file<P: AsRef<Path>>(path: P) -> Result<SourceFileType> {
    let path = path.as_ref();

    // -------------------------------------------------
    // First: detect from extension
    // -------------------------------------------------

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        return match ext {
            "cpp" | "cc" | "cxx" | "hpp" | "h" => Ok(SourceFileType::Cpp),

            "py" => Ok(SourceFileType::Python),

            "rs" => Ok(SourceFileType::Rust),

            _ => anyhow::bail!("unsupported file extension: {}", ext),
        };
    }

    // -------------------------------------------------
    // Fallback: inspect file contents
    // -------------------------------------------------

    let content = fs::read_to_string(path)?;

    // Rust heuristics
    if content.contains("fn main")
        || content.contains("impl ")
        || content.contains("use std")
        || content.contains("println!")
    {
        return Ok(SourceFileType::Rust);
    }

    // Python heuristics
    if content.contains("def ")
        || content.contains("import ")
        || content.contains("if __name__ ==")
        || content.contains("print(")
    {
        return Ok(SourceFileType::Python);
    }

    // C++ heuristics
    if content.contains("#include")
        || content.contains("std::")
        || content.contains("int main(")
        || content.contains("cout <<")
    {
        return Ok(SourceFileType::Cpp);
    }

    bail!("unable to determine source file type")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn detect_cpp_from_extension() {
        let result = detect_source_file("main.cpp").unwrap();

        assert!(matches!(result, SourceFileType::Cpp));
    }

    #[test]
    fn detect_python_from_extension() {
        let result = detect_source_file("main.py").unwrap();

        assert!(matches!(result, SourceFileType::Python));
    }

    #[test]
    fn detect_rust_from_extension() {
        let result = detect_source_file("main.rs").unwrap();

        assert!(matches!(result, SourceFileType::Rust));
    }

    #[test]
    fn unsupported_extension_fails() {
        let result = detect_source_file("main.txt");

        assert!(result.is_err());
    }

    #[test]
    fn detect_rust_from_content() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("file");

        std::fs::write(&path, "fn main() { println!(\"hi\"); }").unwrap();

        let result = detect_source_file(&path).unwrap();

        assert!(matches!(result, SourceFileType::Rust));
    }

    #[test]
    fn detect_python_from_content() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("file");

        std::fs::write(&path, "def hello():\n    print('hi')").unwrap();

        let result = detect_source_file(&path).unwrap();

        assert!(matches!(result, SourceFileType::Python));
    }

    #[test]
    fn detect_cpp_from_content() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("file");

        std::fs::write(&path, "#include <iostream>\nint main() {}").unwrap();

        let result = detect_source_file(&path).unwrap();

        assert!(matches!(result, SourceFileType::Cpp));
    }

    #[test]
    fn unknown_content_fails() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("file");

        std::fs::write(&path, "some unknown content").unwrap();

        let result = detect_source_file(&path);

        assert!(result.is_err());
    }

    #[test]
    fn missing_file_without_extension_fails() {
        let result = detect_source_file("does_not_exist");

        assert!(result.is_err());
    }
}
