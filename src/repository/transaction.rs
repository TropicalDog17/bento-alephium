use std::sync::Arc;

use diesel::insert_into;

use crate::{db::DbPool, models::transaction::TransactionModel};
use anyhow::Result;
use diesel_async::RunQueryDsl;

/// Insert txs into the database.
pub async fn insert_txs_to_db(db: Arc<DbPool>, txs: Vec<TransactionModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::transactions::table).values(&txs).execute(&mut conn).await?;
    Ok(())
}
