use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Sedum is a static site generator. Pass in markdown files and it will automatically generate HTML."
)]
pub struct Opt {
    #[structopt(
        help = "The directory containing the markdown files to be converted to HTML.",
        default_value = "source",
        parse(from_os_str)
    )]
    pub source: PathBuf,

    #[structopt(
        help = "The directory in which to place the HTML. Does not need to exist, Sedum will make it automatically.",
        default_value = "result",
        parse(from_os_str)
    )]
    pub destination: PathBuf,
}