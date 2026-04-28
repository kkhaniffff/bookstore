package main

import (
	"net/http"

	"github.com/kkhaniffff/bookstore/internal/book"
)

func main() {
	bookRepo := book.NewInMemoryRepository()
	bookService := book.NewService(bookRepo)
	bookHandler := book.NewHandler(bookService)

	mux := http.NewServeMux()
	bookHandler.RegisterRoutes(mux)

	server := &http.Server{
		Addr: ":8080",
		Handler: mux,
	}

	server.ListenAndServe()
}
