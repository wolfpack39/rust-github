use gist_api::application::app;

#[tokio::main]
async fn main() {
    // Initializing tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    app::run().await;
}
