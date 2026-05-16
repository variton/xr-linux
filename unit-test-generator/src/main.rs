use anyhow::Result;
use xllm_requester::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse_args();

    xllm_requester::run(&args).await
}
