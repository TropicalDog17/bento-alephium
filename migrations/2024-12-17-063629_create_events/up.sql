-- Your SQL goes here
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    tx_id TEXT NOT NULL,
    contract_address TEXT NOT NULL,
    event_index INTEGER NOT NULL,
    fields JSONB NOT NULL,  
    
    -- Composite unique constraint to prevent duplicates
    UNIQUE(tx_id, contract_address, event_index),
    
    -- Event index should be non-negative
    CONSTRAINT event_index_positive CHECK (event_index >= 0)
);