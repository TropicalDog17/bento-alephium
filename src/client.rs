use crate::types::{
    BlockAndEvents, BlockEntry, BlockHash, BlockHeaderEntry, BlocksAndEventssPerTimesStampRange,
    BlocksPerTimesStampRange, Transaction,
};
use anyhow::Result;
use url::Url;

pub enum Network {
    Testnet,
    Mainnet,
    Custom(String),
}

impl Network {
    pub fn base_url(&self) -> String {
        match self {
            Network::Testnet => "https://node.testnet.alephium.org".to_owned(),
            Network::Mainnet => "https://node.mainnet.alephium.org".to_owned(),
            Network::Custom(url) => url.clone(),
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::Mainnet
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    inner: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Self { inner: reqwest::Client::new(), base_url }
    }

    // List blocks on the given time interval.
    // GET:/blockflow/blocks
    pub async fn get_blocks(&self, from_ts: u64, to_ts: u64) -> Result<BlocksPerTimesStampRange> {
        let endpoint = format!("/blockflow/blocks?fromTs={}&toTs={}", from_ts, to_ts);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    // List blocks with events on the given time interval.
    // GET:/blockflow/blocks-with-events
    pub async fn get_blocks_and_events(
        &self,
        from_ts: u64,
        to_ts: u64,
    ) -> Result<BlocksAndEventssPerTimesStampRange> {
        let endpoint = format!("/blockflow/blocks-with-events?fromTs={}&toTs={}", from_ts, to_ts);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    // Get a block with hash.
    // GET:/blockflow/blocks/{block_hash}
    pub async fn get_block(&self, block_hash: &BlockHash) -> Result<BlockEntry> {
        let endpoint = format!("/blockflow/blocks/{}", block_hash);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    // Get a block and events with hash.
    // GET:/blockflow/blocks-with-events/{block_hash}
    pub async fn get_block_and_events(&self, block_hash: &BlockHash) -> Result<BlockAndEvents> {
        let endpoint = format!("/blockflow/blocks-with-events/{}", block_hash);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    // Get block header.
    // GET:/blockflow/headers/{block_hash}
    pub async fn get_block_header(&self, block_hash: &BlockHash) -> Result<BlockHeaderEntry> {
        let endpoint = format!("/blockflow/headers/{}", block_hash);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    // Get transaction details.
    // GET:/transactions/details/{txId}
    pub async fn get_transaction(&self, tx_id: String) -> Result<Transaction> {
        let endpoint = format!("/transactions/details/{}", tx_id);
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint)).unwrap();
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }
}
