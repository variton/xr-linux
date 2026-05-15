use clap::Parser;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(short, long)]
    pub input: String,

    /// Output file
    #[arg(short, long)]
    pub output: String,

    /// Configuration file
    #[arg(long)]
    pub conf: String,

    /// Main stream language to choose
    #[arg(short, long)]
    pub lang: String,

    /// Number of times
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

impl Args {
    /// Parse CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
