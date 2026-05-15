mod args;
mod config;
mod config_mgr;
mod iofilehdr;
mod llm_requester;
mod prompt;

use anyhow::Result;
use std;
use std::env;

use args::Args;
use iofilehdr::write;
use llm_requester::LLMRequester;
use prompt::get_prompt;

#[tokio::main]
async fn main() -> Result<()> {
    let key = env::var("OPEN_AI_KEY")?;
    let args = Args::parse_args();

    if args.count == 0 {
        eprintln!("missing argument");
        std::process::exit(1);
    }

    let prompt = get_prompt(&args)?;
    let model_name = "gpt-4.1";
    let mut llm_requester = LLMRequester::new(key, model_name, 512);
    llm_requester.init()?;
    let response = llm_requester.request(&prompt).await?;

    write(&args.output, &response.output_text().unwrap())?;
    Ok(())
}
