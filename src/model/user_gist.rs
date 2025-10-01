use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gist {
    id: String,
    description: Option<String>,
    html_url: Url,
    files: serde_json::Value
}