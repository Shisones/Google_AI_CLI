mod api;
mod models;
mod utils;

use api::generate_content;
use clap::{arg, command};
use colored::Colorize;
use std::env;

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let _args = command!()
        .arg(arg!(
            -k --key <KEY> "API Key for Google AI Studio."
        ))
        .get_matches();

    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY")
        .expect(&"[!] API_KEY must be set in the .env file".red());
    
    loop {
        // Ask for input text
        print!("{}", "Prompt > ".blue());
        let input = utils::scan();
        if input == "exit" { break; }

        // Call the API
        if let Err(e) = generate_content(&api_key, &input).await {
            eprintln!("Error: {}", e);
        }
    }

}
