-- +goose Up
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY,
    total INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL,
    book_id UUID NOT NULL,
    price INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders (id),
    FOREIGN KEY (book_id) REFERENCES books (id)
    );

-- +goose Down
DROP TABLE IF EXISTS order_items;

DROP TABLE IF EXISTS orders;
