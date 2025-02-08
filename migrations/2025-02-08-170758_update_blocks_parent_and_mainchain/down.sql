-- This file should undo anything in `up.sql`
ALTER TABLE blocks DROP COLUMN parent;
ALTER TABLE blocks DROP COLUMN main_chain;