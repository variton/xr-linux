use crate::lrconfig::LRConfig;
use anyhow::{Context, Result};
use async_openai::types::responses::Response;
use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};

pub struct LLMRequester<'a> {
    llm_api_key: String,
    llm_model_name: &'a str,
    llm_client: Option<Client<OpenAIConfig>>,
    llm_max_output_tokens: u32,
}

impl<'a> LLMRequester<'a> {
    pub fn new(api_key: String, model_name: &'a str, max_output_tokens: u32) -> Self {
        LLMRequester {
            llm_api_key: api_key,
            llm_model_name: model_name,
            llm_client: None,
            llm_max_output_tokens: max_output_tokens,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        let config = OpenAIConfig::new().with_api_key(self.llm_api_key.clone());
        self.llm_client = Some(Client::with_config(config));
        Ok(())
    }

    pub async fn request(&self, prompt: &str) -> Result<Response> {
        let client = self
            .llm_client
            .as_ref()
            .context("LLM client has not been initialized")?;

        let request = CreateResponseArgs::default()
            .model(self.llm_model_name)
            .input(prompt)
            .max_output_tokens(self.llm_max_output_tokens)
            .build()?;

        let response = client
            .responses() // Get the API "group" (responses, images, etc.) from the client
            .create(request) // Make the API call in that "group"
            .await?;

        Ok(response)
    }
}

pub fn create_llm_requester<'a>(model_name: &'a str, max_tokens: u32) -> Result<LLMRequester<'a>> {
    let key = std::env::var("OPEN_AI_KEY")?;

    let mut requester = LLMRequester::new(key, model_name, max_tokens);
    requester.init()?;

    Ok(requester)
}

pub fn instance_llm_requester(lrconfig: &LRConfig) -> Result<LLMRequester<'_>> {
    let key = std::env::var(&lrconfig.key_env_var)?;

    let mut requester = LLMRequester::new(key, &lrconfig.model, lrconfig.max_tokens);
    requester.init()?;

    Ok(requester)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request_fails_when_client_not_initialized() {
        let requester = LLMRequester::new("fake-key".to_string(), "gpt-4", 100);

        let result = requester.request("hello").await;

        assert!(result.is_err());

        let error = result.unwrap_err().to_string();

        assert!(error.contains("LLM client has not been initialized"));
    }

    #[test]
    fn instance_llm_requester_fails_without_configured_env_var() {
        let lrconfig = LRConfig {
            key_env_var: "MISSING_OPEN_AI_KEY".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: 100,
        };

        unsafe {
            std::env::remove_var(&lrconfig.key_env_var);
        }

        let result = instance_llm_requester(&lrconfig);

        assert!(result.is_err());
    }

    #[test]
    fn create_llm_requester_fails_without_env_var() {
        unsafe {
            std::env::remove_var("OPEN_AI_KEY");
        }

        let result = create_llm_requester("gpt-4", 100);

        assert!(result.is_err());
    }
}
