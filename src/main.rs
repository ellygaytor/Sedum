#![forbid(unsafe_code)]

use crate::structs::Constants;
use clap::StructOpt;
use pulldown_cmark::Options;

mod generation;
mod io;
mod options;
mod structs;

fn main() {
    //! Generate a series of HTML documents from the files in the source directory, command line options, and a settings file.
    let mut pulldown_cmark_options = Options::empty();
    pulldown_cmark_options.insert(Options::ENABLE_STRIKETHROUGH);
    pulldown_cmark_options.insert(Options::ENABLE_TABLES);

    let (source_files, global_settings) = io::traverse();
    let (list_html, list_count) = io::list_files(&source_files);
    let head_include = generation::create_include("head");
    let body_include = generation::create_include("body");
    let opt = options::Opt::from_args();

    let constants = Constants {
        list_html,
        list_count,
        opt,
        head_include,
        body_include,
        pulldown_cmark_options,
        global_settings,
    };

    for source_file in &source_files {
        generation::generate_html(source_file, &constants);
    }
}
