use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventModel {
    pub tx_id: String,
    pub contract_address: String,
    pub event_index: i32,
    pub fields: serde_json::Value,
}
