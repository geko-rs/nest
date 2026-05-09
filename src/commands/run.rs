/// Imports
use crate::{bail, config, errors::Error, resolver};
use camino::Utf8PathBuf;
use std::{
    env,
    process::{Command, Stdio},
};

/// Runs the project
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

    // Checking main file specified
    match config.main {
        // If main file specified
        Some(main) => {
            // Running `geko main.gk`
            match Command::new("geko")
                .arg(main)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
            {
                // If process executed successfully
                Ok(_) => {}
                // If process executed with error
                Err(err) => bail!(Error::IoError(err)),
            }
        }
        // If not
        None => bail!(Error::NoMainFileSpecified),
    }
}
