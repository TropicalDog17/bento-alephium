use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DbConfig {
    PostgresConfig(PostgresConfig),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PostgresConfig {
    pub url: String,
    // Size of the pool for writes/reads to the DB. Limits maximum number of queries in flight
    #[serde(default = "PostgresConfig::default_pool_size")]
    pub pool_size: u32,
}

impl PostgresConfig {
    pub const fn default_pool_size() -> u32 {
        150
    }
}
