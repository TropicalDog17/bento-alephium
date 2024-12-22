use bento_alephium::{db::initialize_db_pool, indexer::IndexerService};
use bento_alephium::indexer::BlockIndexer;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    
    dotenvy::dotenv().expect("Error while loading .env file");
    env_logger::init();

    let pool = initialize_db_pool();
    let mut indexer = IndexerService::new(pool, bento_alephium::client::Network::Testnet);

    loop {
        indexer.index_new_blocks().await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
