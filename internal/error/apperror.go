package apperror

import (
	"fmt"
	"net/http"
)

type AppError struct {
	Message string `json:"message,omitempty"`
	Errors map[string]string `json:"errors,omitempty"`
	Status int `json:"-"`
}

func (e *AppError) Error() string {
	return fmt.Sprintf("%s: %s", http.StatusText(e.Status), e.Message)
}

func NewBadRequestError(message string) *AppError {
	return &AppError {
		Message: message,
		Status: http.StatusBadRequest,
	}
}

func NewBadRequestWithErrors(message string, errors map[string]string) *AppError {
	return &AppError {
		Message: message,
		Errors: errors,
		Status: http.StatusBadRequest,
	}
}

func NewNotFoundError(message string) *AppError {
	return &AppError {
		Message: message,
		Status: http.StatusNotFound,
	}
}
