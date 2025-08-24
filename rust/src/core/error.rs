use std::fmt;
use std::io;
use thiserror::Error;
use std::error::Error; 
use colored::*;

/// Main PyForge errors
#[derive(Error, Debug)]
pub enum PyForgeError {
    // === I/O ERRORS ===
    #[error("File error: {message}")]
    FileError { 
        message: String,
        #[source]
        source: Option<io::Error>,
    },
    
    #[error("Directory '{path}' not found")]
    DirectoryNotFound { path: String },
    
    #[error("Cannot write to '{path}': {reason}")]
    PermissionDenied { path: String, reason: String },
    
    // === PROJECT ERRORS ===
    #[error("Project '{name}' already exists at '{path}'")]
    ProjectAlreadyExists { name: String, path: String },
    
    #[error("No valid Python project detected in current directory")]
    NotAPythonProject,
    
    #[error("Invalid configuration file: {file}")]
    InvalidConfig { 
        file: String,
        #[source] 
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    // === COMMAND ERRORS ===
    #[error("Command '{command}' failed with exit code {code}")]
    CommandFailed { command: String, code: i32 },
    
    #[error("Command not found: '{command}'")]
    CommandNotFound { command: String },
    
    #[error("Timeout executing: '{command}' (expected {timeout}s)")]
    CommandTimeout { command: String, timeout: u64 },
    
    // === VALIDATION ERRORS ===
    #[error("Invalid project name: '{name}'. {reason}")]
    InvalidProjectName { name: String, reason: String },
    
    #[error("Unsupported Python version: {version}")]
    UnsupportedPythonVersion { version: String },
    
    #[error("Template '{template}' not found")]
    TemplateNotFound { template: String },
    
    // === NETWORK ERRORS ===
    #[error("Network error: {message}")]
    NetworkError { 
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },
    
    #[error("Failed to download from '{url}': {status}")]
    DownloadFailed { url: String, status: String },
    
    // === PARSING ERRORS ===
    #[error("Error parsing {file_type}: {message}")]
    ParseError { file_type: String, message: String },
    
    #[error("Invalid JSON in '{file}': {message}")]
    InvalidJson { file: String, message: String },
    
    #[error("Invalid TOML in '{file}': {message}")]
    InvalidToml { file: String, message: String },
    
    // === GENERIC ERRORS ===
    #[error("Internal error: {message}")]
    Internal { message: String },
    
    #[error("Operation cancelled by user")]
    UserCancelled,
    
    #[error("Feature not implemented: {feature}")]
    NotImplemented { feature: String },
}

impl PyForgeError {
    /// Create file error with context
    pub fn file_error(message: impl Into<String>, source: io::Error) -> Self {
        Self::FileError {
            message: message.into(),
            source: Some(source),
        }
    }
    
    /// Create command error
    pub fn command_failed(command: impl Into<String>, code: i32) -> Self {
        Self::CommandFailed {
            command: command.into(),
            code,
        }
    }
    
    /// Create network error
    pub fn network_error(message: impl Into<String>, source: Option<reqwest::Error>) -> Self {
        Self::NetworkError {
            message: message.into(),
            source,
        }
    }
    
    /// Create internal error quickly
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PyForgeError::NetworkError { .. } |
            PyForgeError::CommandTimeout { .. } |
            PyForgeError::UserCancelled
        )
    }
    
    /// Get appropriate exit code
    pub fn exit_code(&self) -> i32 {
        match self {
            PyForgeError::UserCancelled => 130,  // SIGINT
            PyForgeError::CommandNotFound { .. } => 127,
            PyForgeError::PermissionDenied { .. } => 126,
            PyForgeError::FileError { .. } => 2,
            PyForgeError::InvalidProjectName { .. } => 64,
            PyForgeError::NotAPythonProject => 65,
            _ => 1,
        }
    }
    
    /// Display error with colors and formatting
    pub fn display_error(&self) {
        match self {
            PyForgeError::ProjectAlreadyExists { name, path } => {
                eprintln!("{} {}", "❌ Error:".red().bold(), self);
                eprintln!("💡 {}: rm -rf {} && pyforge init {}", 
                    "Suggestion".yellow(), 
                    path.cyan(), 
                    name.green()
                );
            },
            PyForgeError::NotAPythonProject => {
                eprintln!("{} {}", "❌ Error:".red().bold(), self);
                eprintln!("💡 {}: {}", 
                    "Suggestion".yellow(), 
                    "Run 'pyforge init <name>' to create a new project".cyan()
                );
            },
            PyForgeError::CommandNotFound { command } => {
                eprintln!("{} {}", "❌ Error:".red().bold(), self);
                eprintln!("💡 {}: Install {} or make sure it's in your PATH", 
                    "Suggestion".yellow(),
                    command.cyan()
                );
            },
            PyForgeError::InvalidProjectName { name, reason } => {
                eprintln!("{} {}", "❌ Error:".red().bold(), self);
                eprintln!("💡 {}: Names must be valid Python package names", 
                    "Suggestion".yellow()
                );
                eprintln!("   {} my_project, awesome-tool, PyProject2024", 
                    "Valid examples:".green()
                );
            },
            _ => {
                eprintln!("{} {}", "❌ Error:".red().bold(), self);
                
                // Show root cause if exists
                let mut source = self.source();
                if source.is_some() {
                    eprintln!("{}", "Caused by:".yellow());
                    while let Some(err) = source {
                        eprintln!("  - {}", err.to_string().bright_black());
                        source = err.source();
                    }
                }
            }
        }
    }
}

// Automatic conversions from other error types
impl From<io::Error> for PyForgeError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Self::FileError {
                message: "File or directory not found".to_string(),
                source: Some(err),
            },
            io::ErrorKind::PermissionDenied => Self::PermissionDenied {
                path: "unknown".to_string(),
                reason: "Permission denied".to_string(),
            },
            _ => Self::FileError {
                message: "I/O error".to_string(),
                source: Some(err),
            },
        }
    }
}

impl From<reqwest::Error> for PyForgeError {
    fn from(err: reqwest::Error) -> Self {
        Self::NetworkError {
            message: "HTTP connection error".to_string(),
            source: Some(err),
        }
    }
}

impl From<serde_json::Error> for PyForgeError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError {
            file_type: "JSON".to_string(),
            message: err.to_string(),
        }
    }
}

impl From<toml::de::Error> for PyForgeError {
    fn from(err: toml::de::Error) -> Self {
        Self::ParseError {
            file_type: "TOML".to_string(),
            message: err.to_string(),
        }
    }
}

/// Custom Result type for PyForge
pub type Result<T> = std::result::Result<T, PyForgeError>;

/// Macro to create internal errors quickly
#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        PyForgeError::internal($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        PyForgeError::internal(format!($fmt, $($arg)*))
    };
}

/// Macro for validations
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !($condition) {
            return Err($error);
        }
    };
}

/// Extension to convert Option to Result
pub trait OptionExt<T> {
    fn ok_or_internal(self, message: &str) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_internal(self, message: &str) -> Result<T> {
        self.ok_or_else(|| PyForgeError::internal(message))
    }
}

/// Common validations
pub mod validation {
    use super::*;
    use regex::Regex;
    
    pub fn validate_project_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(PyForgeError::InvalidProjectName {
                name: name.to_string(),
                reason: "Name cannot be empty".to_string(),
            });
        }
        
        if name.len() > 50 {
            return Err(PyForgeError::InvalidProjectName {
                name: name.to_string(),
                reason: "Name is too long (maximum 50 characters)".to_string(),
            });
        }
        
        // Validate it's a valid Python package name
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
        if !re.is_match(name) {
            return Err(PyForgeError::InvalidProjectName {
                name: name.to_string(),
                reason: "Only letters, numbers, hyphens and underscores. Must start with letter".to_string(),
            });
        }
        
        // Check it's not a reserved word
        let reserved = ["test", "tests", "lib", "src", "build", "dist"];
        if reserved.contains(&name.to_lowercase().as_str()) {
            return Err(PyForgeError::InvalidProjectName {
                name: name.to_string(),
                reason: format!("'{}' is a reserved word", name),
            });
        }
        
        Ok(())
    }
    
    pub fn ensure_python_project() -> Result<()> {
        let indicators = ["setup.py", "pyproject.toml", "requirements.txt", "Pipfile"];
        let exists = indicators.iter().any(|&file| std::path::Path::new(file).exists());
        
        if !exists {
            Err(PyForgeError::NotAPythonProject)
        } else {
            Ok(())
        }
    }
    
    pub fn validate_python_version(version: &str) -> Result<()> {
        let valid_versions = ["3.8", "3.9", "3.10", "3.11", "3.12"];
        
        if !valid_versions.iter().any(|&v| version.starts_with(v)) {
            return Err(PyForgeError::UnsupportedPythonVersion {
                version: version.to_string(),
            });
        }
        
        Ok(())
    }
}