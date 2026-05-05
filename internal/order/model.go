package order

import (
	"time"

	"github.com/google/uuid"
)

type OrderStatus string

const (
	StatusCreated   OrderStatus = "created"
	StatusCancelled OrderStatus = "cancelled"
)

type Order struct {
	ID        uuid.UUID   `json:"id"`
	Total     int         `json:"total"`
	CreatedAt time.Time   `json:"created_at"`
	Status    OrderStatus `json:"status"`
	Items     []OrderItem `json:"items"`
}

type OrderItem struct {
	ID       uuid.UUID `json:"id"`
	Price    int       `json:"price"`
	Quantity int       `json:"quantity"`
	BookID   uuid.UUID `json:"book_id"`
}

type CreateInput struct {
	BookID   uuid.UUID `json:"book_id"`
	Quantity int       `json:"quantity"`
}

func (i CreateInput) Valid() map[string]string {
	problems := make(map[string]string)
	if i.BookID == uuid.Nil {
		problems["book_id"] = "book id is required"
	}
	if i.Quantity <= 0 {
		problems["quantity"] = "must be positive"
	}
	return problems
}
