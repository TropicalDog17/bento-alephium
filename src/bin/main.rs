use std::env;

use bento_alephium::{client::Network, config::ProcessorConfig, worker::Worker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Setup logger
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut worker =
        Worker::new(ProcessorConfig::BlockProcessor, database_url, Network::Mainnet, None, None)
            .await?;

    worker.run().await;
    Ok(())
}
