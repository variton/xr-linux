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
