use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    config::ProcessorConfig, db::DbPool, models::block::BlockModel, types::BlockAndEvents,
};

use super::ProcessorTrait;

pub struct BlockEventProcessor {
    connection_pool: Arc<DbPool>,
}

impl BlockEventProcessor {
    pub fn new(connection_pool: Arc<DbPool>) -> Self {
        Self { connection_pool }
    }
}

impl Debug for BlockEventProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "BlockEventProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl ProcessorTrait for BlockEventProcessor {
    fn name(&self) -> &'static str {
        ProcessorConfig::BlockEventProcessor.name()
    }

    fn connection_pool(&self) -> &Arc<DbPool> {
        &self.connection_pool
    }

    async fn process_blocks(
        &self,
        _from: i64,
        _to: i64,
        _blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()> {
        // Process blocks and insert to db
        Ok(())
    }
}

pub fn insert_to_db(_blocks: Vec<BlockModel>) -> Result<()> {
    // Insert blocks to db
    Ok(())
}

pub fn convert_to_model(_blocks: Vec<Vec<BlockAndEvents>>) -> Vec<BlockModel> {
    // Process blocks and events
    vec![]
}
