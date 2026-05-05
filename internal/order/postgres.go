package order

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/kkhaniffff/bookstore/internal/postgres"
)

type PostgresRepository struct {
	pool *pgxpool.Pool
}

func NewPostgresRepository(pool *pgxpool.Pool) *PostgresRepository {
	return &PostgresRepository{pool: pool}
}

func (r *PostgresRepository) Create(ctx context.Context, order Order) error {
	query := `
		INSERT INTO orders (id, status, total, created_at) 
		VALUES ($1, $2, $3, $4)`

	executor := postgres.GetExecutor(ctx, r.pool)
	_, err := executor.Exec(ctx, query,
		order.ID, order.Status, order.Total, order.CreatedAt,
	)

	if err != nil {
		return fmt.Errorf("error creating order: %w", err)
	}

	for _, item := range order.Items {
		query := `
			INSERT INTO order_items (id, order_id, book_id, quantity, price)
			VALUES ($1, $2, $3, $4, $5)`
		_, err := executor.Exec(ctx, query,
			item.ID, order.ID, item.BookID, item.Quantity, item.Price,
		)

		if err != nil {
			return fmt.Errorf("error creating order item: %w", err)
		}
	}

	return nil
}

func (r *PostgresRepository) GetAll(ctx context.Context) ([]Order, error) {
	query := `
        SELECT o.id, o.status, o.total, o.created_at,
               oi.id, oi.book_id, oi.quantity, oi.price
        FROM orders o
        INNER JOIN order_items oi ON oi.order_id = o.id
        ORDER BY o.created_at DESC`

	rows, err := postgres.GetExecutor(ctx, r.pool).Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("error querying orders: %w", err)
	}
	defer rows.Close()

	ordersMap := make(map[uuid.UUID]*Order)
	for rows.Next() {
		var (
			orderID uuid.UUID
			status OrderStatus
			total int
			createdAt time.Time
			itemID uuid.UUID
			bookID uuid.UUID
			quantity int
			price int
		)

		err := rows.Scan(&orderID, &status, &total, &createdAt,
		&itemID, &bookID, &quantity, &price)
		if err != nil {
			return nil, fmt.Errorf("error mapping order: %w", err)
		}

		order, exists := ordersMap[orderID]
		if !exists {
			order = &Order{
				ID: orderID,
				Status: status,
				Total: total,
				CreatedAt: createdAt,
				Items: []OrderItem{},
			}

			ordersMap[orderID] = order
		}

		order.Items = append(order.Items, OrderItem{
			ID: itemID,
			BookID: bookID,
			Quantity: quantity,
			Price: price,
		})
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating orders: %w", err)
	}

	orders := make([]Order, 0, len(ordersMap))
	for _, order := range ordersMap {
		orders = append(orders, *order)
	}

	return orders, nil
}
