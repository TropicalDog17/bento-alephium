use std::env;
use std::error::Error;

use bento_alephium::{
    client::Network,
    config::ProcessorConfig,
    worker::{SyncOptions, Worker},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Setup logger
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut worker = Worker::new(
        ProcessorConfig::BlockEventProcessor,
        database_url,
        Network::Mainnet,
        None,
        Some(SyncOptions {
            start_ts: Some(1737624369494),
            step: Some(1000),
            back_step: None,
            sync_duration: None,
        }),
    )
    .await?;

    worker.run().await;
    Ok(())
}
