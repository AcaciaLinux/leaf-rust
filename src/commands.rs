mod install;
mod update;

use super::cli::Cli;
use clap::Subcommand;
use leaf::error::LError;

/// An enum to represent all the possible command for leaf to execute.
#[derive(Subcommand, Debug)]
pub enum Commands {
    Update(update::UpdateCommand),
    Install(install::InstallCommand),
}

/// Takes the command line arguments and executes the matching command
pub fn exec_command(cli: &Cli) -> Result<(), LError> {
    use Commands::*;

    match &cli.command {
        Update(update) => update.execute(cli)?,
        Install(install) => install.execute(cli)?,
    }
    Ok(())
}
