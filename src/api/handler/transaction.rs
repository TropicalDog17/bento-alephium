use axum::extract::{Query, State};
use axum::Json;

use crate::api::error::AppError;
use crate::api::AppState;
use crate::api::Pagination;
use crate::repository::{get_tx_by_hash, get_txs};
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
) -> Result<impl IntoResponse, AppError> {
    let db = state.db;

    let tx_models =
        get_txs(db, pagination.limit.unwrap_or(i64::MAX), pagination.offset.unwrap_or(0)).await?;
    Ok(Json(tx_models))
}

pub async fn get_tx_by_hash_handler(
    hash: Query<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let db = state.db;

    let tx_model = get_tx_by_hash(db, &hash).await?;
    Ok(Json(tx_model))
}
