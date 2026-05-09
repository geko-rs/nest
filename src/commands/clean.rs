/// Imports
use crate::{bail, errors::Error};
use camino::Utf8PathBuf;
use cliclack::confirm;
use owo_colors::OwoColorize;
use std::{env, fs};

/// Cleans `$cwd/eggs` directory
pub fn run() {
    // Preparing path
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e))
    .join("eggs");

    // Asking for confirmation
    match confirm(format!(
        "are you sure you wanna to clean `{}` directory?",
        path
    ))
    .interact()
    {
        // If user answered `yes`
        Ok(true) => {
            // Cleaning
            if let Err(e) = fs::remove_dir_all(path) {
                bail!(Error::IoError(e))
            }
        }
        // If not
        _ => println!("{} cancelled", "clean".yellow()),
    }
}
