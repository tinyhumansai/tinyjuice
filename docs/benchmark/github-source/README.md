# GitHub Source Files

Real source files fetched from popular public GitHub repositories (vscode, django, flask, requests, gin, cobra, leveldb, redis, curl, tokio, ripgrep, gson, guava, okhttp, rails, laravel, swift-argument-parser, Newtonsoft.Json, Maven POMs). Sources and licenses are listed in ATTRIBUTION.md.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `27-xml-gson-pom` | [input](cases/27-xml-gson-pom/input.xml) | [output](cases/27-xml-gson-pom/output.txt) | [diff](cases/27-xml-gson-pom/compression.diff) | 26.0 KB -> 4.3 KB (-84%) | 83.5% | 82.7% | 0.083 ms |
| `28-xml-maven-pom` | [input](cases/28-xml-maven-pom/input.xml) | [output](cases/28-xml-maven-pom/output.txt) | [diff](cases/28-xml-maven-pom/compression.diff) | 29.0 KB -> 7.3 KB (-75%) | 74.8% | 74.0% | 0.124 ms |
| `14-cpp-leveldb-dbimpl` | [input](cases/14-cpp-leveldb-dbimpl/input.cpp) | [output](cases/14-cpp-leveldb-dbimpl/output.cpp) | [diff](cases/14-cpp-leveldb-dbimpl/compression.diff) | 49.1 KB -> 14.6 KB (-70%) | 73.7% | 70.4% | 0.365 ms |
| `30-py-dijkstra` | [input](cases/30-py-dijkstra/input.py) | [output](cases/30-py-dijkstra/output.py) | [diff](cases/30-py-dijkstra/compression.diff) | 14.6 KB -> 5.4 KB (-63%) | 67.9% | 61.8% | 0.521 ms |
| `33-rs-floyd-warshall` | [input](cases/33-rs-floyd-warshall/input.rs) | [output](cases/33-rs-floyd-warshall/output.rs) | [diff](cases/33-rs-floyd-warshall/compression.diff) | 6.3 KB -> 2.4 KB (-62%) | 66.7% | 59.3% | 0.381 ms |
| `15-cpp-leveldb-versionset` | [input](cases/15-cpp-leveldb-versionset/input.cpp) | [output](cases/15-cpp-leveldb-versionset/output.cpp) | [diff](cases/15-cpp-leveldb-versionset/compression.diff) | 51.4 KB -> 21.7 KB (-58%) | 61.9% | 57.9% | 0.386 ms |
| `41-java-bellman-ford` | [input](cases/41-java-bellman-ford/input.java) | [output](cases/41-java-bellman-ford/output.java) | [diff](cases/41-java-bellman-ford/compression.diff) | 6.1 KB -> 2.6 KB (-58%) | 61.8% | 54.7% | 0.033 ms |
| `29-py-red-black-tree` | [input](cases/29-py-red-black-tree/input.py) | [output](cases/29-py-red-black-tree/output.py) | [diff](cases/29-py-red-black-tree/compression.diff) | 25.5 KB -> 10.9 KB (-57%) | 61.8% | 56.9% | 1.358 ms |
| `16-c-redis-sds` | [input](cases/16-c-redis-sds/input.c) | [output](cases/16-c-redis-sds/output.c) | [diff](cases/16-c-redis-sds/compression.diff) | 49.8 KB -> 23.1 KB (-54%) | 56.2% | 53.6% | 0.329 ms |
| `25-swift-argparser-argumentset` | [input](cases/25-swift-argparser-argumentset/input.swift) | [output](cases/25-swift-argparser-argumentset/output.swift) | [diff](cases/25-swift-argparser-argumentset/compression.diff) | 21.7 KB -> 10.1 KB (-54%) | 56.7% | 53.0% | 0.130 ms |
| `36-cpp-a-star-search` | [input](cases/36-cpp-a-star-search/input.cpp) | [output](cases/36-cpp-a-star-search/output.cpp) | [diff](cases/36-cpp-a-star-search/compression.diff) | 26.5 KB -> 12.6 KB (-53%) | 55.2% | 52.1% | 0.140 ms |
| `10-go-gin-gin` | [input](cases/10-go-gin-gin/input.go) | [output](cases/10-go-gin-gin/output.go) | [diff](cases/10-go-gin-gin/compression.diff) | 23.8 KB -> 13.6 KB (-43%) | 47.4% | 42.7% | 0.174 ms |
| `09-py-requests-sessions` | [input](cases/09-py-requests-sessions/input.py) | [output](cases/09-py-requests-sessions/output.py) | [diff](cases/09-py-requests-sessions/compression.diff) | 30.5 KB -> 17.3 KB (-43%) | 46.3% | 42.7% | 1.145 ms |
| `12-go-cobra-command` | [input](cases/12-go-cobra-command/input.go) | [output](cases/12-go-cobra-command/output.go) | [diff](cases/12-go-cobra-command/compression.diff) | 55.2 KB -> 32.3 KB (-41%) | 45.3% | 41.7% | 0.404 ms |
| `17-c-curl-url` | [input](cases/17-c-curl-url/input.c) | [output](cases/17-c-curl-url/output.c) | [diff](cases/17-c-curl-url/compression.diff) | 120.5 KB -> 73.4 KB (-39%) | 41.5% | 39.3% | 0.770 ms |
| `26-cs-newtonsoft-serializer` | [input](cases/26-cs-newtonsoft-serializer/input.cs) | [output](cases/26-cs-newtonsoft-serializer/output.cs) | [diff](cases/26-cs-newtonsoft-serializer/compression.diff) | 50.8 KB -> 31.1 KB (-39%) | 41.2% | 38.7% | 0.241 ms |
| `35-go-segment-tree` | [input](cases/35-go-segment-tree/input.go) | [output](cases/35-go-segment-tree/output.go) | [diff](cases/35-go-segment-tree/compression.diff) | 4.5 KB -> 2.8 KB (-38%) | 45.6% | 33.5% | 0.027 ms |
| `42-js-kruskal-mst` | [input](cases/42-js-kruskal-mst/input.js) | [output](cases/42-js-kruskal-mst/output.js) | [diff](cases/42-js-kruskal-mst/compression.diff) | 3.1 KB -> 1.9 KB (-37%) | 46.2% | 31.1% | 0.157 ms |
| `03-ts-vscode-uri` | [input](cases/03-ts-vscode-uri/input.ts) | [output](cases/03-ts-vscode-uri/output.ts) | [diff](cases/03-ts-vscode-uri/compression.diff) | 22.6 KB -> 14.3 KB (-37%) | 40.5% | 36.2% | 1.019 ms |
| `19-rs-ripgrep-walk` | [input](cases/19-rs-ripgrep-walk/input.rs) | [output](cases/19-rs-ripgrep-walk/output.rs) | [diff](cases/19-rs-ripgrep-walk/compression.diff) | 77.9 KB -> 50.5 KB (-35%) | 37.9% | 35.4% | 3.444 ms |
| `31-rs-huffman-encoding` | [input](cases/31-rs-huffman-encoding/input.rs) | [output](cases/31-rs-huffman-encoding/output.rs) | [diff](cases/31-rs-huffman-encoding/compression.diff) | 16.3 KB -> 10.7 KB (-34%) | 38.4% | 33.6% | 0.753 ms |
| `37-cpp-random-pivot-quicksort` | [input](cases/37-cpp-random-pivot-quicksort/input.cpp) | [output](cases/37-cpp-random-pivot-quicksort/output.cpp) | [diff](cases/37-cpp-random-pivot-quicksort/compression.diff) | 11.9 KB -> 8.2 KB (-31%) | 34.9% | 29.7% | 0.059 ms |
| `39-ts-binary-search-tree` | [input](cases/39-ts-binary-search-tree/input.ts) | [output](cases/39-ts-binary-search-tree/output.ts) | [diff](cases/39-ts-binary-search-tree/compression.diff) | 5.2 KB -> 3.7 KB (-30%) | 38.5% | 26.5% | 0.201 ms |
| `08-py-flask-app` | [input](cases/08-py-flask-app/input.py) | [output](cases/08-py-flask-app/output.py) | [diff](cases/08-py-flask-app/compression.diff) | 60.1 KB -> 42.1 KB (-30%) | 32.0% | 29.9% | 1.744 ms |
| `34-go-avl-tree` | [input](cases/34-go-avl-tree/input.go) | [output](cases/34-go-avl-tree/output.go) | [diff](cases/34-go-avl-tree/compression.diff) | 7.7 KB -> 5.5 KB (-29%) | 34.6% | 27.0% | 0.059 ms |
| `07-py-django-paginator` | [input](cases/07-py-django-paginator/input.py) | [output](cases/07-py-django-paginator/output.py) | [diff](cases/07-py-django-paginator/compression.diff) | 7.9 KB -> 5.6 KB (-29%) | 33.5% | 26.6% | 0.383 ms |
| `46-cs-topological-sort` | [input](cases/46-cs-topological-sort/input.cs) | [output](cases/46-cs-topological-sort/output.cs) | [diff](cases/46-cs-topological-sort/compression.diff) | 14.4 KB -> 10.7 KB (-26%) | 28.9% | 24.6% | 0.060 ms |
| `06-py-django-request` | [input](cases/06-py-django-request/input.py) | [output](cases/06-py-django-request/output.py) | [diff](cases/06-py-django-request/compression.diff) | 25.8 KB -> 19.6 KB (-24%) | 26.3% | 23.1% | 1.239 ms |
| `24-php-laravel-container` | [input](cases/24-php-laravel-container/input.php) | [output](cases/24-php-laravel-container/output.php) | [diff](cases/24-php-laravel-container/compression.diff) | 41.0 KB -> 31.7 KB (-23%) | 25.2% | 22.6% | 0.210 ms |
| `13-go-mux-mux` | [input](cases/13-go-mux-mux/input.go) | [output](cases/13-go-mux-mux/output.go) | [diff](cases/13-go-mux-mux/compression.diff) | 17.8 KB -> 13.8 KB (-22%) | 26.1% | 21.6% | 0.095 ms |
| `45-php-avl-tree` | [input](cases/45-php-avl-tree/input.php) | [output](cases/45-php-avl-tree/output.php) | [diff](cases/45-php-avl-tree/compression.diff) | 10.9 KB -> 8.6 KB (-21%) | 26.0% | 19.6% | 0.064 ms |
| `01-ts-vscode-async` | [input](cases/01-ts-vscode-async/input.ts) | [output](cases/01-ts-vscode-async/output.ts) | [diff](cases/01-ts-vscode-async/compression.diff) | 56.4 KB -> 45.4 KB (-19%) | 22.6% | 19.5% | 2.944 ms |
| `02-ts-vscode-strings` | [input](cases/02-ts-vscode-strings/input.ts) | [output](cases/02-ts-vscode-strings/output.ts) | [diff](cases/02-ts-vscode-strings/compression.diff) | 88.4 KB -> 74.6 KB (-16%) | 17.3% | 15.6% | 2.083 ms |
| `32-rs-knapsack` | [input](cases/32-rs-knapsack/input.rs) | [output](cases/32-rs-knapsack/output.rs) | [diff](cases/32-rs-knapsack/compression.diff) | 12.4 KB -> 10.5 KB (-16%) | 17.8% | 14.1% | 0.776 ms |
| `40-java-lru-cache` | [input](cases/40-java-lru-cache/input.java) | [output](cases/40-java-lru-cache/output.java) | [diff](cases/40-java-lru-cache/compression.diff) | 6.7 KB -> 5.7 KB (-15%) | 20.0% | 12.4% | 0.039 ms |
| `20-java-gson-gson` | [input](cases/20-java-gson-gson/input.java) | [output](cases/20-java-gson-gson/output.java) | [diff](cases/20-java-gson-gson/compression.diff) | 68.5 KB -> 59.0 KB (-14%) | 15.0% | 13.7% | 0.246 ms |
| `11-go-gin-context` | [input](cases/11-go-gin-context/input.go) | [output](cases/11-go-gin-context/output.go) | [diff](cases/11-go-gin-context/compression.diff) | 39.0 KB -> 34.1 KB (-13%) | 14.9% | 12.4% | 0.193 ms |
| `38-ts-heap` | [input](cases/38-ts-heap/input.ts) | [output](cases/38-ts-heap/output.ts) | [diff](cases/38-ts-heap/compression.diff) | 5.7 KB -> 5.0 KB (-12%) | 17.0% | 8.9% | 0.310 ms |
| `21-java-guava-ordering` | [input](cases/21-java-guava-ordering/input.java) | [output](cases/21-java-guava-ordering/output.java) | [diff](cases/21-java-guava-ordering/compression.diff) | 40.4 KB -> 37.8 KB (-6%) | 7.7% | 5.9% | 0.139 ms |
| `18-rs-tokio-builder` | [input](cases/18-rs-tokio-builder/input.rs) | [output](cases/18-rs-tokio-builder/output.rs) | [diff](cases/18-rs-tokio-builder/compression.diff) | 48.2 KB -> 45.6 KB (-5%) | 6.1% | 5.1% | 1.724 ms |
| `47-kt-indexed-priority-queue` | [input](cases/47-kt-indexed-priority-queue/input.kt) | [output](cases/47-kt-indexed-priority-queue/output.kt) | [diff](cases/47-kt-indexed-priority-queue/compression.diff) | 7.0 KB -> 6.7 KB (-5%) | 8.1% | 1.9% | 0.034 ms |
| `22-kt-okhttp-client` | [input](cases/22-kt-okhttp-client/input.kt) | [output](cases/22-kt-okhttp-client/output.kt) | [diff](cases/22-kt-okhttp-client/compression.diff) | 44.0 KB -> 41.9 KB (-5%) | 5.6% | 4.4% | 0.150 ms |
| `43-c-trie` | [input](cases/43-c-trie/input.c) | [output](cases/43-c-trie/output.c) | [diff](cases/43-c-trie/compression.diff) | 5.2 KB -> 5.0 KB (-4%) | 7.9% | 0.4% | 0.027 ms |
| `05-js-axios-core` | [input](cases/05-js-axios-core/input.js) | [output](cases/05-js-axios-core/output.js) | [diff](cases/05-js-axios-core/compression.diff) | 6.4 KB -> 6.3 KB (-1%) | 3.8% | 0.0% | 0.293 ms |
| `44-rb-avl-tree` | [input](cases/44-rb-avl-tree/input.rb) | [output](cases/44-rb-avl-tree/output.rb) | [diff](cases/44-rb-avl-tree/compression.diff) | 7.5 KB -> 7.5 KB (-0%) | 0.1% | 0.0% | 0.033 ms |
| `04-js-express-application` | [input](cases/04-js-express-application/input.js) | [output](cases/04-js-express-application/output.js) | [diff](cases/04-js-express-application/compression.diff) | 14.6 KB -> 14.6 KB (-0%) | 0.0% | 0.0% | 0.719 ms |
| `23-rb-rails-cache` | [input](cases/23-rb-rails-cache/input.rb) | [output](cases/23-rb-rails-cache/output.rb) | [diff](cases/23-rb-rails-cache/compression.diff) | 42.6 KB -> 42.6 KB (-0%) | 0.0% | 0.0% | 0.148 ms |

## What TinyJuice Is Doing

Real-world source compresses the same way as the synthetic corpus: signatures, imports, and top-level structure stay; deep bodies collapse behind per-block retrieval tokens. Languages without a tree-sitter grammar use the brace-depth heuristic; brace-less languages (Ruby) pass through.

## Syntax-Aware Samples

### `27-xml-gson-pom`

- [Full input](cases/27-xml-gson-pom/input.xml)
- [Full output](cases/27-xml-gson-pom/output.txt)
- [Input vs output diff](cases/27-xml-gson-pom/compression.diff)

Input excerpt:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--
  Copyright 2015 Google LLC

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
-->
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4_0_0.xsd" child.project.url.i...
  <modelVersion>4.0.0</modelVersion>

  <groupId>com.google.code.gson</groupId>
  <artifactId>gson-parent</artifactId>
  <version>2.11.0</version>
  <packaging>pom</packaging>

  <name>Gson Parent</name>
  <description>Gson JSON library</description>
  <url>https://github.com/google/gson</url>

  <modules>
    <module>gson</module>
    <module>graal-native-image-test</module>
    <module>shrinker-test</module>
    <module>extras</module>
    <module>metrics</module>
    <module>proto</module>
  </modules>

```

Output excerpt:

```text
4.0.0

com.google.code.gson
gson-parent
2.11.0
pom

Gson Parent
Gson JSON library
https://github.com/google/gson

gson
graal-native-image-test
shrinker-test
extras
metrics
proto

UTF-8
7
11

2024-05-19T18:53:38Z

https://github.com/google/gson/
scm:git:https://github.com/google/gson.git
scm:git:git@github.com:google/gson.git
gson-parent-2.11.0

google
Google
https://www.google.com

GitHub Issues
https://github.com/google/gson/issues


```

### `28-xml-maven-pom`

- [Full input](cases/28-xml-maven-pom/input.xml)
- [Full output](cases/28-xml-maven-pom/output.txt)
- [Input vs output diff](cases/28-xml-maven-pom/compression.diff)

Input excerpt:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
-->
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
  <modelVersion>4.0.0</modelVersion>

  <parent>
    <groupId>org.apache.maven</groupId>
    <artifactId>maven-parent</artifactId>
    <version>42</version>
    <relativePath />
  </parent>

  <artifactId>maven</artifactId>
  <version>3.9.7</version>
  <packaging>pom</packaging>

  <name>Apache Maven</name>
  <description>Maven is a software build management and
    comprehension tool. Based on the concept of a project object model:

```

Output excerpt:

```text
4.0.0

org.apache.maven
maven-parent
42

maven
3.9.7
pom

Apache Maven
Maven is a software build management and
comprehension tool. Based on the concept of a project object model:
builds, dependency management, documentation creation, site
publication, and distribution publication are all controlled from
the declarative file. Maven can be extended by plugins to utilise a
number of other development tools for reporting or the build
process.
https://maven.apache.org/ref/${project.version}/
2001

Stuart McCulloch

Christian Schulte (MNG-2199)

Christopher Tubbs (MNG-4226)

Konstantin Perikov (MNG-4565)

Sébastian Le Merdy (MNG-5613)

Mark Ingram (MNG-5639)

Phil Pratt-Szeliga (MNG-5645)

Florencia Tarditti (PR 41)

```

### `14-cpp-leveldb-dbimpl`

- [Full input](cases/14-cpp-leveldb-dbimpl/input.cpp)
- [Full output](cases/14-cpp-leveldb-dbimpl/output.cpp)
- [Input vs output diff](cases/14-cpp-leveldb-dbimpl/compression.diff)

Input excerpt:

```text
// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/db_impl.h"

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <cstdio>
#include <set>
#include <string>
#include <vector>

#include "db/builder.h"
#include "db/db_iter.h"
#include "db/dbformat.h"
#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "db/version_set.h"
#include "db/write_batch_internal.h"
#include "leveldb/db.h"
#include "leveldb/env.h"
#include "leveldb/status.h"
#include "leveldb/table.h"
#include "leveldb/table_builder.h"
#include "port/port.h"
#include "table/block.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"
#include "util/mutexlock.h"

```

Output excerpt:

```text
// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/db_impl.h"

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <cstdio>
#include <set>
#include <string>
#include <vector>

#include "db/builder.h"
#include "db/db_iter.h"
#include "db/dbformat.h"
#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "db/version_set.h"
#include "db/write_batch_internal.h"
#include "leveldb/db.h"
#include "leveldb/env.h"
#include "leveldb/status.h"
#include "leveldb/table.h"
#include "leveldb/table_builder.h"
#include "port/port.h"
#include "table/block.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"
#include "util/mutexlock.h"

```

### `30-py-dijkstra`

- [Full input](cases/30-py-dijkstra/input.py)
- [Full output](cases/30-py-dijkstra/output.py)
- [Input vs output diff](cases/30-py-dijkstra/compression.diff)

Input excerpt:

```text
# Title: Dijkstra's Algorithm for finding single source shortest path from scratch
# Author: Shubham Malik
# References: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

import math
import sys

# For storing the vertex set to retrieve node with the lowest distance


class PriorityQueue:
    # Based on Min Heap
    def __init__(self):
        """
        Priority queue class constructor method.

        Examples:
        >>> priority_queue_test = PriorityQueue()
        >>> priority_queue_test.cur_size
        0
        >>> priority_queue_test.array
        []
        >>> priority_queue_test.pos
        {}
        """
        self.cur_size = 0
        self.array = []
        self.pos = {}  # To store the pos of node in array

    def is_empty(self):
        """
        Conditional boolean method to determine if the priority queue is empty or not.

        Examples:
        >>> priority_queue_test = PriorityQueue()
        >>> priority_queue_test.is_empty()

```

Output excerpt:

```text
# Title: Dijkstra's Algorithm for finding single source shortest path from scratch
# Author: Shubham Malik
# References: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

import math
import sys

# For storing the vertex set to retrieve node with the lowest distance


class PriorityQueue:
    # Based on Min Heap
    def __init__(self):
        """
        Priority queue class constructor method.

        Examples:
        >>> priority_queue_test = PriorityQueue()
        >>> priority_queue_test.cur_size
        0
        >>> priority_queue_test.array
        []
        >>> priority_queue_test.pos
        {}
        """
        self.cur_size = 0
        self.array = []
        self.pos = {}  # To store the pos of node in array

    def is_empty(self):
        """
        Conditional boolean method to determine if the priority queue is empty or not.
...  # 9 line(s) collapsed ⟦tj:e9a3c55f96e272abfbd740b3ae869ff9⟧
        return self.cur_size == 0

    def min_heapify(self, idx):

```

### `33-rs-floyd-warshall`

- [Full input](cases/33-rs-floyd-warshall/input.rs)
- [Full output](cases/33-rs-floyd-warshall/output.rs)
- [Input vs output diff](cases/33-rs-floyd-warshall/compression.diff)

Input excerpt:

```rust
use num_traits::Zero;
use std::collections::BTreeMap;
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// Performs the Floyd-Warshall algorithm on the input graph.\
/// The graph is a weighted, directed graph with no negative cycles.
///
/// Returns a map storing the distance from each node to all the others.\
/// i.e. For each vertex `u`, `map[u][v] == Some(distance)` means
/// distance is the sum of the weights of the edges on the shortest path
/// from `u` to `v`.
///
/// For a key `v`, if `map[v].len() == 0`, then `v` cannot reach any other vertex, but is in the graph
/// (island node, or sink in the case of a directed graph)
pub fn floyd_warshall<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + num_traits::Zero>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();
    for (u, edges) in graph.iter() {
        if !map.contains_key(u) {
            map.insert(*u, BTreeMap::new());
        }
        map.entry(*u).or_default().insert(*u, Zero::zero());
        for (v, weight) in edges.iter() {
            if !map.contains_key(v) {
                map.insert(*v, BTreeMap::new());
            }
            map.entry(*v).or_default().insert(*v, Zero::zero());
            map.entry(*u).and_modify(|mp| {
                mp.insert(*v, *weight);
            });
        }
    }
    let keys = map.keys().copied().collect::<Vec<_>>();

```

Output excerpt:

```rust
use num_traits::Zero;
use std::collections::BTreeMap;
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// Performs the Floyd-Warshall algorithm on the input graph.\
/// The graph is a weighted, directed graph with no negative cycles.
///
/// Returns a map storing the distance from each node to all the others.\
/// i.e. For each vertex `u`, `map[u][v] == Some(distance)` means
/// distance is the sum of the weights of the edges on the shortest path
/// from `u` to `v`.
///
/// For a key `v`, if `map[v].len() == 0`, then `v` cannot reach any other vertex, but is in the graph
/// (island node, or sink in the case of a directed graph)
pub fn floyd_warshall<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + num_traits::Zero>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();
    for (u, edges) in graph.iter() {
    { … 43 line(s) … ⟦tj:34932fe2c456b844392edaa7eecd2dbe⟧ }
    map
}

#[cfg(test)]
mod tests {
    use super::{floyd_warshall, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
    }

    fn bi_add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        add_edge(graph, v1, v2, c);

```

### `15-cpp-leveldb-versionset`

- [Full input](cases/15-cpp-leveldb-versionset/input.cpp)
- [Full output](cases/15-cpp-leveldb-versionset/output.cpp)
- [Input vs output diff](cases/15-cpp-leveldb-versionset/compression.diff)

Input excerpt:

```text
// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/version_set.h"

#include <algorithm>
#include <cstdio>

#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "leveldb/env.h"
#include "leveldb/table_builder.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"

namespace leveldb {

static size_t TargetFileSize(const Options* options) {
  return options->max_file_size;
}

// Maximum bytes of overlaps in grandparent (i.e., level+2) before we
// stop building a single file in a level->level+1 compaction.
static int64_t MaxGrandParentOverlapBytes(const Options* options) {
  return 10 * TargetFileSize(options);
}

// Maximum number of bytes in all compacted files.  We avoid expanding
// the lower level file set of a compaction if it would make the
// total compaction cover more than this many bytes.

```

Output excerpt:

```text
// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/version_set.h"

#include <algorithm>
#include <cstdio>

#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "leveldb/env.h"
#include "leveldb/table_builder.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"

namespace leveldb {

static size_t TargetFileSize(const Options* options) {
  return options->max_file_size;
}

// Maximum bytes of overlaps in grandparent (i.e., level+2) before we
// stop building a single file in a level->level+1 compaction.
static int64_t MaxGrandParentOverlapBytes(const Options* options) {
  return 10 * TargetFileSize(options);
}

// Maximum number of bytes in all compacted files.  We avoid expanding
// the lower level file set of a compaction if it would make the
// total compaction cover more than this many bytes.

```

### `29-py-red-black-tree`

- [Full input](cases/29-py-red-black-tree/input.py)
- [Full output](cases/29-py-red-black-tree/output.py)
- [Input vs output diff](cases/29-py-red-black-tree/compression.diff)

Input excerpt:

```text
from __future__ import annotations

from collections.abc import Iterator


class RedBlackTree:
    """
    A Red-Black tree, which is a self-balancing BST (binary search
    tree).
    This tree has similar performance to AVL trees, but the balancing is
    less strict, so it will perform faster for writing/deleting nodes
    and slower for reading in the average case, though, because they're
    both balanced binary search trees, both will get the same asymptotic
    performance.
    To read more about them, https://en.wikipedia.org/wiki/Red-black_tree
    Unless otherwise specified, all asymptotic runtimes are specified in
    terms of the size of the tree.
    """

    def __init__(
        self,
        label: int | None = None,
        color: int = 0,
        parent: RedBlackTree | None = None,
        left: RedBlackTree | None = None,
        right: RedBlackTree | None = None,
    ) -> None:
        """Initialize a new Red-Black Tree node with the given values:
        label: The value associated with this node
        color: 0 if black, 1 if red
        parent: The parent to this node
        left: This node's left child
        right: This node's right child
        """
        self.label = label
        self.parent = parent

```

Output excerpt:

```text
from __future__ import annotations

from collections.abc import Iterator


class RedBlackTree:
    """
    A Red-Black tree, which is a self-balancing BST (binary search
    tree).
    This tree has similar performance to AVL trees, but the balancing is
    less strict, so it will perform faster for writing/deleting nodes
    and slower for reading in the average case, though, because they're
    both balanced binary search trees, both will get the same asymptotic
    performance.
    To read more about them, https://en.wikipedia.org/wiki/Red-black_tree
    Unless otherwise specified, all asymptotic runtimes are specified in
    terms of the size of the tree.
    """

    def __init__(
        self,
        label: int | None = None,
        color: int = 0,
        parent: RedBlackTree | None = None,
        left: RedBlackTree | None = None,
        right: RedBlackTree | None = None,
    ) -> None:
        """Initialize a new Red-Black Tree node with the given values:
        label: The value associated with this node
        color: 0 if black, 1 if red
        parent: The parent to this node
        left: This node's left child
        right: This node's right child
        """
        self.label = label
        self.parent = parent

```

### `41-java-bellman-ford`

- [Full input](cases/41-java-bellman-ford/input.java)
- [Full output](cases/41-java-bellman-ford/output.java)
- [Input vs output diff](cases/41-java-bellman-ford/compression.diff)

Input excerpt:

```text
package com.thealgorithms.datastructures.graphs;

import java.util.Scanner;

class BellmanFord /*
                   * Implementation of Bellman ford to detect negative cycles. Graph accepts
                   * inputs
                   * in form of edges which have start vertex, end vertex and weights. Vertices
                   * should be labelled with a
                   * number between 0 and total number of vertices-1,both inclusive
                   */
{

    int vertex;
    int edge;
    private Edge[] edges;
    private int index = 0;

    BellmanFord(int v, int e) {
        vertex = v;
        edge = e;
        edges = new Edge[e];
    }

    class Edge {

        int u;
        int v;
        int w;

        /**
         * @param u Source Vertex
         * @param v End vertex
         * @param c Weight
         */
        Edge(int a, int b, int c) {

```

Output excerpt:

```text
package com.thealgorithms.datastructures.graphs;

import java.util.Scanner;

class BellmanFord /*
                   * Implementation of Bellman ford to detect negative cycles. Graph accepts
                   * inputs
                   * in form of edges which have start vertex, end vertex and weights. Vertices
                   * should be labelled with a
                   * number between 0 and total number of vertices-1,both inclusive
                   */
{

    int vertex;
    int edge;
    private Edge[] edges;
    private int index = 0;

    BellmanFord(int v, int e) {
        vertex = v;
        edge = e;
        edges = new Edge[e];
    }

    class Edge {

        int u;
        int v;
        int w;

        /**
         * @param u Source Vertex
         * @param v End vertex
         * @param c Weight
         */
        Edge(int a, int b, int c) {

```

### `25-swift-argparser-argumentset`

- [Full input](cases/25-swift-argparser-argumentset/input.swift)
- [Full output](cases/25-swift-argparser-argumentset/output.swift)
- [Input vs output diff](cases/25-swift-argparser-argumentset/compression.diff)

Input excerpt:

```text
//===----------------------------------------------------------*- swift -*-===//
//
// This source file is part of the Swift Argument Parser open source project
//
// Copyright (c) 2020 Apple Inc. and the Swift project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://swift.org/LICENSE.txt for license information
//
//===----------------------------------------------------------------------===//

/// A nested tree of argument definitions.
///
/// The main reason for having a nested representation is to build help output.
/// For output like:
///
///     Usage: mytool [-v | -f] <input> <output>
///
/// The `-v | -f` part is one *set* that’s optional, `<input> <output>` is
/// another. Both of these can then be combined into a third set.
struct ArgumentSet {
  var content: [ArgumentDefinition] = []
  var namePositions: [Name: Int] = [:]
  
  init<S: Sequence>(_ arguments: S) where S.Element == ArgumentDefinition {
    self.content = Array(arguments)
    self.namePositions = Dictionary(
      content.enumerated().flatMap { i, arg in arg.names.map { ($0.nameToMatch, i) } },
      uniquingKeysWith: { first, _ in first })
  }
  
  init() {}
  
  init(_ arg: ArgumentDefinition) {
    self.init([arg])
  }

```

Output excerpt:

```text
//===----------------------------------------------------------*- swift -*-===//
//
// This source file is part of the Swift Argument Parser open source project
//
// Copyright (c) 2020 Apple Inc. and the Swift project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://swift.org/LICENSE.txt for license information
//
//===----------------------------------------------------------------------===//

/// A nested tree of argument definitions.
///
/// The main reason for having a nested representation is to build help output.
/// For output like:
///
///     Usage: mytool [-v | -f] <input> <output>
///
/// The `-v | -f` part is one *set* that’s optional, `<input> <output>` is
/// another. Both of these can then be combined into a third set.
struct ArgumentSet {
  var content: [ArgumentDefinition] = []
  var namePositions: [Name: Int] = [:]
  
  init<S: Sequence>(_ arguments: S) where S.Element == ArgumentDefinition {
    self.content = Array(arguments)
    self.namePositions = Dictionary(
      content.enumerated().flatMap { i, arg in arg.names.map { ($0.nameToMatch, i) } },
      uniquingKeysWith: { first, _ in first })
  }
  
  init() {}
  
  init(_ arg: ArgumentDefinition) {
    self.init([arg])
  }

```

### `16-c-redis-sds`

- [Full input](cases/16-c-redis-sds/input.c)
- [Full output](cases/16-c-redis-sds/output.c)
- [Input vs output diff](cases/16-c-redis-sds/compression.diff)

Input excerpt:

```text
/* SDSLib 2.0 -- A C dynamic strings library
 *
 * Copyright (c) 2006-2015, Salvatore Sanfilippo <antirez at gmail dot com>
 * Copyright (c) 2015, Oran Agra
 * Copyright (c) 2015, Redis Labs, Inc
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *   * Neither the name of Redis nor the names of its contributors may be used
 *     to endorse or promote products derived from this software without
 *     specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

```

Output excerpt:

```text
/* SDSLib 2.0 -- A C dynamic strings library
 *
 * Copyright (c) 2006-2015, Salvatore Sanfilippo <antirez at gmail dot com>
 * Copyright (c) 2015, Oran Agra
 * Copyright (c) 2015, Redis Labs, Inc
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *   * Neither the name of Redis nor the names of its contributors may be used
 *     to endorse or promote products derived from this software without
 *     specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

```

### `36-cpp-a-star-search`

- [Full input](cases/36-cpp-a-star-search/input.cpp)
- [Full output](cases/36-cpp-a-star-search/output.cpp)
- [Input vs output diff](cases/36-cpp-a-star-search/compression.diff)

Input excerpt:

```text
/**
 * @brief
 * [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
 * @details
 * A* is an informed search algorithm, or a best-first search, meaning that it
 * is formulated in terms of weighted graphs: starting from a specific starting
 * node of a graph (initial state), it aims to find a path to the given goal
 * node having the smallest cost (least distance travelled, shortest time,
 * etc.). It evaluates by maintaining a tree of paths originating at the start
 * node and extending those paths one edge at a time until it reaches the final
 * state.
 * The weighted edges (or cost) is evaluated on two factors, G score
 * (cost required from starting node or initial state to current state) and H
 * score (cost required from current state to final state). The F(state), then
 * is evaluated as:
 * F(state) = G(state) + H(state).
 *
 * To solve the given search with shortest cost or path possible  is to inspect
 * values having minimum F(state).
 * @author [Ashish Daulatabad](https://github.com/AshishYUO)
 */
#include <algorithm>   /// for `std::reverse` function
#include <array>       /// for `std::array`, representing `EightPuzzle` board
#include <cassert>     /// for `assert`
#include <cstdint>     /// for `std::uint32_t`
#include <functional>  /// for `std::function` STL
#include <iostream>    /// for IO operations
#include <map>         /// for `std::map` STL
#include <memory>      /// for `std::shared_ptr`
#include <set>         /// for `std::set` STL
#include <vector>      /// for `std::vector` STL

/**
 * @namespace machine_learning
 * @brief Machine learning algorithms
 */

```

Output excerpt:

```text
/**
 * @brief
 * [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
 * @details
 * A* is an informed search algorithm, or a best-first search, meaning that it
 * is formulated in terms of weighted graphs: starting from a specific starting
 * node of a graph (initial state), it aims to find a path to the given goal
 * node having the smallest cost (least distance travelled, shortest time,
 * etc.). It evaluates by maintaining a tree of paths originating at the start
 * node and extending those paths one edge at a time until it reaches the final
 * state.
 * The weighted edges (or cost) is evaluated on two factors, G score
 * (cost required from starting node or initial state to current state) and H
 * score (cost required from current state to final state). The F(state), then
 * is evaluated as:
 * F(state) = G(state) + H(state).
 *
 * To solve the given search with shortest cost or path possible  is to inspect
 * values having minimum F(state).
 * @author [Ashish Daulatabad](https://github.com/AshishYUO)
 */
#include <algorithm>   /// for `std::reverse` function
#include <array>       /// for `std::array`, representing `EightPuzzle` board
#include <cassert>     /// for `assert`
#include <cstdint>     /// for `std::uint32_t`
#include <functional>  /// for `std::function` STL
#include <iostream>    /// for IO operations
#include <map>         /// for `std::map` STL
#include <memory>      /// for `std::shared_ptr`
#include <set>         /// for `std::set` STL
#include <vector>      /// for `std::vector` STL

/**
 * @namespace machine_learning
 * @brief Machine learning algorithms
 */

```

### `10-go-gin-gin`

- [Full input](cases/10-go-gin-gin/input.go)
- [Full output](cases/10-go-gin-gin/output.go)
- [Input vs output diff](cases/10-go-gin-gin/compression.diff)

Input excerpt:

```text
// Copyright 2014 Manu Martinez-Almeida. All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

package gin

import (
	"fmt"
	"html/template"
	"net"
	"net/http"
	"os"
	"path"
	"regexp"
	"strings"
	"sync"

	"github.com/gin-gonic/gin/internal/bytesconv"
	"github.com/gin-gonic/gin/render"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"
)

const defaultMultipartMemory = 32 << 20 // 32 MB

var (
	default404Body = []byte("404 page not found")
	default405Body = []byte("405 method not allowed")
)

var defaultPlatform string

var defaultTrustedCIDRs = []*net.IPNet{
	{ // 0.0.0.0/0 (IPv4)
		IP:   net.IP{0x0, 0x0, 0x0, 0x0},
		Mask: net.IPMask{0x0, 0x0, 0x0, 0x0},

```

Output excerpt:

```text
// Copyright 2014 Manu Martinez-Almeida. All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

package gin

import (
	"fmt"
	"html/template"
	"net"
	"net/http"
	"os"
	"path"
	"regexp"
	"strings"
	"sync"

	"github.com/gin-gonic/gin/internal/bytesconv"
	"github.com/gin-gonic/gin/render"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"
)

const defaultMultipartMemory = 32 << 20 // 32 MB

var (
	default404Body = []byte("404 page not found")
	default405Body = []byte("405 method not allowed")
)

var defaultPlatform string

var defaultTrustedCIDRs = []*net.IPNet{
	{ … 9 line(s) … ⟦tj:d30e8e961b5f84edc079373f424d6a7a⟧ }

var regSafePrefix = regexp.MustCompile("[^a-zA-Z0-9/-]+")

```

### `09-py-requests-sessions`

- [Full input](cases/09-py-requests-sessions/input.py)
- [Full output](cases/09-py-requests-sessions/output.py)
- [Input vs output diff](cases/09-py-requests-sessions/compression.diff)

Input excerpt:

```text
"""
requests.sessions
~~~~~~~~~~~~~~~~~

This module provides a Session object to manage and persist settings across
requests (cookies, auth, proxies).
"""
import os
import sys
import time
from collections import OrderedDict
from datetime import timedelta

from ._internal_utils import to_native_string
from .adapters import HTTPAdapter
from .auth import _basic_auth_str
from .compat import Mapping, cookielib, urljoin, urlparse
from .cookies import (
    RequestsCookieJar,
    cookiejar_from_dict,
    extract_cookies_to_jar,
    merge_cookies,
)
from .exceptions import (
    ChunkedEncodingError,
    ContentDecodingError,
    InvalidSchema,
    TooManyRedirects,
)
from .hooks import default_hooks, dispatch_hook

# formerly defined here, reexposed here for backward compatibility
from .models import (  # noqa: F401
    DEFAULT_REDIRECT_LIMIT,
    REDIRECT_STATI,
    PreparedRequest,

```

Output excerpt:

```text
"""
requests.sessions
~~~~~~~~~~~~~~~~~

This module provides a Session object to manage and persist settings across
requests (cookies, auth, proxies).
"""
import os
import sys
import time
from collections import OrderedDict
from datetime import timedelta

from ._internal_utils import to_native_string
from .adapters import HTTPAdapter
from .auth import _basic_auth_str
from .compat import Mapping, cookielib, urljoin, urlparse
from .cookies import (
    RequestsCookieJar,
    cookiejar_from_dict,
    extract_cookies_to_jar,
    merge_cookies,
)
from .exceptions import (
    ChunkedEncodingError,
    ContentDecodingError,
    InvalidSchema,
    TooManyRedirects,
)
from .hooks import default_hooks, dispatch_hook

# formerly defined here, reexposed here for backward compatibility
from .models import (  # noqa: F401
    DEFAULT_REDIRECT_LIMIT,
    REDIRECT_STATI,
    PreparedRequest,

```

### `42-js-kruskal-mst`

- [Full input](cases/42-js-kruskal-mst/input.js)
- [Full output](cases/42-js-kruskal-mst/output.js)
- [Input vs output diff](cases/42-js-kruskal-mst/compression.diff)

Input excerpt:

```text
class DisjointSetTreeNode {
  // Disjoint Set Node to store the parent and rank
  constructor(key) {
    this.key = key
    this.parent = this
    this.rank = 0
  }
}

class DisjointSetTree {
  // Disjoint Set DataStructure
  constructor() {
    // map to from node name to the node object
    this.map = {}
  }

  makeSet(x) {
    // Function to create a new set with x as its member
    this.map[x] = new DisjointSetTreeNode(x)
  }

  findSet(x) {
    // Function to find the set x belongs to (with path-compression)
    if (this.map[x] !== this.map[x].parent) {
      this.map[x].parent = this.findSet(this.map[x].parent.key)
    }
    return this.map[x].parent
  }

  union(x, y) {
    // Function to merge 2 disjoint sets
    this.link(this.findSet(x), this.findSet(y))
  }

  link(x, y) {
    // Helper function for union operation

```

Output excerpt:

```text
class DisjointSetTreeNode {
  // Disjoint Set Node to store the parent and rank
  constructor(key) {
    this.key = key
    this.parent = this
    this.rank = 0
  }
}

class DisjointSetTree {
  // Disjoint Set DataStructure
  constructor() {
    // map to from node name to the node object
    this.map = {}
  }

  makeSet(x) {
    // Function to create a new set with x as its member
    this.map[x] = new DisjointSetTreeNode(x)
  }

  findSet(x) {
    // Function to find the set x belongs to (with path-compression)
    if (this.map[x] !== this.map[x].parent) {
      this.map[x].parent = this.findSet(this.map[x].parent.key)
    }
    return this.map[x].parent
  }

  union(x, y) {
    // Function to merge 2 disjoint sets
    this.link(this.findSet(x), this.findSet(y))
  }

  link(x, y) { … 11 line(s) … ⟦tj:9f99af8cff2f922138fd7f3a02ff748b⟧ }
}

```

### `35-go-segment-tree`

- [Full input](cases/35-go-segment-tree/input.go)
- [Full output](cases/35-go-segment-tree/output.go)
- [Input vs output diff](cases/35-go-segment-tree/compression.diff)

Input excerpt:

```text
// Segment Tree Data Structure for efficient range queries on an array of integers.
// It can query the sum and update the elements to a new value of any range of the array.
// Build: O(n*log(n))
// Query: O(log(n))
// Update: O(log(n))
// reference: https://cp-algorithms.com/data_structures/segment_tree.html
package segmenttree

import (
	"github.com/TheAlgorithms/Go/math/max"
	"github.com/TheAlgorithms/Go/math/min"
)

const emptyLazyNode = 0

// SegmentTree represents the data structure of a segment tree with lazy propagation
type SegmentTree struct {
	Array       []int // The original array
	SegmentTree []int // Stores the sum of different ranges
	LazyTree    []int // Stores the values of lazy propagation
}

// Propagate propagates the lazy updates to the child nodes
func (s *SegmentTree) Propagate(node int, leftNode int, rightNode int) {
	if s.LazyTree[node] != emptyLazyNode {
		//add lazy node value multiplied by (right-left+1), which represents all interval
		//this is the same of adding a value on each node
		s.SegmentTree[node] += (rightNode - leftNode + 1) * s.LazyTree[node]

		if leftNode == rightNode {
			//leaf node
			s.Array[leftNode] += s.LazyTree[node]
		} else {
			//propagate lazy node value for children nodes
			//may propagate multiple times, children nodes should accumulate lazy node value
			s.LazyTree[2*node] += s.LazyTree[node]

```

Output excerpt:

```text
// Segment Tree Data Structure for efficient range queries on an array of integers.
// It can query the sum and update the elements to a new value of any range of the array.
// Build: O(n*log(n))
// Query: O(log(n))
// Update: O(log(n))
// reference: https://cp-algorithms.com/data_structures/segment_tree.html
package segmenttree

import (
	"github.com/TheAlgorithms/Go/math/max"
	"github.com/TheAlgorithms/Go/math/min"
)

const emptyLazyNode = 0

// SegmentTree represents the data structure of a segment tree with lazy propagation
type SegmentTree struct {
	Array       []int // The original array
	SegmentTree []int // Stores the sum of different ranges
	LazyTree    []int // Stores the values of lazy propagation
}

// Propagate propagates the lazy updates to the child nodes
func (s *SegmentTree) Propagate(node int, leftNode int, rightNode int) {
	if s.LazyTree[node] != emptyLazyNode {
		//add lazy node value multiplied by (right-left+1), which represents all interval
	{ … 15 line(s) … ⟦tj:275c6a6ade52bfab833e0cef116197e2⟧ }
	}

// Query returns the sum of elements of the array in the interval [firstIndex, leftIndex].
// node, leftNode and rightNode should always start with 1, 0 and len(Array)-1, respectively.
func (s *SegmentTree) Query(node int, leftNode int, rightNode int, firstIndex int, lastIndex int) int {
	if (firstIndex > lastIndex) || (leftNode > rightNode) {
		//outside the interval
	{ … 17 line(s) … ⟦tj:c4903fc762fa139e9c248aa15c751f78⟧ }
	return leftNodeSum + rightNodeSum

```

### `12-go-cobra-command`

- [Full input](cases/12-go-cobra-command/input.go)
- [Full output](cases/12-go-cobra-command/output.go)
- [Input vs output diff](cases/12-go-cobra-command/compression.diff)

Input excerpt:

```text
// Copyright 2013-2023 The Cobra Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package cobra is a commander providing a simple interface to create powerful modern CLI interfaces.
// In addition to providing an interface, Cobra simultaneously provides a controller to organize your application code.
package cobra

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"

	flag "github.com/spf13/pflag"
)

const (
	FlagSetByCobraAnnotation     = "cobra_annotation_flag_set_by_cobra"
	CommandDisplayNameAnnotation = "cobra_annotation_command_display_name"
)

```

Output excerpt:

```text
// Copyright 2013-2023 The Cobra Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package cobra is a commander providing a simple interface to create powerful modern CLI interfaces.
// In addition to providing an interface, Cobra simultaneously provides a controller to organize your application code.
package cobra

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"

	flag "github.com/spf13/pflag"
)

const (
	FlagSetByCobraAnnotation     = "cobra_annotation_flag_set_by_cobra"
	CommandDisplayNameAnnotation = "cobra_annotation_command_display_name"
)

```

### `17-c-curl-url`

- [Full input](cases/17-c-curl-url/input.c)
- [Full output](cases/17-c-curl-url/output.c)
- [Input vs output diff](cases/17-c-curl-url/compression.diff)

Input excerpt:

```text
/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at https://curl.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 * SPDX-License-Identifier: curl
 *
 ***************************************************************************/

#include "curl_setup.h"

#ifdef HAVE_NETINET_IN_H
#include <netinet/in.h>
#endif
#ifdef HAVE_NETDB_H
#include <netdb.h>
#endif
#ifdef HAVE_ARPA_INET_H
#include <arpa/inet.h>
#endif
#ifdef HAVE_NET_IF_H

```

Output excerpt:

```text
/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at https://curl.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 * SPDX-License-Identifier: curl
 *
 ***************************************************************************/

#include "curl_setup.h"

#ifdef HAVE_NETINET_IN_H
#include <netinet/in.h>
#endif
#ifdef HAVE_NETDB_H
#include <netdb.h>
#endif
#ifdef HAVE_ARPA_INET_H
#include <arpa/inet.h>
#endif
#ifdef HAVE_NET_IF_H

```

### `26-cs-newtonsoft-serializer`

- [Full input](cases/26-cs-newtonsoft-serializer/input.cs)
- [Full output](cases/26-cs-newtonsoft-serializer/output.cs)
- [Input vs output diff](cases/26-cs-newtonsoft-serializer/compression.diff)

Input excerpt:

```text
#region License
// Copyright (c) 2007 James Newton-King
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
#endregion

using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.Serialization.Formatters;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;
using Newtonsoft.Json.Utilities;
using System.Runtime.Serialization;

```

Output excerpt:

```text
#region License
// Copyright (c) 2007 James Newton-King
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
#endregion

using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Runtime.Serialization.Formatters;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;
using Newtonsoft.Json.Utilities;
using System.Runtime.Serialization;

```

### `03-ts-vscode-uri`

- [Full input](cases/03-ts-vscode-uri/input.ts)
- [Full output](cases/03-ts-vscode-uri/output.ts)
- [Input vs output diff](cases/03-ts-vscode-uri/compression.diff)

Input excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CharCode } from './charCode.js';
import { MarshalledId } from './marshallingIds.js';
import * as paths from './path.js';
import { isWindows } from './platform.js';

const _schemePattern = /^\w[\w\d+.-]*$/;
const _singleSlashStart = /^\//;
const _doubleSlashStart = /^\/\//;

function _validateUri(ret: URI, _strict?: boolean): void {

	// scheme, must be set
	if (!ret.scheme && _strict) {
		throw new Error(`[UriError]: Scheme is missing: {scheme: "", authority: "${ret.authority}", path: "${ret.path}", query: "${ret.query}", fragment: "${ret.fragment}"}`);
	}

	// scheme, https://tools.ietf.org/html/rfc3986#section-3.1
	// ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
	if (ret.scheme && !_schemePattern.test(ret.scheme)) {
		throw new Error('[UriError]: Scheme contains illegal characters.');
	}

	// path, http://tools.ietf.org/html/rfc3986#section-3.3
	// If a URI contains an authority component, then the path component
	// must either be empty or begin with a slash ("/") character.  If a URI
	// does not contain an authority component, then the path cannot begin
	// with two slash characters ("//").
	if (ret.path) {
		if (ret.authority) {
			if (!_singleSlashStart.test(ret.path)) {
				throw new Error('[UriError]: If a URI contains an authority component, then the path component must either be empty or begin with a slash ("/") character');

```

Output excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CharCode } from './charCode.js';
import { MarshalledId } from './marshallingIds.js';
import * as paths from './path.js';
import { isWindows } from './platform.js';

const _schemePattern = /^\w[\w\d+.-]*$/;
const _singleSlashStart = /^\//;
const _doubleSlashStart = /^\/\//;

function _validateUri(ret: URI, _strict?: boolean): void {

	// scheme, must be set
	if (!ret.scheme && _strict) {
		throw new Error(`[UriError]: Scheme is missing: {scheme: "", authority: "${ret.authority}", path: "${ret.path}", query: "${ret.query}", fragment: "${ret.fragment}"}`);
	}

	// scheme, https://tools.ietf.org/html/rfc3986#section-3.1
	// ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
	if (ret.scheme && !_schemePattern.test(ret.scheme)) {
		throw new Error('[UriError]: Scheme contains illegal characters.');
	}

	// path, http://tools.ietf.org/html/rfc3986#section-3.3
	// If a URI contains an authority component, then the path component
	// must either be empty or begin with a slash ("/") character.  If a URI
	// does not contain an authority component, then the path cannot begin
	// with two slash characters ("//").
	if (ret.path) {
		if (ret.authority) {
			if (!_singleSlashStart.test(ret.path)) {
				throw new Error('[UriError]: If a URI contains an authority component, then the path component must either be empty or begin with a slash ("/") character');

```

### `39-ts-binary-search-tree`

- [Full input](cases/39-ts-binary-search-tree/input.ts)
- [Full output](cases/39-ts-binary-search-tree/output.ts)
- [Input vs output diff](cases/39-ts-binary-search-tree/compression.diff)

Input excerpt:

```text
/**
 * Represents a node of a binary search tree.
 *
 * @template T The type of the value stored in the node.
 */
class TreeNode<T> {
  constructor(
    public data: T,
    public leftChild?: TreeNode<T>,
    public rightChild?: TreeNode<T>
  ) {}
}

/**
 * An implementation of a binary search tree.
 *
 * A binary tree is a tree with only two children per node. A binary search tree on top sorts the children according
 * to following rules:
 * - left child < parent node
 * - right child > parent node
 * - all children on the left side < root node
 * - all children on the right side > root node
 *
 * For profound information about trees
 * @see https://www.geeksforgeeks.org/introduction-to-tree-data-structure-and-algorithm-tutorials/
 *
 * @template T The data type of the values in the binary tree.
 */
export class BinarySearchTree<T> {
  rootNode?: TreeNode<T>

  /**
   * Instantiates the binary search tree.
   *
   * @param rootNode The root node.
   */

```

Output excerpt:

```text
/**
 * Represents a node of a binary search tree.
 *
 * @template T The type of the value stored in the node.
 */
class TreeNode<T> {
  constructor(
    public data: T,
    public leftChild?: TreeNode<T>,
    public rightChild?: TreeNode<T>
  ) {}
}

/**
 * An implementation of a binary search tree.
 *
 * A binary tree is a tree with only two children per node. A binary search tree on top sorts the children according
 * to following rules:
 * - left child < parent node
 * - right child > parent node
 * - all children on the left side < root node
 * - all children on the right side > root node
 *
 * For profound information about trees
 * @see https://www.geeksforgeeks.org/introduction-to-tree-data-structure-and-algorithm-tutorials/
 *
 * @template T The data type of the values in the binary tree.
 */
export class BinarySearchTree<T> {
  rootNode?: TreeNode<T>

  /**
   * Instantiates the binary search tree.
   *
   * @param rootNode The root node.
   */

```

### `31-rs-huffman-encoding`

- [Full input](cases/31-rs-huffman-encoding/input.rs)
- [Full output](cases/31-rs-huffman-encoding/output.rs)
- [Input vs output diff](cases/31-rs-huffman-encoding/compression.diff)

Input excerpt:

```rust
//! Huffman Encoding implementation
//!
//! Huffman coding is a lossless data compression algorithm that assigns variable-length codes
//! to characters based on their frequency of occurrence. Characters that occur more frequently
//! are assigned shorter codes, while less frequent characters get longer codes.
//!
//! # Algorithm Overview
//!
//! 1. Count the frequency of each character in the input
//! 2. Build a min-heap (priority queue) of nodes based on frequency
//! 3. Build the Huffman tree by repeatedly:
//!    - Remove two nodes with minimum frequency
//!    - Create a parent node with combined frequency
//!    - Insert the parent back into the heap
//! 4. Traverse the tree to assign binary codes to each character
//! 5. Encode the input using the generated codes
//!
//! # Time Complexity
//!
//! - Building frequency map: O(n) where n is input length
//! - Building Huffman tree: O(m log m) where m is number of unique characters
//! - Encoding: O(n)
//!
//! # Usage
//!
//! As a library:
//! ` ` `no_run
//! use the_algorithms_rust::compression::huffman_encode;
//!
//! let text = "hello world";
//! let (encoded, codes) = huffman_encode(text);
//! println!("Original: {}", text);
//! println!("Encoded: {}", encoded);
//! ` ` `
//!
//! As a command-line tool:

```

Output excerpt:

```rust
//! Huffman Encoding implementation
//!
//! Huffman coding is a lossless data compression algorithm that assigns variable-length codes
//! to characters based on their frequency of occurrence. Characters that occur more frequently
//! are assigned shorter codes, while less frequent characters get longer codes.
//!
//! # Algorithm Overview
//!
//! 1. Count the frequency of each character in the input
//! 2. Build a min-heap (priority queue) of nodes based on frequency
//! 3. Build the Huffman tree by repeatedly:
//!    - Remove two nodes with minimum frequency
//!    - Create a parent node with combined frequency
//!    - Insert the parent back into the heap
//! 4. Traverse the tree to assign binary codes to each character
//! 5. Encode the input using the generated codes
//!
//! # Time Complexity
//!
//! - Building frequency map: O(n) where n is input length
//! - Building Huffman tree: O(m log m) where m is number of unique characters
//! - Encoding: O(n)
//!
//! # Usage
//!
//! As a library:
//! ` ` `no_run
//! use the_algorithms_rust::compression::huffman_encode;
//!
//! let text = "hello world";
//! let (encoded, codes) = huffman_encode(text);
//! println!("Original: {}", text);
//! println!("Encoded: {}", encoded);
//! ` ` `
//!
//! As a command-line tool:

```

### `19-rs-ripgrep-walk`

- [Full input](cases/19-rs-ripgrep-walk/input.rs)
- [Full output](cases/19-rs-ripgrep-walk/output.rs)
- [Input vs output diff](cases/19-rs-ripgrep-walk/compression.diff)

Input excerpt:

```rust
use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs::{self, FileType, Metadata},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering},
    sync::Arc,
};

use {
    crossbeam_deque::{Stealer, Worker as Deque},
    same_file::Handle,
    walkdir::{self, WalkDir},
};

use crate::{
    dir::{Ignore, IgnoreBuilder},
    gitignore::GitignoreBuilder,
    overrides::Override,
    types::Types,
    Error, PartialErrorBuilder,
};

/// A directory entry with a possible error attached.
///
/// The error typically refers to a problem parsing ignore files in a
/// particular directory.
#[derive(Clone, Debug)]
pub struct DirEntry {
    dent: DirEntryInner,
    err: Option<Error>,
}

impl DirEntry {
    /// The full path that this entry represents.

```

Output excerpt:

```rust
use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs::{self, FileType, Metadata},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering},
    sync::Arc,
};

use {
    crossbeam_deque::{Stealer, Worker as Deque},
    same_file::Handle,
    walkdir::{self, WalkDir},
};

use crate::{
    dir::{Ignore, IgnoreBuilder},
    gitignore::GitignoreBuilder,
    overrides::Override,
    types::Types,
    Error, PartialErrorBuilder,
};

/// A directory entry with a possible error attached.
///
/// The error typically refers to a problem parsing ignore files in a
/// particular directory.
#[derive(Clone, Debug)]
pub struct DirEntry {
    dent: DirEntryInner,
    err: Option<Error>,
}

impl DirEntry {
    /// The full path that this entry represents.

```

### `37-cpp-random-pivot-quicksort`

- [Full input](cases/37-cpp-random-pivot-quicksort/input.cpp)
- [Full output](cases/37-cpp-random-pivot-quicksort/output.cpp)
- [Input vs output diff](cases/37-cpp-random-pivot-quicksort/compression.diff)

Input excerpt:

```text
/**
 * @file
 * @brief Implementation of the [Random Pivot Quick
 * Sort](https://www.sanfoundry.com/cpp-program-implement-quick-sort-using-randomisation)
 * algorithm.
 * @details
 *          * A random pivot quick sort algorithm is pretty much same as quick
 * sort with a difference of having a logic of selecting next pivot element from
 * the input array.
 *          * Where in quick sort is fast, but still can give you the time
 * complexity of O(n^2) in worst case.
 *          * To avoid hitting the time complexity of O(n^2), we use the logic
 * of randomize the selection process of pivot element.
 *
 *          ### Logic
 *              * The logic is pretty simple, the only change is in the
 * partitioning algorithm, which is selecting the pivot element.
 *              * Instead of selecting the last or the first element from array
 * for pivot we use a random index to select pivot element.
 *              * This avoids hitting the O(n^2) time complexity in practical
 * use cases.
 *
 *       ### Partition Logic
 *           * Partitions are done such as numbers lower than the "pivot"
 * element is arranged on the left side of the "pivot", and number larger than
 * the "pivot" element are arranged on the right part of the array.
 *
 *       ### Algorithm
 *           * Select the pivot element randomly using getRandomIndex() function
 * from this namespace.
 *           * Initialize the pInd (partition index) from the start of the
 * array.
 *           * Loop through the array from start to less than end. (from start
 * to < end). (Inside the loop) :-
 *                   * Check if the current element (arr[i]) is less than the
 * pivot element in each iteration.

```

Output excerpt:

```text
/**
 * @file
 * @brief Implementation of the [Random Pivot Quick
 * Sort](https://www.sanfoundry.com/cpp-program-implement-quick-sort-using-randomisation)
 * algorithm.
 * @details
 *          * A random pivot quick sort algorithm is pretty much same as quick
 * sort with a difference of having a logic of selecting next pivot element from
 * the input array.
 *          * Where in quick sort is fast, but still can give you the time
 * complexity of O(n^2) in worst case.
 *          * To avoid hitting the time complexity of O(n^2), we use the logic
 * of randomize the selection process of pivot element.
 *
 *          ### Logic
 *              * The logic is pretty simple, the only change is in the
 * partitioning algorithm, which is selecting the pivot element.
 *              * Instead of selecting the last or the first element from array
 * for pivot we use a random index to select pivot element.
 *              * This avoids hitting the O(n^2) time complexity in practical
 * use cases.
 *
 *       ### Partition Logic
 *           * Partitions are done such as numbers lower than the "pivot"
 * element is arranged on the left side of the "pivot", and number larger than
 * the "pivot" element are arranged on the right part of the array.
 *
 *       ### Algorithm
 *           * Select the pivot element randomly using getRandomIndex() function
 * from this namespace.
 *           * Initialize the pInd (partition index) from the start of the
 * array.
 *           * Loop through the array from start to less than end. (from start
 * to < end). (Inside the loop) :-
 *                   * Check if the current element (arr[i]) is less than the
 * pivot element in each iteration.

```

### `34-go-avl-tree`

- [Full input](cases/34-go-avl-tree/input.go)
- [Full output](cases/34-go-avl-tree/output.go)
- [Input vs output diff](cases/34-go-avl-tree/compression.diff)

Input excerpt:

```text
// AVL tree is a self-balancing binary search tree.
//
// For more details check out those link below here:
// Wikipedia article: https://en.wikipedia.org/wiki/AVL_tree
// see avl.go

package tree

import (
	"github.com/TheAlgorithms/Go/constraints"
	"github.com/TheAlgorithms/Go/math/max"
)

// Verify Interface Compliance
var _ Node[int] = &AVLNode[int]{}

// AVLNode represents a single node in the AVL.
type AVLNode[T constraints.Ordered] struct {
	key    T
	parent *AVLNode[T]
	left   *AVLNode[T]
	right  *AVLNode[T]
	height int
}

func (n *AVLNode[T]) Key() T {
	return n.key
}

func (n *AVLNode[T]) Parent() Node[T] {
	return n.parent
}

func (n *AVLNode[T]) Left() Node[T] {
	return n.left
}

```

Output excerpt:

```text
// AVL tree is a self-balancing binary search tree.
//
// For more details check out those link below here:
// Wikipedia article: https://en.wikipedia.org/wiki/AVL_tree
// see avl.go

package tree

import (
	"github.com/TheAlgorithms/Go/constraints"
	"github.com/TheAlgorithms/Go/math/max"
)

// Verify Interface Compliance
var _ Node[int] = &AVLNode[int]{}

// AVLNode represents a single node in the AVL.
type AVLNode[T constraints.Ordered] struct {
	key    T
	parent *AVLNode[T]
	left   *AVLNode[T]
	right  *AVLNode[T]
	height int
}

func (n *AVLNode[T]) Key() T {
	return n.key
}

func (n *AVLNode[T]) Parent() Node[T] {
	return n.parent
}

func (n *AVLNode[T]) Left() Node[T] {
	return n.left
}

```

### `07-py-django-paginator`

- [Full input](cases/07-py-django-paginator/input.py)
- [Full output](cases/07-py-django-paginator/output.py)
- [Input vs output diff](cases/07-py-django-paginator/compression.diff)

Input excerpt:

```text
import collections.abc
import inspect
import warnings
from math import ceil

from django.utils.functional import cached_property
from django.utils.inspect import method_has_no_args
from django.utils.translation import gettext_lazy as _


class UnorderedObjectListWarning(RuntimeWarning):
    pass


class InvalidPage(Exception):
    pass


class PageNotAnInteger(InvalidPage):
    pass


class EmptyPage(InvalidPage):
    pass


class Paginator:
    # Translators: String used to replace omitted page numbers in elided page
    # range generated by paginators, e.g. [1, 2, '…', 5, 6, 7, '…', 9, 10].
    ELLIPSIS = _("…")
    default_error_messages = {
        "invalid_page": _("That page number is not an integer"),
        "min_page": _("That page number is less than 1"),
        "no_results": _("That page contains no results"),
    }


```

Output excerpt:

```text
import collections.abc
import inspect
import warnings
from math import ceil

from django.utils.functional import cached_property
from django.utils.inspect import method_has_no_args
from django.utils.translation import gettext_lazy as _


class UnorderedObjectListWarning(RuntimeWarning):
    pass


class InvalidPage(Exception):
    pass


class PageNotAnInteger(InvalidPage):
    pass


class EmptyPage(InvalidPage):
    pass


class Paginator:
    # Translators: String used to replace omitted page numbers in elided page
    # range generated by paginators, e.g. [1, 2, '…', 5, 6, 7, '…', 9, 10].
    ELLIPSIS = _("…")
    default_error_messages = {
        "invalid_page": _("That page number is not an integer"),
        "min_page": _("That page number is less than 1"),
        "no_results": _("That page contains no results"),
    }


```

### `08-py-flask-app`

- [Full input](cases/08-py-flask-app/input.py)
- [Full output](cases/08-py-flask-app/output.py)
- [Input vs output diff](cases/08-py-flask-app/compression.diff)

Input excerpt:

```text
from __future__ import annotations

import collections.abc as cabc
import os
import sys
import typing as t
import weakref
from datetime import timedelta
from inspect import iscoroutinefunction
from itertools import chain
from types import TracebackType
from urllib.parse import quote as _url_quote

import click
from werkzeug.datastructures import Headers
from werkzeug.datastructures import ImmutableDict
from werkzeug.exceptions import BadRequestKeyError
from werkzeug.exceptions import HTTPException
from werkzeug.exceptions import InternalServerError
from werkzeug.routing import BuildError
from werkzeug.routing import MapAdapter
from werkzeug.routing import RequestRedirect
from werkzeug.routing import RoutingException
from werkzeug.routing import Rule
from werkzeug.serving import is_running_from_reloader
from werkzeug.wrappers import Response as BaseResponse

from . import cli
from . import typing as ft
from .ctx import AppContext
from .ctx import RequestContext
from .globals import _cv_app
from .globals import _cv_request
from .globals import current_app
from .globals import g
from .globals import request

```

Output excerpt:

```text
from __future__ import annotations

import collections.abc as cabc
import os
import sys
import typing as t
import weakref
from datetime import timedelta
from inspect import iscoroutinefunction
from itertools import chain
from types import TracebackType
from urllib.parse import quote as _url_quote

import click
from werkzeug.datastructures import Headers
from werkzeug.datastructures import ImmutableDict
from werkzeug.exceptions import BadRequestKeyError
from werkzeug.exceptions import HTTPException
from werkzeug.exceptions import InternalServerError
from werkzeug.routing import BuildError
from werkzeug.routing import MapAdapter
from werkzeug.routing import RequestRedirect
from werkzeug.routing import RoutingException
from werkzeug.routing import Rule
from werkzeug.serving import is_running_from_reloader
from werkzeug.wrappers import Response as BaseResponse

from . import cli
from . import typing as ft
from .ctx import AppContext
from .ctx import RequestContext
from .globals import _cv_app
from .globals import _cv_request
from .globals import current_app
from .globals import g
from .globals import request

```

### `46-cs-topological-sort`

- [Full input](cases/46-cs-topological-sort/input.cs)
- [Full output](cases/46-cs-topological-sort/output.cs)
- [Input vs output diff](cases/46-cs-topological-sort/compression.diff)

Input excerpt:

```text
using DataStructures.Graph;

namespace Algorithms.Graph;

/// <summary>
///     Topological Sort is a linear ordering of vertices in a Directed Acyclic Graph (DAG)
///     such that for every directed edge (u, v), vertex u comes before vertex v in the ordering.
///
///     KEY CONCEPTS:
///     1. Only applicable to Directed Acyclic Graphs (DAGs) - graphs with no cycles.
///     2. A DAG can have multiple valid topological orderings.
///     3. Used in dependency resolution, task scheduling, build systems, and course prerequisites.
///
///     ALGORITHM APPROACHES:
///     1. DFS-based (Depth-First Search): Uses post-order traversal and reverses the result.
///     2. Kahn's Algorithm: Uses in-degree counting and processes vertices with zero in-degree.
///
///     TIME COMPLEXITY: O(V + E) where V is vertices and E is edges.
///     SPACE COMPLEXITY: O(V) for the visited set and result stack.
///
///     Reference: "Introduction to Algorithms" (CLRS) by Cormen, Leiserson, Rivest, and Stein.
///     Also covered in "Algorithm Design Manual" by Steven Skiena.
/// </summary>
/// <typeparam name="T">Vertex data type.</typeparam>
public class TopologicalSort<T> where T : IComparable<T>
{
    /// <summary>
    ///     Performs topological sort on a directed acyclic graph using DFS-based approach.
    ///
    ///     ALGORITHM STEPS (DFS-based approach):
    ///     1. Initialize a visited set to track processed vertices.
    ///     2. Initialize a stack to store the topological ordering.
    ///     3. For each unvisited vertex in the graph:
    ///        a) Perform DFS from that vertex.
    ///        b) After visiting all descendants, push the vertex to the stack.
    ///     4. The stack now contains vertices in reverse topological order.

```

Output excerpt:

```text
using DataStructures.Graph;

namespace Algorithms.Graph;

/// <summary>
///     Topological Sort is a linear ordering of vertices in a Directed Acyclic Graph (DAG)
///     such that for every directed edge (u, v), vertex u comes before vertex v in the ordering.
///
///     KEY CONCEPTS:
///     1. Only applicable to Directed Acyclic Graphs (DAGs) - graphs with no cycles.
///     2. A DAG can have multiple valid topological orderings.
///     3. Used in dependency resolution, task scheduling, build systems, and course prerequisites.
///
///     ALGORITHM APPROACHES:
///     1. DFS-based (Depth-First Search): Uses post-order traversal and reverses the result.
///     2. Kahn's Algorithm: Uses in-degree counting and processes vertices with zero in-degree.
///
///     TIME COMPLEXITY: O(V + E) where V is vertices and E is edges.
///     SPACE COMPLEXITY: O(V) for the visited set and result stack.
///
///     Reference: "Introduction to Algorithms" (CLRS) by Cormen, Leiserson, Rivest, and Stein.
///     Also covered in "Algorithm Design Manual" by Steven Skiena.
/// </summary>
/// <typeparam name="T">Vertex data type.</typeparam>
public class TopologicalSort<T> where T : IComparable<T>
{
    /// <summary>
    ///     Performs topological sort on a directed acyclic graph using DFS-based approach.
    ///
    ///     ALGORITHM STEPS (DFS-based approach):
    ///     1. Initialize a visited set to track processed vertices.
    ///     2. Initialize a stack to store the topological ordering.
    ///     3. For each unvisited vertex in the graph:
    ///        a) Perform DFS from that vertex.
    ///        b) After visiting all descendants, push the vertex to the stack.
    ///     4. The stack now contains vertices in reverse topological order.

```

### `06-py-django-request`

- [Full input](cases/06-py-django-request/input.py)
- [Full output](cases/06-py-django-request/output.py)
- [Input vs output diff](cases/06-py-django-request/compression.diff)

Input excerpt:

```text
import codecs
import copy
from io import BytesIO
from itertools import chain
from urllib.parse import parse_qsl, quote, urlencode, urljoin, urlsplit

from django.conf import settings
from django.core import signing
from django.core.exceptions import (
    BadRequest,
    DisallowedHost,
    ImproperlyConfigured,
    RequestDataTooBig,
    TooManyFieldsSent,
)
from django.core.files import uploadhandler
from django.http.multipartparser import (
    MultiPartParser,
    MultiPartParserError,
    TooManyFilesSent,
)
from django.utils.datastructures import (
    CaseInsensitiveMapping,
    ImmutableList,
    MultiValueDict,
)
from django.utils.encoding import escape_uri_path, iri_to_uri
from django.utils.functional import cached_property
from django.utils.http import is_same_domain, parse_header_parameters
from django.utils.regex_helper import _lazy_re_compile

RAISE_ERROR = object()
host_validation_re = _lazy_re_compile(
    r"^([a-z0-9.-]+|\[[a-f0-9]*:[a-f0-9.:]+\])(?::([0-9]+))?$"
)


```

Output excerpt:

```text
import codecs
import copy
from io import BytesIO
from itertools import chain
from urllib.parse import parse_qsl, quote, urlencode, urljoin, urlsplit

from django.conf import settings
from django.core import signing
from django.core.exceptions import (
    BadRequest,
    DisallowedHost,
    ImproperlyConfigured,
    RequestDataTooBig,
    TooManyFieldsSent,
)
from django.core.files import uploadhandler
from django.http.multipartparser import (
    MultiPartParser,
    MultiPartParserError,
    TooManyFilesSent,
)
from django.utils.datastructures import (
    CaseInsensitiveMapping,
    ImmutableList,
    MultiValueDict,
)
from django.utils.encoding import escape_uri_path, iri_to_uri
from django.utils.functional import cached_property
from django.utils.http import is_same_domain, parse_header_parameters
from django.utils.regex_helper import _lazy_re_compile

RAISE_ERROR = object()
host_validation_re = _lazy_re_compile(
    r"^([a-z0-9.-]+|\[[a-f0-9]*:[a-f0-9.:]+\])(?::([0-9]+))?$"
)


```

### `13-go-mux-mux`

- [Full input](cases/13-go-mux-mux/input.go)
- [Full output](cases/13-go-mux-mux/output.go)
- [Input vs output diff](cases/13-go-mux-mux/compression.diff)

Input excerpt:

```text
// Copyright 2012 The Gorilla Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package mux

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"path"
	"regexp"
)

var (
	// ErrMethodMismatch is returned when the method in the request does not match
	// the method defined against the route.
	ErrMethodMismatch = errors.New("method is not allowed")
	// ErrNotFound is returned when no route match is found.
	ErrNotFound = errors.New("no matching route was found")
)

// NewRouter returns a new router instance.
func NewRouter() *Router {
	return &Router{namedRoutes: make(map[string]*Route)}
}

// Router registers routes to be matched and dispatches a handler.
//
// It implements the http.Handler interface, so it can be registered to serve
// requests:
//
//	var router = mux.NewRouter()
//
//	func main() {

```

Output excerpt:

```text
// Copyright 2012 The Gorilla Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package mux

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"path"
	"regexp"
)

var (
	// ErrMethodMismatch is returned when the method in the request does not match
	// the method defined against the route.
	ErrMethodMismatch = errors.New("method is not allowed")
	// ErrNotFound is returned when no route match is found.
	ErrNotFound = errors.New("no matching route was found")
)

// NewRouter returns a new router instance.
func NewRouter() *Router {
	return &Router{namedRoutes: make(map[string]*Route)}
}

// Router registers routes to be matched and dispatches a handler.
//
// It implements the http.Handler interface, so it can be registered to serve
// requests:
//
//	var router = mux.NewRouter()
//
//	func main() {

```

### `45-php-avl-tree`

- [Full input](cases/45-php-avl-tree/input.php)
- [Full output](cases/45-php-avl-tree/output.php)
- [Input vs output diff](cases/45-php-avl-tree/compression.diff)

Input excerpt:

```text
<?php

/*
 * Created by: Ramy-Badr-Ahmed (https://github.com/Ramy-Badr-Ahmed)
 * in Pull Request #163: https://github.com/TheAlgorithms/PHP/pull/163
 * and #173: https://github.com/TheAlgorithms/PHP/pull/173
 *
 * Please mention me (@Ramy-Badr-Ahmed) in any issue or pull request addressing bugs/corrections to this file.
 * Thank you!
 */

namespace DataStructures\AVLTree;

/**
 * Class AVLTree
 * Implements an AVL Tree data structure with self-balancing capability.
 */
class AVLTree
{
    private ?AVLTreeNode $root;
    private int $counter;

    public function __construct()
    {
        $this->root = null;
        $this->counter = 0;
    }

    /**
     * Get the root node of the AVL Tree.
     */
    public function getRoot(): ?AVLTreeNode
    {
        return $this->root;
    }


```

Output excerpt:

```text
<?php

/*
 * Created by: Ramy-Badr-Ahmed (https://github.com/Ramy-Badr-Ahmed)
 * in Pull Request #163: https://github.com/TheAlgorithms/PHP/pull/163
 * and #173: https://github.com/TheAlgorithms/PHP/pull/173
 *
 * Please mention me (@Ramy-Badr-Ahmed) in any issue or pull request addressing bugs/corrections to this file.
 * Thank you!
 */

namespace DataStructures\AVLTree;

/**
 * Class AVLTree
 * Implements an AVL Tree data structure with self-balancing capability.
 */
class AVLTree
{
    private ?AVLTreeNode $root;
    private int $counter;

    public function __construct()
    {
        $this->root = null;
        $this->counter = 0;
    }

    /**
     * Get the root node of the AVL Tree.
     */
    public function getRoot(): ?AVLTreeNode
    {
        return $this->root;
    }


```

### `24-php-laravel-container`

- [Full input](cases/24-php-laravel-container/input.php)
- [Full output](cases/24-php-laravel-container/output.php)
- [Input vs output diff](cases/24-php-laravel-container/compression.diff)

Input excerpt:

```text
<?php

namespace Illuminate\Container;

use ArrayAccess;
use Closure;
use Exception;
use Illuminate\Contracts\Container\BindingResolutionException;
use Illuminate\Contracts\Container\CircularDependencyException;
use Illuminate\Contracts\Container\Container as ContainerContract;
use LogicException;
use ReflectionClass;
use ReflectionException;
use ReflectionFunction;
use ReflectionParameter;
use TypeError;

class Container implements ArrayAccess, ContainerContract
{
    /**
     * The current globally available container (if any).
     *
     * @var static
     */
    protected static $instance;

    /**
     * An array of the types that have been resolved.
     *
     * @var bool[]
     */
    protected $resolved = [];

    /**
     * The container's bindings.
     *

```

Output excerpt:

```text
<?php

namespace Illuminate\Container;

use ArrayAccess;
use Closure;
use Exception;
use Illuminate\Contracts\Container\BindingResolutionException;
use Illuminate\Contracts\Container\CircularDependencyException;
use Illuminate\Contracts\Container\Container as ContainerContract;
use LogicException;
use ReflectionClass;
use ReflectionException;
use ReflectionFunction;
use ReflectionParameter;
use TypeError;

class Container implements ArrayAccess, ContainerContract
{
    /**
     * The current globally available container (if any).
     *
     * @var static
     */
    protected static $instance;

    /**
     * An array of the types that have been resolved.
     *
     * @var bool[]
     */
    protected $resolved = [];

    /**
     * The container's bindings.
     *

```

### `01-ts-vscode-async`

- [Full input](cases/01-ts-vscode-async/input.ts)
- [Full output](cases/01-ts-vscode-async/output.ts)
- [Input vs output diff](cases/01-ts-vscode-async/compression.diff)

Input excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CancellationToken, CancellationTokenSource } from './cancellation.js';
import { BugIndicatingError, CancellationError } from './errors.js';
import { Emitter, Event } from './event.js';
import { Disposable, DisposableMap, DisposableStore, IDisposable, MutableDisposable, toDisposable } from './lifecycle.js';
import { extUri as defaultExtUri, IExtUri } from './resources.js';
import { URI } from './uri.js';
import { setTimeout0 } from './platform.js';
import { MicrotaskDelay } from './symbols.js';
import { Lazy } from './lazy.js';

export function isThenable<T>(obj: unknown): obj is Promise<T> {
	return !!obj && typeof (obj as unknown as Promise<T>).then === 'function';
}

export interface CancelablePromise<T> extends Promise<T> {
	cancel(): void;
}

export function createCancelablePromise<T>(callback: (token: CancellationToken) => Promise<T>): CancelablePromise<T> {
	const source = new CancellationTokenSource();

	const thenable = callback(source.token);
	const promise = new Promise<T>((resolve, reject) => {
		const subscription = source.token.onCancellationRequested(() => {
			subscription.dispose();
			reject(new CancellationError());
		});
		Promise.resolve(thenable).then(value => {
			subscription.dispose();
			source.dispose();
			resolve(value);

```

Output excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CancellationToken, CancellationTokenSource } from './cancellation.js';
import { BugIndicatingError, CancellationError } from './errors.js';
import { Emitter, Event } from './event.js';
import { Disposable, DisposableMap, DisposableStore, IDisposable, MutableDisposable, toDisposable } from './lifecycle.js';
import { extUri as defaultExtUri, IExtUri } from './resources.js';
import { URI } from './uri.js';
import { setTimeout0 } from './platform.js';
import { MicrotaskDelay } from './symbols.js';
import { Lazy } from './lazy.js';

export function isThenable<T>(obj: unknown): obj is Promise<T> {
	return !!obj && typeof (obj as unknown as Promise<T>).then === 'function';
}

export interface CancelablePromise<T> extends Promise<T> {
	cancel(): void;
}

export function createCancelablePromise<T>(callback: (token: CancellationToken) => Promise<T>): CancelablePromise<T> {
	const source = new CancellationTokenSource();

	{ … 31 line(s) … ⟦tj:909236ff0c71c58ac77dba1335f62bff⟧ }
	};
}

/**
 * Returns a promise that resolves with `undefined` as soon as the passed token is cancelled.
 * @see {@link raceCancellationError}
 */
export function raceCancellation<T>(promise: Promise<T>, token: CancellationToken): Promise<T | undefined>;


```

### `40-java-lru-cache`

- [Full input](cases/40-java-lru-cache/input.java)
- [Full output](cases/40-java-lru-cache/output.java)
- [Input vs output diff](cases/40-java-lru-cache/compression.diff)

Input excerpt:

```text
package com.thealgorithms.datastructures.caches;

import java.util.HashMap;
import java.util.Map;

/**
 * A Least Recently Used (LRU) Cache implementation.
 *
 * <p>An LRU cache is a fixed-size cache that maintains items in order of use. When the cache reaches
 * its capacity and a new item needs to be added, it removes the least recently used item first.
 * This implementation provides O(1) time complexity for both get and put operations.</p>
 *
 * <p>Features:</p>
 * <ul>
 *   <li>Fixed-size cache with configurable capacity</li>
 *   <li>Constant time O(1) operations for get and put</li>
 *   <li>Thread-unsafe - should be externally synchronized if used in concurrent environments</li>
 *   <li>Supports null values but not null keys</li>
 * </ul>
 *
 * <p>Implementation Details:</p>
 * <ul>
 *   <li>Uses a HashMap for O(1) key-value lookups</li>
 *   <li>Maintains a doubly-linked list for tracking access order</li>
 *   <li>The head of the list contains the least recently used item</li>
 *   <li>The tail of the list contains the most recently used item</li>
 * </ul>
 *
 * <p>Example usage:</p>
 * <pre>
 * LRUCache<String, Integer> cache = new LRUCache<>(3); // Create cache with capacity 3
 * cache.put("A", 1); // Cache: A=1
 * cache.put("B", 2); // Cache: A=1, B=2
 * cache.put("C", 3); // Cache: A=1, B=2, C=3
 * cache.get("A");    // Cache: B=2, C=3, A=1 (A moved to end)
 * cache.put("D", 4); // Cache: C=3, A=1, D=4 (B evicted)

```

Output excerpt:

```text
package com.thealgorithms.datastructures.caches;

import java.util.HashMap;
import java.util.Map;

/**
 * A Least Recently Used (LRU) Cache implementation.
 *
 * <p>An LRU cache is a fixed-size cache that maintains items in order of use. When the cache reaches
 * its capacity and a new item needs to be added, it removes the least recently used item first.
 * This implementation provides O(1) time complexity for both get and put operations.</p>
 *
 * <p>Features:</p>
 * <ul>
 *   <li>Fixed-size cache with configurable capacity</li>
 *   <li>Constant time O(1) operations for get and put</li>
 *   <li>Thread-unsafe - should be externally synchronized if used in concurrent environments</li>
 *   <li>Supports null values but not null keys</li>
 * </ul>
 *
 * <p>Implementation Details:</p>
 * <ul>
 *   <li>Uses a HashMap for O(1) key-value lookups</li>
 *   <li>Maintains a doubly-linked list for tracking access order</li>
 *   <li>The head of the list contains the least recently used item</li>
 *   <li>The tail of the list contains the most recently used item</li>
 * </ul>
 *
 * <p>Example usage:</p>
 * <pre>
 * LRUCache<String, Integer> cache = new LRUCache<>(3); // Create cache with capacity 3
 * cache.put("A", 1); // Cache: A=1
 * cache.put("B", 2); // Cache: A=1, B=2
 * cache.put("C", 3); // Cache: A=1, B=2, C=3
 * cache.get("A");    // Cache: B=2, C=3, A=1 (A moved to end)
 * cache.put("D", 4); // Cache: C=3, A=1, D=4 (B evicted)

```

### `32-rs-knapsack`

- [Full input](cases/32-rs-knapsack/input.rs)
- [Full output](cases/32-rs-knapsack/output.rs)
- [Input vs output diff](cases/32-rs-knapsack/compression.diff)

Input excerpt:

```rust
//! This module provides functionality to solve the knapsack problem using dynamic programming.
//! It includes structures for items and solutions, and functions to compute the optimal solution.

use std::cmp::Ordering;

/// Represents an item with a weight and a value.
#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    weight: usize,
    value: usize,
}

/// Represents the solution to the knapsack problem.
#[derive(Debug, PartialEq, Eq)]
pub struct KnapsackSolution {
    /// The optimal profit obtained.
    optimal_profit: usize,
    /// The total weight of items included in the solution.
    total_weight: usize,
    /// The indices of items included in the solution. Indices might not be unique.
    item_indices: Vec<usize>,
}

/// Solves the knapsack problem and returns the optimal profit, total weight, and indices of items included.
///
/// # Arguments:
/// * `capacity` - The maximum weight capacity of the knapsack.
/// * `items` - A vector of `Item` structs, each representing an item with weight and value.
///
/// # Returns:
/// A `KnapsackSolution` struct containing:
/// - `optimal_profit` - The maximum profit achievable with the given capacity and items.
/// - `total_weight` - The total weight of items included in the solution.
/// - `item_indices` - Indices of items included in the solution. Indices might not be unique.
///
/// # Note:

```

Output excerpt:

```rust
//! This module provides functionality to solve the knapsack problem using dynamic programming.
//! It includes structures for items and solutions, and functions to compute the optimal solution.

use std::cmp::Ordering;

/// Represents an item with a weight and a value.
#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    weight: usize,
    value: usize,
}

/// Represents the solution to the knapsack problem.
#[derive(Debug, PartialEq, Eq)]
pub struct KnapsackSolution {
    /// The optimal profit obtained.
    optimal_profit: usize,
    /// The total weight of items included in the solution.
    total_weight: usize,
    /// The indices of items included in the solution. Indices might not be unique.
    item_indices: Vec<usize>,
}

/// Solves the knapsack problem and returns the optimal profit, total weight, and indices of items included.
///
/// # Arguments:
/// * `capacity` - The maximum weight capacity of the knapsack.
/// * `items` - A vector of `Item` structs, each representing an item with weight and value.
///
/// # Returns:
/// A `KnapsackSolution` struct containing:
/// - `optimal_profit` - The maximum profit achievable with the given capacity and items.
/// - `total_weight` - The total weight of items included in the solution.
/// - `item_indices` - Indices of items included in the solution. Indices might not be unique.
///
/// # Note:

```

### `02-ts-vscode-strings`

- [Full input](cases/02-ts-vscode-strings/input.ts)
- [Full output](cases/02-ts-vscode-strings/output.ts)
- [Input vs output diff](cases/02-ts-vscode-strings/compression.diff)

Input excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { LRUCachedFunction } from './cache.js';
import { CharCode } from './charCode.js';
import { Lazy } from './lazy.js';
import { Constants } from './uint.js';

export function isFalsyOrWhitespace(str: string | undefined): boolean {
	if (!str || typeof str !== 'string') {
		return true;
	}
	return str.trim().length === 0;
}

const _formatRegexp = /{(\d+)}/g;

/**
 * Helper to produce a string with a variable number of arguments. Insert variable segments
 * into the string using the {n} notation where N is the index of the argument following the string.
 * @param value string to which formatting is applied
 * @param args replacements for {n}-entries
 */
export function format(value: string, ...args: any[]): string {
	if (args.length === 0) {
		return value;
	}
	return value.replace(_formatRegexp, function (match, group) {
		const idx = parseInt(group, 10);
		return isNaN(idx) || idx < 0 || idx >= args.length ?
			match :
			args[idx];
	});
}

```

Output excerpt:

```text
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { LRUCachedFunction } from './cache.js';
import { CharCode } from './charCode.js';
import { Lazy } from './lazy.js';
import { Constants } from './uint.js';

export function isFalsyOrWhitespace(str: string | undefined): boolean {
	if (!str || typeof str !== 'string') {
		return true;
	}
	return str.trim().length === 0;
}

const _formatRegexp = /{(\d+)}/g;

/**
 * Helper to produce a string with a variable number of arguments. Insert variable segments
 * into the string using the {n} notation where N is the index of the argument following the string.
 * @param value string to which formatting is applied
 * @param args replacements for {n}-entries
 */
export function format(value: string, ...args: any[]): string { … 11 line(s) … ⟦tj:870557e346c33eff08954ba172e68750⟧ }

const _format2Regexp = /{([^}]+)}/g;

/**
 * Helper to create a string from a template and a string record.
 * Similar to `format` but with objects instead of positional arguments.
 */
export function format2(template: string, values: Record<string, unknown>): string {
	if (Object.keys(values).length === 0) {
		return template;

```

### `38-ts-heap`

- [Full input](cases/38-ts-heap/input.ts)
- [Full output](cases/38-ts-heap/output.ts)
- [Input vs output diff](cases/38-ts-heap/compression.diff)

Input excerpt:

```text
/**
 * A heap is a complete binary tree
 * In a complete binary tree each level is filled before lower levels are added
 * Each level is filled from left to right
 *
 * In a (min|max) heap the value of every node is (less|greater) than that of its children
 *
 * The heap is often implemented using an array structure.
 * In the array implementation, the relationship between a parent index and its two children
 * are ((parentindex * 2) + 1) and ((parentindex * 2) + 2)
 */
export abstract class Heap<T> {
  protected heap: T[]
  // A comparison function. Returns true if a should be the parent of b.
  protected compare: (a: T, b: T) => boolean

  constructor(compare: (a: T, b: T) => boolean) {
    this.heap = []
    this.compare = compare
  }

  /**
   * Compares the value at parentIndex with the value at childIndex
   * In a maxHeap, the value at parentIndex should be larger than the value at childIndex
   * In a minHeap, the value at parentIndex should be smaller than the value at childIndex
   */
  private isRightlyPlaced(childIndex: number, parentIndex: number): boolean {
    return this.compare(this.heap[parentIndex], this.heap[childIndex])
  }

  /**
   * In a maxHeap, the index with the larger value is returned
   * In a minHeap, the index with the smaller value is returned
   */
  private getChildIndexToSwap(
    leftChildIndex: number,

```

Output excerpt:

```text
/**
 * A heap is a complete binary tree
 * In a complete binary tree each level is filled before lower levels are added
 * Each level is filled from left to right
 *
 * In a (min|max) heap the value of every node is (less|greater) than that of its children
 *
 * The heap is often implemented using an array structure.
 * In the array implementation, the relationship between a parent index and its two children
 * are ((parentindex * 2) + 1) and ((parentindex * 2) + 2)
 */
export abstract class Heap<T> {
  protected heap: T[]
  // A comparison function. Returns true if a should be the parent of b.
  protected compare: (a: T, b: T) => boolean

  constructor(compare: (a: T, b: T) => boolean) {
    this.heap = []
    this.compare = compare
  }

  /**
   * Compares the value at parentIndex with the value at childIndex
   * In a maxHeap, the value at parentIndex should be larger than the value at childIndex
   * In a minHeap, the value at parentIndex should be smaller than the value at childIndex
   */
  private isRightlyPlaced(childIndex: number, parentIndex: number): boolean {
    return this.compare(this.heap[parentIndex], this.heap[childIndex])
  }

  /**
   * In a maxHeap, the index with the larger value is returned
   * In a minHeap, the index with the smaller value is returned
   */
  private getChildIndexToSwap(
    leftChildIndex: number,

```

### `20-java-gson-gson`

- [Full input](cases/20-java-gson-gson/input.java)
- [Full output](cases/20-java-gson-gson/output.java)
- [Input vs output diff](cases/20-java-gson-gson/compression.diff)

Input excerpt:

```text
/*
 * Copyright (C) 2008 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.gson;

import com.google.gson.annotations.JsonAdapter;
import com.google.gson.internal.ConstructorConstructor;
import com.google.gson.internal.Excluder;
import com.google.gson.internal.GsonBuildConfig;
import com.google.gson.internal.LazilyParsedNumber;
import com.google.gson.internal.Primitives;
import com.google.gson.internal.Streams;
import com.google.gson.internal.bind.ArrayTypeAdapter;
import com.google.gson.internal.bind.CollectionTypeAdapterFactory;
import com.google.gson.internal.bind.DefaultDateTypeAdapter;
import com.google.gson.internal.bind.JsonAdapterAnnotationTypeAdapterFactory;
import com.google.gson.internal.bind.JsonTreeReader;
import com.google.gson.internal.bind.JsonTreeWriter;
import com.google.gson.internal.bind.MapTypeAdapterFactory;
import com.google.gson.internal.bind.NumberTypeAdapter;
import com.google.gson.internal.bind.ObjectTypeAdapter;
import com.google.gson.internal.bind.ReflectiveTypeAdapterFactory;
import com.google.gson.internal.bind.SerializationDelegatingTypeAdapter;

```

Output excerpt:

```text
/*
 * Copyright (C) 2008 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.gson;

import com.google.gson.annotations.JsonAdapter;
import com.google.gson.internal.ConstructorConstructor;
import com.google.gson.internal.Excluder;
import com.google.gson.internal.GsonBuildConfig;
import com.google.gson.internal.LazilyParsedNumber;
import com.google.gson.internal.Primitives;
import com.google.gson.internal.Streams;
import com.google.gson.internal.bind.ArrayTypeAdapter;
import com.google.gson.internal.bind.CollectionTypeAdapterFactory;
import com.google.gson.internal.bind.DefaultDateTypeAdapter;
import com.google.gson.internal.bind.JsonAdapterAnnotationTypeAdapterFactory;
import com.google.gson.internal.bind.JsonTreeReader;
import com.google.gson.internal.bind.JsonTreeWriter;
import com.google.gson.internal.bind.MapTypeAdapterFactory;
import com.google.gson.internal.bind.NumberTypeAdapter;
import com.google.gson.internal.bind.ObjectTypeAdapter;
import com.google.gson.internal.bind.ReflectiveTypeAdapterFactory;
import com.google.gson.internal.bind.SerializationDelegatingTypeAdapter;

```

### `11-go-gin-context`

- [Full input](cases/11-go-gin-context/input.go)
- [Full output](cases/11-go-gin-context/output.go)
- [Input vs output diff](cases/11-go-gin-context/compression.diff)

Input excerpt:

```text
// Copyright 2014 Manu Martinez-Almeida. All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

package gin

import (
	"errors"
	"io"
	"log"
	"math"
	"mime/multipart"
	"net"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/gin-contrib/sse"
	"github.com/gin-gonic/gin/binding"
	"github.com/gin-gonic/gin/render"
)

// Content-Type MIME of the most common data formats.
const (
	MIMEJSON              = binding.MIMEJSON
	MIMEHTML              = binding.MIMEHTML
	MIMEXML               = binding.MIMEXML
	MIMEXML2              = binding.MIMEXML2
	MIMEPlain             = binding.MIMEPlain
	MIMEPOSTForm          = binding.MIMEPOSTForm
	MIMEMultipartPOSTForm = binding.MIMEMultipartPOSTForm
	MIMEYAML              = binding.MIMEYAML

```

Output excerpt:

```text
// Copyright 2014 Manu Martinez-Almeida. All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

package gin

import (
	"errors"
	"io"
	"log"
	"math"
	"mime/multipart"
	"net"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/gin-contrib/sse"
	"github.com/gin-gonic/gin/binding"
	"github.com/gin-gonic/gin/render"
)

// Content-Type MIME of the most common data formats.
const (
	MIMEJSON              = binding.MIMEJSON
	MIMEHTML              = binding.MIMEHTML
	MIMEXML               = binding.MIMEXML
	MIMEXML2              = binding.MIMEXML2
	MIMEPlain             = binding.MIMEPlain
	MIMEPOSTForm          = binding.MIMEPOSTForm
	MIMEMultipartPOSTForm = binding.MIMEMultipartPOSTForm
	MIMEYAML              = binding.MIMEYAML

```

### `47-kt-indexed-priority-queue`

- [Full input](cases/47-kt-indexed-priority-queue/input.kt)
- [Full output](cases/47-kt-indexed-priority-queue/output.kt)
- [Input vs output diff](cases/47-kt-indexed-priority-queue/compression.diff)

Input excerpt:

```text
/*
 * Copyright (c) 2017 Kotlin Algorithm Club
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

package io.uuddlrlrba.ktalgs.datastructures

import java.util.NoSuchElementException

// TODO: resize
class IndexedPriorityQueue<T>(size: Int, val comparator: Comparator<T>? = null) : Collection<T> {
    /**
     * maximum number of elements on PQ
     */
    private val maxN: Int = size

    /**
     * number of elements on PQ
     */

```

Output excerpt:

```text
/*
 * Copyright (c) 2017 Kotlin Algorithm Club
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

package io.uuddlrlrba.ktalgs.datastructures

import java.util.NoSuchElementException

// TODO: resize
class IndexedPriorityQueue<T>(size: Int, val comparator: Comparator<T>? = null) : Collection<T> {
    /**
     * maximum number of elements on PQ
     */
    private val maxN: Int = size

    /**
     * number of elements on PQ
     */

```

### `43-c-trie`

- [Full input](cases/43-c-trie/input.c)
- [Full output](cases/43-c-trie/output.c)
- [Input vs output diff](cases/43-c-trie/compression.diff)

Input excerpt:

```text
/*------------------Trie Data Structure----------------------------------*/
/*-------------Implimented for search a word in dictionary---------------*/

/*-----character - 97 used for get the character from the ASCII value-----*/

// needed for strnlen
#define _POSIX_C_SOURCE 200809L

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ALPHABET_SIZE 26

/*--Node in the Trie--*/
struct trie {
    struct trie *children[ALPHABET_SIZE];
    bool end_of_word;
};


/*--Create new trie node--*/
int trie_new (
    struct trie ** trie
)
{
    *trie = calloc(1, sizeof(struct trie));
    if (NULL == *trie) {
        // memory allocation failed
        return -1;
    }
    return 0;
}



```

Output excerpt:

```text
/*------------------Trie Data Structure----------------------------------*/
/*-------------Implimented for search a word in dictionary---------------*/

/*-----character - 97 used for get the character from the ASCII value-----*/

// needed for strnlen
#define _POSIX_C_SOURCE 200809L

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ALPHABET_SIZE 26

/*--Node in the Trie--*/
struct trie {
    struct trie *children[ALPHABET_SIZE];
    bool end_of_word;
};


/*--Create new trie node--*/
int trie_new (
    struct trie ** trie
)
{
    *trie = calloc(1, sizeof(struct trie));
    if (NULL == *trie) {
        // memory allocation failed
        return -1;
    }
    return 0;
}



```

### `21-java-guava-ordering`

- [Full input](cases/21-java-guava-ordering/input.java)
- [Full output](cases/21-java-guava-ordering/output.java)
- [Input vs output diff](cases/21-java-guava-ordering/compression.diff)

Input excerpt:

```text
/*
 * Copyright (C) 2007 The Guava Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.common.collect;

import static com.google.common.base.Preconditions.checkNotNull;
import static com.google.common.collect.CollectPreconditions.checkNonnegative;

import com.google.common.annotations.GwtCompatible;
import com.google.common.annotations.J2ktIncompatible;
import com.google.common.annotations.VisibleForTesting;
import com.google.common.base.Function;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map.Entry;
import java.util.NoSuchElementException;
import java.util.SortedMap;

```

Output excerpt:

```text
/*
 * Copyright (C) 2007 The Guava Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.common.collect;

import static com.google.common.base.Preconditions.checkNotNull;
import static com.google.common.collect.CollectPreconditions.checkNonnegative;

import com.google.common.annotations.GwtCompatible;
import com.google.common.annotations.J2ktIncompatible;
import com.google.common.annotations.VisibleForTesting;
import com.google.common.base.Function;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map.Entry;
import java.util.NoSuchElementException;
import java.util.SortedMap;

```

### `18-rs-tokio-builder`

- [Full input](cases/18-rs-tokio-builder/input.rs)
- [Full output](cases/18-rs-tokio-builder/output.rs)
- [Input vs output diff](cases/18-rs-tokio-builder/compression.diff)

Input excerpt:

```rust
use crate::runtime::handle::Handle;
use crate::runtime::{blocking, driver, Callback, HistogramBuilder, Runtime};
use crate::util::rand::{RngSeed, RngSeedGenerator};

use std::fmt;
use std::io;
use std::time::Duration;

/// Builds Tokio Runtime with custom configuration values.
///
/// Methods can be chained in order to set the configuration values. The
/// Runtime is constructed by calling [`build`].
///
/// New instances of `Builder` are obtained via [`Builder::new_multi_thread`]
/// or [`Builder::new_current_thread`].
///
/// See function level documentation for details on the various configuration
/// settings.
///
/// [`build`]: method@Self::build
/// [`Builder::new_multi_thread`]: method@Self::new_multi_thread
/// [`Builder::new_current_thread`]: method@Self::new_current_thread
///
/// # Examples
///
/// ` ` `
/// use tokio::runtime::Builder;
///
/// fn main() {
///     // build runtime
///     let runtime = Builder::new_multi_thread()
///         .worker_threads(4)
///         .thread_name("my-custom-name")
///         .thread_stack_size(3 * 1024 * 1024)
///         .build()
///         .unwrap();

```

Output excerpt:

```rust
use crate::runtime::handle::Handle;
use crate::runtime::{blocking, driver, Callback, HistogramBuilder, Runtime};
use crate::util::rand::{RngSeed, RngSeedGenerator};

use std::fmt;
use std::io;
use std::time::Duration;

/// Builds Tokio Runtime with custom configuration values.
///
/// Methods can be chained in order to set the configuration values. The
/// Runtime is constructed by calling [`build`].
///
/// New instances of `Builder` are obtained via [`Builder::new_multi_thread`]
/// or [`Builder::new_current_thread`].
///
/// See function level documentation for details on the various configuration
/// settings.
///
/// [`build`]: method@Self::build
/// [`Builder::new_multi_thread`]: method@Self::new_multi_thread
/// [`Builder::new_current_thread`]: method@Self::new_current_thread
///
/// # Examples
///
/// ` ` `
/// use tokio::runtime::Builder;
///
/// fn main() {
///     // build runtime
///     let runtime = Builder::new_multi_thread()
///         .worker_threads(4)
///         .thread_name("my-custom-name")
///         .thread_stack_size(3 * 1024 * 1024)
///         .build()
///         .unwrap();

```

### `22-kt-okhttp-client`

- [Full input](cases/22-kt-okhttp-client/input.kt)
- [Full output](cases/22-kt-okhttp-client/output.kt)
- [Input vs output diff](cases/22-kt-okhttp-client/compression.diff)

Input excerpt:

```text
/*
 * Copyright (C) 2012 Square, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package okhttp3

import java.net.Proxy
import java.net.ProxySelector
import java.net.Socket
import java.time.Duration
import java.util.Collections
import java.util.Random
import java.util.concurrent.ExecutorService
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeUnit.MILLISECONDS
import javax.net.SocketFactory
import javax.net.ssl.HostnameVerifier
import javax.net.ssl.SSLSocketFactory
import javax.net.ssl.X509TrustManager
import okhttp3.Protocol.HTTP_1_1
import okhttp3.Protocol.HTTP_2
import okhttp3.internal.asFactory
import okhttp3.internal.checkDuration
import okhttp3.internal.concurrent.TaskRunner
import okhttp3.internal.connection.RealCall

```

Output excerpt:

```text
/*
 * Copyright (C) 2012 Square, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package okhttp3

import java.net.Proxy
import java.net.ProxySelector
import java.net.Socket
import java.time.Duration
import java.util.Collections
import java.util.Random
import java.util.concurrent.ExecutorService
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeUnit.MILLISECONDS
import javax.net.SocketFactory
import javax.net.ssl.HostnameVerifier
import javax.net.ssl.SSLSocketFactory
import javax.net.ssl.X509TrustManager
import okhttp3.Protocol.HTTP_1_1
import okhttp3.Protocol.HTTP_2
import okhttp3.internal.asFactory
import okhttp3.internal.checkDuration
import okhttp3.internal.concurrent.TaskRunner
import okhttp3.internal.connection.RealCall

```

### `05-js-axios-core`

- [Full input](cases/05-js-axios-core/input.js)
- [Full output](cases/05-js-axios-core/output.js)
- [Input vs output diff](cases/05-js-axios-core/compression.diff)

Input excerpt:

```text
'use strict';

import utils from './../utils.js';
import buildURL from '../helpers/buildURL.js';
import InterceptorManager from './InterceptorManager.js';
import dispatchRequest from './dispatchRequest.js';
import mergeConfig from './mergeConfig.js';
import buildFullPath from './buildFullPath.js';
import validator from '../helpers/validator.js';
import AxiosHeaders from './AxiosHeaders.js';

const validators = validator.validators;

/**
 * Create a new instance of Axios
 *
 * @param {Object} instanceConfig The default config for the instance
 *
 * @return {Axios} A new instance of Axios
 */
class Axios {
  constructor(instanceConfig) {
    this.defaults = instanceConfig;
    this.interceptors = {
      request: new InterceptorManager(),
      response: new InterceptorManager()
    };
  }

  /**
   * Dispatch a request
   *
   * @param {String|Object} configOrUrl The config specific for this request (merged with this.defaults)
   * @param {?Object} config
   *
   * @returns {Promise} The Promise to be fulfilled

```

Output excerpt:

```text
'use strict';

import utils from './../utils.js';
import buildURL from '../helpers/buildURL.js';
import InterceptorManager from './InterceptorManager.js';
import dispatchRequest from './dispatchRequest.js';
import mergeConfig from './mergeConfig.js';
import buildFullPath from './buildFullPath.js';
import validator from '../helpers/validator.js';
import AxiosHeaders from './AxiosHeaders.js';

const validators = validator.validators;

/**
 * Create a new instance of Axios
 *
 * @param {Object} instanceConfig The default config for the instance
 *
 * @return {Axios} A new instance of Axios
 */
class Axios {
  constructor(instanceConfig) {
    this.defaults = instanceConfig;
    this.interceptors = {
      request: new InterceptorManager(),
      response: new InterceptorManager()
    };
  }

  /**
   * Dispatch a request
   *
   * @param {String|Object} configOrUrl The config specific for this request (merged with this.defaults)
   * @param {?Object} config
   *
   * @returns {Promise} The Promise to be fulfilled

```

### `44-rb-avl-tree`

- [Full input](cases/44-rb-avl-tree/input.rb)
- [Full output](cases/44-rb-avl-tree/output.rb)
- [Input vs output diff](cases/44-rb-avl-tree/compression.diff)

Input excerpt:

```text
class AvlTreeNode

  attr_reader :key
  attr_accessor :parent
  attr_accessor :left
  attr_accessor :right
  attr_accessor :height

  def initialize(key, parent=nil)
    @key = key
    @parent = parent
    @height = 1
  end
end

##
# This class represents an AVL tree (a self-balancing binary search tree) with distinct node keys.
# Starting from the root, every node has up to two children (one left and one right child node).
#
# For the BST property:
# - the keys of nodes in the left subtree of a node are strictly less than the key of the node;
# - the keys of nodes in the right subtree of a node are strictly greater than the key of the node.
#
# Due to self-balancing upon key insertion and deletion, the main operations of this data structure
# (insertion, deletion, membership) run - in worst case - in O(log(n)), where n is the number of nodes in the tree.

class AvlTree

  attr_reader :size
  attr_accessor :root

  def initialize(keys=[])
    @size = 0
    keys.each {|key| insert_key(key) }
  end


```

Output excerpt:

```text
class AvlTreeNode

  attr_reader :key
  attr_accessor :parent
  attr_accessor :left
  attr_accessor :right
  attr_accessor :height

  def initialize(key, parent=nil)
    @key = key
    @parent = parent
    @height = 1
  end
end

##
# This class represents an AVL tree (a self-balancing binary search tree) with distinct node keys.
# Starting from the root, every node has up to two children (one left and one right child node).
#
# For the BST property:
# - the keys of nodes in the left subtree of a node are strictly less than the key of the node;
# - the keys of nodes in the right subtree of a node are strictly greater than the key of the node.
#
# Due to self-balancing upon key insertion and deletion, the main operations of this data structure
# (insertion, deletion, membership) run - in worst case - in O(log(n)), where n is the number of nodes in the tree.

class AvlTree

  attr_reader :size
  attr_accessor :root

  def initialize(keys=[])
    @size = 0
    keys.each {|key| insert_key(key) }
  end


```

### `04-js-express-application`

- [Full input](cases/04-js-express-application/input.js)
- [Full output](cases/04-js-express-application/output.js)
- [Input vs output diff](cases/04-js-express-application/compression.diff)

Input excerpt:

```text
/*!
 * express
 * Copyright(c) 2009-2013 TJ Holowaychuk
 * Copyright(c) 2013 Roman Shtylman
 * Copyright(c) 2014-2015 Douglas Christopher Wilson
 * MIT Licensed
 */

'use strict';

/**
 * Module dependencies.
 * @private
 */

var finalhandler = require('finalhandler');
var Router = require('./router');
var methods = require('methods');
var middleware = require('./middleware/init');
var query = require('./middleware/query');
var debug = require('debug')('express:application');
var View = require('./view');
var http = require('http');
var compileETag = require('./utils').compileETag;
var compileQueryParser = require('./utils').compileQueryParser;
var compileTrust = require('./utils').compileTrust;
var deprecate = require('depd')('express');
var flatten = require('array-flatten');
var merge = require('utils-merge');
var resolve = require('path').resolve;
var setPrototypeOf = require('setprototypeof')

/**
 * Module variables.
 * @private
 */

```

Output excerpt:

```text
/*!
 * express
 * Copyright(c) 2009-2013 TJ Holowaychuk
 * Copyright(c) 2013 Roman Shtylman
 * Copyright(c) 2014-2015 Douglas Christopher Wilson
 * MIT Licensed
 */

'use strict';

/**
 * Module dependencies.
 * @private
 */

var finalhandler = require('finalhandler');
var Router = require('./router');
var methods = require('methods');
var middleware = require('./middleware/init');
var query = require('./middleware/query');
var debug = require('debug')('express:application');
var View = require('./view');
var http = require('http');
var compileETag = require('./utils').compileETag;
var compileQueryParser = require('./utils').compileQueryParser;
var compileTrust = require('./utils').compileTrust;
var deprecate = require('depd')('express');
var flatten = require('array-flatten');
var merge = require('utils-merge');
var resolve = require('path').resolve;
var setPrototypeOf = require('setprototypeof')

/**
 * Module variables.
 * @private
 */

```

### `23-rb-rails-cache`

- [Full input](cases/23-rb-rails-cache/input.rb)
- [Full output](cases/23-rb-rails-cache/output.rb)
- [Input vs output diff](cases/23-rb-rails-cache/compression.diff)

Input excerpt:

```text
# frozen_string_literal: true

require "zlib"
require "active_support/core_ext/array/extract_options"
require "active_support/core_ext/enumerable"
require "active_support/core_ext/module/attribute_accessors"
require "active_support/core_ext/numeric/bytes"
require "active_support/core_ext/object/to_param"
require "active_support/core_ext/object/try"
require "active_support/core_ext/string/inflections"
require_relative "cache/coder"
require_relative "cache/entry"
require_relative "cache/serializer_with_fallback"

module ActiveSupport
  # See ActiveSupport::Cache::Store for documentation.
  module Cache
    autoload :FileStore,        "active_support/cache/file_store"
    autoload :MemoryStore,      "active_support/cache/memory_store"
    autoload :MemCacheStore,    "active_support/cache/mem_cache_store"
    autoload :NullStore,        "active_support/cache/null_store"
    autoload :RedisCacheStore,  "active_support/cache/redis_cache_store"

    # These options mean something to all cache implementations. Individual cache
    # implementations may support additional options.
    UNIVERSAL_OPTIONS = [
      :coder,
      :compress,
      :compress_threshold,
      :compressor,
      :expire_in,
      :expired_in,
      :expires_in,
      :namespace,
      :race_condition_ttl,
      :serializer,

```

Output excerpt:

```text
# frozen_string_literal: true

require "zlib"
require "active_support/core_ext/array/extract_options"
require "active_support/core_ext/enumerable"
require "active_support/core_ext/module/attribute_accessors"
require "active_support/core_ext/numeric/bytes"
require "active_support/core_ext/object/to_param"
require "active_support/core_ext/object/try"
require "active_support/core_ext/string/inflections"
require_relative "cache/coder"
require_relative "cache/entry"
require_relative "cache/serializer_with_fallback"

module ActiveSupport
  # See ActiveSupport::Cache::Store for documentation.
  module Cache
    autoload :FileStore,        "active_support/cache/file_store"
    autoload :MemoryStore,      "active_support/cache/memory_store"
    autoload :MemCacheStore,    "active_support/cache/mem_cache_store"
    autoload :NullStore,        "active_support/cache/null_store"
    autoload :RedisCacheStore,  "active_support/cache/redis_cache_store"

    # These options mean something to all cache implementations. Individual cache
    # implementations may support additional options.
    UNIVERSAL_OPTIONS = [
      :coder,
      :compress,
      :compress_threshold,
      :compressor,
      :expire_in,
      :expired_in,
      :expires_in,
      :namespace,
      :race_condition_ttl,
      :serializer,

```

