package order

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/kkhaniffff/bookstore/internal/book"
	"github.com/kkhaniffff/bookstore/internal/errors"
)

type TxManager interface {
	Do(ctx context.Context, fn func(ctx context.Context) error) error
}

type OrderRepository interface {
	Create(ctx context.Context, order Order) error
	GetAll(ctx context.Context) ([]Order, error)
}

type BookRepository interface {
	GetByID(ctx context.Context, id uuid.UUID) (book.Book, error)
	DecreaseStock(ctx context.Context, id uuid.UUID, quantity int) error
}

type Service struct {
	tx        TxManager
	orderRepo OrderRepository
	bookRepo  BookRepository
}

func NewService(tx TxManager, orderRepo OrderRepository, bookRepo BookRepository) *Service {
	return &Service{tx: tx, orderRepo: orderRepo, bookRepo: bookRepo}
}

func (s *Service) Create(ctx context.Context, items []CreateInput) (Order, error) {
	if len(items) == 0 {
		return Order{}, errors.NewBadRequestError("order must contain at least one item")
	}

	order := Order{
		ID:        uuid.New(),
		Status:    StatusCreated,
		CreatedAt: time.Now(),
	}

	err := s.tx.Do(ctx, func(txCtx context.Context) error {
		for _, item := range items {
			if problems := item.Valid(); len(problems) > 0 {
				return errors.NewBadRequestWithErrors("validation failed", problems)
			}

			book, err := s.bookRepo.GetByID(txCtx, item.BookID)
			if err != nil {
				return err
			}

			if err := s.bookRepo.DecreaseStock(txCtx, item.BookID, item.Quantity); err != nil {
				return err
			}

			order.Items = append(order.Items, OrderItem{
				ID:       uuid.New(),
				BookID:   item.BookID,
				Quantity: item.Quantity,
				Price:    book.Price,
			})
			order.Total += book.Price * item.Quantity
		}

		err := s.orderRepo.Create(txCtx, order)
		if err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		return Order{}, err
	}

	return order, nil
}

func (s *Service) GetAll(ctx context.Context) ([]Order, error) {
	return s.orderRepo.GetAll(ctx)
}
