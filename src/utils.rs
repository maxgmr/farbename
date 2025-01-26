use std::{env, fs};

use camino::Utf8PathBuf;
use color_eyre::eyre::{self, eyre};
use directories::ProjectDirs;

const QUALIFIER: &str = "ca";
const ORGANIZATION: &str = "maxgmr";

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")\r\n",
    env!("VERGEN_SYSINFO_OS_VERSION"),
);

const TOTAL_MEMORY: &str = env!("VERGEN_SYSINFO_TOTAL_MEMORY");

/// Get the version, author info, and directories of the package.
pub fn info() -> String {
    let authors = clap::crate_authors!();
    format!(
        "{VERSION_MESSAGE}
Authors:\t\t\t{authors}
Data Path:\t\t\t{}
Total Memory:\t\t\t{}",
        data_dir().unwrap_or_default(),
        TOTAL_MEMORY,
    )
}

/// Get the path to the data directory, creating said path if necessary.
pub fn data_dir_setup() -> eyre::Result<Utf8PathBuf> {
    let dir = data_dir()?;
    if fs::metadata(&dir).is_err() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

fn data_dir() -> eyre::Result<Utf8PathBuf> {
    if let Some(path) = env_var_path("DATA") {
        // Prioritise env vars
        Ok(path)
    } else if let Some(proj_dirs) = project_dir() {
        // Second priority: XDG-standardised local directory.
        match Utf8PathBuf::from_path_buf(proj_dirs.data_local_dir().to_path_buf()) {
            Ok(utf8_path_buf) => Ok(utf8_path_buf),
            Err(_) => Err(eyre!(
                "Path to config directory is not a valid UTF-8 sequence."
            )),
        }
    } else {
        Err(eyre!("No config directory found."))
    }
}

fn project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME"))
}

fn env_var_path(suffix: &str) -> Option<Utf8PathBuf> {
    env::var(format!("{}_{}", pkg_name_constant_case(), suffix))
        .ok()
        .map(Utf8PathBuf::from)
}

fn pkg_name_constant_case() -> String {
    env!("CARGO_PKG_NAME").to_uppercase().to_string()
}
