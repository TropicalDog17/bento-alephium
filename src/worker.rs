use anyhow::{Context, Result};
use std::sync::Arc;

use crate::{
    client::{Client, Network},
    config::ProcessorConfig,
    db::{new_db_pool, DbPool},
    processors::{
        block_processor::BlockProcessor, default_processor::DefaultProcessor,
        event_processor::EventProcessor, transaction_processor::TransactionProcessor, Processor,
        ProcessorTrait,
    },
};

#[derive(Debug, Default)]
pub struct SyncOptions {
    pub start_ts: Option<i64>,
    pub end_ts: Option<i64>,
    pub step: Option<i64>,
    pub back_step: Option<i64>,
}

/// Worker manages the lifecycle of a processor.
///
/// In the initialization phase, we make sure we get at least one timestamp other than the genesis one
///
/// The syncing algorithm goes as follow:
/// 1. Getting maximum timestamps from both the local chains and the remote ones.
/// 2. Build timestamp ranges of X minutes each, starting from local max to remote max.
/// 3. For each of those range, we get all the blocks inbetween that time.
/// 4. Insert all blocks (with `mainChain = false`).
/// 5. For each last block of each chains, mark it as part of the main chain and travel
///    down the parents recursively until we found back a parent that is part of the main chain.
/// 6. During step 5, if a parent is missing, we download it and continue the procces at 5.
///
/// TODO: Step 5 is costly, but it's an easy way to handle reorg. In step 3 we know we receive the current main chain
/// for that timerange, so in step 4 we could directly insert them as `mainChain = true`, but we need to sync
/// to a sanity check process, wich could be an external proccess, that regularly goes down the chain to make
/// sure we have the right one in DB.
pub struct Worker {
    pub db_pool: Arc<DbPool>,
    pub client: Arc<Client>,
    pub processor_config: ProcessorConfig,
    pub db_url: String,
    pub sync_opts: SyncOptions,
}

impl Worker {
    pub async fn new(
        processor_config: ProcessorConfig,
        db_url: String,
        network: Network,
        db_pool_size: Option<u32>,
        sync_opts: Option<SyncOptions>,
    ) -> Result<Self> {
        let processor_name = processor_config.name();
        tracing::info!(processor_name = processor_name, "Creating worker");

        tracing::info!(processor_name = processor_name, "Creating connection pool");
        let db_pool =
            new_db_pool(&db_url, db_pool_size).await.context("Failed to create connection pool")?;
        tracing::info!(processor_name = processor_name, "Finish creating the connection pool");

        let sync_opts = sync_opts.unwrap_or_default();

        Ok(Self {
            db_pool,
            processor_config,
            db_url,
            sync_opts,
            client: Arc::new(Client::new(network)),
        })
    }

    pub async fn run(&mut self) {
        let processor_name = self.processor_config.name();
        tracing::info!(processor_name = processor_name, "Starting worker");

        tracing::info!(processor_name = processor_name, "Run migrations");
        let migration_time = std::time::Instant::now();
        self.run_migrations().await;
        tracing::info!(
            processor_name = processor_name,
            duration_in_secs = migration_time.elapsed().as_secs_f64(),
            "Finished migrations"
        );

        let from_ts = self.sync_opts.start_ts.unwrap_or(0);
        let to_ts = self.sync_opts.end_ts.unwrap_or(0);

        let processor = build_processor(&self.processor_config, self.db_pool.clone());

        let blocks =
            self.client.get_blocks_and_events(from_ts as u128, to_ts as u128).await.unwrap();

        processor.process_blocks(from_ts, to_ts, blocks.blocks_and_events).await.unwrap();

        tracing::info!(processor_name = processor_name, "Stopping worker");
    }

    // For the normal processor build we just use standard Diesel with the postgres
    // feature enabled (which uses libpq under the hood, hence why we named the feature
    // this way).
    #[cfg(feature = "libpq")]
    async fn run_migrations(&self) {
        use diesel::{pg::PgConnection, Connection};

        use crate::db::run_pending_migrations;

        tracing::info!("Running migrations: {:?}", self.db_url);
        let mut conn = PgConnection::establish(&self.db_url).expect("migrations failed!");
        run_pending_migrations(&mut conn);
    }
}

/// Build a processor based on the configuration.
pub fn build_processor(config: &ProcessorConfig, db_pool: Arc<DbPool>) -> Processor {
    match config {
        ProcessorConfig::DefaultProcessor => {
            Processor::DefaultProcessor(DefaultProcessor::new(db_pool))
        }
        ProcessorConfig::EventProcessor => Processor::EventProcessor(EventProcessor::new(db_pool)),
        ProcessorConfig::TransactionProcessor => {
            Processor::TransactionProcessor(TransactionProcessor::new(db_pool))
        }
        ProcessorConfig::BlockProcessor => Processor::BlockProcessor(BlockProcessor::new(db_pool)),
    }
}
