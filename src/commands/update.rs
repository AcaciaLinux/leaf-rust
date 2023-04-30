use crate::config::load_config_file;
use crate::*;
use clap::Args;
use leaf::actions::update;
use leaf::error::LError;

/// Represents the command `update`
#[derive(Args, Debug)]
pub struct UpdateCommand {}

impl UpdateCommand {
    /// Executes the update command respecting the supplied cli
    /// # Arguments
    /// * `cli` - The command line arguments to respect while executing
    pub fn execute(&self, cli: &Cli) -> Result<(), LError> {
        let mut config_file = load_config_file(cli)?;
        cli.update_config(&mut config_file.leaf);

        match update(&config_file.leaf, &config_file.mirrors.unwrap_or(vec![])) {
            Ok(_) => Ok(()),
            Err(e) => match e.first() {
                Some(e) => Err(e.clone()),
                None => Err(LError::new_class(leaf::error::LErrorClass::Unknown)),
            },
        }
    }
}
