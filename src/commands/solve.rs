/// Imports
use crate::{bail, config, errors::Error, resolver};
use camino::Utf8PathBuf;
use std::env;

/// Runs the dependencies solving
pub fn run() {
    // Preparing cwd
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e));

    // Reading config
    let config = config::load(&path).unwrap_or_else(|e| bail!(e));

    // Resolving dependencies
    resolver::resolve(&path, &config);
}
