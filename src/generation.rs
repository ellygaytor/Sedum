use std::{fs::{self, File}, io::Write, path::{Path}};

use extract_frontmatter::Extractor;
use pulldown_cmark::{Parser, html};
use structopt::StructOpt;

use crate::{options::{self}, structs::{Constants, Page}};

pub fn create_include(name: &str) -> String {
    let opt = options::Opt::from_args();
    let mut include_path = opt.source;
    include_path.push(name);
    include_path.push(".include");
    let include: String = fs::read_to_string(include_path).unwrap_or_default();
    include
}
pub fn generate_html(
    source_file: &Path,
    constants: &Constants,
) {
    
    let relative = match source_file.strip_prefix(&constants.opt.source) {
        Ok(path) => path,
        Err(_) => {
            println!("Could not remove prefix. Skipping this file.");
            return;
        }
    };
    let mut target = constants.opt.destination.join(relative);
    let source_contents = match fs::read_to_string(source_file) {
        Ok(source_contents) => source_contents,
        Err(e) => {
            println!("Could not read the markdown file: {}", e);
            return;
        }
    };
    let mut extractor = Extractor::new(&source_contents);
    extractor.select_by_terminator("---");
    extractor.strip_prefix("---");
    let settings_yaml: String = extractor.extract();

    let (content, settings) = match serde_yaml::from_str(&settings_yaml) {
        Ok(settings) => (extractor.remove().trim(), settings),
        Err(_) => (
            source_contents.as_str(),
            Page {
                title: None,
                description: None,
                language: None,
                author: None,
                list: None,
            },
        ),
    };

    let parser = Parser::new_ext(content, constants.pulldown_cmark_options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    let lang_string: String = match settings.language {
        None => String::from(""),
        Some(lang) => format!(" lang='{}'", lang),
    };
    let title_string: String = match settings.title {
        None => String::from(
            source_file
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ),
        Some(title) => title,
    };
    let description_string = match settings.description {
        None => String::from(""),
        Some(description) => format!("<meta name='description' content='{}'>", description),
    };
    let author_string = match settings.author {
        None => String::from(&constants.global_settings.default_author),
        Some(author) => author,
    };
    let mut page = format!("<!DOCTYPE html>\n<html{}>{}<head>\n<meta charset='utf-8'>\n<title>{}</title>\n{}\n<meta name='author' content='{}'>\n<meta name='viewport' content='width=device-width, initial-scale=1'>\n<link rel='stylesheet' href='/main.css'>\n</head>\n<body>\n{}\n{}</body>\n</html>", lang_string, constants.head_include, &title_string, description_string, author_string, html_content, constants.body_include);
    if constants.list_count == 0 {
        page = str::replace(&page, "|LIST|", "");
    } else {
        page = str::replace(&page, "|LIST|", &constants.list_html);
    }
    let prefix = &target.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    target.set_extension("html");
    let mut target_file = match File::create(target) {
        Ok(target_file) => target_file,
        Err(e) => {
            println!("Could not create target file: {}", e);
            return;
        }
    };
    match write!(&mut target_file, "{}", page) {
        Ok(_) => (),
        Err(e) => {
            println!("Could not write to target file: {}", e);
        }
    };
}
