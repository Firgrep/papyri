use crate::File;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownMetadataConfig {
    pub author: String,
    #[serde(rename = "defaultTags")]
    pub default_tags: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "markdownMetadata")]
    pub markdown_metadata: MarkdownMetadataConfig,
    #[serde(rename = "outputFolder")]
    pub output_folder: String,
}

pub fn get_or_create_config(target_file: &str) -> Result<Config, std::io::Error> {
    let config = match File::open(target_file) {
        Ok(_) => {
            let config_str = fs::read_to_string(target_file)?;
            let config: Config = serde_json::from_str(&config_str)?;
            config
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!(
                "Config file not found. Creating a new one at '{}'. Customize it as needed.",
                target_file
            );
            // If config file not found, proceed to create a new Config with default values
            let default_config = Config {
                markdown_metadata: MarkdownMetadataConfig {
                    author: String::from("Default Author"),
                    default_tags: vec![String::from("tag1"), String::from("tag2")],
                },
                output_folder: String::from("./output"),
            };

            // Write the default config to the new file
            let serialized_json = serde_json::to_string_pretty(&default_config).unwrap();
            let mut file = File::create(target_file)?;
            file.write_all(serialized_json.as_bytes())?;

            // Return the default config
            default_config
        }
        Err(err) => return Err(err),
    };

    Ok(config)
}
