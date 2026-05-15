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
use llm_requester::create_llm_requester;
use prompt::get_prompt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse_args();

    if args.count == 0 {
        eprintln!("missing argument");
        std::process::exit(1);
    }

    let prompt = get_prompt(&args)?;
    let model_name = "gpt-4.1";

    let requester = create_llm_requester(model_name, 512)?;
    let response = requester.request(&prompt).await?;

    write(&args.output, &response.output_text().unwrap())?;
    Ok(())
}
