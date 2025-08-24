
pub mod init;
pub mod build;

use crate::cli::args::Commands;
use crate::core::error::PyForgeError;

pub fn execute_command(command: Commands) -> Result<(), PyForgeError> {
    match command {
        Commands::Init { name, template } => init::run(&name, &template),
        Commands::Build => build::run(),
    }
}
