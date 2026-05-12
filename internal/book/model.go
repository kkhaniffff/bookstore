package book

import (
	"net/url"
	"strconv"

	"github.com/google/uuid"
)

type Book struct {
	ID       uuid.UUID `json:"id"`
	Title    string    `json:"title"`
	Author   string    `json:"author"`
	Price    int       `json:"price"`
	Quantity int       `json:"quantity"`
}

type CreateInput struct {
	Title    string `json:"title"`
	Author   string `json:"author"`
	Price    int    `json:"price"`
	Quantity int    `json:"quantity"`
}

func (i CreateInput) Valid() map[string]string {
	problems := make(map[string]string)
	if i.Title == "" {
		problems["title"] = "title is required"
	}
	if i.Author == "" {
		problems["author"] = "author is required"
	}
	if i.Price < 0 {
		problems["price"] = "must be non-negative"
	}
	if i.Quantity < 0 {
		problems["quantity"] = "must be non-negative"
	}
	return problems
}

type UpdateInput struct {
	Title    *string `json:"title,omitempty"`
	Author   *string `json:"author,omitempty"`
	Price    *int    `json:"price,omitempty"`
	Quantity *int    `json:"quantity,omitempty"`
}

func (i UpdateInput) Valid() map[string]string {
	problems := make(map[string]string)
	if i.Title != nil && *i.Title == "" {
		problems["title"] = "cannot be empty"
	}
	if i.Author != nil && *i.Author == "" {
		problems["author"] = "cannot be empty"
	}
	if i.Price != nil && *i.Price < 0 {
		problems["price"] = "must be non-negative"
	}
	if i.Quantity != nil && *i.Quantity < 0 {
		problems["quantity"] = "must be non-negative"
	}
	return problems
}

type FilterInput struct {
	Title    string
	Author   string
	MinPrice *int
	MaxPrice *int
}

func NewFilterInput(q url.Values) FilterInput {
	i := FilterInput{
		Title:  q.Get("title"),
		Author: q.Get("author"),
	}

	if minPrice := q.Get("min_price"); minPrice != "" {
		if v, err := strconv.Atoi(minPrice); err == nil {
			i.MinPrice = &v
		}
	}
	if maxPrice := q.Get("max_price"); maxPrice != "" {
		if v, err := strconv.Atoi(maxPrice); err == nil {
			i.MaxPrice = &v
		}
	}

	return i
}
