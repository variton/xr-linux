use crate::config::Config;
use crate::config_mgr::load_config;
use anyhow::Result;

pub fn get_prompt(lang: &str, json_path: &str) -> Result<String> {
    let config = load_config::<Config>(json_path)?;

    let prompt = match lang {
        "rust" => config.rust,
        "python" => config.python,
        "cpp" => config.cpp,
        _ => anyhow::bail!("unsupported language: {}", lang),
    };
    Ok(prompt)
}
