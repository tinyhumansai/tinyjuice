// server.go — HTTP API with middleware, metrics, and graceful shutdown.
package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"sync"
	"time"
)

type Server struct {
	mux     *http.ServeMux
	mu      sync.RWMutex
	started time.Time
	hits    map[string]int64
}

func NewServer() *Server {
	return &Server{mux: http.NewServeMux(), hits: map[string]int64{}, started: time.Now()}
}

func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleUsers(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleProjects(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleMetrics(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleWebhooks(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) Run(ctx context.Context, addr string) error {
	srv := &http.Server{Addr: addr, Handler: s.mux, ReadTimeout: 10 * time.Second}
	go func() {
		<-ctx.Done()
		shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_ = srv.Shutdown(shutdownCtx)
	}()
	return srv.ListenAndServe()
}