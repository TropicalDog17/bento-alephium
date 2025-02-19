use axum::extract::{Query, State};
use axum::Json;

use crate::api::Pagination;
use crate::repository::{get_block_by_hash, get_block_by_height, get_block_transactions};
use crate::{api::AppState, repository::get_blocks};
use axum::response::IntoResponse;

pub struct BlockApiModule;

impl BlockApiModule {
    pub fn register() -> axum::Router<crate::api::AppState> {
        axum::Router::new()
            .route("/blocks", axum::routing::get(get_blocks_handler))
            .route("/blocks/hash", axum::routing::get(get_block_by_hash_handler))
            .route("/blocks/height", axum::routing::get(get_block_by_height_handler))
            .route("/blocks/transactions", axum::routing::get(get_block_transactions_handler))
    }
}

pub async fn get_blocks_handler(
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let block_models =
        get_blocks(db, pagination.limit.unwrap_or(i64::MAX), pagination.offset.unwrap_or(0))
            .await
            .unwrap();
    Json(block_models)
}

pub async fn get_block_by_hash_handler(
    hash: Query<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let block_model = get_block_by_hash(db, &hash).await.unwrap();
    Json(block_model)
}

pub async fn get_block_by_height_handler(
    height: Query<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let block_model = get_block_by_height(db, *height).await.unwrap();
    Json(block_model)
}

pub async fn get_block_transactions_handler(
    hash: Query<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let transaction_models = get_block_transactions(db, hash.to_string()).await.unwrap();
    Json(transaction_models)
}
