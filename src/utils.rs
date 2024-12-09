use colored::Colorize;

use crate::models::response::*;
use std::io::*;
use regex::Regex;

pub fn process_response(response: ApiResponse) {
    for candidate in response.candidates {
        let text = remove_markdown(&candidate.content.parts[0].text);

        print!("{}\n{}", "Gemini:".blue(), text);
        //println!("Finish Reason: {}", candidate.finish_reason);
        //println!("Finish Reason: {}", candidate.content.role);
        //
        //if let Some(metadata) = candidate.citation_metadata {
        //    for source in metadata.citation_sources {
        //        println!(
        //            "Citation Source: {} ({}-{})",
        //            source.uri, source.start_index, source.end_index
        //        );
        //    }
        //}
        //
        //if let Some(avg_logprobs) = candidate.avg_logprobs {
        //    println!("Average Logprobs: {}", avg_logprobs);
        //}
    }

    //println!("Model Version: {}", response.model_version);
    println!(
        "{}{}{}{}{}{}",
        "[i] Token Usage: Prompt = ".yellow(),
        response.usage_metadata.prompt_token_count,
        ", Candidates = ".yellow(),
        response.usage_metadata.candidates_token_count,
        ", Total = ".yellow(),
        response.usage_metadata.total_token_count
    );
}

pub fn scan() -> String {
    // Declare a stdin and return variable
    let mut input = String::new();
     
    // Read the stdin and send it to input
    stdout().flush().unwrap();
    stdin().read_line(&mut input)
        .expect(&"[!] Failed to read line".red());
    
    // Trim whitespaces
    return input.trim().to_string(); // Convert &str to String and return
}

fn remove_markdown(input: &str) -> String {
    let markdown_regex = Regex::new(r"([*_`~#\[\]\(\)!\-+>])").unwrap();
    markdown_regex.replace_all(input, "").to_string()
}

