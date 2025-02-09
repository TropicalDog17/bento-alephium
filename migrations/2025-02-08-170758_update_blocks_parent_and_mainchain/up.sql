-- Your SQL goes here
ALTER TABLE blocks ADD COLUMN main_chain BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE transactions ADD COLUMN main_chain BOOLEAN NOT NULL DEFAULT FALSE; 
ALTER TABLE transactions ADD COLUMN block_hash TEXT NOT NULL;

ALTER TABLE blocks DROP COLUMN deps;
ALTER TABLE blocks 
  ADD COLUMN deps TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[]


