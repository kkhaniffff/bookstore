ALTER TABLE orders
    DROP CONSTRAINT IF EXISTS orders_user_fk;

ALTER TABLE orders
    DROP COLUMN IF EXISTS user_id;

