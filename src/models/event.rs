use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, AsChangeset, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventModel {
    pub id: i32,
    pub tx_id: String,
    pub contract_address: String,
    pub event_index: i32,
    pub fields: serde_json::Value,
}
