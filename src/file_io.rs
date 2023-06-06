use std::fs::File;
use std::io::Write;
use glob::glob;
use crate::MarkdownFile;
use std::io::Read;
use log::debug;

use crate::parse_markdown_file;

/// Writes a vector of HTML strings to files with names based on the "title" metadata in the corresponding MarkdownFile.
///
/// # Arguments
///
/// * `files` - A vector of HTML strings to write to files.
/// * `markdown_files` - A vector of MarkdownFile structs containing metadata and content.
pub fn write_files(files: Vec<String>, markdown_files: &Vec<MarkdownFile>) {
    for (file, markdown_file) in files.into_iter().zip(markdown_files) {
        if !markdown_file.title.is_empty() {
            let file_name = format!("output/{}.html", markdown_file.title);  // Adjust the directory to your needs
            debug!("Writing file: {}", file_name);
            let mut output_file = File::create(file_name).expect("Failed to create file");
            output_file.write_all(file.as_bytes()).expect("Failed to write to file");
        }
    }
}

/// Reads all Markdown files in a directory and returns a vector of MarkdownFile structs containing metadata and content.
///
/// # Arguments
///
/// * `directory` - A string containing the path to the directory containing the Markdown files.
///
/// # Returns
///
/// A vector of MarkdownFile structs containing metadata and content.
pub fn read_markdown_files(directory: &str) -> Vec<MarkdownFile> {
    let mut files: Vec<MarkdownFile> = Vec::new();

    for entry in glob(&format!("{}/*.md", directory)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                debug!("Reading markdown file: {:?}", path);
                let mut file: File = File::open(path).expect("Failed to open file");
                let mut text: String = String::new();
                file.read_to_string(&mut text).expect("Failed to read file");
                let mf = parse_markdown_file(&text);
                files.push(mf);
            },
            Err(e) => println!("{:?}", e),
        }
    }

    files
}

