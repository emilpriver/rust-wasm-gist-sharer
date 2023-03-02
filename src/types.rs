use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct CodePasteInput {
    pub code: String,
    pub language: String,
}

#[derive(Deserialize, Serialize)]
pub struct CodePaseResponse {
    pub id: String,
    pub permalink: String,
}

#[derive(Serialize)]
pub struct CodeTemplateContext {
    pub code: String,
    pub lang: String,
}
