use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use extract_frontmatter::Extractor;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Page {
    title: String,
    description: String,
    language: String,
    author: String,
}

fn main() {
    let mut pulldown_cmark_options = Options::empty();
    pulldown_cmark_options.insert(Options::ENABLE_STRIKETHROUGH);
    pulldown_cmark_options.insert(Options::ENABLE_TABLES);

    let args: Vec<String> = env::args().collect();

    let (source, destination) = parse_config(&args);

    let mut source_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            if entry.path().extension().unwrap() == "md" {
                source_files.push(entry.path().to_path_buf())
            } else if entry.path().extension().unwrap() != "include" {
                let relative = &entry.path().strip_prefix(source).expect("Not a prefix");
                let target = destination.join(relative);
                let prefix = &target.parent().unwrap();
                std::fs::create_dir_all(prefix).unwrap();
                fs::copy(entry.path(), target).expect("Could not copy file");
            }
        }
    }

    for source_file in source_files {
        let relative = source_file.strip_prefix(source).expect("Not a prefix");
        let mut target = destination.join(relative);
        let contents = fs::read_to_string(source_file)
            .expect("There was an error reading the markdown file.");
        let mut extractor = Extractor::new(&contents);
        extractor.select_by_terminator("---");
        extractor.strip_prefix("---");
        let settings_yaml: String = extractor.extract();
        let content: &str = extractor.remove().trim();
        let settings = serde_yaml::from_str::<Page>(&settings_yaml).unwrap();
        let parser = Parser::new_ext(content, pulldown_cmark_options);
        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);
        let mut head_include_path = PathBuf::from(source);
        head_include_path.push("head.include");
        let head_include: String = fs::read_to_string(head_include_path).unwrap_or_default();
        let mut body_include_path = PathBuf::from(source);
        body_include_path.push("body.include");
        let body_include: String = fs::read_to_string(body_include_path).unwrap_or_default();
        let page = format!("<!DOCTYPE html>\n<html lang='{}'>{}<head>\n<meta charset='utf-8'>\n<title>{}</title>\n<meta name='description' content='{}'>\n<meta name='author' content='{}'>\n<meta name='viewport' content='width=device-width, initial-scale=1'>\n<link rel='stylesheet' href='/main.css'>\n</head>\n<body>\n{}\n{}</body>\n</html>", settings.language, head_include, settings.title, settings.description, settings.author, html_content, body_include);
        let prefix = &target.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        target.set_extension("html");
        let mut target_file = File::create(target).expect("Unable to create.");
        write!(&mut target_file, "{}", page).expect("Could not write to target file.");
    }
}

fn parse_config(args: &[String]) -> (&Path, &Path) {
    let source = Path::new(&args[1]);
    let destination = Path::new(&args[2]);

    (source, destination)
}
