use std::path::PathBuf;

use camino::Utf8PathBuf;
/// Imports
use miette::Diagnostic;
use thiserror::Error;

/// Defines a nest error
#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(nest::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(nest::toml_de_error))]
    TomlDeError(#[from] toml::de::Error),

    #[error(transparent)]
    #[diagnostic(code(nest::toml_ser_error))]
    TomlSerError(#[from] toml::ser::Error),

    #[error("url `{0}` is invalid.")]
    #[diagnostic(code(nest::invalid_url))]
    InvalidUrl(String),

    #[error("failed to clone repo `{0}` due to error: `{1}`")]
    #[diagnostic(code(nest::clone_failure))]
    FailedToCloneRepo(String, git2::Error),

    #[error("no `nest.toml` was found in `{0}`")]
    #[diagnostic(
        code(nest::no_config),
        help("if you want to create new project, use `nest new`")
    )]
    NoConfig(Utf8PathBuf),

    #[error("path `{0}` has non-utf8 chars")]
    #[diagnostic(code(nest::non_ut8_path))]
    NonUtf8Path(PathBuf),

    #[error("egg `{0}` depends on its own")]
    #[diagnostic(code(nest::circular_dependency))]
    CircularDependency(String),
}
