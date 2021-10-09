use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use extract_frontmatter::Extractor;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Page {
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    list: String,
}

fn main() {
    let mut pulldown_cmark_options = Options::empty();
    pulldown_cmark_options.insert(Options::ENABLE_STRIKETHROUGH);
    pulldown_cmark_options.insert(Options::ENABLE_TABLES);

    let (source, destination) = parse_config();

    let mut source_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            match entry.path().extension().and_then(OsStr::to_str) {
                Some("md") => source_files.push(entry.path().to_path_buf()),
                Some("include") => (),
                None => (),
                _ => {
                    let relative = &entry.path().strip_prefix(&source).expect("Not a prefix");
                    let target = destination.join(relative);
                    let prefix = &target.parent().unwrap();
                    std::fs::create_dir_all(prefix).unwrap();
                    fs::copy(entry.path(), target).expect("Could not copy file");
                }
            }
        }
    }

    let mut list_html = String::from("<ul>");
    let mut list_count = 0;
    for source_file in &source_files {
        let relative = source_file.strip_prefix(&source).expect("Not a prefix");
        let contents = match fs::read_to_string(source_file){
            Ok(contents) => contents,
            Err(e) => {
                println!("Could not read the markdown file: {}", e);
                continue;
            }
        };
        let mut extractor = Extractor::new(&contents);
        extractor.select_by_terminator("---");
        extractor.strip_prefix("---");
        let settings_yaml: String = extractor.extract();
        let settings = match serde_yaml::from_str(&settings_yaml) {
            Ok(settings) => (settings),
            Err(_) => {
                Page {
                    title: "".to_string(),
                    description: "".to_string(),
                    language: "".to_string(),
                    author: "Sedum".to_string(),
                    list: "".to_string(),
                }
            }
        };
        if settings.list == "True" {
            let title_string;
            if !settings.title.is_empty() {
                title_string = settings.title.to_string();
            } else {
                let title_file = source_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                title_string = String::from(title_file);
            }
            list_html = format!(
                "{}<li><a href='{}'>{}</a></li>",
                list_html,
                relative.display(),
                title_string
            );
            list_count += 1;
        }

        list_html = format!("{}</ul>", list_html);

        let head_include = create_include("head");
        let body_include = create_include("body");

        for source_file in &source_files {
            let relative = source_file.strip_prefix(&source).expect("Not a prefix");
            let mut target = destination.join(relative);
            let contents = match fs::read_to_string(source_file){
                Ok(contents) => contents,
                Err(e) => {
                    println!("Could not read the markdown file: {}", e);
                    continue;
                }
            };
            let mut extractor = Extractor::new(&contents);
            extractor.select_by_terminator("---");
            extractor.strip_prefix("---");
            let settings_yaml: String = extractor.extract();

            let (content, settings) = match serde_yaml::from_str(&settings_yaml) {
                Ok(settings) => (extractor.remove().trim(), settings),
                Err(_) => (
                    contents.as_str(),
                    Page {
                        title: "".to_string(),
                        description: "".to_string(),
                        language: "".to_string(),
                        author: "Sedum".to_string(),
                        list: "".to_string(),
                    },
                ),
            };

            let parser = Parser::new_ext(content, pulldown_cmark_options);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);
            let mut lang_string = String::new();
            if !settings.language.is_empty() {
                lang_string = format!(" lang='{}'", settings.language);
            }
            let title_string;
            if !settings.title.is_empty() {
                title_string = settings.title.to_string();
            } else {
                let title_file = source_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                title_string = String::from(title_file);
            }
            let mut description_string = String::new();
            if !settings.description.is_empty() {
                description_string = format!(" lang='{}'", settings.description);
            }
            let mut page = format!("<!DOCTYPE html>\n<html{}>{}<head>\n<meta charset='utf-8'>\n<title>{}</title>\n<meta name='description' content='{}'>\n<meta name='author' content='{}'>\n<meta name='viewport' content='width=device-width, initial-scale=1'>\n<link rel='stylesheet' href='/main.css'>\n</head>\n<body>\n{}\n{}</body>\n</html>", lang_string, head_include, &title_string, description_string, settings.author, html_content, body_include);
            if list_count != 0 {
                page = str::replace(&page, "|LIST|", &list_html);
            } else {
                page = str::replace(&page, "|LIST|", "");
            }
            let prefix = &target.parent().unwrap();
            fs::create_dir_all(prefix).unwrap();
            target.set_extension("html");
            let mut target_file = match File::create(target) {
                Ok(target_file) => target_file,
                Err(e) =>{
                    println!("Could not create target file: {}", e);
                    continue;
                }
            };
            write!(&mut target_file, "{}", page).expect("Could not write to target file.");
        }
    }
}

fn parse_config() -> (PathBuf, PathBuf) {
    let mut args = env::args();
    let _ = args.next().unwrap_or_default();
    let source = args.next().unwrap_or_else(|| "source".to_string());
    let destination = args.next().unwrap_or_else(|| "result".to_string());
    (source.into(), destination.into())
}

fn create_include(name: &str) -> String {
    let (source, _) = parse_config();
    let mut include_path = source;
    include_path.push(name);
    include_path.push(".include");
    let include: String = fs::read_to_string(include_path).unwrap_or_default();
    include
}
