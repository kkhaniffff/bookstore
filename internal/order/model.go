package order

import (
	"net/url"
	"strconv"
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

type FilterInput struct {
	Status   *OrderStatus
	MinTotal *int
	MaxTotal *int
	From     *time.Time
	To       *time.Time
}

func NewFilterInput(q url.Values) FilterInput {
	var i FilterInput

	if status := q.Get("status"); status != "" {
		s := OrderStatus(status)
		i.Status = &s
	}
	if minTotal := q.Get("min_total"); minTotal != "" {
		if v, err := strconv.Atoi(minTotal); err == nil {
			i.MinTotal = &v
		}
	}
	if maxTotal := q.Get("max_total"); maxTotal != "" {
		if v, err := strconv.Atoi(maxTotal); err == nil {
			i.MaxTotal = &v
		}
	}
	if from := q.Get("from"); from != "" {
		if t, err := time.Parse(time.RFC3339, from); err == nil {
			i.From = &t
		}
	}
	if to := q.Get("to"); to != "" {
		if t, err := time.Parse(time.RFC3339, to); err == nil {
			i.To = &t
		}
	}

	return i
}
