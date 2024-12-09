use serde::Deserialize;

// Response structures
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct ApiResponse {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: UsageMetadata,
    pub model_version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: ContentResponse,
    pub finish_reason: String,
    pub citation_metadata: Option<CitationMetadata>,
    pub avg_logprobs: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentResponse {
    pub parts: Vec<PartResponse>,
    pub role: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartResponse {
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    pub start_index: usize,
    pub end_index: usize,
    pub uri: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: usize,
    pub candidates_token_count: usize,
    pub total_token_count: usize,
}

