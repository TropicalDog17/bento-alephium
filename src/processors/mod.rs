pub mod default_processor;


use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use crate::types::{BlockAndEvents, BlocksAndEventsPerTimestampRange, Transaction};
use std::fmt::Debug;

#[async_trait]
#[enum_dispatch]
pub trait ProcessorTrait: Send + Sync + Debug {
    async fn process_blocks_and_events(&self,blocks_and_events: BlocksAndEventsPerTimestampRange) -> Result<(), anyhow::Error>;
}