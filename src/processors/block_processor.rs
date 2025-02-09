use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::insert_into;
use diesel_async::RunQueryDsl;

use crate::models::convert_bwe_to_block_models;
use crate::{
    config::ProcessorConfig, db::DbPool, models::block::BlockModel, types::BlockAndEvents,
};

use super::ProcessorTrait;
use crate::repository::insert_blocks_to_db;

pub struct BlockProcessor {
    connection_pool: Arc<DbPool>,
}

impl BlockProcessor {
    pub fn new(connection_pool: Arc<DbPool>) -> Self {
        Self { connection_pool }
    }
}

impl Debug for BlockProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "BlockProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl ProcessorTrait for BlockProcessor {
    fn name(&self) -> &'static str {
        ProcessorConfig::BlockProcessor.name()
    }

    fn connection_pool(&self) -> &Arc<DbPool> {
        &self.connection_pool
    }

    async fn process_blocks(
        &self,
        _from: i64,
        _to: i64,
        blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()> {
        // Process blocks and insert to db
        let models = convert_bwe_to_block_models(blocks);
        if !models.is_empty() {
            insert_blocks_to_db(self.connection_pool.clone(), models).await?;
        }
        // handle reorgs
        Ok(())
    }
}

/// Insert blocks into the database.
pub async fn insert_to_db(db: Arc<DbPool>, blocks: Vec<BlockModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::blocks::table).values(&blocks).execute(&mut conn).await?;
    Ok(())
}
