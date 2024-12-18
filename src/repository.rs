use crate::{db::DbPool, models::Block, schema::blocks::hash};
use diesel::prelude::*;

pub struct BlockRepository {
    pub pool: DbPool,
}

impl BlockRepository {
    pub fn find_by_hashes(&self, hashes: Vec<String>) -> Result<Vec<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let results = blocks
            .filter(hash.eq_any(hashes))
            .load::<Block>(&mut conn)
            .expect("Error loading blocks");
        Ok(results)
    }

    pub fn find_by_hash(&self, hash_val: &str) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let result = blocks.filter(hash.eq(hash_val)).first::<Block>(&mut conn)?;
        Ok(Some(result))
    }

    pub fn find_by_height(&self, height_val: i64) -> Result<Option<Block>, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        let result = blocks.filter(height.eq(height_val)).first::<Block>(&mut conn).optional()?;
        Ok(result)
    }

    pub fn store_block(&self, block: Block) -> Result<Block, diesel::result::Error> {
        use crate::schema::blocks::dsl::*;
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(blocks).values(block).get_result(&mut conn)
    }
}
