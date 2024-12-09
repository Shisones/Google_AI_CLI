use serde::Serialize;

// Request structures
#[derive(Serialize)]
pub struct RequestPayload {
    pub contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Part {
    pub text: String,
}

impl RequestPayload {
    pub fn new(text: String) -> Self {
        RequestPayload {
            contents: vec![Content {
                parts: vec![Part { text }],
            }],
        }
    }
}

