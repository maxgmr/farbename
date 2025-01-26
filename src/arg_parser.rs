use camino::Utf8PathBuf;
use clap::Parser;

use crate::utils;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "farbename")]
#[command(author)]
#[command(version = utils::info())]
#[command(about = "Get the name of the dominant colour in an image.")]
pub struct Cli {
    /// The path to the image file
    pub input: Utf8PathBuf,

    /// The path to a custom colours database file
    #[clap(short = 'd', long = "database")]
    pub database: Option<Utf8PathBuf>,
}
