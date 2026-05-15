use crate::args::Args;
use crate::config::Config;
use crate::config_mgr::load_config;
use crate::file_mgr::SourceFileType;
use crate::file_mgr::detect_source_file;
use crate::iofilehdr::read;

use anyhow::Result;

pub fn get_prompt(args: &Args) -> Result<String> {
    let code = read(&args.input)?;
    let file_type = detect_source_file(&args.input)?;

    let config = load_config::<Config>(&args.conf)?;

    let prompt_template = match file_type {
        SourceFileType::Rust => config.rust,
        SourceFileType::Python => config.python,
        SourceFileType::Cpp => config.cpp,
    };

    let prompt = format!("{} {}", prompt_template, code);

    Ok(prompt)
}
