use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use extract_frontmatter::Extractor;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use structopt::StructOpt;
use walkdir::WalkDir;

mod io;
mod options;

#[derive(Deserialize, Debug)]
struct Page {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    list: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Settings {
    #[serde(default = "default_author")]
    default_author: String,
}

fn default_author() -> String {
    "Sedum".to_string()
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_author: ("Sedum").to_string(),
        }
    }
}



fn main() {
    let mut pulldown_cmark_options = Options::empty();
    pulldown_cmark_options.insert(Options::ENABLE_STRIKETHROUGH);
    pulldown_cmark_options.insert(Options::ENABLE_TABLES);

    let opt = options::Opt::from_args();

    let (source_files, global_settings) = traverse();

    let (list_html, list_count) = list_files(&source_files);

    let head_include = create_include("head");
    let body_include = create_include("body");

    for source_file in &source_files {
        let relative = match source_file.strip_prefix(&opt.source) {
            Ok(path) => path,
            Err(_) => {
                println!("Could not remove prefix. Skipping this file.");
                continue;
            }
        };
        let mut target = opt.destination.join(relative);
        let source_contents = match fs::read_to_string(source_file) {
            Ok(source_contents) => source_contents,
            Err(e) => {
                println!("Could not read the markdown file: {}", e);
                continue;
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

        let parser = Parser::new_ext(content, pulldown_cmark_options);
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
            None => String::from(&global_settings.default_author),
            Some(author) => author,
        };
        let mut page = format!("<!DOCTYPE html>\n<html{}>{}<head>\n<meta charset='utf-8'>\n<title>{}</title>\n{}\n<meta name='author' content='{}'>\n<meta name='viewport' content='width=device-width, initial-scale=1'>\n<link rel='stylesheet' href='/main.css'>\n</head>\n<body>\n{}\n{}</body>\n</html>", lang_string, head_include, &title_string, description_string, author_string, html_content, body_include);
        if list_count == 0 {
            page = str::replace(&page, "|LIST|", "");
        } else {
            page = str::replace(&page, "|LIST|", &list_html);
        }
        let prefix = &target.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        target.set_extension("html");
        let mut target_file = match File::create(target) {
            Ok(target_file) => target_file,
            Err(e) => {
                println!("Could not create target file: {}", e);
                continue;
            }
        };
        match write!(&mut target_file, "{}", page) {
            Ok(_) => (),
            Err(e) => {
                println!("Could not write to target file: {}", e);
                continue;
            }
        };
    }
}

fn create_include(name: &str) -> String {
    let opt = options::Opt::from_args();
    let mut include_path = opt.source;
    include_path.push(name);
    include_path.push(".include");
    let include: String = fs::read_to_string(include_path).unwrap_or_default();
    include
}

fn traverse() -> (Vec<PathBuf>, Settings) {
    let opt = options::Opt::from_args();

    let mut source_files: Vec<PathBuf> = Vec::new();

    let mut global_settings: Settings = Default::default();

    for entry in WalkDir::new(&opt.source).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            match entry.path().extension().and_then(OsStr::to_str) {
                Some("md") => source_files.push(entry.path().to_path_buf()),
                Some("include") => (),
                None => {
                    match entry
                        .path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                    {
                        "settings" => {
                            let settings_contents = match fs::read_to_string(entry.path()) {
                                Ok(source_contents) => source_contents,
                                Err(e) => {
                                    println!("Could not read the settings file: {}", e);
                                    continue;
                                }
                            };
                            if let Ok(settings) = serde_yaml::from_str(&settings_contents) {
                                global_settings = settings
                            }
                        }
                        _ => io::copy_file_to_target(entry.path().to_path_buf()),
                    }
                }
                _ => io::copy_file_to_target(entry.path().to_path_buf()),
            }
        }
    }

    (source_files, global_settings)
}

fn list_files(source_files: &[PathBuf]) -> (String, i64) {
    let opt = options::Opt::from_args();

    let mut list_html = String::from("<ul>");
    let mut list_count: i64 = 0;
    for source_file in source_files {
        let relative = match source_file.strip_prefix(&opt.source) {
            Ok(path) => path,
            Err(_) => {
                println!("Could not remove prefix. Skipping this file.");
                continue;
            }
        };
        let source_contents = match fs::read_to_string(source_file) {
            Ok(source_contents) => source_contents,
            Err(e) => {
                println!("Could not read the markdown file: {}", e);
                continue;
            }
        };
        let mut extractor = Extractor::new(&source_contents);
        extractor.select_by_terminator("---");
        extractor.strip_prefix("---");
        let settings_yaml: String = extractor.extract();
        let settings = match serde_yaml::from_str(&settings_yaml) {
            Ok(settings) => (settings),
            Err(_) => Page {
                title: None,
                description: None,
                language: None,
                author: None,
                list: None,
            },
        };
        if let Some(list) = settings.list {
            if list == "True" {
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
                list_html = format!(
                    "{}<li><a href='{}'>{}</a></li>",
                    list_html,
                    relative.display(),
                    title_string
                );
                list_count += 1;
            }
        };
    }
    list_html = format!("{}</ul>", list_html);

    (list_html, list_count)
}