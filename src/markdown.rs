
use serde_yaml::Value;
use std::collections::HashMap;

pub fn parse_metadata(metadata: &str) -> HashMap<String, Value> {
    let metadata_map: HashMap<String, Value> = serde_yaml::from_str(metadata).unwrap();
    metadata_map
}

pub fn parse_markdown_file(content: &str) -> (HashMap<String, Value>, String) {
    let split: Vec<&str> = content.splitn(3, "---").collect();
    let metadata: &&str = split.get(1).unwrap_or(&"");
    let content: &&str = split.get(2).unwrap_or(&"");

    let metadata_map: HashMap<String, Value> = parse_metadata(metadata);
    let html_content: String= markdown_to_html(content);

    (metadata_map, html_content)
}

use pulldown_cmark::{Parser, Options, html};

pub fn markdown_to_html(markdown_input: &str) -> String {
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output: String = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}