/// Imports
use crate::{bail, errors::Error};
use camino::Utf8PathBuf;
use git2::Repository;
use url::Url;

/// Clones repository from provided url to specified path
pub fn clone_repo(path: &Utf8PathBuf, url: &str) {
    // Clonning using git
    match Url::parse(url) {
        Ok(_) => match Repository::clone(url, &path) {
            Err(err) => {
                bail!(Error::FailedToCloneRepo(url.to_string(), err))
            }
            Ok(_) => {}
        },
        Err(_) => bail!(Error::InvalidUrl(url.to_string())),
    }
}
