use std::path::PathBuf;

use clap::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Sedum is a static site generator. Pass in markdown files and it will automatically generate HTML."
)]

/// The command line options
pub struct Opt {
    /// The directory containing the markdown files to be converted to HTML
    #[structopt(
        help = "The directory containing the markdown files to be converted to HTML.",
        default_value = "source",
        parse(from_os_str)
    )]
    pub source: PathBuf,

    /// The directory in which to place the HTML. Does not need to exist, Sedum will make it automatically
    #[structopt(
        help = "The directory in which to place the HTML. Does not need to exist, Sedum will make it automatically.",
        default_value = "result",
        parse(from_os_str)
    )]
    pub destination: PathBuf,

    /// (Optional) Include a timestamp in all generated HTML
    #[structopt(short = 't', long = "timestamp")]
    pub timestamp: bool,
}
