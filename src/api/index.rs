use anyhow::Result;
use axum::{routing::get, Router};

use super::AppState;
use crate::api::handler::{BlockApiModule, EventApiModule, TransactionApiModule};
use crate::config::Config;

pub async fn start(config: Config) -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create our application state
    let state = AppState { db: config.db_client };

    // create our application stack
    let app = configure_api().with_state(state);

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("{:}:{:}", host, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello Alephium Indexer API"
}

/// Setup the API routes
#[allow(clippy::let_and_return)]
pub fn configure_api() -> Router<AppState> {
    let router = Router::new()
        .merge(BlockApiModule::register())
        .merge(TransactionApiModule::register())
        .merge(EventApiModule::register())
        .route("/", get(root));

    // Users can extend with their modules:
    // router.merge(YourApiModule::register())

    router
}
