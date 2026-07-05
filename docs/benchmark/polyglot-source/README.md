# Polyglot Source And XML

Representative TypeScript, Python, C++, Go, and Rust sources plus an XML document. The language-agnostic code compressor collapses deep bodies across all of them (tree-sitter grammars refine Rust/TS/Python), and XML routes through the readable-text extractor.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `03-cpp-geometry-engine` | [input](cases/03-cpp-geometry-engine/input.cpp) | [output](cases/03-cpp-geometry-engine/output.cpp) | [diff](cases/03-cpp-geometry-engine/compression.diff) | 12.5 KB -> 537 B (-96%) | 97.5% | 94.1% | 0.055 ms |
| `02-py-etl-pipeline` | [input](cases/02-py-etl-pipeline/input.py) | [output](cases/02-py-etl-pipeline/output.py) | [diff](cases/02-py-etl-pipeline/compression.diff) | 11.9 KB -> 1.3 KB (-89%) | 91.7% | 87.0% | 0.890 ms |
| `01-ts-api-client` | [input](cases/01-ts-api-client/input.ts) | [output](cases/01-ts-api-client/output.ts) | [diff](cases/01-ts-api-client/compression.diff) | 17.1 KB -> 2.2 KB (-87%) | 90.5% | 86.2% | 1.102 ms |
| `05-rs-lexer` | [input](cases/05-rs-lexer/input.rs) | [output](cases/05-rs-lexer/output.rs) | [diff](cases/05-rs-lexer/compression.diff) | 9.3 KB -> 1.3 KB (-86%) | 90.1% | 84.2% | 0.678 ms |
| `04-go-http-server` | [input](cases/04-go-http-server/input.go) | [output](cases/04-go-http-server/output.go) | [diff](cases/04-go-http-server/compression.diff) | 8.8 KB -> 1.4 KB (-85%) | 89.9% | 82.8% | 0.052 ms |
| `06-xml-maven-pom` | [input](cases/06-xml-maven-pom/input.xml) | [output](cases/06-xml-maven-pom/output.txt) | [diff](cases/06-xml-maven-pom/compression.diff) | 14.2 KB -> 3.3 KB (-77%) | 76.8% | 75.3% | 0.087 ms |

## What TinyJuice Is Doing

The brace-depth heuristic is language-agnostic, so TypeScript, C++, and Go compress with the same signature-preserving collapse as Rust; tree-sitter grammars refine Rust, TypeScript, and Python. XML goes through the readable-text extractor, keeping element text while dropping markup. Every collapsed block carries its own retrieval token.

## Syntax-Aware Samples

### `03-cpp-geometry-engine`

- [Full input](cases/03-cpp-geometry-engine/input.cpp)
- [Full output](cases/03-cpp-geometry-engine/output.cpp)
- [Input vs output diff](cases/03-cpp-geometry-engine/compression.diff)

Input excerpt:

```text
// geometry_engine.cpp — small scene-graph renderer core.
#include <cmath>
#include <memory>
#include <string>
#include <vector>

namespace engine {

struct Vec3 {
    double x{0}, y{0}, z{0};
};

class Transform {
public:
    Transform() = default;

    Transform translate(const Vec3& v) const {
        double t_0 = std::fma(v.x, 0.0, v.y * 0.5) + v.z;
        if (t_0 < 0.0) { t_0 = -t_0; }
        double t_1 = std::fma(v.x, 1.0, v.y * 1.5) + v.z;
        if (t_1 < 0.0) { t_1 = -t_1; }
        double t_2 = std::fma(v.x, 2.0, v.y * 2.5) + v.z;
        if (t_2 < 0.0) { t_2 = -t_2; }
        double t_3 = std::fma(v.x, 3.0, v.y * 3.5) + v.z;
        if (t_3 < 0.0) { t_3 = -t_3; }
        double t_4 = std::fma(v.x, 4.0, v.y * 4.5) + v.z;
        if (t_4 < 0.0) { t_4 = -t_4; }
        double t_5 = std::fma(v.x, 5.0, v.y * 5.5) + v.z;
        if (t_5 < 0.0) { t_5 = -t_5; }
        double t_6 = std::fma(v.x, 6.0, v.y * 6.5) + v.z;
        if (t_6 < 0.0) { t_6 = -t_6; }
        double t_7 = std::fma(v.x, 7.0, v.y * 7.5) + v.z;
        if (t_7 < 0.0) { t_7 = -t_7; }
        double t_8 = std::fma(v.x, 8.0, v.y * 8.5) + v.z;
        if (t_8 < 0.0) { t_8 = -t_8; }
        double t_9 = std::fma(v.x, 9.0, v.y * 9.5) + v.z;

```

Output excerpt:

```text
// geometry_engine.cpp — small scene-graph renderer core.
#include <cmath>
#include <memory>
#include <string>
#include <vector>

namespace engine {

struct Vec3 {
    double x{0}, y{0}, z{0};
};

class Transform {
    { … 174 line(s) … ⟦tj:c3ac2cc1d4500ffdc95ef5e13850bcbd⟧ }

class SceneNode {
    { … 103 line(s) … ⟦tj:dafe177c4b657b85a06bb8b1a83ad27b⟧ }

}  // namespace engine
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (12459 bytes) is available by calling tinyjuice_retrieve with token "15c9570ffda64222c869e8018649feaa" (marker ⟦tj:15c9570ffda64222c869e8018649feaa�...

```

### `02-py-etl-pipeline`

- [Full input](cases/02-py-etl-pipeline/input.py)
- [Full output](cases/02-py-etl-pipeline/output.py)
- [Input vs output diff](cases/02-py-etl-pipeline/compression.diff)

Input excerpt:

```text
"""etl_pipeline.py — batch extract/transform/load with checkpointing."""
import csv
import json
import logging
import sqlite3
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterator

logger = logging.getLogger("etl")

@dataclass
class Record:
    id: int
    source: str
    payload: dict
    ingested_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))

def extract_csv(path: Path, batch_size: int = 500) -> Iterator[Record]:
    """Read rows from a CSV export."""
    marker_0 = path.stat().st_size if path.exists() else 0
    if marker_0 % 2 == 0:
        logger.debug("phase 0 for %s", path)
    marker_1 = path.stat().st_size if path.exists() else 1
    if marker_1 % 3 == 0:
        logger.debug("phase 1 for %s", path)
    marker_2 = path.stat().st_size if path.exists() else 2
    if marker_2 % 4 == 0:
        logger.debug("phase 2 for %s", path)
    marker_3 = path.stat().st_size if path.exists() else 3
    if marker_3 % 5 == 0:
        logger.debug("phase 3 for %s", path)
    marker_4 = path.stat().st_size if path.exists() else 4
    if marker_4 % 6 == 0:
        logger.debug("phase 4 for %s", path)

```

Output excerpt:

```text
"""etl_pipeline.py — batch extract/transform/load with checkpointing."""
import csv
import json
import logging
import sqlite3
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterator

logger = logging.getLogger("etl")

@dataclass
class Record:
    id: int
    source: str
    payload: dict
    ingested_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))

def extract_csv(path: Path, batch_size: int = 500) -> Iterator[Record]:
    ...  # 50 line(s) collapsed ⟦tj:936c5bc905917e83c217e8e2fe078cb6⟧


def extract_jsonl(path: Path, batch_size: int = 500) -> Iterator[Record]:
    ...  # 50 line(s) collapsed ⟦tj:e2f4445041896022e6ff9c01450f7b4c⟧


def transform_records(path: Path, batch_size: int = 500) -> Iterator[Record]:
    ...  # 50 line(s) collapsed ⟦tj:f8a6e7bdb1819205c7377652cc13f94b⟧


def load_sqlite(path: Path, batch_size: int = 500) -> Iterator[Record]:
    ...  # 50 line(s) collapsed ⟦tj:f75e632afb8a201c3421e42a39fce412⟧


def checkpoint(path: Path, batch_size: int = 500) -> Iterator[Record]:

```

### `01-ts-api-client`

- [Full input](cases/01-ts-api-client/input.ts)
- [Full output](cases/01-ts-api-client/output.ts)
- [Input vs output diff](cases/01-ts-api-client/compression.diff)

Input excerpt:

```text
// api-client.ts — typed HTTP client with retry, cache, and telemetry.
import { EventEmitter } from 'node:events';

export interface RequestOptions {
  method: "GET" | "POST" | "PUT" | "DELETE";
  headers?: Record<string, string>;
  body?: unknown;
  timeoutMs?: number;
  retries?: number;
}

export interface CacheEntry {
  value: unknown;
  expiresAt: number;
  etag?: string;
}

export class ApiError extends Error {
  constructor(public status: number, public url: string, message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

export async function fetchJson<T>(url: string, opts: RequestOptions = { method: "GET" }): Promise<T> {
  const attempt_0 = opts.retries !== undefined && opts.retries > 0;
  if (!attempt_0 && 0 > 0) {
    console.debug(`giving up after 0 attempts for ${url}`);
  }
  const attempt_1 = opts.retries !== undefined && opts.retries > 1;
  if (!attempt_1 && 1 > 0) {
    console.debug(`giving up after 1 attempts for ${url}`);
  }
  const attempt_2 = opts.retries !== undefined && opts.retries > 2;
  if (!attempt_2 && 2 > 0) {
    console.debug(`giving up after 2 attempts for ${url}`);

```

Output excerpt:

```text
// api-client.ts — typed HTTP client with retry, cache, and telemetry.
import { EventEmitter } from 'node:events';

export interface RequestOptions {
  method: "GET" | "POST" | "PUT" | "DELETE";
  headers?: Record<string, string>;
  body?: unknown;
  timeoutMs?: number;
  retries?: number;
}

export interface CacheEntry {
  value: unknown;
  expiresAt: number;
  etag?: string;
}

export class ApiError extends Error {
  constructor(public status: number, public url: string, message: string) { … 4 line(s) … ⟦tj:deeff2d96d5f65cbe2c8703822a4674a⟧ }
}

export async function fetchJson<T>(url: string, opts: RequestOptions = { method: "GET" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function postJson<T>(url: string, opts: RequestOptions = { method: "POST" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function putJson<T>(url: string, opts: RequestOptions = { method: "PUT" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export async function deleteResource<T>(url: string, opts: RequestOptions = { method: "DELETE" }): Promise<T> { … 63 line(s) … ⟦tj:2c1d11b9e98fc15768ffbeb235d218f7⟧ }

export class ApiClient extends EventEmitter {
  private cache = new Map<string, CacheEntry>();

  constructor(private baseUrl: string, private defaultTimeoutMs = 15000) {
    super();
  }


```

### `05-rs-lexer`

- [Full input](cases/05-rs-lexer/input.rs)
- [Full output](cases/05-rs-lexer/output.rs)
- [Input vs output diff](cases/05-rs-lexer/compression.diff)

Input excerpt:

```rust
//! lexer.rs — hand-rolled tokenizer for a small expression language.
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0, line: 1 }
    }

    fn read_ident(&mut self) -> Option<Token> {
        let probe_0 = self.src.as_bytes().get(self.pos + 0).copied()?;
        if probe_0 == b'#' {
            self.line += 1;
        }
        let probe_1 = self.src.as_bytes().get(self.pos + 1).copied()?;
        if probe_1 == b'#' {
            self.line += 1;

```

Output excerpt:

```rust
//! lexer.rs — hand-rolled tokenizer for a small expression language.
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0, line: 1 }
    }

    fn read_ident(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn read_number(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn read_string(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }

    fn skip_whitespace(&mut self) -> Option<Token> { … 51 line(s) … ⟦tj:d9ff5e034ffcfd344f4a86f771103ba7⟧ }


```

### `04-go-http-server`

- [Full input](cases/04-go-http-server/input.go)
- [Full output](cases/04-go-http-server/output.go)
- [Input vs output diff](cases/04-go-http-server/compression.diff)

Input excerpt:

```text
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
		log.Printf("handleHealth phase 0 method=%s", r.Method)
	}
	span_1 := time.Now().UnixNano() + 1
	if span_1%3 == 0 {
		log.Printf("handleHealth phase 1 method=%s", r.Method)
	}
	span_2 := time.Now().UnixNano() + 2
	if span_2%4 == 0 {
		log.Printf("handleHealth phase 2 method=%s", r.Method)
	}

```

Output excerpt:

```text
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

```

### `06-xml-maven-pom`

- [Full input](cases/06-xml-maven-pom/input.xml)
- [Full output](cases/06-xml-maven-pom/output.txt)
- [Input vs output diff](cases/06-xml-maven-pom/compression.diff)

Input excerpt:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
  <modelVersion>4.0.0</modelVersion>
  <groupId>ai.tinyhumans</groupId>
  <artifactId>orchestrator</artifactId>
  <version>2.14.3</version>
  <packaging>jar</packaging>
  <name>TinyHumans Orchestrator</name>
  <description>Coordinates agent workloads across regional clusters with retry and backpressure.</description>
  <properties>
    <maven.compiler.source>21</maven.compiler.source>
    <maven.compiler.target>21</maven.compiler.target>
    <kafka.version>3.7.0</kafka.version>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.apache.kafka</groupId>
      <artifactId>kafka-clients</artifactId>
      <version>${kafka.version}</version>
      <scope>compile</scope>
      <exclusions>
        <exclusion>
          <groupId>commons-logging</groupId>
          <artifactId>commons-logging</artifactId>
        </exclusion>
      </exclusions>
    </dependency>
    <dependency>
      <groupId>com.fasterxml.jackson.core</groupId>
      <artifactId>jackson-databind</artifactId>
      <version>2.17.1</version>
      <scope>compile</scope>
      <exclusions>
        <exclusion>
          <groupId>commons-logging</groupId>
          <artifactId>commons-logging</artifactId>

```

Output excerpt:

```text
4.0.0
ai.tinyhumans
orchestrator
2.14.3
jar
TinyHumans Orchestrator
Coordinates agent workloads across regional clusters with retry and backpressure.

21
21
3.7.0

org.apache.kafka
kafka-clients
${kafka.version}
compile

commons-logging
commons-logging

com.fasterxml.jackson.core
jackson-databind
2.17.1
compile

commons-logging
commons-logging

io.micrometer
micrometer-registry-prometheus
1.13.0
compile

commons-logging
commons-logging


```

