use crate::cli::Cli;
use leaf::{config::ConfigFile, error::LError};
use std::path::PathBuf;

/// Utility function to load a leaf configuration from a file respecting the command line
/// arguments.
/// # Arguments
/// * `cli` - The cli to refer for configuration options.
pub fn load_config_file(cli: &Cli) -> Result<ConfigFile, LError> {
    match &cli.config {
        None => leaf::config::load_config_file_from_root(
            cli.root.clone().unwrap_or(PathBuf::from("/")).as_path(),
        ),
        Some(config) => leaf::config::parse_config_file(config),
    }
}
