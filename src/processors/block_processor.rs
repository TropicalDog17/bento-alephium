use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::insert_into;
use diesel_async::RunQueryDsl;

use crate::client::Client;
use crate::models::{convert_bwe_to_block_models, convert_bwe_to_event_models};
use crate::types::{BlockEntry, BlockHash};
use crate::{
    config::ProcessorConfig, db::DbPool, models::block::BlockModel, types::BlockAndEvents, utils,
};

use super::ProcessorTrait;
use crate::repository::{insert_block_and_events, insert_blocks_to_db};

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
        Ok(())
    }
}

/// Insert blocks into the database.
pub async fn insert_to_db(db: Arc<DbPool>, blocks: Vec<BlockModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::blocks::table).values(&blocks).execute(&mut conn).await?;
    Ok(())
}

/// 5. For each last block of each chains, mark it as part of the main chain and travel
///    down the parents recursively until we found back a parent that is part of the main chain.
/// 6. During step 5, if a parent is missing, we download it and continue the procces at 5.
///
/// TODO: Step 5 is costly, but it's an easy way to handle reorg. In step 3 we know we receive the current main chain
/// for that timerange, so in step 4 we could directly insert them as `mainChain = true`, but we need to sync
/// to a sanity check process, wich could be an external proccess, that regularly goes down the chain to make
/// sure we have the right one in DB.

// TODO: organize insert block and event methods
async fn handle_missing_main_chain_block(
    client: Arc<Client>,
    db: Arc<DbPool>,
    missing: BlockHash,
) -> Result<()> {
    let bwe = client.get_block_and_events_by_hash(&missing).await?;


    let block_models = convert_bwe_to_block_models(vec![vec![bwe.clone()]]);
    let event_models = convert_bwe_to_event_models(vec![vec![bwe]]);

    insert_block_and_events(db, block_models[0].clone(), event_models).await?;
    Ok(())
}

/// Insert block into the database, handle reorg if needed.
async fn insert_block(db: Arc<DbPool>, block: BlockEntry) {
    todo!()
}
