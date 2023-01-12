use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    about = "Sedum is a static site generator. Pass in markdown files and it will automatically generate HTML."
)]

/// The command line arguments
pub struct Opt {
    /// The directory containing the markdown files to be converted to HTML
    #[arg(
        help = "The directory containing the markdown files to be converted to HTML.",
        default_value = "source",
    )]
    pub source: PathBuf,

    /// The directory in which to place the HTML. Does not need to exist, Sedum will make it automatically
    #[arg(
        help = "The directory in which to place the HTML. Does not need to exist, Sedum will make it automatically.",
        default_value = "result",
    )]
    pub destination: PathBuf,

    /// (Optional) Include a timestamp in all generated HTML
    #[arg(short = 't', long = "timestamp")]
    pub timestamp: bool,
}
