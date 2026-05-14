use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rust: String,
    pub python: String,
    pub cpp: String,
}
