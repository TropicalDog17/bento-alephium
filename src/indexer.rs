use std::{env, time};

use crate::{
    client::{Client, Network}, db::DbPool, models::BlockModel, repository::{BlockRepository, EventRepository, TransactionRepository}, schema::transactions, types::TimestampRange, ws::WsClient
};
use async_trait::async_trait;

#[async_trait]
pub trait BlockIndexer: Send + Sync {
    // Forward indexing
    async fn index_new_blocks(&mut self) -> Result<(), anyhow::Error>;
    async fn handle_reorg(&mut self, fork_point: u64) -> Result<(), anyhow::Error>;
    async fn get_latest_height(&self, chain_from: i64) -> Result<i64, anyhow::Error>;

    // Backfilling
    async fn backfill(&mut self, range: TimestampRange) -> Result<(), anyhow::Error>;
}

pub struct IndexerService {
    pub alephium_client: Client,
    pub blocks: BlockRepository,
    pub events: EventRepository,
    pub transactions: TransactionRepository,
    
}

impl IndexerService {
    pub fn new(db_pool: DbPool, network: Network) -> Self {
        let alephium_client = Client::new(network);
        let blocks = BlockRepository { pool: db_pool.clone() };
        let events = EventRepository { pool: db_pool.clone() };
        let transactions = TransactionRepository { pool: db_pool.clone() };
        // let websocket_client = WsClient::connect_async(&env::var("WEBSOCKET_URL").unwrap()).await.unwrap();
        Self { alephium_client, blocks, events, transactions }
    }
}

#[async_trait]
impl BlockIndexer for IndexerService {
    async fn index_new_blocks(&mut self) -> Result<(), anyhow::Error>{
        let lookback = 5000; // 5000 ms
        let curr_time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
        let response = self.alephium_client.get_blocks(curr_time - lookback, curr_time).await.unwrap();
        match response.blocks[..] {
            [] => return Ok(()),
            _ => {
                log::info!("Indexing {} blocks of {} chain shards", response.blocks.iter().flatten().count(), response.blocks.len());
            }
        }

        let blocks_to_save = response.blocks.iter().flatten().map(|block_entry| {
            BlockModel::from((*block_entry).clone())
        }).collect::<Vec<_>>();

        self.blocks.store_batch(&blocks_to_save)?;
        Ok(())
    }
    async fn handle_reorg(&mut self, fork_point: u64) -> Result<(), anyhow::Error>{
        todo!()
    }
    async fn get_latest_height(&self, chain_id: i64) -> Result<i64, anyhow::Error>{
        self.blocks.get_latest_height(chain_id)
    }

    // Backfilling
    async fn backfill(&mut self, range: TimestampRange) -> Result<(), anyhow::Error>{
        todo!()
    }
}
