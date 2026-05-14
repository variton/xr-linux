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
