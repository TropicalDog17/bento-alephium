use std::{sync::Arc, time::Instant};

use anyhow::Result;
use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    StreamExt,
};

use crate::{
    client::Client,
    traits::BlockProvider,
    types::{BlockAndEvents, BlockBatch, BlockRange, MAX_TIMESTAMP_RANGE},
};

pub async fn fetch_parallel(
    client: Arc<Client>,
    range: BlockRange,
    num_workers: usize,
) -> Result<BlockBatch> {
    let total_time: i64 = range.to_ts - range.from_ts;
    let chunk_size = total_time / num_workers as i64;

    let mut futures = FuturesOrdered::new();

    for i in 0..num_workers {
        let from = range.from_ts + (i as i64 * chunk_size);
        let to = if i == num_workers - 1 { range.to_ts } else { from + chunk_size };

        let range = BlockRange { from_ts: from, to_ts: to };
        futures.push_back(fetch_chunk(client.clone(), range));
    }

    let mut results = Vec::new();
    while let Some(result) = futures.next().await {
        results.push(result?);
    }

    // merge to one large batch
    let mut blocks = Vec::new();
    for batch in results.iter() {
        blocks.extend(batch.blocks.clone());
    }

    let merged_batch =
        BlockBatch { blocks, range: BlockRange { from_ts: range.from_ts, to_ts: range.to_ts } };

    Ok(merged_batch)
}

pub async fn fetch_chunk(client: Arc<Client>, range: BlockRange) -> Result<BlockBatch> {
    if (range.to_ts - range.from_ts) > MAX_TIMESTAMP_RANGE {
        return Err(anyhow::anyhow!(
            "Timestamp range exceeds maximum limit, maximum {}, got {}",
            MAX_TIMESTAMP_RANGE,
            range.to_ts - range.from_ts,
        ));
    }

    let start = Instant::now();
    let blocks: Vec<BlockAndEvents> = client
        .get_blocks_and_events(range.from_ts, range.to_ts)
        .await?
        .blocks_and_events
        .iter()
        .flatten()
        .cloned()
        .collect();

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
