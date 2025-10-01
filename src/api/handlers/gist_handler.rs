use axum::{extract::{Path, State}, http::{StatusCode}, response::IntoResponse, Json};

use crate::{application::state::SharedState, model::user_gist::Gist};

pub async fn get_user_gists(
    State(state): State<SharedState>,
    Path(username): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!("Received a request for gists of user {}", username);

    let url = format!("https://api.github.com/users/{}/gists", username);

    // GitHub API requires a User-Agent header
    let response = state.github_client
        .get(&url)
        .header("User-Agent", "RustAxumApp/1.0")
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Request to GitHub failed {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect to GitHub {}", e))
        })?;

    if response.status().is_success() {
        // Retrieving list of gists for the user
        tracing::info!("Sending http get request to fetcher gists for user {}", username);

        let gists: Vec<Gist> = response.json().await.map_err(|e| {
            tracing::error!("Failed to parse GitHub response {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse response {}", e))
        })?;

        tracing::info!("Successfulyl fetched {} gists for user {}", gists.len(), username);
        Ok(Json(gists).into_response())
        
    } else if response.status().as_u16() == StatusCode::NOT_FOUND.as_u16() {
        // User not found
        let status = StatusCode::NOT_FOUND;
        let message = format!("Github user {} not found", username);
        tracing::error!(message);
        Err((status, message))
    } else {
        // Handl other errors
        let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
        let error_message = format!("Uknown error. Failed to get gists from GitHub.");
        Err((status, error_message))
    }
}