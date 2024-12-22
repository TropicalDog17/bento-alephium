use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Hash(pub String);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHash(pub String);

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeaderEntry {
    pub hash: BlockHash,
    pub timestamp: i64,
    pub chain_from: i64,
    pub chain_to: i64,
    pub height: i64,
    pub deps: Vec<BlockHash>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockEntry {
    pub hash: BlockHash,
    pub timestamp: i64,
    pub chain_from: i64,
    pub chain_to: i64,
    pub height: i64,
    pub deps: Vec<BlockHash>,
    pub transactions: Vec<Transaction>,
    pub nonce: String,
    pub version: i8,
    pub dep_state_hash: Hash,
    pub txs_hash: Hash,
    pub target: String,
    pub ghost_uncles: Vec<GhostUncleBlockEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GhostUncleBlockEntry {
    pub block_hash: BlockHash,
    pub miner: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlocksPerTimestampRange {
    pub blocks: Vec<Vec<BlockEntry>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockAndEvents {
    pub block: BlockEntry,
    pub events: Vec<ContractEventByBlockHash>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlocksAndEventsPerTimestampRange {
    pub blocks_and_events: Vec<Vec<BlockAndEvents>>,
}

#[derive(Deserialize, Debug)]
pub enum Val {
    Address { value: String, typ: String },
    Array { value: Vec<Val>, typ: String },
    Bool { value: bool, typ: String },
    ByteVec { value: String, typ: String },
    I256 { value: String, typ: String },
    U256 { value: String, typ: String },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractEventByBlockHash {
    pub tx_id: String,
    pub contract_address: String,
    pub event_index: i32,
    pub fields: Vec<Val>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnsignedTx {
    pub tx_id: String,
    pub version: i32,
    pub network_id: i32,
    pub script_opt: Option<String>,
    pub gas_amount: i32,
    pub gas_price: String,
    pub inputs: Vec<AssetInput>,
    pub fixed_outputs: Vec<FixedAssetOutput>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub unsigned: UnsignedTx,
    pub script_execution_ok: bool,
    pub contract_inputs: Vec<OutputRef>,
    pub generated_outputs: Vec<Output>,
    pub input_signatures: Vec<String>,
    pub script_signatures: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputRef {
    pub hint: i32,
    pub key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractOutput {
    pub hint: i32,
    pub key: String,
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub typ: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetInput {
    pub output_ref: OutputRef,
    pub unlock_script: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetOutput {
    pub hint: i32,
    pub key: String,
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub lock_time: i64,
    pub message: String,
    pub typ: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FixedAssetOutput {
    pub hint: i32,
    pub key: String,
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub lock_time: i64,
    pub message: String,
}

pub struct TimestampRange {
    from: u64,
    to: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_hash_display() {
        let hash =
            Hash("00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850".to_string());
        assert_eq!(
            format!("{}", hash),
            "00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850"
        );

        let block_hash = BlockHash(
            "00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850".to_string(),
        );
        assert_eq!(
            format!("{}", block_hash),
            "00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850"
        );
    }

    #[test]
    fn test_block_entry_deser() {
        let json_data = json!({
            "hash": "00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850",
            "timestamp": 1672531200,
            "chainFrom": 1,
            "chainTo": 2,
            "height": 1000,
            "deps": ["hash1", "hash2"],
            "transactions": [],
            "nonce": "nonce_value",
            "version": 1,
            "depStateHash": "dep_hash",
            "txsHash": "txs_hash",
            "target": "target_value",
            "ghostUncles": [
                {
                    "blockHash": "unclehash1",
                    "miner": "miner1"
                }
            ]
        });

        let block: BlockEntry = serde_json::from_value(json_data).unwrap();

        assert_eq!(
            block.hash.0,
            "00000000000006f8c2bcaac93c5a23df8fba7119ba139d80a49d0303bbf84850"
        );
        assert_eq!(block.timestamp, 1672531200);
        assert_eq!(block.chain_from, 1);
        assert_eq!(block.chain_to, 2);
        assert_eq!(block.height, 1000);
        assert_eq!(block.deps.len(), 2);
        assert_eq!(block.deps[0].0, "hash1");
        assert_eq!(block.nonce, "nonce_value");
        assert_eq!(block.version, 1);
        assert_eq!(block.dep_state_hash.0, "dep_hash");
        assert_eq!(block.txs_hash.0, "txs_hash");
        assert_eq!(block.target, "target_value");
        assert_eq!(block.ghost_uncles.len(), 1);
        assert_eq!(block.ghost_uncles[0].block_hash.0, "unclehash1");
        assert_eq!(block.ghost_uncles[0].miner, "miner1");
    }

    #[test]
    fn test_transaction_deser() {
        let json_data = json!({
            "unsigned": {
                "txId": "tx123",
                "version": 1,
                "networkId": 42,
                "scriptOpt": "script",
                "gasAmount": 1000,
                "gasPrice": "1000000000",
                "inputs": [],
                "fixedOutputs": []
            },
            "scriptExecutionOk": true,
            "contractInputs": [],
            "generatedOutputs": [],
            "inputSignatures": [],
            "scriptSignatures": []
        });

        let transaction: Transaction = serde_json::from_value(json_data).unwrap();

        assert_eq!(transaction.unsigned.tx_id, "tx123");
        assert!(transaction.script_execution_ok);
        assert_eq!(transaction.unsigned.version, 1);
    }

    #[test]
    fn test_blocks_and_events_deser() {
        let json_data = json!({
            "blocksAndEvents": [
                [
                    {
                        "block": {
                            "hash": "blockhash123",
                            "timestamp": 1672531200,
                            "chainFrom": 1,
                            "chainTo": 2,
                            "height": 1000,
                            "deps": ["hash1", "hash2"],
                            "transactions": [],
                            "nonce": "nonce_value",
                            "version": 1,
                            "depStateHash": "dep_hash",
                            "txsHash": "txs_hash",
                            "target": "target_value",
                            "ghostUncles": []
                        },
                        "events": []
                    }
                ]
            ]
        });

        let blocks_and_events: BlocksAndEventsPerTimestampRange =
            serde_json::from_value(json_data).unwrap();

        assert_eq!(blocks_and_events.blocks_and_events.len(), 1);
        let block_and_event = &blocks_and_events.blocks_and_events[0][0];
        assert_eq!(block_and_event.block.hash.0, "blockhash123");
    }
}
