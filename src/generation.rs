use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    time::SystemTime,
    env,
};

use extract_frontmatter::{config::Splitter, Extractor};
use pulldown_cmark::{html, Parser as PulldownParser};
use clap::Parser;

use crate::{
    options,
    structs::{Constants, Page, PageUnwrapped},
};

use epoch_converter::epoch_units;

/// Get the system time in seconds since epoch
fn get_time() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Get a string containing the contents of (source directory)/[passed in name].include
pub fn create_include(name: &str) -> String {
    let opt = options::Opt::parse();
    let mut include_path = opt.source;
    include_path.push(format!("{}.include", name));
    let include: String = fs::read_to_string(include_path).unwrap_or_default();
    include
}

/// Process dynamic replacements for the passed in HTML
fn dynamic_replace(page: String, constants: &Constants, page_unwrapped: &PageUnwrapped) -> String {
    let mut page: String = page;
    if constants.list_count == 0 {
        page = str::replace(&page, "|LIST|", "");
    } else {
        page = str::replace(&page, "|LIST|", &constants.list_html);
    }
    page = str::replace(&page, "|TIMESTAMP|", format!("{}", get_time()).as_str());
    page = str::replace(
        &page,
        "|COPYRIGHT|",
        format!(
            "© {} {}",
            epoch_units(get_time()).years,
            page_unwrapped.author_string
        )
        .as_str(),
    );
    page
}

/// Generate a complete HTML document from the passed in markdown
pub fn generate_html(source_file: &Path, constants: &Constants) {
    let relative = if let Ok(path) = source_file.strip_prefix(&constants.opt.source) {
        path
    } else {
        println!("Could not remove prefix. Skipping this file.");
        return;
    };
    let mut target = constants.opt.destination.join(relative);
    let source_contents = match fs::read_to_string(source_file) {
        Ok(source_contents) => source_contents,
        Err(e) => {
            println!("Could not read the markdown file: {}", e);
            return;
        }
    };

    let (settings_yaml, markdown_content) =
        Extractor::new(Splitter::EnclosingLines("---")).extract(&source_contents);

    let (content, settings) = match serde_yaml::from_str(&settings_yaml) {
        Ok(settings) => (markdown_content.trim(), settings),
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

    let parser = PulldownParser::new_ext(content, constants.pulldown_cmark_options);
    let mut html_content = String::default();
    html::push_html(&mut html_content, parser);
    let page_unwrapped = PageUnwrapped {
        lang_string: match settings.language {
            None => String::from(""),
            Some(lang) => format!(" lang='{}'", lang),
        },
        title_string: match settings.title {
            None => String::from(
                source_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ),
            Some(title) => title,
        },
        description_string: match settings.description {
            None => String::from(""),
            Some(description) => format!("<meta name='description' content='{}'>", description),
        },
        author_string: match settings.author {
            None => String::from(&constants.global_settings.default_author),
            Some(author) => author,
        },
        timestamp_string: if constants.opt.timestamp {
            format!(
                "\n<!--\nGenerated at {} seconds since epoch.\n-->\n",
                get_time()
            )
        } else {
            String::default()
        },
    };
    let mut page = format!("<!DOCTYPE html>\n<html{}>{}{}<head>\n<meta charset='utf-8'>\n<title>{}</title>\n{}\n<meta name='author' content='{}'>\n<meta name='viewport' content='width=device-width, initial-scale=1'>\n<link rel='stylesheet' href='/main.css'>\n</head>\n<body>\n{}\n{}</body>\n</html>", page_unwrapped.lang_string, constants.head_include, page_unwrapped.timestamp_string, &page_unwrapped.title_string, page_unwrapped.description_string, page_unwrapped.author_string, html_content, constants.body_include);
    page = dynamic_replace(page, constants, &page_unwrapped);
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

/// If flag specified, generate a metadata file
/// This file should contain information such as the time and date
/// The operating system and the version of the program
/// This file should be in the same directory as the generated HTML
/// This file should be named metadata.txt
pub fn generate_metadata(opt: &options::Opt) {
    if opt.metadata {
        let mut metadata_path = opt.destination.clone();
        metadata_path.push("metadata.txt");
        let mut metadata_file = match File::create(metadata_path) {
            Ok(metadata_file) => metadata_file,
            Err(e) => {
                println!("Could not create metadata file: {}", e);
                return;
            }
        };
        let metadata = format!(
            "Generated at {} seconds since epoch. Generated on {} using {} {}.", get_time(), env::consts::OS, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        match write!(&mut metadata_file, "{}", metadata) {
            Ok(_) => (),
            Err(e) => {
                println!("Could not write to metadata file: {}", e);
            }
        };
    }
} 
