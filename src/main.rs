use camino::Utf8PathBuf;
use clap::Parser;
use color_eyre::eyre;

mod arg_parser;
mod db;
mod utils;

use arg_parser::Cli;

const DB_NAME: &str = "colours.db";

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let db = db::load_db(get_db_path(&args)?)?;

    Ok(())
}

fn get_db_path(args: &Cli) -> eyre::Result<Utf8PathBuf> {
    let db_path = if let Some(path) = &args.database {
        path.clone()
    } else {
        let mut path = utils::data_dir_setup()?;
        path.push(DB_NAME);
        path
    };
    Ok(db_path)
}
