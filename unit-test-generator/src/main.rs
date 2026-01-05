use anyhow::{Context, Result};
use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use clap::Parser;
use std::env;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file
    input: String,

    /// Number of times
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn read_file(path: &str) -> Result<String> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read file `{}`", path))?;
    Ok(contents)
}

fn write_file(path: &str, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write to `{}`", path))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let key = env::var("OPEN_AI_KEY")?;
    let args = Args::parse();

    let config = OpenAIConfig::new().with_api_key(key);
    let client = Client::with_config(config);

    let code = read_file(&args.input)?;
    let prompt = format!(
        "Only generate unit tests runable with pytest \
        and do not gegenerate any test with unittest.mock for the following code: \n {}",
        code
    );

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

    write_file("output.txt", &response.output_text().unwrap())?;
    Ok(())
}
