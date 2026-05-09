/// Imports
use crate::{bail, errors::Error, project};
use camino::Utf8PathBuf;
use std::env;

/// Initializes new project
pub fn run() {
    // Preparing cwd
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e));

    // Preparing project
    project::prepare(&path).unwrap_or_else(|e| bail!(e));
}
