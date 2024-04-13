mod case_utils;
mod handle_config;

use crate::handle_config::Config;
use chrono::Local;
extern crate inflector;
use std::env;
use std::fs::File;
use std::io::prelude::*;

macro_rules! markdown_template {
    () => {
        r#"---
title: "{}"
pubDate: {}
description: ""
author: "{}"
tags: ["{}"]
---

# {}
"#
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a filename.");
        std::process::exit(1);
    }
    if args[1] == "" {
        eprintln!("Filename cannot be empty string.");
        std::process::exit(1);
    }

    let config: Config = handle_config::Config::from_file("config.json")?;
    let title: &String = &args[1];

    println!("Author: {}", config.author);
    println!("Default Tags: {:?}", config.default_tags);
    println!("Output Folder: {}", config.output_folder);

    let file_name: String = title
        .to_lowercase()
        .replace(" ", "-")
        .replace("'", "%27")
        .replace(",", "%2C");
    let folder_path = &config.output_folder;

    std::fs::create_dir_all(folder_path)?;

    let file_with_date = format!(
        "{}/{}_{}.md",
        folder_path,
        Local::now().format("%Y-%m-%d"),
        &file_name
    );

    let mut file = File::create(&file_with_date)?;
    let capitalized_title = case_utils::qualified_capitalize(title);

    write!(
        &mut file,
        markdown_template!(),
        title,
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        config.author,
        config.default_tags.join("\", \""),
        capitalized_title
    )?;

    println!("Note created: {}", file_with_date);
    Ok(())
}
