ALTER TABLE orders
    DROP CONSTRAINT IF EXISTS orders_user_id_fkey;

ALTER TABLE orders
    DROP COLUMN IF EXISTS user_id;

