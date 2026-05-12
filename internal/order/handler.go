package order

import (
	"net/http"

	"github.com/kkhaniffff/bookstore/internal/json"
)

type Handler struct {
	service *Service
}

func NewHandler(service *Service) *Handler {
	return &Handler{service: service}
}

func (h *Handler) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("POST /orders", h.handleCreate)
	mux.HandleFunc("GET /orders", h.handleGetAll)
}

func (h *Handler) handleCreate(w http.ResponseWriter, r *http.Request) {
	input, err := json.Decode[[]CreateInput](r)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	order, err := h.service.Create(r.Context(), input)
	if err != nil {
		json.WriteError(w, err)
		return
	}

	json.Encode(w, http.StatusCreated, order)
}

func (h *Handler) handleGetAll(w http.ResponseWriter, r *http.Request) {
	filter := NewFilterInput(r.URL.Query())
	orders, err := h.service.GetAll(r.Context(), filter)
	if err != nil {
		json.WriteError(w, err)
		return
	}
	json.Encode(w, http.StatusOK, orders)
}
