/// Imports
use crate::{bail, config, errors::Error, resolver};
use camino::Utf8PathBuf;
use std::env;

/// Runs the project
pub fn run(url: String) {
    // Preparing cwd
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e));

    // Reading config
    let mut config = config::load(&path).unwrap_or_else(|e| bail!(e));

    // Checking for dependency
    if config.dependencies.contains(&url) {
        bail!(Error::EggAlreadyExists(url))
    }

    // Modifying config
    config.dependencies.push(url);

    // Saving new config
    config::create(&path, &config).unwrap_or_else(|e| bail!(e));

    // Resolving dependencies
    resolver::resolve(&path, &config);
}
