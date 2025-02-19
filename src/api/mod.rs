use crate::db::DbPool;
use serde::Deserialize;
use std::sync::Arc;

pub mod error;
pub mod handler;
pub mod index;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbPool>,
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
