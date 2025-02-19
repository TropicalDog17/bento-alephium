use std::usize;

use actix_web::http::StatusCode;
use axum::extract::{Query, State};
use axum::Json;

use crate::api::Pagination;
use crate::repository::{get_tx_by_hash, get_txs};
use crate::{api::AppState, models::block::BlockModel, repository::get_blocks};
use axum::response::IntoResponse;

pub struct TransactionApiModule;

impl TransactionApiModule {
    pub fn register() -> axum::Router<crate::api::AppState> {
        axum::Router::new()
            .route("/transactions", axum::routing::get(get_txs_handler))
            .route("/transactions/hash", axum::routing::get(get_tx_by_hash_handler))
    }
}

pub async fn get_txs_handler(
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let tx_models =
        get_txs(db, pagination.limit.unwrap_or(i64::MAX), pagination.offset.unwrap_or(0))
            .await
            .unwrap();
    Json(tx_models)
}

pub async fn get_tx_by_hash_handler(
    hash: Query<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let tx_model = get_tx_by_hash(db, &hash).await.unwrap();
    Json(tx_model)
}
