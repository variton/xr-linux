use crate::args::Args;
use crate::config::Config;
use crate::config_mgr::load_config;
use crate::iofilehdr::read;
use anyhow::Result;

pub fn get_prompt(args: &Args) -> Result<String> {
    let code = read(&args.input)?;

    let config = load_config::<Config>(&args.conf)?;

    let prompt_template = match args.lang.as_str() {
        "rust" => config.rust,
        "python" => config.python,
        "cpp" => config.cpp,
        _ => anyhow::bail!("unsupported language: {}", args.lang),
    };

    let prompt = format!("{} {}", prompt_template, code);

    Ok(prompt)
}
