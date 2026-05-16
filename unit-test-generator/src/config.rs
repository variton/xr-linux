use serde::Deserialize;

/// Prompt configuration for supported programming languages.
///
/// This structure is typically loaded from a JSON configuration file
/// and provides the prompt template associated with each language.
///
/// # Example
///
/// ```json
/// {
///   "rust": "Explain the following Rust code:",
///   "python": "Explain the following Python code:",
///   "cpp": "Explain the following C++ code:"
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Prompt template used for Rust source files.
    pub rust: String,

    /// Prompt template used for Python source files.
    pub python: String,

    /// Prompt template used for C++ source files.
    pub cpp: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_config_success() {
        let json = r#"
        {
            "rust": "cargo",
            "python": "python3",
            "cpp": "g++"
        }
        "#;

        let config: Config = serde_json::from_str(json).unwrap();

        assert_eq!(config.rust, "cargo");
        assert_eq!(config.python, "python3");
        assert_eq!(config.cpp, "g++");
    }

    #[test]
    fn deserialize_config_missing_field() {
        let json = r#"
        {
            "rust": "cargo",
            "python": "python3"
        }
        "#;

        let result: Result<Config, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }
}
