extern crate pulldown_cmark;
extern crate serde_yaml;

mod file_io;
mod markdown;
mod templating;

use file_io::{read_markdown_files, write_files};
use markdown::parse_markdown_file;
use templating::render_files;

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

#[derive(Clone)]
pub struct MarkdownFile {
    title: String,
    author: String,
    datetime: String,
    tags: Vec<String>,
    categories: Vec<String>,
    content: String,
}

