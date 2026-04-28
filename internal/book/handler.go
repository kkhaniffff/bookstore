package book

import (
	"encoding/json"
	"errors"
	"net/http"

	"github.com/google/uuid"
	apperror "github.com/kkhaniffff/bookstore/internal/error"
)

type Handler struct {
	service *Service
}

func NewHandler(service *Service) *Handler {
	return &Handler{service: service}
}

func (h *Handler) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("GET /books", h.handleGetBooks)
	mux.HandleFunc("POST /books", h.handleCreateBook)
	mux.HandleFunc("PUT /books/{id}", h.handleUpdateBook)
	mux.HandleFunc("DELETE /books/{id}", h.handleDeleteBook)
}

func (h *Handler) handleGetBooks(w http.ResponseWriter, r *http.Request) {
	books := h.service.GetAll()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(books)
}

func (h *Handler) handleCreateBook(w http.ResponseWriter, r *http.Request) {
	var i CreateInput
	if err := json.NewDecoder(r.Body).Decode(&i); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}

	book, err := h.service.Create(i)
	if appErr, ok := errors.AsType[*apperror.AppError](err); ok {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(appErr.Status)
		json.NewEncoder(w).Encode(appErr)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(book)
}

func (h *Handler) handleUpdateBook(w http.ResponseWriter, r *http.Request) {
	id, err := uuid.Parse(r.PathValue("id"))
	if err != nil {
		http.Error(w, "invalid uuid", http.StatusBadRequest)
		return
	}

	var i UpdateInput
	if err := json.NewDecoder(r.Body).Decode(&i); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}

	book, err := h.service.Update(id, i)
	if appErr, ok := errors.AsType[*apperror.AppError](err); ok {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(appErr.Status)
		json.NewEncoder(w).Encode(appErr)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(book)
}

func (h *Handler) handleDeleteBook(w http.ResponseWriter, r *http.Request) {
	id, err := uuid.Parse(r.PathValue("id"))
	if err != nil {
		http.Error(w, "invalid uuid", http.StatusBadRequest)
		return
	}

	err = h.service.Delete(id)
	if appErr, ok := errors.AsType[*apperror.AppError](err); ok {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(appErr.Status)
		json.NewEncoder(w).Encode(appErr)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}
