use std::fs::File;
use std::io::Write;
use glob::glob;
use crate::MarkdownFile;
use serde_yaml::Value;
use std::io::Read;

// mod markdown;
use crate::parse_markdown_file;
// use markdown::{parse_metadata, parse_markdown_file, markdown_to_html};

pub fn write_files(files: Vec<String>, markdown_files: &Vec<MarkdownFile>) {
    for (file, markdown_file) in files.into_iter().zip(markdown_files) {
        if let Value::String(title) = markdown_file.metadata.get("title").unwrap() {
            let file_name = format!("output/{}.html", title);  // Adjust the directory to your needs
            let mut output_file = File::create(file_name).expect("Failed to create file");
            output_file.write_all(file.as_bytes()).expect("Failed to write to file");
        }
    }
}

pub fn read_markdown_files(directory: &str) -> Vec<MarkdownFile> {
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


