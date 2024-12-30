-- This file should undo anything in `up.sql`
- Start transaction
BEGIN;

-- Try to add the constraint
ALTER TABLE events 
ADD CONSTRAINT unique_tx_event UNIQUE (tx_id, event_index);

-- Check if there are any violations before committing
DO $$ 
BEGIN
    -- Check for duplicates that would violate the constraint
    IF EXISTS (
        SELECT tx_id, event_index, COUNT(*)
        FROM events
        GROUP BY tx_id, event_index
        HAVING COUNT(*) > 1
    ) THEN
        RAISE EXCEPTION 'Duplicate records found - constraint cannot be added';
    END IF;
EXCEPTION
    WHEN OTHERS THEN
        -- If any error occurs, rollback
        RAISE NOTICE 'Error occurred: %', SQLERRM;
        ROLLBACK;
        RETURN;
END $$;

-- If no violations found, commit the transaction
COMMIT;