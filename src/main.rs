mod api;
mod models;
mod utils;

use api::*;
use clap::{arg, command};
use colored::Colorize;
use std::{env, process};

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args = command!()
        .arg(arg!(
            -k --key <KEY> "API Key for Google AI Studio."
        ))
        .arg(arg!(
            -v --verbose <OPTION> "Verbosity level for outputs (normal, metadata, token)."
        ))
        .get_matches();

    let api_key = args
        .get_one::<String>("key")
        .map(String::from)
        .or_else(|| env::var("API_KEY").ok())
        .unwrap_or_else(|| {
            eprintln!("{}", "[!] API_KEY must be set via the -k argument or in the environment variable".red());
            process::exit(1);
        });

    let verbosity = args
        .get_one::<String>("verbose")
        .map(String::from)
        .unwrap_or("normal".to_string());

    if !["normal", "debug", "token"].contains(&verbosity.as_str()) {
        eprintln!("{} Verbosity is invalid, exiting.", "[!]".red());
        process::exit(1);
     }

    println!("{} Running in {} mode.", "[o]".green(), verbosity);

    match test_api(&api_key).await {
        Ok(_response) => { println!("{} Ready to chat.", "{o]".green()); }
        Err(e) => { 
            eprintln!("{} API Error: {}", "[!]".red(), e); 
            process::exit(1);
        }
    }

    loop {
        print!("{}", "Prompt > ".blue());
        let input = utils::scan();

        if input.trim().is_empty() {
            println!("{} Input cannot be empty.", "[i]".yellow());
            continue;
        }

        if input.trim().eq_ignore_ascii_case("exit") {
            println!("{} Goodbye", "[o]".green());
            break;
        }

        // Call the API
        match generate_content(&api_key, &input, &verbosity).await {
            Ok(_response) => {}
            Err(e) => { eprintln!("{} API Error: {}", "[!]".red(), e); }
        }
    }
}

