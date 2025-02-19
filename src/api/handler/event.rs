use axum::extract::{Query, State};
use axum::Json;

use crate::api::AppState;
use crate::api::Pagination;
use crate::repository::{get_events, get_events_by_contract, get_events_by_tx};
use axum::response::IntoResponse;

pub struct EventApiModule;

impl EventApiModule {
    pub fn register() -> axum::Router<crate::api::AppState> {
        axum::Router::new()
            .route("/events", axum::routing::get(get_events_handler))
            .route("/events/contract", axum::routing::get(get_events_by_contract_handler))
            .route("/events/tx", axum::routing::get(get_events_by_tx_id_handler))
    }
}

pub async fn get_events_handler(
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let event_models =
        get_events(db, pagination.limit.unwrap_or(i64::MAX), pagination.offset.unwrap_or(0))
            .await
            .unwrap();
    Json(event_models)
}

pub async fn get_events_by_contract_handler(
    address: Query<String>,
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let event_models = get_events_by_contract(
        db,
        address.to_string(),
        pagination.limit.unwrap_or(i64::MAX),
        pagination.offset.unwrap_or(0),
    )
    .await
    .unwrap();
    Json(event_models)
}

pub async fn get_events_by_tx_id_handler(
    tx_id: Query<String>,
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db = state.db;

    let event_models = get_events_by_tx(
        db,
        tx_id.to_string(),
        pagination.limit.unwrap_or(i64::MAX),
        pagination.offset.unwrap_or(0),
    )
    .await
    .unwrap();
    Json(event_models)
}
