/// Imports
use crate::{
    bail,
    config::{self, Egg, EggConfig, EggMeta},
    errors::Error,
    git,
};
use camino::Utf8PathBuf;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::{collections::HashMap, time::Duration};
use url::Url;

/// Returns name retrieved from url
fn url_to_name(url: &str) -> String {
    match Url::parse(url) {
        Ok(ok) => match ok
            .path_segments()
            .and_then(|mut segments| segments.next_back())
        {
            Some(segment) => match segment.strip_suffix(".git") {
                Some(name) => name.to_string(),
                None => segment.to_string(),
            },
            None => bail!(Error::InvalidUrl(url.to_string())),
        },
        Err(_) => bail!(Error::InvalidUrl(url.to_string())),
    }
}

/// Performs download of egg to path if it's not already downloaded
/// and returns its name and config
pub fn prepare(path: &Utf8PathBuf, url: &str) -> Egg {
    // Getting egg name
    let name = url_to_name(&url);

    // Preparing path
    let path = path.join(&name);

    // If egg is not downloaded => downloading it
    if !path.exists() {
        // Starting spinner
        let spinner = ProgressBar::new_spinner()
            .with_message(format!(
                "... downloading egg: {}",
                name.yellow()
            ))
            .with_style(ProgressStyle::default_spinner().tick_strings(&[
                "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘", "🌘", "🌗",
                "🌖", "🌕", "🌔", "🌓", "🌒",
            ]));
        spinner.enable_steady_tick(Duration::from_millis(100));

        // Clonning repository
        git::clone_repo(&path, url);

        // Ending spinner
        spinner.finish();
    }

    // Reading config
    let config = config::load(&path).unwrap_or_else(|e| bail!(e));

    // Done!
    Egg {
        meta: EggMeta {
            name,
            url: url.to_string(),
        },
        config,
        path,
    }
}

/// Performs resolution of egg dependencies
fn _resolve(
    path: &Utf8PathBuf,
    resolved: &mut HashMap<String, Egg>,
    egg: &EggConfig,
) {
    // Iterating over dependences
    for url in &egg.dependencies {
        // Preparing dependency
        let egg = prepare(&path, &url);

        // Checking if not already resolved
        match resolved.get(&egg.meta.name) {
            // If resolved already
            Some(it) => {
                // Checking url match
                if it.meta.url != egg.meta.url {
                    // If url doesn't match, raising error
                    bail!(Error::OneNameDifferentUrls(
                        egg.meta.name,
                        it.meta.url.clone(),
                        egg.meta.url
                    ))
                }
            }
            // If not resolved already
            None => {
                // Marking egg as resolved
                resolved.insert(egg.meta.name.clone(), egg.clone());

                // Resolving its dependencies
                _resolve(path, resolved, &egg.config)
            }
        }
    }
}

/// Performs resolution of egg dependencies
pub fn resolve(path: &Utf8PathBuf, egg: &EggConfig) {
    // Preparing eggs directory by joining cwd with `eggs`
    let path = path.join("eggs");

    // Resolving dependencies
    _resolve(&path, &mut HashMap::new(), egg)
}
