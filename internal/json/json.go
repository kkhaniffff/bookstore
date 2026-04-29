package json

import (
	"encoding/json"
	"net/http"

	"github.com/kkhaniffff/bookstore/internal/errors"
)

func Encode(w http.ResponseWriter, status int, v any) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(v); err != nil {
		return errors.NewInternalServerError(err)
	}
	return nil
}

func Decode[T any](r *http.Request) (T, error) {
	var v T
	if err := json.NewDecoder(r.Body).Decode(&v); err != nil {
		return v, errors.NewBadRequestError("invalid json")
	}
	return v, nil
}

func WriteError(w http.ResponseWriter, err error) error {
	w.Header().Set("Content-Type", "application/json")

	appErr, ok := err.(*errors.AppError)
	if !ok {
		appErr = errors.NewInternalServerError(err)
	}

	return Encode(w, appErr.Status, appErr)
}
