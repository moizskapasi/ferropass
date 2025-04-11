mod models;
mod encryption;
mod password;
mod cli;
mod clipboard;

use cli::CLI;
use std::process::exit;

fn main() {
    // Create a new CLI instance
    let mut cli = CLI::new();
    
    // Run the CLI
    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}