use colored::Colorize;
use regex::Regex;
use std::io::*;

use crate::models::response::*;

/// Processes the API response and optionally displays detailed information.
pub fn process_response(response: ApiResponse, verbose: &str) {
    for candidate in response.candidates {
        let text = remove_markdown(&candidate.content.parts[0].text);

        println!("{}\n{}", "Gemini:".blue(), text);
        
        match verbose {
            "metadata" => {
                let print_info = |label: &str, value: String| {
                    println!("{}", format!("[i] {}: {}", label, value).yellow());
                };

                print_info("Finish Reason", candidate.finish_reason);
                print_info("Model Version", response.model_version.clone());
                print_info("Role", candidate.content.role);

                if let Some(metadata) = &candidate.citation_metadata {
                    for source in &metadata.citation_sources {
                        print_info(
                            "Citation Source",
                            format!("{} ({}-{})", source.uri, source.start_index, source.end_index),
                        );
                    }
                }

                if let Some(avg_logprobs) = candidate.avg_logprobs {
                    print_info("Average Logprobs", avg_logprobs.to_string());
                }
            },
            "token" => {
                let print_info = |label: &str, value: String| {
                    println!("{}", format!("[i] {}: {}", label, value).yellow());
                };

                print_info(
                    "Token Usage",
                    format!(
                        "Prompt = {}, Candidates = {}, Total = {}",
                        response.usage_metadata.prompt_token_count,
                        response.usage_metadata.candidates_token_count,
                        response.usage_metadata.total_token_count
                    ),
                );
            },
            _ => {},
        }
    }
}

/// Reads user input from stdin and handles potential errors gracefully.
pub fn scan() -> String {
    let mut input = String::new();
    stdout().flush().unwrap();

    // Read from stdin and handle EOF (Ctrl+D)
    match stdin().read_line(&mut input) {
        Ok(0) => {
            // EOF detected, exit gracefully
            println!("{}", "[o] Goodbye!".green());
            std::process::exit(0);
        }
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} {}",
                "[!] Failed to read input:".red(),
                e
            );
            std::process::exit(1);
        }
    }

    input.trim().to_string()
}

/// Removes Markdown formatting from the input text.
fn remove_markdown(input: &str) -> String {
    let markdown_regex = Regex::new(r"([*_`~#\[\]\(\)!\-+>])").unwrap();
    markdown_regex.replace_all(input, "").to_string()
}

