use std::path::Path;

use rusqlite::{Connection, OpenFlags};

/// Load the SQLite colours database from the given file path.
pub fn load_db<T: AsRef<Path>>(path: T) -> Result<Connection, rusqlite::Error> {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
}
