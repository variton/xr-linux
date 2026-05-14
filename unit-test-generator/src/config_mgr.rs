use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fs;

pub fn load_config<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}
