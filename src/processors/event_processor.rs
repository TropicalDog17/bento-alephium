use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    config::ProcessorConfig, db::DbPool, types::BlockAndEvents,
};

use super::ProcessorTrait;

pub struct EventProcessor {
    connection_pool: Arc<DbPool>,
}

impl EventProcessor {
    pub fn new(connection_pool: Arc<DbPool>) -> Self {
        Self { connection_pool }
    }
}

impl Debug for EventProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "EventProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl ProcessorTrait for EventProcessor {
    fn name(&self) -> &'static str {
        ProcessorConfig::EventProcessor.name()
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
        // Process blocks and events
        Ok(())
    }
}
