package book

import (
	"net/http"

	"github.com/google/uuid"
	"github.com/kkhaniffff/bookstore/internal/errors"
	"github.com/kkhaniffff/bookstore/internal/json"
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
	filter := NewFilterInput(r.URL.Query())
	books, err := h.service.GetAll(r.Context(), filter)
	if err != nil {
		json.WriteError(w, err)
		return
	}
	json.Encode(w, http.StatusOK, books)
	r.URL.Query()
}

func (h *Handler) handleCreateBook(w http.ResponseWriter, r *http.Request) {
	input, err := json.Decode[CreateInput](r)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	book, err := h.service.Create(r.Context(), input)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	json.Encode(w, http.StatusCreated, book)
}

func (h *Handler) handleUpdateBook(w http.ResponseWriter, r *http.Request) {
	id, err := uuid.Parse(r.PathValue("id"))
	if err != nil {
		json.WriteError(w, errors.NewBadRequestError("invalid uuid"))
		return
	}

	input, err := json.Decode[UpdateInput](r)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	book, err := h.service.Update(r.Context(), id, input)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	json.Encode(w, http.StatusOK, book)
}

func (h *Handler) handleDeleteBook(w http.ResponseWriter, r *http.Request) {
	id, err := uuid.Parse(r.PathValue("id"))
	if err != nil {
		json.WriteError(w, errors.NewBadRequestError("invalid uuid"))
		return
	}

	err = h.service.Delete(r.Context(), id)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}
