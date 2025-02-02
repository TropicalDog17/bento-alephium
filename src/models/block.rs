use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockModel {
    pub hash: String,
    pub timestamp: NaiveDateTime,
    pub chain_from: i64,
    pub chain_to: i64,
    pub height: i64,
    pub deps: Option<Vec<Option<String>>>,
    pub nonce: String,
    pub version: String,
    pub dep_state_hash: String,
    pub txs_hash: String,
    pub tx_number: i64,
    pub target: String,
    pub ghost_uncles: serde_json::Value,
}
