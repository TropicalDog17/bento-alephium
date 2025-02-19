use bento_alephium::{api::index::start, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    println!("Starting server...");
    let config = Config::from_env().await?;
    start(config).await?;

    println!("Server is ready and running on http://0.0.0.0:8080");

    Ok(())
}
