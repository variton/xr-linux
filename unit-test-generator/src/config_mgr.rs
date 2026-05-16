use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fs;

/// Loads and deserializes a JSON configuration file.
///
/// The file contents are read into memory and deserialized into the
/// requested type `T`.
///
/// # Type Parameters
///
/// * `T` - A type implementing [`DeserializeOwned`] that represents
///   the configuration structure.
///
/// # Arguments
///
/// * `path` - Path to the JSON configuration file.
///
/// # Errors
///
/// Returns an error if:
///
/// - the file cannot be read,
/// - the file contents are not valid JSON,
/// - deserialization into `T` fails.
///
/// # Examples
///
/// ```no_run
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     value: String,
/// }
///
/// let config: Config =
///     xllm_requester::config_mgr::load_config("config.json")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load_config<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tempfile::tempdir;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        name: String,
    }

    #[test]
    fn load_config_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");

        std::fs::write(
            &file_path,
            r#"{
                "name": "alice"
            }"#,
        )
        .unwrap();

        let config: Config = load_config(file_path.to_str().unwrap()).unwrap();

        assert_eq!(
            config,
            Config {
                name: "alice".to_string(),
            }
        );
    }

    #[test]
    fn load_config_file_not_found() {
        let result: Result<Config, _> = load_config("does_not_exist.json");

        assert!(result.is_err());
    }

    #[test]
    fn load_config_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");

        std::fs::write(&file_path, "not valid json").unwrap();

        let result: Result<Config, _> = load_config(file_path.to_str().unwrap());

        assert!(result.is_err());
    }
}
