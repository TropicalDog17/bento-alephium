use crate::{
    client::{Client, Network},
    models::BlockModel,
    types::BlockHash,
};

#[tokio::test]
async fn test_get_blocks() {
    let testnet_client = Client::new(Network::Testnet);
    let block_range = testnet_client.get_blocks(1734442735405, 1734442740808).await.unwrap();

    assert_eq!(block_range.blocks.len(), 16);
    assert_eq!(block_range.blocks.iter().filter(|&b| b.len() != 0).count(), 8);
}
#[tokio::test]
async fn test_get_blocks_and_events() {
    let testnet_client = Client::new(Network::Testnet);

    let block_and_event_range =
        testnet_client.get_blocks_and_events(1734442735405, 1734442740808).await.unwrap();

    assert_eq!(block_and_event_range.blocks_and_events.len(), 16);
    assert_eq!(block_and_event_range.blocks_and_events.iter().filter(|&b| b.len() != 0).count(), 8);
}

#[tokio::test]
async fn test_get_block() {
    let testnet_client = Client::new(Network::Testnet);

    let block = testnet_client
        .get_block(&BlockHash(
            "000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string(),
        ))
        .await
        .unwrap();
    println!("{:?}", block);
    assert_eq!(
        block.hash,
        BlockHash("000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string())
    );
}

#[tokio::test]
async fn test_get_block_and_events_by_hash() {
    let testnet_client = Client::new(Network::Testnet);

    let block_and_events = testnet_client
        .get_block_and_events_by_hash(&BlockHash(
            "000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string(),
        ))
        .await
        .unwrap();
    assert_eq!(
        block_and_events.block.hash,
        BlockHash("000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string())
    );
}

#[tokio::test]
async fn test_get_block_header() {
    let testnet_client = Client::new(Network::Testnet);

    let block_header_entry = testnet_client
        .get_block_header(&BlockHash(
            "000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string(),
        ))
        .await
        .unwrap();

    assert_eq!(
        block_header_entry.hash,
        BlockHash("000000406321d177ccb6bedcb3910c60494a9e6c0259a85c7ab40e7c66c5b8b0".to_string())
    );
}

#[tokio::test]
async fn test_get_transaction() {
    let testnet_client = Client::new(Network::Testnet);

    let transaction = testnet_client
        .get_transaction("8841d638216eab8dad91351eb5f46aeb7bff5bd53a4c261272fdca8550fb0284")
        .await
        .unwrap();

    assert_eq!(transaction.script_execution_ok, true);
    assert_eq!(
        transaction.unsigned.tx_id,
        "8841d638216eab8dad91351eb5f46aeb7bff5bd53a4c261272fdca8550fb0284"
    )
}
