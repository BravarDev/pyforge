use colored::*;

pub fn print_welcome() {
    println!();
    println!("{}", r#"
    ____        ______                    
   / __ \__  __/ ____/___  _________ ____ 
  / /_/ / / / / /_  / __ \/ ___/ __ `/ _ \
 / ____/ /_/ / __/ / /_/ / /  / /_/ /  __/
/_/    \__, /_/    \____/_/   \__, /\___/ 
      /____/                /____/       
    "#.red().bold());

    println!("Welcome to PyForge!");
    println!("PyForge is a blazing fast, flexible, and user-friendly tool for building Python projects.");
    println!("Get started by running '{}'.", "pyforge --help".yellow().bold());
    println!("Happy coding! 🚀");
}
