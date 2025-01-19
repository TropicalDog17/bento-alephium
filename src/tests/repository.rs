use crate::{
    db::initialize_db_pool,
    models,
    repository::{BlockRepository, EventRepository, TransactionRepository},
    types::{BlockEntry, BlockHash, FixedAssetOutput, Hash, Transaction, UnsignedTx},
};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;
use serde_json::json;

fn mock_block() -> BlockEntry {
    BlockEntry {
        hash: BlockHash(
            "000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string(),
        ),
        timestamp: 1734442737861,
        chain_from: 0,
        chain_to: 0,
        height: 1625873,
        deps: vec![BlockHash(
            "0000003cda92c67b87dd368c28c60fe254371f206f4d3042988078be3e4ef625".to_string(),
        )],
        transactions: vec![Transaction {
            unsigned: UnsignedTx {
                tx_id: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a4"
                    .to_string(),
                version: 0,
                network_id: 1,
                script_opt: None,
                gas_amount: 20000,
                gas_price: "1000000000".to_string(),
                inputs: vec![],
                fixed_outputs: vec![],
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
fn test_block_repository() {
    dotenvy::from_path(".env.test").ok();
    let db_pool = initialize_db_pool();
    let repo = BlockRepository { pool: db_pool };
    let mock_block = mock_block();

    let block = models::BlockModel {
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
        uncles: json!([]),
        tx_number: mock_block.transactions.len() as i64,
        main_chain: true,
        hash_rate: BigDecimal::from_i64(2391273971).unwrap(),
        parent_hash: Some(mock_block.deps[0].0.clone()),
    };

    // Test store_block
    let stored_block = repo.store_block(block.clone()).unwrap();
    assert_eq!(stored_block.hash, mock_block.hash.0);

    // Test find_by_hash
    let found_block = repo.find_by_hash(&mock_block.hash.0).unwrap().unwrap();
    assert_eq!(found_block.hash, mock_block.hash.0);

    // Test find_by_height
    let found_block = repo.find_by_height(mock_block.height).unwrap().unwrap();
    assert_eq!(found_block.height, mock_block.height);

    // Test list_blocks
    let blocks = repo.list_blocks(10, 0).unwrap();
    assert!(!blocks.is_empty());

    // Test update_block
    let mut updated_block = block.clone();
    updated_block.main_chain = false;
    let updated = repo.update_block(&mock_block.hash.0, updated_block).unwrap();
    assert!(!updated.main_chain);

    // Test delete_block
    let deleted = repo.delete_block(&mock_block.hash.0).unwrap();
    assert_eq!(deleted, 1);
}

#[test]
fn test_event_repository() {
    dotenvy::from_path(".env.test").ok();
    let db_pool = initialize_db_pool();
    let repo = EventRepository { pool: db_pool };

    let event = models::EventModel {
        tx_id: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a4".to_string(),
        contract_address: "1AuWeE5Cwt2ES3473qnpKFV96z57CYL6mbTY7hva9Xz3h".to_string(),
        event_index: 0,
        fields: json!({}),
    };

    // Test store_event
    let stored_event = repo.store_event(event.clone()).unwrap();
    assert_eq!(stored_event.tx_id, event.tx_id);

    // Test find_by_tx_id
    let found_event = repo.find_by_tx_id(&event.tx_id).unwrap().unwrap();
    assert_eq!(found_event.tx_id, event.tx_id);

    // Test list_events
    let events = repo.list_events(10, 0).unwrap();
    assert!(!events.is_empty());

    // Test update_event
    let mut updated_event = event.clone();
    updated_event.fields = json!({"updated": true});
    let updated = repo.update_event(&event.tx_id, event.event_index, updated_event).unwrap();
    assert_eq!(updated.fields, json!({"updated": true}));

    // Test delete_event
    let deleted = repo.delete_event(&event.tx_id, event.event_index).unwrap();
    assert_eq!(deleted, 1);
}

#[test]
fn test_transaction_repository() {
    dotenvy::from_path(".env.test").ok();
    let db_pool = initialize_db_pool();
    let repo = TransactionRepository { pool: db_pool };

    let transaction = models::TransactionModel {
        tx_hash: "f8dd97f971f383f2554a075ac7665cf2a4280b12cea9f28bd63055c0de4764a4".to_string(),
        unsigned: json!({}),
        script_execution_ok: true,
        contract_inputs: json!({}),
        generated_outputs: json!({}),
        input_signatures: vec![],
        script_signatures: vec![],
        created_at: None,
        updated_at: None,
    };

    // Test store_transaction
    let stored_tx = repo.store_transaction(transaction.clone()).unwrap();
    assert_eq!(stored_tx.tx_hash, transaction.tx_hash);

    // Test find_by_hash
    let found_tx = repo.find_by_hash(&transaction.tx_hash).unwrap().unwrap();
    assert_eq!(found_tx.tx_hash, transaction.tx_hash);

    // Test list_transactions
    let transactions = repo.list_transactions(10, 0).unwrap();
    assert!(!transactions.is_empty());

    // Test update_transaction
    let mut updated_tx = transaction.clone();
    updated_tx.script_execution_ok = false;
    let updated = repo.update_transaction(&transaction.tx_hash, updated_tx).unwrap();
    assert!(!updated.script_execution_ok);

    // Test delete_transaction
    let deleted = repo.delete_transaction(&transaction.tx_hash).unwrap();
    assert_eq!(deleted, 1);
}
