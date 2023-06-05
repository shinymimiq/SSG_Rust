extern crate pulldown_cmark;
extern crate serde_yaml;

fn main() {
    let markdown_files = read_markdown_files("markdown_files");

    let rendered_files = render_files(markdown_files.clone());

    // At this point, rendered_files is a Vec<String>, where each String is the HTML content of a page.
    // You could print them to the console to check the output:
    for file in &rendered_files {
        println!("{}", file);
    }

    write_files(rendered_files, &markdown_files);
}

use glob::glob;
use std::io::prelude::*;
use std::collections::HashMap;
use serde_yaml::Value;

#[derive(Clone)]
struct MarkdownFile {
    metadata: HashMap<String, Value>,
    content: String,
}

fn parse_metadata(metadata: &str) -> HashMap<String, Value> {
    let metadata_map: HashMap<String, Value> = serde_yaml::from_str(metadata).unwrap();
    metadata_map
}

fn read_markdown_files(directory: &str) -> Vec<MarkdownFile> {
    let mut files: Vec<MarkdownFile> = Vec::new();

    for entry in glob(&format!("{}/*.md", directory)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut file: File = File::open(path).expect("Failed to open file");
                let mut text: String = String::new();
                file.read_to_string(&mut text).expect("Failed to read file");
                let (metadata, content) = parse_markdown_file(&text);
                files.push(MarkdownFile{
                    metadata: metadata,
                    content: content.to_string(),
                });
            },
            Err(e) => println!("{:?}", e),
        }
    }

    files
}

fn parse_markdown_file(content: &str) -> (HashMap<String, Value>, String) {
    let split: Vec<&str> = content.splitn(3, "---").collect();
    let metadata: &&str = split.get(1).unwrap_or(&"");
    let content: &&str = split.get(2).unwrap_or(&"");

    let metadata_map: HashMap<String, Value> = parse_metadata(metadata);
    let html_content: String= markdown_to_html(content);

    (metadata_map, html_content)
}

use pulldown_cmark::{Parser, Options, html};

fn markdown_to_html(markdown_input: &str) -> String {
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output: String = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}


use tera::{Context, Tera};

fn render_files(files: Vec<MarkdownFile>) -> Vec<String> {
    let tera: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut rendered_files: Vec<String> = Vec::new();

    for file in files {
        let mut context: Context = Context::new();
        if let Value::String(title) = file.metadata.get("title").unwrap() {
            context.insert("title", title);
        }
        context.insert("content", &file.content);

        match tera.render("post.html", &context) {
            Ok(rendered) => rendered_files.push(rendered),
            Err(e) => println!("Rendering error: {}", e),
        };
    }

    rendered_files
}

use std::fs::File;
use std::io::Write;

// ...

fn write_files(files: Vec<String>, markdown_files: &Vec<MarkdownFile>) {
    for (file, markdown_file) in files.into_iter().zip(markdown_files) {
        if let Value::String(title) = markdown_file.metadata.get("title").unwrap() {
            let file_name = format!("output/{}.html", title);  // Adjust the directory to your needs
            let mut output_file = File::create(file_name).expect("Failed to create file");
            output_file.write_all(file.as_bytes()).expect("Failed to write to file");
        }
    }
}