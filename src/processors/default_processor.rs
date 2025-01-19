use crate::{db::DbPool, models::{BlockModel, EventModel, TransactionModel}, types::{BlockAndEvents, BlockEntry, BlocksAndEventsPerTimestampRange, Event, Transaction}};
use std::sync::Arc;
use async_trait::async_trait;
use super::ProcessorTrait;
#[derive(Debug)]
pub struct DefaultProcessor {
    pub connection_pool: Arc<DbPool>,
}

#[async_trait]
impl ProcessorTrait for DefaultProcessor {
    async fn process_blocks_and_events(&self, blocks_and_events: BlocksAndEventsPerTimestampRange) -> Result<(), anyhow::Error> {
            todo!()
        }
    }


pub fn process_blocks_and_events(blocks_and_events: BlocksAndEventsPerTimestampRange) -> (Vec<BlockModel>, Vec<TransactionModel>, Vec<EventModel>){
    let mut blocks = vec![];
    let mut transactions = vec![];
    let mut events = vec![];

    for blocks_and_events_per_pair in blocks_and_events.blocks_and_events {
        let mut pair_blocks: Vec<BlockModel> = vec![];
        let mut pair_events: Vec<EventModel> = vec![];
        let mut pair_transactions: Vec<TransactionModel> = vec![];

        pair_blocks = blocks_and_events_per_pair.iter().map(|entry| BlockEntry::into(entry.block.clone())).collect::<Vec<_>>();
        // Vec<BlockAndEvents> -> Vec<Vec<Events>> -> Vec<Events>
        let  _ = blocks_and_events_per_pair.iter().map(|entry| {
            let events = entry.events.clone();
            let mut event_models = events.iter().map(|event| {
                event.clone().into()
            }).collect::<Vec<EventModel>>();
            pair_events.append(&mut event_models);
        });
        

        pair_transactions = blocks_and_events_per_pair.iter().flat_map(|entry| {
            entry.block.transactions.iter().map(|tx| {
                tx.clone().into()
            }).collect::<Vec<TransactionModel>>()
        }).collect::<Vec<TransactionModel>>();  

        blocks.append(&mut pair_blocks);
        transactions.append(&mut pair_transactions);
        events.append(&mut pair_events);  
    }

    (blocks, transactions, events)

}





