mod models;
mod encryption;
mod password;
mod cli;
mod clipboard;

use cli::CLI;
use std::process::exit;

fn main() {
    let mut cli = CLI::new();
    
    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}