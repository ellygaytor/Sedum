use extract_frontmatter::Extractor;
use std::{ffi::OsStr, fs, path::PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

use crate::structs::{Page, Settings};
use crate::{options, structs};

/// Copy the passed in file to the target directory
pub fn copy_file_to_target(path: PathBuf) {
    let opt = options::Opt::from_args();
    let relative = if let Ok(path) = path.strip_prefix(&opt.source) {
        path
    } else {
        println!("Could not remove prefix. Skipping this file.");
        return;
    };
    let target = opt.destination.join(relative);
    let prefix = &target.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    match fs::copy(path, target) {
        Ok(_) => (),
        Err(e) => {
            println!("Could not copy file: {}", e);
        }
    };
}

/// Create an HTML list of all files in the source directory that have list == True
pub fn list_files(source_files: &[PathBuf]) -> (String, i64) {
    let opt = options::Opt::from_args();

    let mut list_html = String::from("<ul>");
    let mut list_count: i64 = 0;
    for source_file in source_files {
        let relative = if let Ok(path) = source_file.strip_prefix(&opt.source) {
            path
        } else {
            println!("Could not remove prefix. Skipping this file.");
            continue;
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
                    relative.with_extension("html").display(),
                    title_string
                );
                list_count += 1;
            }
        };
    }
    list_html = format!("{}</ul>", list_html);

    (list_html, list_count)
}

/// Traverse the source directory and take actions based on each file's extension
pub fn traverse() -> (Vec<PathBuf>, Settings) {
    let opt = options::Opt::from_args();

    let mut source_files: Vec<PathBuf> = Vec::new();

    let mut global_settings: Settings = structs::Settings::default();

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
                                global_settings = settings;
                            }
                        }
                        _ => copy_file_to_target(entry.path().to_path_buf()),
                    }
                }
                _ => copy_file_to_target(entry.path().to_path_buf()),
            }
        }
    }

    (source_files, global_settings)
}
