/// Imports
use crate::errors::Error;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents egg configuration
#[derive(Serialize, Deserialize)]
pub struct Egg {
    /// Vector of dependencies
    pub dependencies: Vec<String>,
}

/// Reads the config
fn read(path: &Utf8PathBuf) -> Result<Egg, Error> {
    let text = fs::read_to_string(path).map_err(|e| Error::IoError(e))?;
    toml::from_str(&text).map_err(|e| Error::TomlDeError(e))
}

/// Finds the config in specified path
fn find(path: &Utf8PathBuf) -> Result<Utf8PathBuf, Error> {
    // Getting cwd and adding `nest.toml` suffix
    let path = path.join("nest.toml");

    // Checking existence
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::NoConfig(path))
    }
}

/// Loads the config located in specified folder
pub fn load(path: &Utf8PathBuf) -> Result<Egg, Error> {
    let path = find(path)?;
    read(&path)
}

/// Generates new config in specified path
pub fn generate(mut path: Utf8PathBuf) -> Result<(), Error> {
    // Getting cwd and adding `nest.toml` suffix
    path.push("nest.toml");

    // Checking existence
    if path.exists() {
        todo!()
    }
    // If config not exists, generating it
    else {
        // Preparing config
        let config = Egg {
            dependencies: Vec::new(),
        };

        // Writing config
        fs::write(
            path,
            toml::to_string(&config)
                .map_err(|e| Error::TomlSerError(e))?,
        )
        .map_err(|e| Error::IoError(e))?;

        // Done!
        Ok(())
    }
}
