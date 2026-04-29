package main

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

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

	go func() {
		slog.Info("Server starting", "addr", server.Addr)
		if err := server.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			slog.Error("Server failed to start", "error", err)
			os.Exit(1)
		}
	}()

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM)
	<-stop

	slog.Info("Server shutting down...")

	ctx, cancel := context.WithTimeout(context.Background(), 30 * time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		slog.Error("Server forced to shutdown", "error", err)
		os.Exit(1)
	}

	slog.Info("Server exited gracefully")
}
