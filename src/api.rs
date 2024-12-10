use crate::{
    models::{
        request::*,
        response::*
    },
    utils::*
};
use reqwest::Client;

pub async fn generate_content(api_key: &str, prompt: &str, verbose: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    // Prepare the payload and execute HTTP request
    let payload = RequestPayload::new(prompt.to_string());
    let client = Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    // Process response
    process_response(response, verbose);

    Ok(())
}

