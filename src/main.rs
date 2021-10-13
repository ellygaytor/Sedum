use pulldown_cmark::Options;

mod generation;
mod io;
mod options;
mod structs;

fn main() {
    let mut pulldown_cmark_options = Options::empty();
    pulldown_cmark_options.insert(Options::ENABLE_STRIKETHROUGH);
    pulldown_cmark_options.insert(Options::ENABLE_TABLES);

    let (source_files, global_settings) = io::traverse();

    let (list_html, list_count) = io::list_files(&source_files);

    let head_include = generation::create_include("head");
    let body_include = generation::create_include("body");

    for source_file in &source_files {
        generation::generate_html(
            source_file,
            pulldown_cmark_options,
            &global_settings,
            &head_include,
            &body_include,
            &list_html,
            list_count,
        );
    }
}
