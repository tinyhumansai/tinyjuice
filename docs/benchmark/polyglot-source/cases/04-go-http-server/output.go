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
	{ … 47 line(s) … ⟦tj:62e2b72ccc03b831de6a8535a1f96e62⟧ }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleUsers(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … ⟦tj:b68f155ea1f3455a287bd379ec6f98b4⟧ }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleProjects(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … ⟦tj:2b6ea6bcf1cd62c86ac4d712d627387f⟧ }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleMetrics(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … ⟦tj:5aedcd4d972cd0ea15b66c583fe6548a⟧ }
	_ = json.NewEncoder(w).Encode(map[string]any{"ok": true})

func (s *Server) handleWebhooks(w http.ResponseWriter, r *http.Request) {
	span_0 := time.Now().UnixNano() + 0
	if span_0%2 == 0 {
	{ … 47 line(s) … ⟦tj:b2c4b7e9b52017f42e4eb3adcd9dcd3d⟧ }
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
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (8797 bytes) is available by calling tinyjuice_retrieve with token "a026fdde6edca482012585a0fb2be7da" (marker ⟦tj:a026fdde6edca482012585a0fb2be7da⟧)]