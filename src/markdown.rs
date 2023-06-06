use serde_yaml::Value;
use std::collections::HashMap;
use crate::MarkdownFile;

use log::{info, debug};

/// Parses the metadata from a string and returns a HashMap of key-value pairs.
///
/// # Arguments
///
/// * `metadata` - A string containing metadata in YAML format.
///
/// # Returns
///
/// A HashMap containing the key-value pairs from the metadata.
pub fn parse_metadata(metadata: &str) -> HashMap<String, Value> {
    let metadata_map: HashMap<String, Value> = serde_yaml::from_str(metadata).unwrap();
    metadata_map
}

/// Parses a markdown file and returns a tuple containing the metadata and the HTML content.
///
/// # Arguments
///
/// * `content` - A string containing the markdown content.
///
/// # Returns
///
/// A tuple containing a HashMap of key-value pairs from the metadata and a string of HTML content.
pub fn parse_markdown_file(content: &str) -> MarkdownFile {
    debug!("Parsing markdown file");
    let split: Vec<&str> = content.splitn(3, "---").collect();
    let metadata: &&str = split.get(1).unwrap_or(&"");
    let content: &&str = split.get(2).unwrap_or(&"");

    let metadata_map: HashMap<String, Value> = parse_metadata(metadata);
    let html_content: String= markdown_to_html(content);

    let title = match metadata_map.get("title") {
        Some(Value::String(title)) => title.clone(),
        _ => String::new(),
    };

    let author = match metadata_map.get("author") {
        Some(Value::String(author)) => author.clone(),
        _ => String::new(),
    };

    let datetime = match metadata_map.get("datetime") {
        Some(Value::String(datetime)) => datetime.clone(),
        _ => String::new(),
    };

    let tags: Vec<String> = match metadata_map.get("tags") {
        Some(Value::Sequence(tags)) => tags.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect(),
        _ => Vec::new(),
    };

    let categories: Vec<String> = match metadata_map.get("categories") {
        Some(Value::Sequence(categories)) => categories.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect(),
        _ => Vec::new(),
    };

    MarkdownFile {
        title,
        author,
        datetime,
        tags,
        categories,
        content: html_content,
    }
}

use pulldown_cmark::{Parser, Options, html};

/// Converts a markdown string to HTML.
///
/// # Arguments
///
/// * `markdown_input` - A string containing markdown content.
///
/// # Returns
///
/// A string containing the HTML representation of the markdown content.
pub fn markdown_to_html(markdown_input: &str) -> String {
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output: String = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

