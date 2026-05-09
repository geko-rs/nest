/// Imports
use crate::{bail, config, errors::Error};
use camino::Utf8PathBuf;
use cliclack::{confirm, input};
use std::fs;

/// Constant main file contents
const MAIN_FILE_CONTENTS: &'static str = "putln(\"Hello, world!\")";

/// Generates `nest.toml` config file
fn generate_config(
    path: &Utf8PathBuf,
    main: &Option<String>,
) -> Result<(), Error> {
    // Checking config exists
    if path.join("nest.toml").exists() {
        // Asking for override
        if !confirm(
            "specified folder contains `nest.toml`, do you want to override it?",
        ).interact().unwrap_or_else(|e| bail!(Error::IoError(e))) {
            return Ok(())
        }
    }

    // Generating config
    config::generate(&path, main.clone())?;

    // Done!
    Ok(())
}

/// Generates main script file
fn generate_main(path: &Utf8PathBuf, main: String) -> Result<(), Error> {
    // Checking main file exists
    let path = path.join(&main);
    if path.exists() {
        // Asking for override
        if !confirm(
            format!("specified folder contains `{main}`, do you want to override it?"),
        ).interact().unwrap_or_else(|e| bail!(Error::IoError(e))) {
            return Ok(())
        }
    }

    // Generating config
    fs::write(path, MAIN_FILE_CONTENTS)?;

    // Done!
    Ok(())
}

/// Prepares project in specified folder
pub fn prepare(path: &Utf8PathBuf) -> Result<(), Error> {
    // Asking for main file
    let main: Option<String> =
        if confirm("do you want to create main script?").interact()? {
            // Asking for main file name
            Some(
                input("what should the main script be called?")
                    .placeholder("main.gk")
                    .validate(|input: &String| {
                        if input.is_empty() {
                            Err("value is required!")
                        } else if !input.ends_with(".gk") {
                            Err("name should end with `.gk` extension")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?,
            )
        } else {
            None
        };

    // Generating config
    generate_config(&path, &main)?;

    // Generating main
    if let Some(main) = main {
        generate_main(&path, main)?;
    }

    Ok(())
}

/// Creates new project in specified path if not already exists
pub fn create(path: Utf8PathBuf) -> Result<(), Error> {
    // Preparing project directory
    if path.exists() {
        // Asking for override
        if confirm(format!(
            "project folder is already exists, do you want to override it?"
        ))
        .interact()
        .unwrap_or_else(|e| bail!(Error::IoError(e)))
        {
            fs::create_dir_all(&path)?
        }
    } else {
        fs::create_dir(&path)?
    }

    // Preparing project
    prepare(&path)
}
