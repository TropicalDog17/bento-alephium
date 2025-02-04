use crate::{
    db::{DbPool, DbPoolConnection},
    types::{BlockAndEvents, LatestBlock},
};
use anyhow::Result;
use async_trait::async_trait;
use block_processor::BlockProcessor;
use default_processor::DefaultProcessor;
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use event_processor::EventProcessor;
use std::{fmt::Debug, sync::Arc};
use transaction_processor::TransactionProcessor;

pub mod block_processor;
pub mod default_processor;
pub mod event_processor;
pub mod transaction_processor;

/// Base trait for all processors
#[async_trait]
pub trait ProcessorTrait: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn connection_pool(&self) -> &Arc<DbPool>;

    fn get_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        let pool = self.connection_pool();
        pool.clone()
    }

    async fn get_conn(&self) -> DbPoolConnection {
        let pool = self.connection_pool();
        loop {
            match pool.get().await {
                Ok(conn) => {
                    return conn;
                }
                Err(err) => {
                    tracing::error!(
                        "Could not get DB connection from pool, will retry. Err: {:?}",
                        err
                    );
                }
            };
        }
    }

    async fn process_blocks(
        &self,
        from_ts: i64,
        to_ts: i64,
        blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()>;

    async fn update_last_processed_block(_block: LatestBlock, _timestamp: i64) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum Processor {
    BlockProcessor(BlockProcessor),
    DefaultProcessor(DefaultProcessor),
    TransactionProcessor(TransactionProcessor),
    EventProcessor(EventProcessor),
}

#[async_trait]
impl ProcessorTrait for Processor {
    fn connection_pool(&self) -> &Arc<DbPool> {
        match self {
            Processor::DefaultProcessor(p) => p.connection_pool(),
            Processor::TransactionProcessor(p) => p.connection_pool(),
            Processor::BlockProcessor(p) => p.connection_pool(),
            Processor::EventProcessor(p) => p.connection_pool(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Processor::DefaultProcessor(p) => p.name(),
            Processor::TransactionProcessor(p) => p.name(),
            Processor::BlockProcessor(p) => p.name(),
            Processor::EventProcessor(p) => p.name(),
        }
    }

    async fn process_blocks(
        &self,
        from_ts: i64,
        to_ts: i64,
        blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()> {
        match self {
            Processor::DefaultProcessor(p) => p.process_blocks(from_ts, to_ts, blocks).await,
            Processor::TransactionProcessor(p) => p.process_blocks(from_ts, to_ts, blocks).await,
            Processor::BlockProcessor(p) => p.process_blocks(from_ts, to_ts, blocks).await,
            Processor::EventProcessor(p) => p.process_blocks(from_ts, to_ts, blocks).await,
        }
    }
}
