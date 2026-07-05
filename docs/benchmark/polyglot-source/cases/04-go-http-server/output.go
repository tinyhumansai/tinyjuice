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
    { … 5 line(s) … ⟦tj:414cf7f43248996c5670df93689e649d⟧ }

func NewServer() *Server {
	return &Server{mux: http.NewServeMux(), hits: map[string]int64{}, started: time.Now()}
}

func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
    { … 51 line(s) … ⟦tj:62e2b72ccc03b831de6a8535a1f96e62⟧ }

func (s *Server) handleUsers(w http.ResponseWriter, r *http.Request) {
    { … 51 line(s) … ⟦tj:b68f155ea1f3455a287bd379ec6f98b4⟧ }

func (s *Server) handleProjects(w http.ResponseWriter, r *http.Request) {
    { … 51 line(s) … ⟦tj:2b6ea6bcf1cd62c86ac4d712d627387f⟧ }

func (s *Server) handleMetrics(w http.ResponseWriter, r *http.Request) {
    { … 51 line(s) … ⟦tj:5aedcd4d972cd0ea15b66c583fe6548a⟧ }

func (s *Server) handleWebhooks(w http.ResponseWriter, r *http.Request) {
    { … 51 line(s) … ⟦tj:b2c4b7e9b52017f42e4eb3adcd9dcd3d⟧ }

func (s *Server) Run(ctx context.Context, addr string) error {
    { … 9 line(s) … ⟦tj:de4851294ec25b80d5cacda547b92539⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (8797 bytes) is available by calling tinyjuice_retrieve with token "a026fdde6edca482012585a0fb2be7da" (marker ⟦tj:a026fdde6edca482012585a0fb2be7da⟧)]