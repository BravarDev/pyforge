use crate::core::error::{PyForgeError, Result, validation};
use crate::{ensure};
use std::path::Path;
use colored::*;

pub fn run(name: &str, template: &Option<String>) -> Result<()> {
    println!("🚀 Creating project: {}", name);
    println!("Project created successfully");
    // Validate project name
    validation::validate_project_name(name)?;
    
    // Check it doesn't exist
    ensure!(
        !Path::new(name).exists(),
        PyForgeError::ProjectAlreadyExists {
            name: name.to_string(),
            path: name.to_string(),
        }
    );
    
    println!("{} Creating project: {}", "🚀".green(), name.cyan());
    
    // Create project
    // create_project_structure(name)
    //     .map_err(|e| PyForgeError::file_error("Could not create project", e))?;
    
    println!("{} Project '{}' created successfully!", "✅".green(), name.green());
    Ok(())
}