use tera::{Context, Tera};
use crate::MarkdownFile;
use serde_yaml::Value;

pub fn render_files(files: Vec<MarkdownFile>) -> Vec<String> {
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
