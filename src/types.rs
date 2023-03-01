use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
    pub message: String,
}

impl JsonResponse {
    pub fn to_string(&self) -> Result<String> {
        match serde_json::to_string(self) {
            Ok(json) => Ok(json),
            Err(error) => bail!(error),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CodePasteInput {
    pub code: String,
    pub language: String,
}
