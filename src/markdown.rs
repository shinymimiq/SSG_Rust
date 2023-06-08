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


#[cfg(test)]
mod tests {
    use super::*;

    /// test function for parse_metadata
    #[test]
    fn test_parse_metadata() {
        let metadata = r#"title: "Test Title"
author: "Test Author"
datetime: "2020-01-01"
tags:
- "tag1"
- "tag2"
categories:
    - "category1"
    - "category2""#;

        let metadata_map: HashMap<String, Value> = parse_metadata(metadata);

        assert_eq!(metadata_map.get("title").unwrap(), &Value::String("Test Title".to_string()));
        assert_eq!(metadata_map.get("author").unwrap(), &Value::String("Test Author".to_string()));
        assert_eq!(metadata_map.get("datetime").unwrap(), &Value::String("2020-01-01".to_string()));
        assert_eq!(metadata_map.get("tags").unwrap(), &Value::Sequence(vec![Value::String("tag1".to_string()), Value::String("tag2".to_string())]));
        assert_eq!(metadata_map.get("categories").unwrap(), &Value::Sequence(vec![Value::String("category1".to_string()), Value::String("category2".to_string())]));
    }

    /// test function for markdown_to_html
    #[test]
    fn test_markdown_to_html() {
        let markdown_input = r#"# Test Title
This is a test paragraph.

This is another test paragraph.

This is a paragraph with **bold** and *italic* text.

This is a paragraph with ~~strikethrough~~ text.

This is a paragraph with a [link](https://example.com).

This is a paragraph with an image: ![alt text](https://example.com/image.jpg "Image Title")

This is a paragraph with a `code block`.

This is a paragraph with a block quote:

> This is a block quote.

This is a paragraph with a list:
- item 1
- item 2
- item 3

This is a paragraph with a numbered list:
1. item 1
2. item 2
3. item 3

"#;

        let html_output = markdown_to_html(markdown_input);

        assert_eq!(html_output, "<h1>Test Title</h1>\n<p>This is a test paragraph.</p>\n<p>This is another test paragraph.</p>\n<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>\n<p>This is a paragraph with <del>strikethrough</del> text.</p>\n<p>This is a paragraph with a <a href=\"https://example.com\">link</a>.</p>\n<p>This is a paragraph with an image: <img src=\"https://example.com/image.jpg\" alt=\"alt text\" title=\"Image Title\" /></p>\n<p>This is a paragraph with a <code>code block</code>.</p>\n<p>This is a paragraph with a block quote:</p>\n<blockquote>\n<p>This is a block quote.</p>\n</blockquote>\n<p>This is a paragraph with a list:</p>\n<ul>\n<li>item 1</li>\n<li>item 2</li>\n<li>item 3</li>\n</ul>\n<p>This is a paragraph with a numbered list:</p>\n<ol>\n<li>item 1</li>\n<li>item 2</li>\n<li>item 3</li>\n</ol>\n");
    }

}


