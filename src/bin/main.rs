use std::env;
use std::time::Duration;

use bento_alephium::ws::{self, WsClient};
use bento_alephium::{db::initialize_db_pool, indexer::IndexerService};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().expect("Error while loading .env file");
    env_logger::init();
    let (mut conn, _) = WsClient::connect_async(&env::var("WS_URL").expect("no WS_URL present")).await.expect("Failed to connect");

    conn.subscribe_blocks().await;
    let timer = tokio::time::Instant::now();
    let duration = Duration::new(10, 0);
    while let Some(message) = conn.as_mut().next().await {
        if timer.elapsed() >= duration {
            break;
        }
        match message {
            Ok(message) => {
                let data = message.into_data();
                let string_data = String::from_utf8(data.to_vec()).expect("Found invalid UTF-8 chars");
                tracing::info!("Received: {}", string_data);
            }
            Err(_) => break,
        }
    }
    conn.close().await.expect("Failed to disconnect");
    Ok(())
}
