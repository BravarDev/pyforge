use clap::Parser;

#[derive(Parser)]
#[command(name = "pyforge")]
#[command(about = "CLI application for managing python projects", long_about = None)]
#[command(version = "1.0")]
pub struct Cli {
    /// verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// input file
    #[arg(short, long)]
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Init a new project
    Init {
        name: String,
        #[arg(long)]
        template: Option<String>,
    },
    
    Build,
}

impl Cli {
    pub fn parse() -> Result<Self, clap::Error> {
        <Self as Parser>::try_parse()
    }
}