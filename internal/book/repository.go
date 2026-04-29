package book

import (
	"fmt"
	"sync"

	"github.com/google/uuid"
	"github.com/kkhaniffff/bookstore/internal/errors"
)

type InMemoryRepository struct {
	mutex sync.RWMutex
	books map[uuid.UUID]Book
}

func NewInMemoryRepository() *InMemoryRepository {
	return &InMemoryRepository{
		books: make(map[uuid.UUID]Book),
	}
}

func (r *InMemoryRepository) GetAll() []Book {
	r.mutex.RLock()
	defer r.mutex.RUnlock()

	books := make([]Book, 0, len(r.books))
	for _, b := range r.books {
		books = append(books, b)
	}

	return books
}

func (r *InMemoryRepository) GetByID(id uuid.UUID) (Book, error) {
	r.mutex.RLock()
	defer r.mutex.RUnlock()

	book, ok := r.books[id]
	if !ok {
		return Book{}, errors.NewNotFoundError(fmt.Sprintf("book not found: %s", id))
	}

	return book, nil
}

func (r *InMemoryRepository) Save(b Book) Book {
	r.mutex.Lock()
	defer r.mutex.Unlock()

	r.books[b.ID] = b
	return b
}

func (r *InMemoryRepository) Delete(id uuid.UUID) error {
	r.mutex.Lock()
	defer r.mutex.Unlock()

	_, ok := r.books[id]
	if !ok {
		return errors.NewNotFoundError(fmt.Sprintf("book not found: %s", id))
	}

	delete(r.books, id)
	return nil
}
