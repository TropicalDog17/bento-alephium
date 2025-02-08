use crate::{
    db::DbPool,
    models::{block::BlockModel, event::EventModel, transaction::TransactionModel},
};
use anyhow::Result;
use diesel::insert_into;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

/// Insert block and events into the database.
pub async fn insert_block_and_events(
    db: Arc<DbPool>,
    block: BlockModel,
    events: Vec<EventModel>,
) -> Result<()> {
    let mut conn = db.get().await?;
    conn.transaction(|conn| {
        async move {
            insert_into(crate::schema::blocks::table).values(&block).execute(conn).await?;
            insert_into(crate::schema::events::table).values(&events).execute(conn).await?;
            diesel::result::QueryResult::Ok(())
        }
        .scope_boxed()
    })
    .await?;
    Ok(())
}

/// Insert blocks into the database.
pub async fn insert_blocks_to_db(db: Arc<DbPool>, blocks: Vec<BlockModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::blocks::table).values(&blocks).execute(&mut conn).await?;
    Ok(())
}

/// Insert events into the database.
pub async fn insert_events_to_db(db: Arc<DbPool>, events: Vec<EventModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::events::table).values(&events).execute(&mut conn).await?;
    Ok(())
}

/// Insert txs into the database.
pub async fn insert_txs_to_db(db: Arc<DbPool>, txs: Vec<TransactionModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::transactions::table).values(&txs).execute(&mut conn).await?;
    Ok(())
}
