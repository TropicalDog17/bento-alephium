use bento_alephium::{api::index::start, config::Config};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting server...");
    let config = Config::from_env().await?;
    start(config).await?;

    println!("Server is ready and running on http://0.0.0.0:8080");

    Ok(())
}
