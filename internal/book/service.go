package book

import (
	"github.com/google/uuid"
	apperror "github.com/kkhaniffff/bookstore/internal/error"
)

type Repository interface {
	GetAll() []Book
	GetByID(id uuid.UUID) (Book, error)
	Save(b Book) Book
	Delete(id uuid.UUID) error
}

type Service struct {
	repo Repository
}

func NewService(repo Repository) *Service {
	return &Service{repo: repo}
}

func (s *Service) GetAll() []Book {
	return s.repo.GetAll()
}

func (s *Service) Create(i CreateInput) (Book, error) {
	if problems := i.Valid(); len(problems) > 0 {
		return Book{}, apperror.NewBadRequestWithErrors("Validation failed", problems)
	}
	book := Book{
		ID:       uuid.New(),
		Title:    i.Title,
		Author:   i.Author,
		Price:    i.Price,
		Quantity: i.Quantity,
	}
	return s.repo.Save(book), nil
}

func (s *Service) Update(id uuid.UUID, i UpdateInput) (Book, error) {
	if problems := i.Valid(); len(problems) > 0 {
		return Book{}, apperror.NewBadRequestWithErrors("Validation failed", problems)
	}

	existing, err := s.repo.GetByID(id)
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

	return s.repo.Save(existing), nil
}

func (s *Service) Delete(id uuid.UUID) error {
	return s.repo.Delete(id)
}
