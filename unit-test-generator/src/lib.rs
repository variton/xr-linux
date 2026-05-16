use anyhow::Result;

pub mod args;
pub mod config;
pub mod config_mgr;
pub mod file_mgr;
pub mod iofilehdr;
pub mod llm_requester;
pub mod lrconfig;
pub mod prompt;

use crate::args::Args;
use crate::config_mgr::load_config;
use crate::iofilehdr::write;
use crate::llm_requester::instance_llm_requester;
use crate::lrconfig::LRConfig;
use crate::prompt::get_prompt;

pub async fn run(args: &Args) -> Result<()> {
    if args.count == 0 {
        anyhow::bail!("missing argument");
    }

    let prompt = get_prompt(args)?;
    let lrconfig = load_config::<LRConfig>(&args.lrconf)?;
    let requester = instance_llm_requester(&lrconfig)?;

    let response = requester.request(&prompt).await?;
    write(&args.output, &response.output_text().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_args(input: String, lrconf: String, pconf: String, count: usize) -> Args {
        Args {
            input,
            output: "out.txt".to_string(),
            lrconf,
            pconf,
            count,
        }
    }

    #[tokio::test]
    async fn run_fails_when_count_is_zero() {
        let args = create_args(
            "input.rs".to_string(),
            "lrconfig.json".to_string(),
            "prompt.json".to_string(),
            0,
        );

        let result = run(&args).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn run_fails_when_input_file_is_missing() {
        let dir = tempdir().unwrap();

        let args = create_args(
            dir.path().join("missing.rs").to_str().unwrap().to_string(),
            dir.path()
                .join("lrconfig.json")
                .to_str()
                .unwrap()
                .to_string(),
            dir.path().join("prompt.json").to_str().unwrap().to_string(),
            1,
        );

        let result = run(&args).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn run_fails_when_prompt_config_is_missing() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.rs");
        std::fs::write(&input, "fn main() {}").unwrap();

        let args = create_args(
            input.to_str().unwrap().to_string(),
            dir.path()
                .join("lrconfig.json")
                .to_str()
                .unwrap()
                .to_string(),
            dir.path()
                .join("missing_prompt_config.json")
                .to_str()
                .unwrap()
                .to_string(),
            1,
        );

        let result = run(&args).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn run_fails_when_llm_config_is_missing() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.rs");
        let pconf = dir.path().join("prompt_config.json");

        std::fs::write(&input, "fn main() {}").unwrap();

        std::fs::write(
            &pconf,
            r#"{
                "rust": "RUST_PROMPT",
                "python": "PYTHON_PROMPT",
                "cpp": "CPP_PROMPT"
            }"#,
        )
        .unwrap();

        let args = create_args(
            input.to_str().unwrap().to_string(),
            dir.path()
                .join("missing_llm_config.json")
                .to_str()
                .unwrap()
                .to_string(),
            pconf.to_str().unwrap().to_string(),
            1,
        );

        let result = run(&args).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn run_fails_when_llm_key_env_var_is_missing() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.rs");
        let pconf = dir.path().join("prompt_config.json");
        let lrconf = dir.path().join("llm_config.json");

        std::fs::write(&input, "fn main() {}").unwrap();

        std::fs::write(
            &pconf,
            r#"{
                "rust": "RUST_PROMPT",
                "python": "PYTHON_PROMPT",
                "cpp": "CPP_PROMPT"
            }"#,
        )
        .unwrap();

        std::fs::write(
            &lrconf,
            r#"{
                "key_env_var": "MISSING_OPEN_AI_KEY",
                "model": "gpt-4",
                "max_tokens": 100
            }"#,
        )
        .unwrap();

        unsafe {
            std::env::remove_var("MISSING_OPEN_AI_KEY");
        }

        let args = create_args(
            input.to_str().unwrap().to_string(),
            lrconf.to_str().unwrap().to_string(),
            pconf.to_str().unwrap().to_string(),
            1,
        );

        let result = run(&args).await;

        assert!(result.is_err());
    }
}
