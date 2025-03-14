use anyhow::{Context, Result};
use std::{sync::Arc, thread::sleep, time::Duration};
use tokio::{sync::{mpsc}, time::sleep as tokio_sleep};
use futures::{lock::Mutex, stream::{FuturesUnordered, StreamExt}};
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use std::time::Instant;

use crate::{
    client::{Client, Network}, config::ProcessorConfig, db::{new_db_pool, DbPool}, processors::{
        block_processor::BlockProcessor, default_processor::DefaultProcessor, event_processor::EventProcessor, lending_marketplace_processor::{insert_loan_actions_to_db, insert_loan_details_to_db, LendingContractProcessor}, tx_processor::TxProcessor, Processor, ProcessorOutput, ProcessorTrait
    }, repository::{insert_blocks_to_db, insert_events_to_db, insert_txs_to_db}, schema::processor_status, traits::BlockProvider, types::BlockAndEvents
};

const MAX_TIMESTAMP_RANGE : i64 = 1800000;

// Message types for different stages
#[derive(Clone)]
pub enum FetchStrategy {
    Simple,
    Chunked { chunk_size: i64 },
    Parallel { num_workers: usize },
}

#[derive(Clone, Copy)]
pub struct BlockRange {
    from_ts: i64,
    to_ts: i64,
}

#[derive(Clone)]
pub struct BlockBatch {
    blocks: Vec<BlockAndEvents>,
    range: BlockRange,
}




// Stage implementations
pub struct FetcherStage {
    client: Arc<Client>,
    strategy: FetchStrategy,
    sync_options: SyncOptions,
    remaining_batches: Arc<Mutex<Vec<BlockBatch>>>, 
}

impl FetcherStage {
    pub fn new(client: Arc<Client>, strategy: FetchStrategy, sync_options: SyncOptions) -> Self {
        Self { client, strategy, sync_options, remaining_batches: Arc::new(Mutex::new(Vec::new())) }
    }

    fn chunk_size(&self) -> i64 {
        match &self.strategy {
            FetchStrategy::Simple => MAX_TIMESTAMP_RANGE, 
            FetchStrategy::Chunked { chunk_size } => *chunk_size,
            FetchStrategy::Parallel {num_workers } => self.sync_options.step.unwrap_or(MAX_TIMESTAMP_RANGE) as i64,
        }
    }

    
// Add this at the top with other imports

// Add this to the FetcherStage::fetch_chunk method
async fn fetch_chunk(&self, range: BlockRange) -> Result<BlockBatch> {
    if (range.to_ts - range.from_ts) > MAX_TIMESTAMP_RANGE {
        return Err(anyhow::anyhow!("Timestamp range exceeds maximum limit"));
    }
    
    let start = Instant::now();
    let blocks: Vec<BlockAndEvents> = self.client
        .get_blocks_and_events(range.from_ts, range.to_ts)
        .await?
        .blocks_and_events.iter().flatten().cloned().collect();
    
    let elapsed = start.elapsed();
    
    tracing::info!(
        "Fetched {} blocks from timestamp {} to timestamp {} in {:.2?}",
        blocks.clone().len(),
        range.from_ts,
        range.to_ts,
        elapsed
    );
    Ok(BlockBatch { blocks, range })
}


    async fn fetch_parallel(&self, range: BlockRange,  num_workers: usize) -> Result<Vec<BlockBatch>> {
        let total_time: i64 = range.to_ts - range.from_ts;
        let chunk_size = total_time / num_workers as i64;
        
        let mut futures = FuturesUnordered::new();
        
        for i in 0..num_workers {
            let from = range.from_ts + (i as i64 * chunk_size);
            let to = if i == num_workers - 1 {
                range.to_ts
            } else {
                from + chunk_size
            };
            
            let range = BlockRange { from_ts: from, to_ts: to };
            futures.push(self.fetch_chunk(range));
        }

        let mut results = Vec::new();
        while let Some(result) = futures.next().await {
            results.push(result?);
        }
        
        Ok(results)
    }
}

#[async_trait::async_trait]
impl StageHandler for FetcherStage {
    async fn handle(&self, input: StageMessage) -> Result<StageMessage> {
        {
            let mut remaining = self.remaining_batches.lock().await;
            tracing::info!("Remaining batches: {}", remaining.len());
            if !remaining.is_empty() {
                // Use the next batch from the queue
                let next_batch = remaining.remove(0);
                return Ok(StageMessage::Batch(next_batch));
            }
        }

        match input {
            StageMessage::Range(range) => {
                match &self.strategy {
                    FetchStrategy::Simple => {
                        let batch = self.fetch_chunk(range).await?;
                        Ok(StageMessage::Batch(batch))
                    }
                    FetchStrategy::Chunked { chunk_size } => {
                        let total_time = range.to_ts - range.from_ts;
                        let num_chunks = (total_time / chunk_size) + 1;
                        let mut batches = Vec::new();
                        
                        for i in 0..num_chunks {
                            let from = range.from_ts + (i * chunk_size);
                            let to = (from + chunk_size).min(range.to_ts);
                            let chunk_range = BlockRange { from_ts: from, to_ts: to };
                            let batch = self.fetch_chunk(chunk_range).await?;
                            batches.push(batch);
                        }
                        
                        if batches.is_empty() {
                            Ok(StageMessage::Complete)
                        } else {
                            Ok(StageMessage::Batch(batches.remove(0))) // Send first batch
                        }
                    }
                    FetchStrategy::Parallel {num_workers } => {
                        let batches = self.fetch_parallel(range, *num_workers).await?;
                        if batches.is_empty() {
                            Ok(StageMessage::Complete)
                        } else {
                            // Store all batches except the first one
                            if batches.len() > 1 {
                                let mut remaining = self.remaining_batches.lock().await;
                                remaining.extend(batches.clone().into_iter().skip(1));
                            }
                            
                            let first_batch = batches[0].clone();
                            Ok(StageMessage::Batch(first_batch))
                        }
                    }
                }
            }
            StageMessage::Complete => Ok(StageMessage::Complete),
            _ => Ok(StageMessage::Complete),
        }
    }
}

pub struct ProcessorStage {
    processor: Processor,
}

// Modify the ProcessorStage::handle method
#[async_trait::async_trait]
impl StageHandler for ProcessorStage {
    async fn handle(&self, input: StageMessage) -> Result<StageMessage> {
        match input {
            StageMessage::Batch(batch) => {
                let block_count = batch.blocks.len();
                let start = Instant::now();
                
                // Process blocks
                let output = self.processor.process_blocks(
                    batch.range.from_ts,
                    batch.range.to_ts,
                    batch.blocks,
                ).await?;
                
                let elapsed = start.elapsed();
                
                tracing::info!(
                    "Processed {} blocks (range: {} to {}) in {:.2?}",
                    block_count,
                    batch.range.from_ts,
                    batch.range.to_ts,
                    elapsed
                );
                
                Ok(StageMessage::Processed(output))
            }
            _ => Ok(StageMessage::Complete),
        }
    }
}
pub struct StorageStage {
    db_pool: Arc<DbPool>,
}

// Modify the StorageStage::handle method
#[async_trait::async_trait]
impl StageHandler for StorageStage {
    async fn handle(&self, input: StageMessage) -> Result<StageMessage> {
        match input {
            StageMessage::Processed(output) => {
                let start = Instant::now();
                let operation_type = match &output {
                    ProcessorOutput::Block(_) => "blocks",
                    ProcessorOutput::Event(_) => "events",
                    ProcessorOutput::LendingContract(_) => "lending contracts",
                    ProcessorOutput::Tx(_) => "transactions",
                    ProcessorOutput::Default(_) => "default",
                };
                
                let result = match output {
                    ProcessorOutput::Block(blocks) => {
                        let count = blocks.len();
                        match insert_blocks_to_db(self.db_pool.clone(), blocks).await {
                            Ok(_) => {
                                let elapsed = start.elapsed();
                                tracing::info!("Successfully stored {} blocks in {:.2?}", count, elapsed);
                                Ok(())
                            },
                            Err(e) => {
                                tracing::error!("Failed to store blocks: {}", e);
                                Err(e)
                            }
                        }
                    }
                    ProcessorOutput::Event(events) => {
                        let count = events.len();
                        match insert_events_to_db(self.db_pool.clone(), events).await {
                            Ok(_) => {
                                let elapsed = start.elapsed();
                                tracing::info!("Successfully stored {} events in {:.2?}", count, elapsed);
                                Ok(())
                            },
                            Err(e) => {
                                tracing::error!("Failed to store events: {}", e);
                                Err(e)
                            }
                        }
                    }
                    ProcessorOutput::LendingContract((loan_actions, loan_details)) => {
                        let action_count = loan_actions.len();
                        let details_count = loan_details.len();
                        let store_start = Instant::now();
                        
                        let actions_result = insert_loan_actions_to_db(self.db_pool.clone(), loan_actions).await;
                        let actions_elapsed = store_start.elapsed();
                        
                        let details_start = Instant::now();
                        let details_result = insert_loan_details_to_db(self.db_pool.clone(), loan_details).await;
                        let details_elapsed = details_start.elapsed();
                        
                        match (actions_result, details_result) {
                            (Ok(_), Ok(_)) => {
                                let total_elapsed = start.elapsed();
                                tracing::info!(
                                    "Successfully stored lending contract data: {} actions in {:.2?}, {} details in {:.2?}, total {:.2?}",
                                    action_count,
                                    actions_elapsed,
                                    details_count,
                                    details_elapsed,
                                    total_elapsed
                                );
                                Ok(())
                            },
                            (Err(e), _) | (_, Err(e)) => {
                                tracing::error!("Failed to store lending contract data: {}", e);
                                Err(e)
                            }
                        }
                    }
                    ProcessorOutput::Tx(txs) => {
                        let count = txs.len();
                        match insert_txs_to_db(self.db_pool.clone(), txs).await {
                            Ok(_) => {
                                let elapsed = start.elapsed();
                                tracing::info!("Successfully stored {} transactions in {:.2?}", count, elapsed);
                                Ok(())
                            },
                            Err(e) => {
                                tracing::error!("Failed to store transactions: {}", e);
                                Err(e)
                            }
                        }
                    }
                    ProcessorOutput::Default(()) => {
                        let elapsed = start.elapsed();
                        tracing::info!("Processed default output in {:.2?}", elapsed);
                        Ok(())
                    }
                };

                match result {
                    Ok(_) => Ok(StageMessage::Complete),
                    Err(e) => Err(e)
                }
            }
            _ => Ok(StageMessage::Complete),
        }
    }
}

pub struct Pipeline {
    fetcher: Arc<FetcherStage>,
    processor: Arc<ProcessorStage>,
    storage: Arc<StorageStage>,
}

impl Pipeline {
    pub fn new(
        client: Arc<Client>,
        db_pool: Arc<DbPool>,
        processor: Processor,
        fetch_strategy: FetchStrategy,
        sync_opts: SyncOptions,
    ) -> Self {
        Self {
            fetcher: Arc::new(FetcherStage::new(client, fetch_strategy, sync_opts)),
            processor: Arc::new(ProcessorStage { processor }),
            storage: Arc::new(StorageStage { db_pool }),
        }
    }

    pub async fn run(&self, initial_range: BlockRange) -> Result<()> {
        let channel_capacity = 100;
        let (fetch_tx, fetch_rx) = mpsc::channel(channel_capacity);
        let (process_tx, process_rx) = mpsc::channel(channel_capacity);
        let (storage_tx, storage_rx) = mpsc::channel(channel_capacity);
        let (completion_tx, mut completion_rx) = mpsc::channel::<BlockRange>(1);

        // Monitor channel capacity
        let process_tx_clone = process_tx.clone();
        let storage_tx_clone = storage_tx.clone();
        
        let monitor_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                let process_capacity = process_tx_clone.capacity();
                let storage_capacity = storage_tx_clone.capacity();
                
                tracing::info!(
                    "Channel capacities - Process: {}/{}, Storage: {}/{}",
                    channel_capacity - process_capacity,
                    channel_capacity,
                    channel_capacity - storage_capacity,
                    channel_capacity
                );
            }
        });

        // Spawn stage handlers
        let fetcher = self.fetcher.clone();
        let processor = self.processor.clone();
        let storage = self.storage.clone();

        // Fetcher stage
        let fetch_handle = tokio::spawn(async move {
            let mut rx = fetch_rx;
            
            while let Some(msg) = rx.recv().await {
                if let StageMessage::Range(range) = msg {
                    tracing::info!("Fetcher starting to fetch range: {} to {}", range.from_ts, range.to_ts);
                    
                    let result = fetcher.handle(msg).await?;
                    
                    match result {
                        StageMessage::Batch(batch) => {
                            // Send first batch to processor
                            process_tx.send(StageMessage::Batch(batch.clone())).await?;
                            
                            // Track this range for completion notification
                            completion_tx.send(range).await?;
                            
                            // Check for remaining batches from parallel fetching
                            let mut remaining = fetcher.remaining_batches.lock().await;
                            tracing::info!("Fetcher has {} remaining batches", remaining.len());
                            
                            // Process any remaining batches
                            while !remaining.is_empty() {
                                let next_batch = remaining.remove(0);
                                process_tx.send(StageMessage::Batch(next_batch)).await?;
                            }
                        }
                        _ => {
                            tracing::info!("Fetcher received unexpected message type");
                        }
                    }
                }
            }
            
            // Close processor channel when fetcher is done
            drop(process_tx);
            drop(completion_tx);
            
            Ok::<_, anyhow::Error>(())
        });

        // Processor stage
        let process_handle = tokio::spawn(async move {
            let mut rx = process_rx;
            
            while let Some(msg) = rx.recv().await {
                if let StageMessage::Batch(batch) = msg {
                    let blocks_count = batch.blocks.len();
                    let range = batch.range;
                    
                    tracing::info!(
                        "Processor processing batch with {} blocks (range: {} to {})", 
                        blocks_count, range.from_ts, range.to_ts
                    );
                    
                    let result = processor.handle(StageMessage::Batch(batch)).await?;
                    
                    if let StageMessage::Processed(output) = result {
                        storage_tx.send(StageMessage::Processed(output)).await?;
                    }
                }
            }
            
            // Close storage channel when processor is done
            drop(storage_tx);
            
            Ok::<_, anyhow::Error>(())
        });

        // Storage stage
        let storage_handle = tokio::spawn(async move {
            let mut rx = storage_rx;
            
            while let Some(msg) = rx.recv().await {
                if let StageMessage::Processed(output) = msg {
                    storage.handle(StageMessage::Processed(output)).await?;
                }
            }
            
            Ok::<_, anyhow::Error>(())
        });

        // Main pipeline control
        fetch_tx.send(StageMessage::Range(initial_range)).await?;
        
        // Wait for completion of the range
        let completed_range = completion_rx.recv().await.ok_or_else(|| {
            tracing::error!("Completion channel closed unexpectedly");
            sleep(Duration::from_secs(10));
        }
        ).expect("Completion channel closed unexpectedly");
        
        // Wait for all tasks to complete
        let _ = tokio::join!(fetch_handle, process_handle, storage_handle);
        
        Ok(())
    }
}
pub struct Worker {
    pub db_pool: Arc<DbPool>,
    pub client: Arc<Client>,
    pub processor_configs: Vec<ProcessorConfig>,
    pub db_url: String,
    pub sync_opts: SyncOptions,
    pub fetch_strategy: FetchStrategy,
}

impl Worker {
    pub async fn new(
        processor_configs: Vec<ProcessorConfig>,
        db_url: String,
        network: Network,
        db_pool_size: Option<u32>,
        sync_opts: Option<SyncOptions>,
        fetch_strategy: Option<FetchStrategy>,
    ) -> Result<Self> {
        let db_pool = new_db_pool(&db_url, db_pool_size)
            .await
            .context("Failed to create connection pool")?;
        
        Ok(Self {
            db_pool: db_pool.clone(),
            processor_configs,
            db_url,
            sync_opts: sync_opts.unwrap_or_default(),
            client: Arc::new(Client::new(network)),
            fetch_strategy: fetch_strategy.unwrap_or(FetchStrategy::Simple),
        })
    }
    pub async fn run(&self) -> Result<()> {
        self.run_migrations().await;
        let mut handles = Vec::new();
    
        for processor_config in self.processor_configs.clone() {
            let pool_clone = self.db_pool.clone();
            let client_clone = self.client.clone();
            let fetch_strategy_clone = self.fetch_strategy.clone();
            let sync_opts_clone = self.sync_opts.clone();
            let processor_config = processor_config.clone();
            
            let handle = tokio::spawn(async move {
                let processor = build_processor(&processor_config, pool_clone.clone());
                let processor_name = processor.name();
                
                let pipeline = Pipeline::new(
                    client_clone,
                    pool_clone.clone(),
                    processor,
                    fetch_strategy_clone,
                    sync_opts_clone,
                );
    
                let last_ts = get_last_timestamp(&pool_clone, processor_name).await?;
                let mut current_ts = sync_opts_clone.start_ts.unwrap_or(0).max(last_ts);
                let step = sync_opts_clone.step.unwrap_or(1000);
                let sync_duration = Duration::from_secs(
                    sync_opts_clone.sync_duration.unwrap_or(1) as u64
                );
    
                loop {
                    let to_ts = current_ts + step;
                    let range = BlockRange {
                        from_ts: current_ts,
                        to_ts,
                    };
    
                    if let Err(err) = pipeline.run(range).await {
                        tracing::error!(
                            processor_name = processor_name,
                            error = ?err,
                            "Pipeline execution failed, retrying in {:?}",
                            sync_duration
                        );
                    } else {
                        update_last_timestamp(&pool_clone, processor_name, to_ts).await?;
                        current_ts = to_ts + 1;
                    }
    
                    tokio_sleep(sync_duration).await;
                }
                
                #[allow(unreachable_code)]
                Ok::<(), anyhow::Error>(())
            });
    
            handles.push(handle);
        }
    
        // Start all handlers by infinite loop.
        for handle in handles {
            match handle.await {
                Ok(result) => result?,
                Err(e) => return Err(anyhow::anyhow!("Task panicked: {}", e)),
            }
        }
        
        Ok(())
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


pub async fn get_last_timestamp(db_pool: &Arc<DbPool>, processor_name: &str) -> Result<i64> {
    tracing::info!(processor = processor_name, "Getting last timestamp");
    let mut conn = db_pool.get().await?;
    let ts = processor_status::table
        .filter(processor_status::processor.eq(processor_name))
        .select(processor_status::last_timestamp)
        .first::<i64>(&mut conn)
        .await
        .optional()?;
    Ok(ts.unwrap_or(0))
}

pub async fn update_last_timestamp(
    _db_pool: &Arc<DbPool>,
    processor_name: &str,
    last_timestamp: i64,
) -> Result<()> {
    tracing::info!(
        processor = processor_name,
        last_timestamp = last_timestamp,
        "Updating last timestamp"
    );
    let mut conn = _db_pool.get().await?;
    insert_into(processor_status::table)
        .values((
            processor_status::processor.eq(processor_name),
            processor_status::last_timestamp.eq(last_timestamp),
        ))
        .on_conflict(processor_status::processor)
        .do_update()
        .set(processor_status::last_timestamp.eq(last_timestamp))
        .execute(&mut conn)
        .await
        .map(|_| ())
        .map_err(anyhow::Error::new)
}

/// Build a processor based on the configuration.
pub fn build_processor(config: &ProcessorConfig, db_pool: Arc<DbPool>) -> Processor {
    match config {
        ProcessorConfig::DefaultProcessor => {
            Processor::DefaultProcessor(DefaultProcessor::new(db_pool))
        }
        ProcessorConfig::BlockProcessor => Processor::BlockProcessor(BlockProcessor::new(db_pool)),
        ProcessorConfig::EventProcessor => Processor::EventProcessor(EventProcessor::new(db_pool)),
        ProcessorConfig::LendingContractProcessor(contract_address) => {
            Processor::LendingContractProcessor(LendingContractProcessor::new(
                db_pool,
                contract_address.clone(),
            ))
        },
        ProcessorConfig::TxProcessor => Processor::TxProcessor(TxProcessor::new(db_pool)),
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SyncOptions {
    pub start_ts: Option<i64>,
    pub step: Option<i64>,
    pub back_step: Option<i64>,
    pub sync_duration: Option<i64>,
}
