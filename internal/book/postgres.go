package book

import (
	"context"
	"fmt"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/kkhaniffff/bookstore/internal/errors"
)

type PostgresRepository struct {
	pool *pgxpool.Pool
}

func NewPostgresRepository(pool *pgxpool.Pool) *PostgresRepository {
	return &PostgresRepository{pool: pool}
}

func (r *PostgresRepository) GetAll(ctx context.Context) ([]Book, error) {
	query := `SELECT id, title, author, price, quantity FROM books`

	rows, err := r.pool.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("error querying books: %w", err)
	}
	defer rows.Close()

	var books []Book
	for rows.Next() {
		var b Book
		err := rows.Scan(&b.ID, &b.Title, &b.Author, &b.Price, &b.Quantity)
		if err != nil {
			return nil, fmt.Errorf("error mapping book: %w", err)
		}
		books = append(books, b)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating books: %w", err)
	}

	if books == nil {
		books = []Book{}
	}

	return books, nil
}

func (r *PostgresRepository) GetByID(ctx context.Context, id uuid.UUID) (Book, error) {
	query := `SELECT id, title, author, price, quantity FROM books WHERE id = $1`

	var b Book
	err := r.pool.QueryRow(ctx, query, id).Scan(&b.ID, &b.Title, &b.Author, &b.Price, &b.Quantity)

	if err == pgx.ErrNoRows {
		return Book{}, errors.NewNotFoundError(fmt.Sprintf("book not found: %s", id))
	}
	if err != nil {
		return Book{}, fmt.Errorf("error getting book: %s", id)
	}

	return b, nil
}

func (r *PostgresRepository) Save(ctx context.Context, b Book) (Book, error) {
	if b.ID == uuid.Nil {
		b.ID = uuid.New()
	}

	query := `
		INSERT INTO books (id, title, author, price, quantity) 
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			title = EXCLUDED.title,
			author = EXCLUDED.author,
			price = EXCLUDED.price,
			quantity = EXCLUDED.quantity
		RETURNING id, title, author, price, quantity`

	var saved Book
	err := r.pool.QueryRow(ctx, query,
		b.ID, b.Title, b.Author, b.Price, b.Quantity,
	).Scan(&saved.ID, &saved.Title, &saved.Author, &saved.Price, &saved.Quantity)

	if err != nil {
		return Book{}, fmt.Errorf("error saving book: %w", err)
	}

	return saved, nil
}

func (r *PostgresRepository) Delete(ctx context.Context, id uuid.UUID) error {
	query := `DELETE FROM books WHERE id = $1`

	result, err := r.pool.Exec(ctx, query, id)
	if err != nil {
		return fmt.Errorf("error deleting book: %w", err)
	}

	if result.RowsAffected() == 0 {
		return errors.NewNotFoundError(fmt.Sprintf("book not found: %s", id))
	}

	return nil
}
