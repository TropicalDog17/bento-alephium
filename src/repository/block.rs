use std::sync::Arc;

use diesel::{insert_into, query_dsl::methods::FilterDsl, SelectableHelper};

use crate::{db::DbPool, models::block::BlockModel};
use anyhow::Result;
use diesel::ExpressionMethods;

use diesel::query_dsl::methods::{LimitDsl, OffsetDsl, SelectDsl};
use diesel_async::RunQueryDsl;

/// Insert blocks into the database.
#[allow(clippy::get_first)]
pub async fn insert_blocks_to_db(db: Arc<DbPool>, block_models: Vec<BlockModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::blocks::table).values(&block_models).execute(&mut conn).await?;
    tracing::info!(
        "Inserted {} blocks from {} to {}",
        block_models.len(),
        block_models.get(0).unwrap().height,
        block_models.last().unwrap().height
    );
    Ok(())
}

/// Get Block by hash
pub async fn get_block_by_hash(db: Arc<DbPool>, block_hash: &str) -> Result<Option<BlockModel>> {
    use crate::schema::blocks::dsl::*;

    let mut conn = db.get().await?;
    let block = blocks
        .filter(hash.eq(block_hash))
        .select(BlockModel::as_select())
        .first(&mut conn)
        .await
        .ok();
    Ok(block)
}

/** Fetch bloch-hashes belonging to the input chain-index at a height, ignoring/filtering-out one
 * block-hash.
 *
 * @param fromGroup
 *   `chain_from` of the blocks
 * @param toGroup
 *   `chain_to` of the blocks
 * @param height
 *   `height` of the blocks
 * @param hashToIgnore
 *   the `block-hash` to ignore or filter-out.
 */
pub async fn fetch_block_hashes_at_height_filter_one(
    db: Arc<DbPool>,
    from_group: i64,
    to_group: i64,
    height_value: i64,
    hash_to_ignore: &str,
) -> Result<Vec<String>> {
    use crate::schema::blocks::dsl::*;

    let mut conn = db.get().await?;
    let block_hashes = blocks
        .filter(chain_from.eq(from_group))
        .filter(chain_to.eq(to_group))
        .filter(height.eq(height_value))
        .filter(hash.ne(hash_to_ignore))
        .select(hash)
        .load(&mut conn)
        .await?;

    Ok(block_hashes)
}

pub async fn get_blocks(db: Arc<DbPool>, limit: i64, offset: i64) -> Result<Vec<BlockModel>> {
    use crate::schema::blocks::dsl::*;

    let mut conn = db.get().await?;
    let block_models =
        blocks.limit(limit).offset(offset).select(BlockModel::as_select()).load(&mut conn).await?;

    Ok(block_models)
}

pub async fn get_block_by_height(db: Arc<DbPool>, height_value: i64) -> Result<Option<BlockModel>> {
    use crate::schema::blocks::dsl::*;

    let mut conn = db.get().await?;
    let block_model = blocks
        .filter(height.eq(height_value))
        .select(BlockModel::as_select())
        .first(&mut conn)
        .await
        .ok();

    Ok(block_model)
}
