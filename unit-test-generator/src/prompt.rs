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

    let config = load_config::<Config>(&args.pconf)?;

    let prompt_template = match file_type {
        SourceFileType::Rust => config.rust,
        SourceFileType::Python => config.python,
        SourceFileType::Cpp => config.cpp,
    };

    let prompt = format!("{} {}", prompt_template, code);

    Ok(prompt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_config(path: &std::path::Path) {
        std::fs::write(
            path,
            r#"
            {
                "rust": "RUST_PROMPT",
                "python": "PYTHON_PROMPT",
                "cpp": "CPP_PROMPT"
            }
            "#,
        )
        .unwrap();
    }

    #[test]
    fn get_prompt_rust_success() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.rs");
        let config = dir.path().join("config.json");

        std::fs::write(&input, "fn main() {}").unwrap();

        create_config(&config);

        let args = Args {
            input: input.to_str().unwrap().to_string(),
            output: "out.txt".to_string(),
            pconf: config.to_str().unwrap().to_string(),
            count: 1,
        };

        let result = get_prompt(&args).unwrap();

        assert_eq!(result, "RUST_PROMPT fn main() {}");
    }

    #[test]
    fn get_prompt_python_success() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.py");
        let config = dir.path().join("config.json");

        std::fs::write(&input, "print('hi')").unwrap();

        create_config(&config);

        let args = Args {
            input: input.to_str().unwrap().to_string(),
            output: "out.txt".to_string(),
            pconf: config.to_str().unwrap().to_string(),
            count: 1,
        };

        let result = get_prompt(&args).unwrap();

        assert_eq!(result, "PYTHON_PROMPT print('hi')");
    }

    #[test]
    fn get_prompt_cpp_success() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.cpp");
        let config = dir.path().join("config.json");

        std::fs::write(&input, "int main() {}").unwrap();

        create_config(&config);

        let args = Args {
            input: input.to_str().unwrap().to_string(),
            output: "out.txt".to_string(),
            pconf: config.to_str().unwrap().to_string(),
            count: 1,
        };

        let result = get_prompt(&args).unwrap();

        assert_eq!(result, "CPP_PROMPT int main() {}");
    }

    #[test]
    fn get_prompt_fails_when_input_missing() {
        let dir = tempdir().unwrap();

        let config = dir.path().join("config.json");

        create_config(&config);

        let args = Args {
            input: "missing.rs".to_string(),
            output: "out.txt".to_string(),
            pconf: config.to_str().unwrap().to_string(),
            count: 1,
        };

        let result = get_prompt(&args);

        assert!(result.is_err());
    }

    #[test]
    fn get_prompt_fails_when_config_missing() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.rs");

        std::fs::write(&input, "fn main() {}").unwrap();

        let args = Args {
            input: input.to_str().unwrap().to_string(),
            output: "out.txt".to_string(),
            pconf: "missing.json".to_string(),
            count: 1,
        };

        let result = get_prompt(&args);

        assert!(result.is_err());
    }

    #[test]
    fn get_prompt_fails_for_unsupported_extension() {
        let dir = tempdir().unwrap();

        let input = dir.path().join("main.txt");
        let config = dir.path().join("config.json");

        std::fs::write(&input, "hello").unwrap();

        create_config(&config);

        let args = Args {
            input: input.to_str().unwrap().to_string(),
            output: "out.txt".to_string(),
            pconf: config.to_str().unwrap().to_string(),
            count: 1,
        };

        let result = get_prompt(&args);

        assert!(result.is_err());
    }
}
