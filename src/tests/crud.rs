use crate::{
    db::initialize_db_pool,
    models,
    types::{BlockEntry, BlockHash, FixedAssetOutput, Hash, Transaction, UnsignedTx},
};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{dsl::insert_into, RunQueryDsl};

fn mock_block() -> BlockEntry {
    BlockEntry {
        hash: BlockHash(
            "000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string(),
        ),
        timestamp: 1734442737861,
        chain_from: 0,
        chain_to: 0,
        height: 1625873,
        deps: vec![
            BlockHash(
                "0000003cda92c67b87dd368c28c60fe254371f206f4d3042988078be3e4ef625".to_string(),
            ),
            BlockHash(
                "000000024402d0667fef893880d4ab02692837b1244e0ac802b4bc77a90a740a".to_string(),
            ),
            BlockHash(
                "0000000e049c30444155bfa8c3ff822d27b130208ac77b1fd6af7d4a325327ff".to_string(),
            ),
            BlockHash(
                "0000004f586b8c48f379bf26a55b62c16d18a4f8d8910fa75c073f8e1c7fbfa0".to_string(),
            ),
            BlockHash(
                "0000004a60cea8ba0a201a857fd58f3ae6de61daa1bc10f1728e19b821119131".to_string(),
            ),
            BlockHash(
                "0000002cbb895e44ea0a5fff9f60174acfb00b9303fbaf239358ab1e21abbff2".to_string(),
            ),
            BlockHash(
                "0000004f095b40186491bd6d9905704a8a371cd15b59c834b6589c729fdcdbb3".to_string(),
            ),
        ],
        transactions: vec![Transaction {
            unsigned: UnsignedTx {
                tx_id: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1"
                    .to_string(),
                version: 0,
                network_id: 1,
                script_opt: None,
                gas_amount: 20000,
                gas_price: "1000000000".to_string(),
                inputs: vec![],
                fixed_outputs: vec![FixedAssetOutput {
                    hint: -14043139,
                    key: "51245f230fc1b20b620f96b3bba0038ea064da388afb63038567d4add4cb642a"
                        .to_string(),
                    atto_alph_amount: "456886563259203887".to_string(),
                    address: "1AuWeE5Cwt2ES3473qnpKFV96z57CYL6mbTY7hva9Xz3h".to_string(),
                    tokens: vec![],
                    lock_time: 1734443337861,
                    message: "000000000193d4d7e0c500".to_string(),
                }],
            },
            script_execution_ok: true,
            contract_inputs: vec![],
            generated_outputs: vec![],
            input_signatures: vec![],
            script_signatures: vec![],
        }],
        nonce: "8428f2af2fae4dea24e357be5d2e5bace4d8d31860c73ed0".to_string(),
        version: 0,
        dep_state_hash: Hash(
            "e1b605b7740694b5f1436e906a77613ed84adda659cfc4d91a10faba6069220b".to_string(),
        ),
        txs_hash: Hash(
            "d791928ff3bd80f53e4d78c79e1070c2781b10fc0a385aeb0e05eccb1d52e38f".to_string(),
        ),
        target: "1d6cdf2d".to_string(),
        ghost_uncles: vec![],
    }
}

#[test]
fn test_crud_block() {
    use crate::schema::blocks::dsl::*;

    // Load .env.test file
    // TODO: setup db for testing
    dotenvy::from_path(".env.test").ok();
    let mock_block = mock_block();

    let block = models::Block {
        hash: mock_block.hash.0.clone(),
        timestamp: NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        chain_from: mock_block.chain_from,
        chain_to: mock_block.chain_to,
        height: mock_block.height,
        deps: Some(Vec::new()),
        nonce: mock_block.nonce,
        version: mock_block.version.to_string(),
        dep_state_hash: mock_block.dep_state_hash.0,
        txs_hash: mock_block.txs_hash.0,
        target: mock_block.target,
        uncles: serde_json::Value::String("[]".to_string()),
        tx_number: mock_block.transactions.len() as i64,
        main_chain: true,
        hash_rate: BigDecimal::from_i64(2391273971).unwrap(),
        parent_hash: Some(mock_block.deps[0].0.clone()),
    };

    let db_pool = initialize_db_pool();
    let mut conn = db_pool.get().unwrap();
    insert_into(blocks).values(&block).execute(&mut conn).unwrap();
    let results = blocks
        .filter(hash.eq(mock_block.hash.0.clone()))
        .select(models::Block::as_select())
        .load(&mut conn)
        .unwrap();
    println!("{:?}", results);
    assert_eq!(results.len(), 1);

    // Clean up
    diesel::delete(blocks.filter(hash.eq(mock_block.hash.0.clone()))).execute(&mut conn).unwrap();

    let results = blocks
        .filter(hash.eq(mock_block.hash.0.clone()))
        .select(models::Block::as_select())
        .load(&mut conn)
        .unwrap();

    assert_eq!(results.len(), 0);
}

#[test]
fn test_crud_event() {
    use crate::schema::events::dsl::*;

    // Load .env.test file
    dotenvy::from_path(".env.test").ok();
    let db_pool = initialize_db_pool();
    let mut conn = db_pool.get().unwrap();
    let event = models::Event {
        tx_id: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string(),
        contract_address: "1AuWeE5Cwt2ES3473qnpKFV96z57CYL6mbTY7hva9Xz3h".to_string(),
        event_index: 0,
        fields: serde_json::Value::String("{}".to_string()),
    };
    insert_into(events).values(&event).execute(&mut conn).unwrap();
    let results = events
        .filter(
            tx_id
                .eq("f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string()),
        )
        .select(models::Event::as_select())
        .load(&mut conn)
        .unwrap();
    println!("{:?}", results);
    assert_eq!(results.len(), 1);

    // Clean up
    diesel::delete(events.filter(
        tx_id.eq("f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string()),
    ))
    .execute(&mut conn)
    .unwrap();

    let results = events
        .filter(
            tx_id
                .eq("f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string()),
        )
        .select(models::Event::as_select())
        .load(&mut conn)
        .unwrap();

    assert_eq!(results.len(), 0);
}

#[test]
fn test_crud_transaction() {
    use crate::schema::transactions::dsl::*;

    // Load .env.test file
    dotenvy::from_path(".env.test").ok();
    let db_pool = initialize_db_pool();
    let mut conn = db_pool.get().unwrap();
    let transaction = models::Transaction {
        tx_hash: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string(),
        unsigned: serde_json::Value::String("{}".to_string()),
        script_execution_ok: true,
        contract_inputs: serde_json::Value::String("{}".to_string()),
        generated_outputs: serde_json::Value::String("{}".to_string()),
        input_signatures: vec![],
        script_signatures: vec![],
        created_at: None,
        updated_at: None,
    };

    insert_into(transactions).values(&transaction).execute(&mut conn).unwrap();
    let results = transactions
        .filter(
            tx_hash
                .eq("f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string()),
        )
        .select(models::Transaction::as_select())
        .load(&mut conn)
        .unwrap();
    println!("{:?}", results);
    assert_eq!(results.len(), 1);

    // Cleanup
    diesel::delete(transactions.filter(
        tx_hash.eq("f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a1".to_string()),
    ))
    .execute(&mut conn)
    .unwrap();
}
