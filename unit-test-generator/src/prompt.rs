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

// use serde::Deserialize;
// use std;
// use std::fs;

// #[derive(Deserialize, Debug)]
// struct Config {
//     rust: String,
//     python: String,
//     cpp: String,
// }

// fn load_config(json_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
//     let text = fs::read_to_string(json_path)?;
//     let config: Config = serde_json::from_str(&text)?;
//     Ok(config)
// }

// pub fn get_prompt(lang: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let config = load_config("prompt.json")?;

//     let prompt: &str = match lang {
//         "rust" => config.rust.as_str(),
//         "python" => config.python.as_str(),
//         "cpp" => config.cpp.as_str(),
//         _ => {
//             eprintln!("unsupported language: {}", lang);
//             std::process::exit(1);
//         }
//     };
//     Ok(prompt.to_string())
// }
