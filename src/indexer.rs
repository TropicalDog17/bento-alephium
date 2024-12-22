use std::{env, time};

use crate::{
    client::{Client, Network},
    db::DbPool,
    repository::{BlockRepository, EventRepository, TransactionRepository},
    schema::transactions,
    types::TimestampRange, ws::WsClient,
};
use async_trait::async_trait;

#[async_trait]
pub trait BlockIndexer: Send + Sync {
    // Forward indexing
    async fn index_new_blocks(&mut self) -> Result<(), anyhow::Error>;
    async fn handle_reorg(&mut self, fork_point: u64) -> Result<(), anyhow::Error>;
    async fn get_latest_height(&self, chain_from: u64) -> Result<u64, anyhow::Error>;

    // Backfilling
    async fn backfill(&mut self, range: TimestampRange) -> Result<(), anyhow::Error>;
    async fn get_backfill_progress(&self) -> Result<Option<(u64, u64)>, anyhow::Error>;
    async fn pause_backfill(&mut self) -> Result<(), anyhow::Error>;
    async fn resume_backfill(&mut self) -> Result<(), anyhow::Error>;
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
        let lookback = 5; // 5000 ms
        let curr_time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_micros();
        let blocks = self.alephium_client.get_blocks(curr_time - lookback, curr_time);
        todo!()
    }
    async fn handle_reorg(&mut self, fork_point: u64) -> Result<(), anyhow::Error>{
        todo!()
    }
    async fn get_latest_height(&self, chain_from: u64) -> Result<u64, anyhow::Error>{
        todo!()
    }

    // Backfilling
    async fn backfill(&mut self, range: TimestampRange) -> Result<(), anyhow::Error>{
        todo!()
    }
    async fn get_backfill_progress(&self) -> Result<Option<(u64, u64)>, anyhow::Error>{
        todo!()
    }
    async fn pause_backfill(&mut self) -> Result<(), anyhow::Error>{
        todo!()
    }
    async fn resume_backfill(&mut self) -> Result<(), anyhow::Error>{
        todo!()
    }
}
