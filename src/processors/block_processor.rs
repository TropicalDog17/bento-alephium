use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::insert_into;
use diesel_async::RunQueryDsl;

use crate::{
    config::ProcessorConfig, db::DbPool, models::block::BlockModel, types::BlockAndEvents, utils,
};

use super::ProcessorTrait;

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
        let models = convert_to_model(blocks);
        if !models.is_empty() {
            insert_to_db(self.connection_pool.clone(), models).await?;
        }
        Ok(())
    }
}

/// Insert blocks into the database.
pub async fn insert_to_db(db: Arc<DbPool>, blocks: Vec<BlockModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::blocks::table).values(&blocks).execute(&mut conn).await?;
    Ok(())
}

pub fn convert_to_model(blocks: Vec<Vec<BlockAndEvents>>) -> Vec<BlockModel> {
    let mut models = Vec::new();
    for bes in blocks {
        for be in bes {
            let b = be.block;
            models.push(BlockModel {
                hash: b.hash,
                timestamp: utils::timestamp_millis_to_naive_datetime(b.timestamp),
                chain_from: b.chain_from,
                chain_to: b.chain_to,
                height: b.height,
                deps: Some(b.deps.into_iter().map(Some).collect()),
                nonce: b.nonce,
                version: b.version.to_string(),
                dep_state_hash: b.dep_state_hash,
                txs_hash: b.txs_hash.to_string(),
                tx_number: b.transactions.len() as i64,
                target: b.target,
                ghost_uncles: serde_json::to_value(b.ghost_uncles).unwrap_or_default(),
            });
        }
    }
    models
}
