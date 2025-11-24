CREATE TABLE IF NOT EXISTS books (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    publication_date DATE,
    stock_quantity INTEGER NOT NULL,
    price INTEGER NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false
    );
