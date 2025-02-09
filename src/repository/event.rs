use std::sync::Arc;

use diesel::insert_into;

use crate::{
    db::DbPool,
    models::{block::BlockModel, event::EventModel},
};
use anyhow::Result;
use diesel_async::RunQueryDsl;

/// Insert events into the database.
pub async fn insert_events_to_db(db: Arc<DbPool>, events: Vec<EventModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::events::table).values(&events).execute(&mut conn).await?;
    Ok(())
}
