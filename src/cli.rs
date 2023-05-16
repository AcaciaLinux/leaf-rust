use leaf::error::*;
use std::path::PathBuf;

pub use clap::{command, ArgAction, Parser};
pub use leaf::config::Config;

use crate::commands::{exec_command, Commands};

/// Represents the command line arguments passed to leaf
#[derive(Parser, Debug)]
#[command(name = "leaf")]
#[command(author = "Max Kofler and the AcaciaLinux developers")]
#[command(version = "0.1.0")]
#[command(
    about = "Leaf - The AcaciaLinux package manager",
    long_about = "Leaf is an easy to use package manager that helps you manage and install packages on your system."
)]
pub struct Cli {
    /// Specify a custom config file
    #[arg(short = 'C', long)]
    pub config: Option<PathBuf>,

    /// Specify the root to operate on
    #[arg(short = 'R', long)]
    pub root: Option<PathBuf>,

    /// Specify the directory for download caching
    #[arg(long)]
    pub download_cache: Option<PathBuf>,

    /// Turn off progress bar rendering
    #[arg(long = "noProgress", action = ArgAction::SetFalse)]
    pub progress: Option<bool>,

    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Executes the supplied command line arguments
/// # Arguments
/// * `cli` - The arguments to process
pub fn execute(cli: &Cli) -> Result<(), LError> {
    exec_command(cli)
}

impl From<&Cli> for Config {
    fn from(value: &Cli) -> Self {
        Config {
            root: value.root.clone(),
            render_bar: value.progress.unwrap_or(true),
            ..Default::default()
        }
    }
}

impl Cli {
    /// Takes in a mutable config to apply the configurations from this command line arguments.
    /// This allows the command line arguments to override the config file options.
    /// # Arguments
    /// *  `config` - The config to modify
    pub fn update_config(&self, config: &mut Config) {
        config.root = match &self.root {
            None => config.root.clone(),
            Some(root) => Some(root.clone()),
        };

        config.download_dir = match &self.download_cache {
            None => config.download_dir.clone(),
            Some(dir) => Some(dir.clone()),
        };

        config.render_bar = match self.progress {
            None => config.render_bar,
            Some(progress) => progress,
        };
    }
}
