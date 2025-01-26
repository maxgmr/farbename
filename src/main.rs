use clap::Parser;
use color_eyre::eyre;

mod arg_parser;
mod utils;

use arg_parser::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    println!("Hello, world!");
    Ok(())
}
