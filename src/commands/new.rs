/// Imports
use crate::{bail, errors::Error, project};
use camino::Utf8PathBuf;
use std::env;

/// Creates new project
pub fn run(name: String) {
    // Preparing path
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e))
    .join(name);

    // Creating project
    project::create(path).unwrap_or_else(|e| bail!(e));
}
