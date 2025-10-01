use std::{ net::SocketAddr, str::FromStr, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;

use crate::{api::handlers::gist_handler, application::state::SharedState};

pub async fn start(state: SharedState) {
    let router = Router::new()
        .route("/:user", get(gist_handler::get_user_gists))
        .with_state(Arc::clone(&state));

    let addr = SocketAddr::from_str(&format!("0.0.0.0:8080")).unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    // Run the server
    tracing::info!("App listening on port :8080");
    axum::serve(listener, router)
        .await
        .unwrap();
}

pub async fn root_handler(
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Json(json!({"message": "hello from exum"})))
}