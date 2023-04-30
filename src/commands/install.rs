use crate::config::load_config_file;
use crate::*;
use clap::Args;
use leaf::actions::install;
use leaf::error::LError;

/// Install the specified packages to the system
#[derive(Args, Debug)]
pub struct InstallCommand {
    packages: Vec<String>,
}

impl InstallCommand {
    /// Executes the install command respecting the supplied cli
    /// # Arguments
    /// * `cli` - The command line arguments to respect while executing
    pub fn execute(&self, cli: &Cli) -> Result<(), LError> {
        let mut config_file = load_config_file(cli)?;
        cli.update_config(&mut config_file.leaf);

        match install(
            &config_file.leaf,
            &self.packages,
            &mut config_file.mirrors.unwrap_or(vec![]),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
