use std::env;

use camino::Utf8PathBuf;

use crate::errors::Error;

mod commands;
mod config;
mod errors;
mod git;
mod macros;
mod resolver;

fn main() {
    // Current working directory
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e));
    let egg = config::load(&path).unwrap_or_else(|e| bail!(e));
    resolver::resolve(&mut Vec::new(), egg);
}
