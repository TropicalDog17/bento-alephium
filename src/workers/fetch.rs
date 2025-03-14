use futures::channel::mpsc::{Receiver, Sender};

use super::worker_v2::StageMessage;


pub async fn fetcher_worker(
    fetch_rx: Receiver<StageMessage>,
    process_tx: Sender<StageMessage>,
    completion_tx: Sender<Range>,
) -> anyhow::Result<()> {
    let fetcher = Fetcher::new().await?;
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
        }