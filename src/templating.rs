use tera::{Context, Tera};
use crate::MarkdownFile;

use std::fs;
use toml;

// At the beginning of your file
const DEFAULT_TEMPLATE_PATH: &str = "templates/**/*";
/// Renders a vector of Markdown files using Tera templates and returns a vector of rendered HTML strings.
///
/// # Arguments
///
/// * `files` - A vector of MarkdownFile structs containing metadata and content.
///
/// # Returns
///
/// A vector of rendered HTML strings.
pub fn render_files(files: Vec<MarkdownFile>) -> Vec<String> {
    // Read the configuration file
    let config: String = fs::read_to_string("config/config.toml")
    .expect("Could not read configuration file");

    // Parse the configuration file
    let value = config.parse::<toml::Value>()
    .expect("Could not parse configuration file");

    let template_path: String = value["templates"]["path"]
    .as_str()
    .and_then(|path| {
        let user_template_path = format!("{}/**/*", path);
        if fs::metadata(&user_template_path).is_ok() {
            Some(user_template_path)
        } else {
            None
        }
    })
    .unwrap_or_else(|| DEFAULT_TEMPLATE_PATH.to_string());

    let tera: Tera = match Tera::new(&template_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut rendered_files: Vec<String> = Vec::new();

    for file in files {
        let mut context: Context = Context::new();
        context.insert("title", &file.title);
        context.insert("author", &file.author);
        context.insert("datetime", &file.datetime);
        context.insert("tags", &file.tags);
        context.insert("categories", &file.categories);
        context.insert("content", &file.content);

        match tera.render("post.html", &context) {
            Ok(rendered) => rendered_files.push(rendered),
            Err(e) => println!("Rendering error: {}", e),
    };
}

    rendered_files
}

