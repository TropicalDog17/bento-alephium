use std::sync::Arc;

use anyhow::{Context, Result};

use crate::db::{new_db_pool, DbPool};
pub enum ProcessorConfig {
    DefaultProcessor,
    BlockProcessor,
    EventProcessor,
    LendingContractProcessor(String),
}

impl ProcessorConfig {
    pub fn name(&self) -> &'static str {
        match self {
            ProcessorConfig::DefaultProcessor => "default_processor",
            ProcessorConfig::BlockProcessor => "block_processor",
            ProcessorConfig::EventProcessor => "event_processor",
            ProcessorConfig::LendingContractProcessor(_) => "lending_contract_processor",
        }
    }
}

pub struct Config {
    pub db_client: Arc<DbPool>,
}

impl Config {
    pub async fn from_env() -> Result<Self> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_client =
            new_db_pool(&db_url, None).await.context("Failed to create connection pool")?;
        Ok(Self { db_client })
    }
}
