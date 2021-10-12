use std::{fs, path::PathBuf};
use structopt::StructOpt;

use crate::options;

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
