package book

import (
	"context"

	"github.com/google/uuid"
	"github.com/kkhaniffff/bookstore/internal/errors"
)

type Repository interface {
	GetAll(ctx context.Context) ([]Book, error)
	GetByID(ctx context.Context, id uuid.UUID) (Book, error)
	Save(ctx context.Context, b Book) (Book, error)
	Delete(ctx context.Context, id uuid.UUID) error
}

type Service struct {
	repo Repository
}

func NewService(repo Repository) *Service {
	return &Service{repo: repo}
}

func (s *Service) GetAll(ctx context.Context) ([]Book, error) {
	return s.repo.GetAll(ctx)
}

func (s *Service) Create(ctx context.Context, i CreateInput) (Book, error) {
	if problems := i.Valid(); len(problems) > 0 {
		return Book{}, errors.NewBadRequestWithErrors("validation failed", problems)
	}
	book := Book{
		ID:       uuid.New(),
		Title:    i.Title,
		Author:   i.Author,
		Price:    i.Price,
		Quantity: i.Quantity,
	}
	return s.repo.Save(ctx, book)
}

func (s *Service) Update(ctx context.Context, id uuid.UUID, i UpdateInput) (Book, error) {
	if problems := i.Valid(); len(problems) > 0 {
		return Book{}, errors.NewBadRequestWithErrors("validation failed", problems)
	}

	existing, err := s.repo.GetByID(ctx, id)
	if err != nil {
		return Book{}, err
	}

	if i.Title != nil {
		existing.Title = *i.Title
	}
	if i.Author != nil {
		existing.Author = *i.Author
	}
	if i.Price != nil {
		existing.Price = *i.Price
	}
	if i.Quantity != nil {
		existing.Quantity = *i.Quantity
	}

	return s.repo.Save(ctx, existing)
}

func (s *Service) Delete(ctx context.Context, id uuid.UUID) error {
	return s.repo.Delete(ctx, id)
}
