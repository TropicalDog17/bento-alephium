pub mod default_processor;


use anyhow::Error;
use async_trait::async_trait;
use crate::types::{BlocksAndEventsPerTimestampRange};
use std::fmt::Debug;

#[async_trait]
pub trait ProcessorTrait: Send + Sync + Debug {
    async fn process_blocks_and_events(&self,blocks_and_events: BlocksAndEventsPerTimestampRange) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait Processable
where
    Self:  Send + Sized + 'static,
{
    type Input: Send + 'static;
    type Output: Send + 'static;

    /// Lifecycle methods
    async fn init(&mut self) {}
    async fn cleanup(
        &mut self,
    ) -> Result<Option<Vec<Self::Output>>, Error> {
        Ok(None)
    }

    /// Processes a batch of input items and returns a batch of output items.
    async fn process(
        &mut self,
        items: Self::Input,
    ) -> Result<Option<Self::Output>, Error>;
}