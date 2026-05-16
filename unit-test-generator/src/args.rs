use clap::Parser;

/// Command-line arguments for the application.
///
/// These arguments define the input source file, output file,
/// prompt configuration, LLM requester configuration, and execution count.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the input source file.
    #[arg(short, long)]
    pub input: String,

    /// Path where the generated output should be written.
    #[arg(short, long)]
    pub output: String,

    /// Path to the LLM requester configuration file.
    #[arg(long)]
    pub lrconf: String,

    /// Path to the prompt configuration file.
    #[arg(long)]
    pub pconf: String,

    /// Number of times the request should be executed.
    ///
    /// Defaults to `1` when not provided.
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

impl Args {
    /// Parses command-line arguments from the current process.
    ///
    /// This uses [`clap::Parser::parse`] internally.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parse_args_success_with_all_fields() {
        let args = Args::parse_from([
            "app",
            "--input",
            "in.txt",
            "--output",
            "out.txt",
            "--lrconf",
            "requester.json",
            "--pconf",
            "config.json",
            "--count",
            "5",
        ]);

        assert_eq!(args.input, "in.txt");
        assert_eq!(args.output, "out.txt");
        assert_eq!(args.lrconf, "requester.json");
        assert_eq!(args.pconf, "config.json");
        assert_eq!(args.count, 5);
    }

    #[test]
    fn parse_args_uses_default_count() {
        let args = Args::parse_from([
            "app",
            "--input",
            "in.txt",
            "--output",
            "out.txt",
            "--lrconf",
            "requester.json",
            "--pconf",
            "config.json",
        ]);

        assert_eq!(args.count, 1);
    }

    #[test]
    fn parse_args_missing_required_argument() {
        let result = Args::try_parse_from(["app", "--input", "in.txt", "--output", "out.txt"]);

        assert!(result.is_err());
    }
}
