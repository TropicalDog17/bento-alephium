use crate::{db::DbPool, models::{BlockModel, EventModel, TransactionModel}, processors::Processable, types::{BlockAndEvents, BlockEntry, BlocksAndEventsPerTimestampRange, Event, Transaction}};
use std::sync::Arc;
use async_trait::async_trait;
#[derive(Debug)]
pub struct DefaultExtractor {
    pub connection_pool: Arc<DbPool>,
}

#[async_trait]
impl Processable for DefaultExtractor 
where Self: Send + Sized + 'static  
{
    type Input = BlocksAndEventsPerTimestampRange;
    type Output = (Vec<BlockModel>, Vec<TransactionModel>, Vec<EventModel>);

    async fn process(
        &mut self, 
        blocks_and_events: BlocksAndEventsPerTimestampRange
    ) -> Result<Option<Self::Output>, anyhow::Error> {
        let mut blocks = vec![];
        let mut transactions = vec![];
        let mut events = vec![];
    
        for blocks_and_events_per_pair in blocks_and_events.blocks_and_events {
            let mut pair_blocks: Vec<BlockModel>;
            let mut pair_events: Vec<EventModel> = vec![];
            let mut pair_transactions: Vec<TransactionModel>;
    
            pair_blocks = blocks_and_events_per_pair.iter()
                .map(|entry| BlockEntry::into(entry.block.clone()))
                .collect::<Vec<_>>();
            
            blocks_and_events_per_pair.iter().for_each(|entry| {
                let events = entry.events.clone();
                let mut event_models = events.iter()
                    .map(|event| event.clone().into())
                    .collect::<Vec<EventModel>>();
                pair_events.append(&mut event_models);
            });
    
            pair_transactions = blocks_and_events_per_pair.iter()
                .flat_map(|entry| 
                    entry.block.transactions.iter()
                        .map(|tx| tx.clone().into())
                        .collect::<Vec<TransactionModel>>()
                )
                .collect();
    
            blocks.append(&mut pair_blocks);
            transactions.append(&mut pair_transactions);
            events.append(&mut pair_events);  
        }
    
        Ok(Some((blocks, transactions, events)))
    }
}