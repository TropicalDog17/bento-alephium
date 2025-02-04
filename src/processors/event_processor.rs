use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::insert_into;
use diesel_async::RunQueryDsl;

use crate::{
    config::ProcessorConfig, db::DbPool, models::event::EventModel, types::BlockAndEvents,
};

use super::ProcessorTrait;

pub struct EventProcessor {
    connection_pool: Arc<DbPool>,
}

impl EventProcessor {
    pub fn new(connection_pool: Arc<DbPool>) -> Self {
        Self { connection_pool }
    }
}

impl Debug for EventProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "EventProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl ProcessorTrait for EventProcessor {
    fn name(&self) -> &'static str {
        ProcessorConfig::EventProcessor.name()
    }

    fn connection_pool(&self) -> &Arc<DbPool> {
        &self.connection_pool
    }

    async fn process_blocks(
        &self,
        _from: i64,
        _to: i64,
        blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()> {
        // Process blocks and insert to db
        let models = convert_to_model(blocks);
        insert_to_db(self.connection_pool.clone(), models).await?;
        Ok(())
    }
}

/// Insert events into the database.
pub async fn insert_to_db(db: Arc<DbPool>, events: Vec<EventModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::events::table).values(&events).execute(&mut conn).await?;
    Ok(())
}

pub fn convert_to_model(blocks: Vec<Vec<BlockAndEvents>>) -> Vec<EventModel> {
    let mut models = Vec::new();
    for bes in blocks {
        for be in bes {
            for e in be.events {
                models.push(EventModel {
                    tx_id: e.tx_id,
                    contract_address: e.contract_address,
                    event_index: e.event_index,
                    fields: serde_json::Value::Array(e.fields),
                });
            }
        }
    }
    models
}
