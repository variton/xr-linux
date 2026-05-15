mod config;
mod config_mgr;
mod iofilehdr;
mod prompt;

use anyhow::{Context, Result};
use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use clap::Parser;
use std;
use std::env;
use std::fs;

use iofilehdr::{read, write};
use prompt::get_prompt;

#[derive(Parser, Debug)]
#[command(author, version, about,long_about = None)]
struct Args {
    /// Input file
    input: String,

    //main stream language to choose
    #[arg(short, long)]
    lang: String,

    /// Number of times
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let key = env::var("OPEN_AI_KEY")?;
    let args = Args::parse();

    if args.count == 0 {
        eprintln!("missing argument");
        std::process::exit(1);
    }

    let config = OpenAIConfig::new().with_api_key(key);
    let client = Client::with_config(config);

    let code = read(&args.input)?;
    let prompt_template = get_prompt(&args.lang, "prompts.json")?;
    let prompt = format!("{} {}", prompt_template, code);

    let request = CreateResponseArgs::default()
        .model("gpt-4.1")
        .input(&prompt)
        .max_output_tokens(512u32)
        .build()?;

    // Call API
    let response = client
        .responses() // Get the API "group" (responses, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await?;

    write("output.txt", &response.output_text().unwrap())?;
    Ok(())
}
