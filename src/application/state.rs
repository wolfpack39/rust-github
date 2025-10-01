use std::sync::Arc;
use reqwest::Client;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub github_client: Client
}