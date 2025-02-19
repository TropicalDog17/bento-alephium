use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use super::AppState;
use crate::api::handler::{BlockApiModule, EventApiModule, TransactionApiModule};
use crate::{db::new_db_pool, models::block::BlockModel};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // build our application with a route
    let db_pool = new_db_pool(&database_url, None).await.unwrap();
    let state = AppState { db: db_pool };
    let app = configure_api().with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
