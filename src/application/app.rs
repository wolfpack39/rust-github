use std::sync::Arc;

use reqwest::Client;

use crate::{api::server, application::state::AppState};

pub async fn run() {

    // Creating the shared application state. The tokio run, which is what Rust uses 
    let shared_state = Arc::new(AppState {
        github_client: Client::new()
    });

    server::start(shared_state).await;
}