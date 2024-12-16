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
pub struct BlockHeaderEntry {
    pub hash: BlockHash,
    pub timestamp: i64,
    pub chain_from: i64,
    pub chain_to: i64,
    pub height: i64,
    pub deps: Vec<BlockHash>,
}

#[derive(Deserialize, Debug)]
pub struct BlockEntry {
    pub hash: BlockHash,
    pub timestamp: i64,
    pub chain_from: i64,
    pub chain_to: i64,
    pub height: i64,
    pub deps: Vec<BlockHash>,
    pub transactions: Vec<Transaction>,
    pub nonce: String,
    pub version: String,
    pub dep_state_hash: Hash,
    pub txs_hash: Hash,
    pub target: String,
    pub ghost_uncles: Vec<GhostUncleBlockEntry>,
}

#[derive(Deserialize, Debug)]
pub struct GhostUncleBlockEntry {
    pub block_hash: BlockHash,
    pub miner: String,
}

#[derive(Deserialize, Debug)]
pub struct BlocksPerTimesStampRange {
    pub blocks: Vec<Vec<BlockEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct BlockAndEvents {
    pub block: BlockEntry,
    pub events: Vec<ContractEventByBlockHash>,
}

#[derive(Deserialize, Debug)]
pub struct BlocksAndEventssPerTimesStampRange {
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
pub struct ContractEventByBlockHash {
    pub tx_id: String,
    pub contract_address: String,
    pub event_index: i32,
    pub fields: Vec<Val>,
}

#[derive(Deserialize, Debug)]
pub struct UnsignedTx {
    pub tx_id: String,
    pub version: i32,
    pub network_id: i32,
    pub script_opt: String,
    pub gas_amount: i32,
    pub gas_price: String,
    pub inputs: Vec<AssetInput>,
    pub fixed_outputs: Vec<FixedAssetOutput>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub unsigned: UnsignedTx,
    pub script_execution_ok: bool,
    pub contract_inputs: Vec<OutputRef>,
    pub generated_outputs: Vec<Output>,
    pub input_signatures: Vec<String>,
    pub script_signatures: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct OutputRef {
    pub hint: i32,
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct Output {}

#[derive(Deserialize, Debug)]
pub struct ContractOutput {
    pub hint: i32,
    pub key: String,
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub typ: String,
}

#[derive(Deserialize, Debug)]
pub struct AssetInput {
    pub output_ref: OutputRef,
    pub unlock_script: String,
}

#[derive(Deserialize, Debug)]
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
pub struct Token {
    pub id: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct FixedAssetOutput {
    pub hint: i32,
    pub key: String,
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub lock_time: i64,
    pub message: String,
}
