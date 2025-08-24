mod cli;
mod core;

use cli::Cli;
use core::utils;
use core::error::{PyForgeError, Result};

fn main() {
    if let Err(error) = run() {
        error.display_error();
        std::process::exit(error.exit_code());
    }
}


fn run() -> Result<()> {
    let cli = Cli::parse()
        .map_err(|e| PyForgeError::internal(format!("Error parsing arguments: {}", e)))?;
    
    match cli.command {
        Some(cmd) => cli::execute_command(cmd),
        None => {
            utils::print_welcome();
            Ok(())
        }
    }
}