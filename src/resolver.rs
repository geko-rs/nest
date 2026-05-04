/// Imports
use crate::{
    bail,
    config::{self, Egg},
    errors::Error,
    git,
};
use camino::Utf8PathBuf;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::{env, time::Duration};
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
pub fn prepare(path: &Utf8PathBuf, url: &str) -> (String, Egg) {
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
    (name, config)
}

/// Performs resolution of egg dependencies
pub fn resolve(resolved: &mut Vec<String>, egg: Egg) {
    // Preparing eggs directory
    let path = Utf8PathBuf::from_path_buf(
        env::current_dir().unwrap_or_else(|e| bail!(Error::IoError(e))),
    )
    .map_err(|path| Error::NonUtf8Path(path))
    .unwrap_or_else(|e| bail!(e))
    .join("eggs");

    // Iterating over dependences
    for url in egg.dependencies {
        // Preparing dependency
        let (name, config) = prepare(&path, &url);

        // Checking if not already resolved
        if !resolved.contains(&name) {
            bail!(Error::CircularDependency(name))
        } else {
            // Resolving its dependencies
            resolve(resolved, config)
        }
    }
}
