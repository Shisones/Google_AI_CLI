use crate::models::{
    request::*,
    response::*
};
use reqwest::Client;

pub async fn generate_content(api_key: &str, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    // Prepare the payload
    let payload = RequestPayload::new(prompt.to_string());

    // Create HTTP client
    let client = Client::new();

    // Send POST request
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    // Process response
    crate::utils::process_response(response);

    Ok(())
}

