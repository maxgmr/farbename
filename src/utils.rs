use std::env;

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
Total Memory:\t\t\t{}",
        TOTAL_MEMORY,
    )
}
