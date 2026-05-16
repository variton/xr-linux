use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LRConfig {
    pub key_env_var: String,
    pub model: String,
    pub max_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_lrconfig_success() {
        let json = r#"
        {
            "key_env_var": "OPEN_AI_KEY",
            "model": "gpt-4",
            "max_tokens": 100
        }
        "#;

        let config: LRConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.key_env_var, "OPEN_AI_KEY");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 100);
    }

    #[test]
    fn deserialize_lrconfig_missing_field() {
        let json = r#"
        {
            "key_env_var": "OPEN_AI_KEY",
            "model": "gpt-4"
        }
        "#;

        let result: Result<LRConfig, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn deserialize_lrconfig_invalid_max_tokens_type() {
        let json = r#"
        {
            "key_env_var": "OPEN_AI_KEY",
            "model": "gpt-4",
            "max_tokens": "invalid"
        }
        "#;

        let result: Result<LRConfig, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }
}
