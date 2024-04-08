use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub author: String,
    #[serde(rename = "defaultTags")]
    pub default_tags: Vec<String>,
    #[serde(rename = "outputFolder")]
    pub output_folder: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
