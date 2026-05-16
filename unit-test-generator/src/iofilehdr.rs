use anyhow::{Context, Result};
use std::fs;

/// Reads the contents of a file into a string.
///
/// # Arguments
///
/// * `path` - Path to the file to read.
///
/// # Errors
///
/// Returns an error if:
///
/// - the file does not exist,
/// - the file cannot be opened,
/// - the file contents are not valid UTF-8,
/// - the file cannot be read.
///
/// The returned error includes additional context containing the file path.
///
/// # Examples
///
/// ```no_run
/// let contents = xllm_requester::iofilehdr::read("input.txt")?;
///
/// println!("{contents}");
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn read(path: &str) -> Result<String> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read file `{}`", path))?;

    Ok(contents)
}

/// Writes string contents to a file.
///
/// If the file already exists, its contents are replaced.
/// If the file does not exist, it is created.
///
/// # Arguments
///
/// * `path` - Path to the output file.
/// * `contents` - Text contents to write.
///
/// # Errors
///
/// Returns an error if:
///
/// - the file cannot be created,
/// - the file cannot be written,
/// - the parent directory does not exist,
/// - filesystem permissions prevent writing.
///
/// The returned error includes additional context containing the file path.
///
/// # Examples
///
/// ```no_run
/// xllm_requester::iofilehdr::write(
///     "output.txt",
///     "hello world",
/// )?;
/// # Ok::<(), anyhow::Error>(())
/// ```

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
