use std::fs;

use structopt::StructOpt;

use crate::options;

pub fn create_include(name: &str) -> String {
    let opt = options::Opt::from_args();
    let mut include_path = opt.source;
    include_path.push(name);
    include_path.push(".include");
    let include: String = fs::read_to_string(include_path).unwrap_or_default();
    include
}