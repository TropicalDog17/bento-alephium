CREATE TABLE blocks (
    hash TEXT NOT NULL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    chain_from BIGINT NOT NULL,
    chain_to BIGINT NOT NULL,
    height BIGINT NOT NULL,
    deps TEXT[],
    nonce TEXT NOT NULL,
    version TEXT NOT NULL,
    dep_state_hash TEXT NOT NULL,
    txs_hash TEXT NOT NULL,
    tx_number BIGINT NOT NULL,
    target TEXT NOT NULL,
    main_chain BOOLEAN NOT NULL,
    hash_rate NUMERIC NOT NULL,
    parent_hash TEXT,
    uncles JSONB NOT NULL DEFAULT '[]',
    
    CONSTRAINT chain_valid CHECK (chain_to >= chain_from),
    CONSTRAINT height_positive CHECK (height >= 0),
    CONSTRAINT tx_number_positive CHECK (tx_number >= 0),
    CONSTRAINT hash_rate_positive CHECK (hash_rate >= 0)
);