use crate::{
    db::DbPool,
    models::{self, BlockModel, EventModel, TransactionModel}, schema::blocks::chain_from,
};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;

pub struct BlockRepository {
    pub pool: DbPool,
}

impl BlockRepository {
    pub fn find_by_hashes(&self, hashes: Vec<String>) -> Result<Vec<BlockModel>> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        blocks.filter(hash.eq_any(hashes)).load::<BlockModel>(&mut conn).map_err(anyhow::Error::msg)
    }

    pub fn find_by_hash(&self, hash_val: &str) -> Result<Option<BlockModel>> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        blocks
            .filter(hash.eq(hash_val))
            .first::<BlockModel>(&mut conn)
            .optional()
            .map_err(anyhow::Error::msg)
    }

    pub fn find_by_height(&self, height_val: i64) -> Result<Option<BlockModel>> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        blocks
            .filter(height.eq(height_val))
            .first::<BlockModel>(&mut conn)
            .optional()
            .map_err(anyhow::Error::msg)
    }

    pub fn store_block(&self, block: BlockModel) -> Result<BlockModel> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(blocks)
            .values(&block)
            .on_conflict(hash)
            .do_update()
            .set(&block)
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn store_batch(&self, block_values: &[BlockModel]) -> Result<Vec<BlockModel>, anyhow::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(blocks)
            .values(block_values)
            .on_conflict_do_nothing()
            .returning(BlockModel::as_returning())
            .get_results(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn update_block(&self, hash_val: &str, block: BlockModel) -> Result<BlockModel> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::update(blocks.filter(hash.eq(hash_val)))
            .set(&block)
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn delete_block(&self, hash_val: &str) -> Result<usize> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::delete(blocks.filter(hash.eq(hash_val)))
            .execute(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn list_blocks(&self, limit: i64, offset: i64) -> Result<Vec<BlockModel>> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        blocks
            .order(height.desc())
            .limit(limit)
            .offset(offset)
            .load::<BlockModel>(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn get_latest_height(&self, chain_id: i64) -> Result<i64, anyhow::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        blocks
            .filter(chain_from.eq(chain_id))
            .select(height)
            .order(height.desc())
            .first::<i64>(&mut conn)
            .map_err(anyhow::Error::msg)
    }
}

pub struct EventRepository {
    pub pool: DbPool,
}

impl EventRepository {
    pub fn find_by_tx_id(&self, tx_id_val: &str) -> Result<Option<EventModel>> {
        use crate::schema::events::dsl::*;
        let mut conn = self.pool.get().unwrap();
        events
            .filter(tx_id.eq(tx_id_val))
            .select(EventModel::as_select())
            .first::<EventModel>(&mut conn)
            .optional()
            .map_err(anyhow::Error::msg)
    }

    pub fn store_event(&self, event: EventModel) -> Result<EventModel> {
        use crate::schema::events::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(events)
            .values(&event)
            .on_conflict((tx_id, event_index))
            .do_update()
            .set(&event)
            .returning(EventModel::as_select())
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn update_event(
        &self,
        tx_id_val: &str,
        event_index_val: i32,
        event: EventModel,
    ) -> Result<EventModel> {
        use crate::schema::events::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::update(events.filter(tx_id.eq(tx_id_val).and(event_index.eq(event_index_val))))
            .set(&event)
            .returning(EventModel::as_select())
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn delete_event(&self, tx_id_val: &str, event_index_val: i32) -> Result<usize> {
        use crate::schema::events::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::delete(events.filter(tx_id.eq(tx_id_val).and(event_index.eq(event_index_val))))
            .execute(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn list_events(&self, limit: i64, offset: i64) -> Result<Vec<EventModel>> {
        use crate::schema::events::dsl::*;
        let mut conn = self.pool.get().unwrap();
        events
            .select(EventModel::as_select())
            .order(tx_id.desc())
            .limit(limit)
            .offset(offset)
            .load::<EventModel>(&mut conn)
            .map_err(anyhow::Error::msg)
    }
}

pub struct TransactionRepository {
    pub pool: DbPool,
}

impl TransactionRepository {
    pub fn find_by_hash(&self, hash_val: &str) -> Result<Option<TransactionModel>> {
        use crate::schema::transactions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        transactions
            .filter(tx_hash.eq(hash_val))
            .first::<TransactionModel>(&mut conn)
            .optional()
            .map_err(anyhow::Error::msg)
    }

    pub fn store_transaction(&self, transaction: TransactionModel) -> Result<TransactionModel> {
        use crate::schema::transactions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(transactions)
            .values(&transaction)
            .on_conflict(tx_hash)
            .do_update()
            .set(&transaction)
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn update_transaction(
        &self,
        hash_val: &str,
        transaction: TransactionModel,
    ) -> Result<TransactionModel> {
        use crate::schema::transactions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::update(transactions.filter(tx_hash.eq(hash_val)))
            .set(&transaction)
            .get_result(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn delete_transaction(&self, hash_val: &str) -> Result<usize> {
        use crate::schema::transactions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::delete(transactions.filter(tx_hash.eq(hash_val)))
            .execute(&mut conn)
            .map_err(anyhow::Error::msg)
    }

    pub fn list_transactions(&self, limit: i64, offset: i64) -> Result<Vec<TransactionModel>> {
        use crate::schema::transactions::dsl::*;
        let mut conn = self.pool.get().unwrap();
        transactions
            .order(tx_hash.desc())
            .limit(limit)
            .offset(offset)
            .load::<TransactionModel>(&mut conn)
            .map_err(anyhow::Error::msg)
    }
}
