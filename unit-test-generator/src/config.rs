use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rust: String,
    pub python: String,
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
