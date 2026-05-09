/// Imports
use crate::errors::Error;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents egg configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct EggConfig {
    /// Vector of dependencies
    pub dependencies: Vec<String>,

    /// Main file
    pub main: Option<String>,
}

/// Represents egg metadata
#[derive(Serialize, Deserialize, Clone)]
pub struct EggMeta {
    /// Egg name
    pub name: String,

    /// Egg url
    pub url: String,
}

/// Represents single egg
#[derive(Clone)]
pub struct Egg {
    /// Egg meta info
    pub meta: EggMeta,

    /// Defines egg config
    pub config: EggConfig,

    /// Path to downloaded egg
    pub path: Utf8PathBuf,
}

/// Reads the config
fn read(path: &Utf8PathBuf) -> Result<EggConfig, Error> {
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
pub fn load(path: &Utf8PathBuf) -> Result<EggConfig, Error> {
    let path = find(path)?;
    read(&path)
}

/// Generates new config in specified path
pub fn generate(path: Utf8PathBuf) -> Result<(), Error> {
    // Preparing config
    let config = EggConfig {
        dependencies: Vec::new(),
        main: Some("main.gk".to_string()),
    };

    // Creating `nest.toml`
    create(path, config)
}

/// Replaces or creates `nest.toml` with provided config in specified path
pub fn create(
    mut path: Utf8PathBuf,
    config: EggConfig,
) -> Result<(), Error> {
    // Getting cwd and adding `nest.toml` suffix
    path.push("nest.toml");

    // Checking existence
    if path.exists() {
        todo!()
    }
    // If config not exists, generating it
    else {
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
