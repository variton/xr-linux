use anyhow::{Context, Result};
use std::fs;

pub fn read(path: &str) -> Result<String> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read file `{}`", path))?;
    Ok(contents)
}

pub fn write(path: &str, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write to `{}`", path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn read_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        std::fs::write(&file_path, "hello").unwrap();

        let result = read(file_path.to_str().unwrap()).unwrap();

        assert_eq!(result, "hello");
    }

    #[test]
    fn read_failure() {
        let result = read("does_not_exist.txt");

        assert!(result.is_err());

        let error = result.unwrap_err().to_string();
        assert!(error.contains("failed to read file"));
    }

    #[test]
    fn write_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write(file_path.to_str().unwrap(), "hello").unwrap();

        let contents = std::fs::read_to_string(&file_path).unwrap();

        assert_eq!(contents, "hello");
    }

    #[test]
    fn write_failure() {
        let dir = tempdir().unwrap();

        // directory does not exist
        let file_path = dir.path().join("missing").join("test.txt");

        let result = write(file_path.to_str().unwrap(), "hello");

        assert!(result.is_err());

        let error = result.unwrap_err().to_string();
        assert!(error.contains("failed to write to"));
    }
}
