use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorParameter {
    #[serde(rename(deserialize = "error"))]
    pub error: Option<String>,
    #[serde(rename(deserialize = "error_description"))]
    pub error_description: Option<String>,
}