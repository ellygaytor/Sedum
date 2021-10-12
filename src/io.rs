use std::{fs, path::PathBuf};
use extract_frontmatter::Extractor;
use structopt::StructOpt;

use crate::options;
use crate::page::Page;

pub fn copy_file_to_target(path: PathBuf) {
    let opt = options::Opt::from_args();
    let relative = match path.strip_prefix(&opt.source) {
        Ok(path) => path,
        Err(_) => {
            println!("Could not remove prefix. Skipping this file.");
            return;
        }
    };
    let target = opt.destination.join(relative);
    let prefix = &target.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    match fs::copy(path, target) {
        Ok(_) => (),
        Err(e) => {
            println!("Could not copy file: {}", e)
        }
    };
}

pub fn list_files(source_files: &[PathBuf]) -> (String, i64) {
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