# Unified Diffs

Real TinyJuice and OpenHuman patches. The diff compressor keeps file headers, hunk headers, and changed lines while collapsing long unchanged context.

Each row links to the full raw input and the exact compacted output used by the benchmark.

## Cases

| Case | Input | Output | Original | Compacted | Est. token reduction | Avg latency | CCR |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `01-tinyjuice-worktree` | [input](cases/01-tinyjuice-worktree/input.diff) | [output](cases/01-tinyjuice-worktree/output.diff) | 24.8 KB | 8.3 KB | 66.5% | 0.040 ms | true |
| `02-openhuman-commit-2` | [input](cases/02-openhuman-commit-2/input.diff) | [output](cases/02-openhuman-commit-2/output.diff) | 63.9 KB | 4.2 KB | 93.4% | 0.127 ms | true |
| `03-openhuman-commit-3` | [input](cases/03-openhuman-commit-3/input.diff) | [output](cases/03-openhuman-commit-3/output.diff) | 192.1 KB | 90.0 KB | 53.2% | 0.367 ms | true |
| `04-openhuman-commit-4` | [input](cases/04-openhuman-commit-4/input.diff) | [output](cases/04-openhuman-commit-4/output.diff) | 186.4 KB | 106.7 KB | 42.7% | 0.351 ms | true |
| `05-openhuman-commit-5` | [input](cases/05-openhuman-commit-5/input.diff) | [output](cases/05-openhuman-commit-5/output.diff) | 177.3 KB | 61.3 KB | 65.4% | 0.264 ms | true |
| `06-openhuman-commit-6` | [input](cases/06-openhuman-commit-6/input.diff) | [output](cases/06-openhuman-commit-6/output.diff) | 87.2 KB | 7.1 KB | 91.8% | 0.173 ms | true |
| `07-openhuman-commit-7` | [input](cases/07-openhuman-commit-7/input.diff) | [output](cases/07-openhuman-commit-7/output.diff) | 25.1 KB | 6.1 KB | 76.6% | 0.042 ms | true |
| `08-openhuman-commit-8` | [input](cases/08-openhuman-commit-8/input.diff) | [output](cases/08-openhuman-commit-8/output.diff) | 39.4 KB | 7.6 KB | 80.7% | 0.052 ms | true |
| `09-openhuman-commit-9` | [input](cases/09-openhuman-commit-9/input.diff) | [output](cases/09-openhuman-commit-9/output.diff) | 29.3 KB | 14.6 KB | 50.2% | 0.045 ms | true |
| `10-openhuman-commit-10` | [input](cases/10-openhuman-commit-10/input.diff) | [output](cases/10-openhuman-commit-10/output.diff) | 65.9 KB | 13.5 KB | 79.5% | 0.102 ms | true |

## What TinyJuice Is Doing

Diff compression preserves review-critical structure: file headers, hunk headers, additions, and removals. Long unchanged context is collapsed because it is recoverable and usually lower value.

## Syntax-Aware Samples

### `01-tinyjuice-worktree`

- [Full input](cases/01-tinyjuice-worktree/input.diff)
- [Full output](cases/01-tinyjuice-worktree/output.diff)

Input excerpt:

```diff
diff --git a/examples/compression_benchmark.rs b/examples/compression_benchmark.rs
index cd47c7c..52199c7 100644
--- a/examples/compression_benchmark.rs
+++ b/examples/compression_benchmark.rs
@@ -1,148 +1,188 @@
 //! Fixture-driven TinyJuice compression benchmark.
 //!
 //! Run with:
 //!
 //! ` ` `sh
 //! cargo run --release --example compression_benchmark -- --iterations 20
 //! cargo run --release --example compression_benchmark -- --format json
 //! ` ` `
 //!
 //! The report intentionally emits metadata only: sizes, token estimates,
 //! latency, compressor labels, CCR recovery, and named accuracy checks.
 
 use serde::Serialize;
 use serde_json::json;
 use std::fmt::Write as _;
 use std::path::{Path, PathBuf};
 use std::time::Instant;
 use tinyjuice::cache;
 use tinyjuice::tokens::estimate_tokens;
 use tinyjuice::types::{CompressInput, CompressOptions, ContentHint, ContentKind};
 
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 enum ReportFormat {
     Markdown,
     Json,
 }
 
 #[derive(Debug, Clone)]
 struct Args {
     iterations: usize,
     format: ReportFormat,

```

Output excerpt:

```diff
diff --git a/examples/compression_benchmark.rs b/examples/compression_benchmark.rs
index cd47c7c..52199c7 100644
--- a/examples/compression_benchmark.rs
+++ b/examples/compression_benchmark.rs
@@ -1,148 +1,188 @@
 //! Fixture-driven TinyJuice compression benchmark.
 //!
 //! Run with:
[... 60 context line(s) omitted ...]
     let path = Path::new(env!("CARGO_MANIFEST_DIR"))
         .join("docs/benchmark")
         .join(doc_dir)
-        .join("full-input.txt");
-    std::fs::read_to_string(&path).unwrap_or(fallback)
+        .join(input_artifact_name(doc_dir));
+    std::fs::read_to_string(&path)
+        .or_else(|_| {
+            std::fs::read_to_string(
+                Path::new(env!("CARGO_MANIFEST_DIR"))
+                    .join("docs/benchmark")
+                    .join(doc_dir)
+                    .join("full-input.txt"),
+            )
+        })
+        .unwrap_or(fallback)
+}
+
+fn category_name(doc_dir: &str) -> &str {
+    doc_dir.split('/').next().unwrap_or(doc_dir)
+}
+
+fn input_artifact_name(doc_dir: &str) -> &'static str {
+    match category_name(doc_dir) {
+        "json-smartcrusher" => "input.json",
+        "test-failure-log" => "input.log",
+        "service-log" => "input.log",

```

### `02-openhuman-commit-2`

- [Full input](cases/02-openhuman-commit-2/input.diff)
- [Full output](cases/02-openhuman-commit-2/output.diff)

Input excerpt:

```diff
diff --git a/Cargo.lock b/Cargo.lock
index 1259721a2..b44a50c40 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -74,172 +74,172 @@ dependencies = [
  "version_check",
  "zerocopy",
 ]
 
 [[package]]
 name = "aho-corasick"
 version = "1.1.4"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "ddd31a130427c27518df266943a5308ed92d4b226cc639f5a8f1002816174301"
 dependencies = [
  "memchr",
 ]
 
 [[package]]
 name = "alsa"
 version = "0.9.1"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "ed7572b7ba83a31e20d1b48970ee402d2e3e0537dcfe0a3ff4d6eb7508617d43"
 dependencies = [
  "alsa-sys",
  "bitflags 2.11.1",
  "cfg-if",
  "libc",
 ]
 
 [[package]]
 name = "alsa-sys"
 version = "0.3.1"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "db8fee663d06c4e303404ef5f40488a53e062f89ba8bfed81f42325aafad1527"
 dependencies = [

```

Output excerpt:

```diff
diff --git a/Cargo.lock b/Cargo.lock
index 1259721a2..b44a50c40 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -74,172 +74,172 @@ dependencies = [
[... lockfile/bundle hunk: +2/-2 line(s) omitted ...]
@@ -1588,161 +1588,161 @@ checksum = "25f104b501bf2364e78d0d3974cbc774f738f5865306ed128e1e0d7499c0ad96"
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -1845,161 +1845,161 @@ dependencies = [
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -3162,161 +3162,161 @@ source = "registry+https://github.com/rust-lang/crates.io-index"
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -3811,161 +3811,161 @@ checksum = "8c196769dd60fd4f363e11d948139556a344e79d451aeb2fa2fd040738ef7691"
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -5654,220 +5654,220 @@ name = "rlp-derive"
[... lockfile/bundle hunk: +2/-2 line(s) omitted ...]
@@ -6098,160 +6098,161 @@ dependencies = [
[... lockfile/bundle hunk: +1/-0 line(s) omitted ...]
@@ -6317,161 +6318,161 @@ dependencies = [
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -6618,161 +6619,161 @@ dependencies = [
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
@@ -6810,168 +6811,168 @@ dependencies = [
[... lockfile/bundle hunk: +2/-2 line(s) omitted ...]
@@ -7255,192 +7256,193 @@ checksum = "121c2a6cda46980bb0fcd1647ffaf6cd3fc79a013de288782836f6df9c48780e"
[... lockfile/bundle hunk: +7/-6 line(s) omitted ...]
@@ -8169,161 +8171,161 @@ dependencies = [
[... lockfile/bundle hunk: +1/-1 line(s) omitted ...]
diff --git a/Cargo.toml b/Cargo.toml
index 241e0d09f..70240e869 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -2,164 +2,164 @@
 name = "openhuman"
 version = "0.58.11"
 edition = "2021"

```

### `03-openhuman-commit-3`

- [Full input](cases/03-openhuman-commit-3/input.diff)
- [Full output](cases/03-openhuman-commit-3/output.diff)

Input excerpt:

```diff
diff --git a/.github/workflows/release-production.yml b/.github/workflows/release-production.yml
index e66dba04d..836b49c6d 100644
--- a/.github/workflows/release-production.yml
+++ b/.github/workflows/release-production.yml
@@ -345,161 +345,161 @@ jobs:
               owner,
               repo,
               tag_name: tag,
               target_commitish: target,
               name: `OpenHuman v${version}`,
               body,
               draft: true,
               prerelease: false,
             });
             core.setOutput('release_id', String(release.data.id));
             core.setOutput('upload_url', release.data.upload_url);
 
   # =========================================================================
   # Phase 3a: Build desktop artifacts (delegated to reusable workflow)
   # =========================================================================
   build-desktop:
     name: Build desktop matrix
     needs: [prepare-build, create-release]
     if: always() && needs.create-release.result == 'success'
     uses: ./.github/workflows/build-desktop.yml
     secrets: inherit
     with:
       build_ref: ${{ needs.prepare-build.outputs.build_ref }}
       tag: ${{ needs.prepare-build.outputs.tag }}
       version: ${{ needs.prepare-build.outputs.version }}
       sha: ${{ needs.prepare-build.outputs.sha }}
       short_sha: ${{ needs.prepare-build.outputs.short_sha }}
       base_url: ${{ needs.prepare-build.outputs.base_url }}
       app_env: production
       build_profile: release
       telegram_bot_username: openhumanaibot

```

Output excerpt:

```diff
diff --git a/.github/workflows/release-production.yml b/.github/workflows/release-production.yml
index e66dba04d..836b49c6d 100644
--- a/.github/workflows/release-production.yml
+++ b/.github/workflows/release-production.yml
@@ -345,161 +345,161 @@ jobs:
               owner,
               repo,
               tag_name: tag,
[... 74 context line(s) omitted ...]
       # fork the core image doesn't need. The Dockerfile COPYs vendor/ because
       # [patch.crates-io] resolves Rust SDK crates from vendor/.
       - name: Init vendored Rust submodules
-        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinyplace
+        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinychannels vendor/tinyplace
       - name: Set up Docker Buildx
         uses: docker/setup-buildx-action@v3
       - name: Log in to GHCR
[... 74 context line(s) omitted ...]
         with:
           ref: ${{ needs.prepare-build.outputs.build_ref }}
           fetch-depth: 1
diff --git a/.github/workflows/release-staging.yml b/.github/workflows/release-staging.yml
index 3b7a830c2..79a796ff6 100644
--- a/.github/workflows/release-staging.yml
+++ b/.github/workflows/release-staging.yml
@@ -240,161 +240,161 @@ jobs:
           else
             echo "build_ref=$SHA" >> "$GITHUB_OUTPUT"
           fi
[... 74 context line(s) omitted ...]
       # fork the core image doesn't need. The Dockerfile COPYs vendor/ because
       # [patch.crates-io] resolves Rust SDK crates from vendor/.
       - name: Init vendored Rust submodules
-        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinyplace
+        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinychannels vendor/tinyplace
       - name: Set up Docker Buildx

```

### `04-openhuman-commit-4`

- [Full input](cases/04-openhuman-commit-4/input.diff)
- [Full output](cases/04-openhuman-commit-4/output.diff)

Input excerpt:

```diff
diff --git a/src/openhuman/credentials/http_creds.rs b/src/openhuman/credentials/http_creds.rs
new file mode 100644
index 000000000..96f0a088b
--- /dev/null
+++ b/src/openhuman/credentials/http_creds.rs
@@ -0,0 +1,519 @@
+//! Named HTTP credentials for `http_request` flow nodes.
+//!
+//! A flow's `http_request` node can carry a `connection_ref` of the shape
+//! `"http_cred:<name>"`. This module is the host-side store those names resolve
+//! against: each record is an **injection template** (bearer token, HTTP basic
+//! user:pass, or a raw custom header) whose secret material is encrypted at
+//! rest with the same [`SecretStore`](crate::openhuman::keyring::SecretStore)
+//! (ChaCha20-Poly1305) the auth-profile store uses.
+//!
+//! **Security contract:** the secret value NEVER leaves this module except as
+//! the header it is injected into, server-side, inside
+//! `tinyflows::caps::OpenHumanHttp::request`. It is never returned to the UI,
+//! handed to the flow engine/graph, or logged. List/summary shapes carry only
+//! the name + scheme + non-secret template fields ([`HttpCredentialSummary`]).
+
+use std::collections::BTreeMap;
+use std::fs;
+use std::path::{Path, PathBuf};
+
+use anyhow::{Context, Result};
+use base64::engine::Engine as _;
+use chrono::{DateTime, Utc};
+use serde::{Deserialize, Serialize};
+
+use crate::openhuman::config::Config;
+use crate::openhuman::keyring::SecretStore;
+
+const STORE_FILENAME: &str = "http-credentials.json";
+const CURRENT_SCHEMA_VERSION: u32 = 1;
+

```

Output excerpt:

```diff
diff --git a/src/openhuman/credentials/http_creds.rs b/src/openhuman/credentials/http_creds.rs
new file mode 100644
index 000000000..96f0a088b
--- /dev/null
+++ b/src/openhuman/credentials/http_creds.rs
@@ -0,0 +1,519 @@
+//! Named HTTP credentials for `http_request` flow nodes.
+//!
+//! A flow's `http_request` node can carry a `connection_ref` of the shape
+//! `"http_cred:<name>"`. This module is the host-side store those names resolve
+//! against: each record is an **injection template** (bearer token, HTTP basic
+//! user:pass, or a raw custom header) whose secret material is encrypted at
+//! rest with the same [`SecretStore`](crate::openhuman::keyring::SecretStore)
+//! (ChaCha20-Poly1305) the auth-profile store uses.
+//!
+//! **Security contract:** the secret value NEVER leaves this module except as
+//! the header it is injected into, server-side, inside
+//! `tinyflows::caps::OpenHumanHttp::request`. It is never returned to the UI,
+//! handed to the flow engine/graph, or logged. List/summary shapes carry only
+//! the name + scheme + non-secret template fields ([`HttpCredentialSummary`]).
+
+use std::collections::BTreeMap;
+use std::fs;
+use std::path::{Path, PathBuf};
+
+use anyhow::{Context, Result};
+use base64::engine::Engine as _;
+use chrono::{DateTime, Utc};
+use serde::{Deserialize, Serialize};
+
+use crate::openhuman::config::Config;
+use crate::openhuman::keyring::SecretStore;
+
+const STORE_FILENAME: &str = "http-credentials.json";
+const CURRENT_SCHEMA_VERSION: u32 = 1;
+

```

### `05-openhuman-commit-5`

- [Full input](cases/05-openhuman-commit-5/input.diff)
- [Full output](cases/05-openhuman-commit-5/output.diff)

Input excerpt:

```diff
diff --git a/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md b/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md
new file mode 100644
index 000000000..73fd65eb3
--- /dev/null
+++ b/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md
@@ -0,0 +1,193 @@
+# C4 — Journal-backed progress projection & `progress_tracing` deletion
+
+Status: execution plan (2026-07-04), written after a ground-truth code map of
+both the OpenHuman progress surface and the vendored crate observability
+primitives. This is the actionable plan for the C4 workstream's July 2026
+continuation notes, and it is the gated prerequisite for **doc 03 (V3) Step
+5** — deleting the `ProviderDelta` bridge and `progress_tracing`.
+
+## 1. Corrected architecture (what the map found)
+
+- **There is no crate `SpanCollector`.** The only span state machine is
+  OpenHuman's `SpanCollector` in `src/openhuman/agent/progress_tracing.rs`
+  (1272 lines). The crate does **not** build spans — it journals raw
+  `AgentObservation`s and lets exporters project.
+- **The journal/status/persistence stack already exists and is attached to
+  every run.** `run_turn_via_tinyagents_shared`
+  (`src/openhuman/tinyagents/mod.rs:420`) mints a run id, seeds the `EventSink`
+  with it, and `attach_turn_journal` (`src/openhuman/tinyagents/journal.rs:304`)
+  installs `StoreEventJournal` (over `JsonlAppendStore`) via a
+  `JournalSink → RedactingSink → FanOutSink`, plus a durable `FileStatusStore`.
+  Every run already durably records the crate `AgentEvent` stream as
+  `AgentObservation`s.
+- **Two producers of `AgentProgress`:**
+  - Crate path: `OpenhumanEventBridge` (`src/openhuman/tinyagents/observability.rs:464`)
+    maps `AgentEvent` → `AgentProgress` live (stateful: iteration cursor,
+    subagent `scope`, `tool_names` recovery, display labels, failure class).
+  - Legacy path: `session/tool_progress.rs` `TurnProgress` +
+    `spawn_delta_forwarder` maps engine callbacks + `ProviderDelta` →
+    `AgentProgress` (the **Step-5 deletion target**, 253 lines).
+- **`SpanCollector` consumes `AgentProgress`** (the bridge *output*), while the

```

Output excerpt:

```diff
diff --git a/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md b/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md
new file mode 100644
index 000000000..73fd65eb3
--- /dev/null
+++ b/docs/tinyagents-full-migration-plan/C4-journal-progress-parity-plan.md
@@ -0,0 +1,193 @@
+# C4 — Journal-backed progress projection & `progress_tracing` deletion
+
+Status: execution plan (2026-07-04), written after a ground-truth code map of
+both the OpenHuman progress surface and the vendored crate observability
+primitives. This is the actionable plan for the C4 workstream's July 2026
+continuation notes, and it is the gated prerequisite for **doc 03 (V3) Step
+5** — deleting the `ProviderDelta` bridge and `progress_tracing`.
+
+## 1. Corrected architecture (what the map found)
+
+- **There is no crate `SpanCollector`.** The only span state machine is
+  OpenHuman's `SpanCollector` in `src/openhuman/agent/progress_tracing.rs`
+  (1272 lines). The crate does **not** build spans — it journals raw
+  `AgentObservation`s and lets exporters project.
+- **The journal/status/persistence stack already exists and is attached to
+  every run.** `run_turn_via_tinyagents_shared`
+  (`src/openhuman/tinyagents/mod.rs:420`) mints a run id, seeds the `EventSink`
+  with it, and `attach_turn_journal` (`src/openhuman/tinyagents/journal.rs:304`)
+  installs `StoreEventJournal` (over `JsonlAppendStore`) via a
+  `JournalSink → RedactingSink → FanOutSink`, plus a durable `FileStatusStore`.
+  Every run already durably records the crate `AgentEvent` stream as
+  `AgentObservation`s.
+- **Two producers of `AgentProgress`:**
+  - Crate path: `OpenhumanEventBridge` (`src/openhuman/tinyagents/observability.rs:464`)
+    maps `AgentEvent` → `AgentProgress` live (stateful: iteration cursor,
+    subagent `scope`, `tool_names` recovery, display labels, failure class).
+  - Legacy path: `session/tool_progress.rs` `TurnProgress` +
+    `spawn_delta_forwarder` maps engine callbacks + `ProviderDelta` →
+    `AgentProgress` (the **Step-5 deletion target**, 253 lines).
+- **`SpanCollector` consumes `AgentProgress`** (the bridge *output*), while the

```

### `06-openhuman-commit-6`

- [Full input](cases/06-openhuman-commit-6/input.diff)
- [Full output](cases/06-openhuman-commit-6/output.diff)

Input excerpt:

```diff
diff --git a/.github/workflows/release-production.yml b/.github/workflows/release-production.yml
index 6f13136e7..e66dba04d 100644
--- a/.github/workflows/release-production.yml
+++ b/.github/workflows/release-production.yml
@@ -345,161 +345,161 @@ jobs:
               owner,
               repo,
               tag_name: tag,
               target_commitish: target,
               name: `OpenHuman v${version}`,
               body,
               draft: true,
               prerelease: false,
             });
             core.setOutput('release_id', String(release.data.id));
             core.setOutput('upload_url', release.data.upload_url);
 
   # =========================================================================
   # Phase 3a: Build desktop artifacts (delegated to reusable workflow)
   # =========================================================================
   build-desktop:
     name: Build desktop matrix
     needs: [prepare-build, create-release]
     if: always() && needs.create-release.result == 'success'
     uses: ./.github/workflows/build-desktop.yml
     secrets: inherit
     with:
       build_ref: ${{ needs.prepare-build.outputs.build_ref }}
       tag: ${{ needs.prepare-build.outputs.tag }}
       version: ${{ needs.prepare-build.outputs.version }}
       sha: ${{ needs.prepare-build.outputs.sha }}
       short_sha: ${{ needs.prepare-build.outputs.short_sha }}
       base_url: ${{ needs.prepare-build.outputs.base_url }}
       app_env: production
       build_profile: release
       telegram_bot_username: openhumanaibot

```

Output excerpt:

```diff
diff --git a/.github/workflows/release-production.yml b/.github/workflows/release-production.yml
index 6f13136e7..e66dba04d 100644
--- a/.github/workflows/release-production.yml
+++ b/.github/workflows/release-production.yml
@@ -345,161 +345,161 @@ jobs:
               owner,
               repo,
               tag_name: tag,
[... 74 context line(s) omitted ...]
       # fork the core image doesn't need. The Dockerfile COPYs vendor/ because
       # [patch.crates-io] resolves Rust SDK crates from vendor/.
       - name: Init vendored Rust submodules
-        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice
+        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinyplace
       - name: Set up Docker Buildx
         uses: docker/setup-buildx-action@v3
       - name: Log in to GHCR
[... 74 context line(s) omitted ...]
         with:
           ref: ${{ needs.prepare-build.outputs.build_ref }}
           fetch-depth: 1
diff --git a/.github/workflows/release-staging.yml b/.github/workflows/release-staging.yml
index 9913ff760..3b7a830c2 100644
--- a/.github/workflows/release-staging.yml
+++ b/.github/workflows/release-staging.yml
@@ -240,161 +240,161 @@ jobs:
           else
             echo "build_ref=$SHA" >> "$GITHUB_OUTPUT"
           fi
[... 74 context line(s) omitted ...]
       # fork the core image doesn't need. The Dockerfile COPYs vendor/ because
       # [patch.crates-io] resolves Rust SDK crates from vendor/.
       - name: Init vendored Rust submodules
-        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice
+        run: git submodule update --init vendor/tinyagents vendor/tinyflows vendor/tinycortex vendor/tinyjuice vendor/tinyplace
       - name: Set up Docker Buildx

```

### `07-openhuman-commit-7`

- [Full input](cases/07-openhuman-commit-7/input.diff)
- [Full output](cases/07-openhuman-commit-7/output.diff)

Input excerpt:

```diff
diff --git a/app/src/pages/__tests__/Conversations.attachments.test.tsx b/app/src/pages/__tests__/Conversations.attachments.test.tsx
index e24e569ce..8167f088c 100644
--- a/app/src/pages/__tests__/Conversations.attachments.test.tsx
+++ b/app/src/pages/__tests__/Conversations.attachments.test.tsx
@@ -1,100 +1,104 @@
 /**
  * Attachment feature tests for Conversations.tsx — covers the new lines added
  * for multimodal image attachments: handleAttachFiles, error display,
  * attachment-only sends, and user bubble image rendering.
  */
 import { combineReducers, configureStore } from '@reduxjs/toolkit';
-import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
+import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
 import { Provider } from 'react-redux';
 import { MemoryRouter } from 'react-router-dom';
-import { beforeEach, describe, expect, it, vi } from 'vitest';
+import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
 
 import { SidebarSlotOutlet, SidebarSlotProvider } from '../../components/layout/shell/SidebarSlot';
 import agentProfileReducer from '../../store/agentProfileSlice';
 import chatRuntimeReducer from '../../store/chatRuntimeSlice';
 import socketReducer from '../../store/socketSlice';
 import threadReducer from '../../store/threadSlice';
 import type { Thread } from '../../types/thread';
 
 // ── Hoisted mock state ──────────────────────────────────────────────────────
 
+const TINY_PNG_DATA_URI = 'data:image/png;base64,iVBORw0KGgo=';
+const originalCreateObjectURL = URL.createObjectURL;
+const originalRevokeObjectURL = URL.revokeObjectURL;
+
 const {
   mockGetThreads,
   mockGetThreadMessages,
   mockSelectAgentProfile,
   mockUseUsageState,

```

Output excerpt:

```diff
diff --git a/app/src/pages/__tests__/Conversations.attachments.test.tsx b/app/src/pages/__tests__/Conversations.attachments.test.tsx
index e24e569ce..8167f088c 100644
--- a/app/src/pages/__tests__/Conversations.attachments.test.tsx
+++ b/app/src/pages/__tests__/Conversations.attachments.test.tsx
@@ -1,100 +1,104 @@
 /**
  * Attachment feature tests for Conversations.tsx — covers the new lines added
  * for multimodal image attachments: handleAttachFiles, error display,
  * attachment-only sends, and user bubble image rendering.
  */
 import { combineReducers, configureStore } from '@reduxjs/toolkit';
-import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
+import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
 import { Provider } from 'react-redux';
 import { MemoryRouter } from 'react-router-dom';
-import { beforeEach, describe, expect, it, vi } from 'vitest';
+import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
 
 import { SidebarSlotOutlet, SidebarSlotProvider } from '../../components/layout/shell/SidebarSlot';
 import agentProfileReducer from '../../store/agentProfileSlice';
[... 4 context line(s) omitted ...]
 
 // ── Hoisted mock state ──────────────────────────────────────────────────────
 
+const TINY_PNG_DATA_URI = 'data:image/png;base64,iVBORw0KGgo=';
+const originalCreateObjectURL = URL.createObjectURL;
+const originalRevokeObjectURL = URL.revokeObjectURL;
+
 const {
   mockGetThreads,
   mockGetThreadMessages,
[... 74 context line(s) omitted ...]
     persistReaction: vi.fn().mockResolvedValue({}),
   },
 }));
@@ -190,180 +194,200 @@ vi.mock('../../lib/coreState/store', () => ({

```

### `08-openhuman-commit-8`

- [Full input](cases/08-openhuman-commit-8/input.diff)
- [Full output](cases/08-openhuman-commit-8/output.diff)

Input excerpt:

```diff
diff --git a/app/src/pages/Conversations.tsx b/app/src/pages/Conversations.tsx
index d649681fb..ffe82de44 100644
--- a/app/src/pages/Conversations.tsx
+++ b/app/src/pages/Conversations.tsx
@@ -70,164 +70,198 @@ import {
   type ToolTimelineEntry,
 } from '../store/chatRuntimeSlice';
 import { useAppDispatch, useAppSelector } from '../store/hooks';
 import { selectSocketStatus } from '../store/socketSelectors';
 import {
   addMessageLocal,
   clearThreadInferenceActive,
   createNewThread,
   deleteThread,
   loadThreadMessages,
   loadThreads,
   markThreadInferenceActive,
   persistReaction,
   setSelectedThread,
   THREAD_NOT_FOUND_MESSAGE,
   updateThreadTitle,
 } from '../store/threadSlice';
 import type { ConfirmationModal as ConfirmationModalType } from '../types/intelligence';
 import type { ThreadMessage } from '../types/thread';
 import { splitAgentMessageIntoBubbles } from '../utils/agentMessageBubbles';
 import { chatThreadPath } from '../utils/chatRoutes';
 import { CHAT_ATTACHMENTS_ENABLED } from '../utils/config';
 import { BILLING_DASHBOARD_URL } from '../utils/links';
 import { openUrl } from '../utils/openUrl';
 import {
   isTauri,
   notifyOverlaySttState,
   openhumanAutocompleteAccept,
   openhumanAutocompleteCurrent,
   openhumanVoiceStatus,
   openhumanVoiceTranscribeBytes,

```

Output excerpt:

```diff
diff --git a/app/src/pages/Conversations.tsx b/app/src/pages/Conversations.tsx
index d649681fb..ffe82de44 100644
--- a/app/src/pages/Conversations.tsx
+++ b/app/src/pages/Conversations.tsx
@@ -70,164 +70,198 @@ import {
   type ToolTimelineEntry,
 } from '../store/chatRuntimeSlice';
 import { useAppDispatch, useAppSelector } from '../store/hooks';
[... 74 context line(s) omitted ...]
 const AUTOCOMPLETE_POLL_DEBOUNCE_MS = 320;
 const AUTOCOMPLETE_MIN_CONTEXT_CHARS = 3;
 const debug = debugFactory('conversations');
-const SAFE_IMAGE_DATA_URI_RE = /^data:image\/(?:png|jpe?g|gif|webp|bmp);base64,[a-z0-9+/=\s]+$/i;
+const SAFE_IMAGE_DATA_URI_RE =
+  /^data:(image\/(?:png|jpe?g|gif|webp|bmp));base64,([a-z0-9+/=\s]+)$/i;
+const EMPTY_IMAGE_SRC = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw==';
+
+function imageDataUriToObjectUrl(src: string): string | null {
+  const match = SAFE_IMAGE_DATA_URI_RE.exec(src);
+  if (!match) return null;
+  try {
+    const mime = match[1];
+    const binary = atob(match[2].replace(/\s/g, ''));
+    const bytes = new Uint8Array(binary.length);
+    for (let i = 0; i < binary.length; i += 1) {
+      bytes[i] = binary.charCodeAt(i);
+    }
+    return URL.createObjectURL(new Blob([bytes], { type: mime }));
+  } catch {
+    return null;
+  }
+}
 
-function isSafeAttachmentImageSrc(src: string): boolean {
-  return SAFE_IMAGE_DATA_URI_RE.test(src);
+function AttachmentImage({ dataUri }: { dataUri: string }) {

```

### `09-openhuman-commit-9`

- [Full input](cases/09-openhuman-commit-9/input.diff)
- [Full output](cases/09-openhuman-commit-9/output.diff)

Input excerpt:

```diff
diff --git a/src/openhuman/memory_store/chunks/store.rs b/src/openhuman/memory_store/chunks/store.rs
index 61b4d12fb..7237cd6e9 100644
--- a/src/openhuman/memory_store/chunks/store.rs
+++ b/src/openhuman/memory_store/chunks/store.rs
@@ -1,122 +1,120 @@
 //! SQLite-backed persistence for ingested chunks (Phase 1 / issue #707).
 //!
 //! The store lives at `<workspace>/memory_tree/chunks.db`. Schema is applied
 //! lazily on first access via `with_connection`, so the DB is created on
 //! demand without an explicit migration step.
 //!
 //! Upsert semantics: writes are idempotent on `chunk.id` so re-ingesting the
 //! same raw source yields no duplicates.
 //!
 //! ## Connection cache (#2206)
 //!
 //! `with_connection()` previously opened a new SQLite connection and re-ran
 //! the full schema init (8 tables, 15+ indexes, 8+ migrations) on **every**
 //! call. With 4 workers polling every 5 s this amounted to ~69K connection
 //! opens/day, and a family of WAL/SHM cold-start I/O codes (1546
 //! IOERR_TRUNCATE, 4618 IOERR_SHMOPEN, 4874 IOERR_SHMSIZE, 14 CANTOPEN)
 //! flooded Sentry with ~19K events in 4 days.
 //!
 //! Fix: a process-level `ConnectionCache` keyed by DB path. Each entry holds
 //! one `parking_lot::Mutex<Connection>` that is initialised once (schema +
 //! migrations + legacy-embedding migration) and then reused for all subsequent
 //! calls. A per-entry `CircuitBreaker` stops retrying after 3 consecutive
 //! init failures for 30 s so a broken install does not busy-loop.
 
 use anyhow::{Context, Result};
-use chrono::{DateTime, TimeZone, Utc};
+use chrono::Utc;
 use rusqlite::{params, Connection, OptionalExtension, Transaction};
 use std::collections::{HashMap, HashSet};
 #[cfg(test)]
 use std::sync::Arc;

```

Output excerpt:

```diff
diff --git a/src/openhuman/memory_store/chunks/store.rs b/src/openhuman/memory_store/chunks/store.rs
index 61b4d12fb..7237cd6e9 100644
--- a/src/openhuman/memory_store/chunks/store.rs
+++ b/src/openhuman/memory_store/chunks/store.rs
@@ -1,122 +1,120 @@
 //! SQLite-backed persistence for ingested chunks (Phase 1 / issue #707).
 //!
 //! The store lives at `<workspace>/memory_tree/chunks.db`. Schema is applied
[... 19 context line(s) omitted ...]
 //! init failures for 30 s so a broken install does not busy-loop.
 
 use anyhow::{Context, Result};
-use chrono::{DateTime, TimeZone, Utc};
+use chrono::Utc;
 use rusqlite::{params, Connection, OptionalExtension, Transaction};
 use std::collections::{HashMap, HashSet};
 #[cfg(test)]
 use std::sync::Arc;
 use std::time::Duration;
 
 use crate::openhuman::config::Config;
 use crate::openhuman::memory::util::redact::{self, redact as redact_value};
-use crate::openhuman::memory_store::chunks::types::{Chunk, Metadata, SourceKind, SourceRef};
+use crate::openhuman::memory_store::chunks::types::{Chunk, SourceKind};
 use crate::openhuman::memory_store::content::StagedChunk;
 use crate::openhuman::tinycortex::memory_config_from;
 
 const DB_DIR: &str = "memory_tree";
 const DB_FILE: &str = "chunks.db";
-const DEFAULT_LIST_LIMIT: usize = 100;
-const MAX_LIST_LIMIT: usize = 10_000;
 // 15s gives the busy-handler enough headroom that transient write-lock
 // contention (4 job workers + scheduler + ingest producers all writing the
 // same `memory_tree/chunks.db`) is absorbed inside rusqlite instead of
[... 74 context line(s) omitted ...]
     PRIMARY KEY (chunk_id, model_signature)

```

### `10-openhuman-commit-10`

- [Full input](cases/10-openhuman-commit-10/input.diff)
- [Full output](cases/10-openhuman-commit-10/output.diff)

Input excerpt:

```diff
diff --git a/app/src/pages/Conversations.tsx b/app/src/pages/Conversations.tsx
index 928fc0968..d649681fb 100644
--- a/app/src/pages/Conversations.tsx
+++ b/app/src/pages/Conversations.tsx
@@ -70,160 +70,165 @@ import {
   type ToolTimelineEntry,
 } from '../store/chatRuntimeSlice';
 import { useAppDispatch, useAppSelector } from '../store/hooks';
 import { selectSocketStatus } from '../store/socketSelectors';
 import {
   addMessageLocal,
   clearThreadInferenceActive,
   createNewThread,
   deleteThread,
   loadThreadMessages,
   loadThreads,
   markThreadInferenceActive,
   persistReaction,
   setSelectedThread,
   THREAD_NOT_FOUND_MESSAGE,
   updateThreadTitle,
 } from '../store/threadSlice';
 import type { ConfirmationModal as ConfirmationModalType } from '../types/intelligence';
 import type { ThreadMessage } from '../types/thread';
 import { splitAgentMessageIntoBubbles } from '../utils/agentMessageBubbles';
 import { chatThreadPath } from '../utils/chatRoutes';
 import { CHAT_ATTACHMENTS_ENABLED } from '../utils/config';
 import { BILLING_DASHBOARD_URL } from '../utils/links';
 import { openUrl } from '../utils/openUrl';
 import {
   isTauri,
   notifyOverlaySttState,
   openhumanAutocompleteAccept,
   openhumanAutocompleteCurrent,
   openhumanVoiceStatus,
   openhumanVoiceTranscribeBytes,

```

Output excerpt:

```diff
diff --git a/app/src/pages/Conversations.tsx b/app/src/pages/Conversations.tsx
index 928fc0968..d649681fb 100644
--- a/app/src/pages/Conversations.tsx
+++ b/app/src/pages/Conversations.tsx
@@ -70,160 +70,165 @@ import {
   type ToolTimelineEntry,
 } from '../store/chatRuntimeSlice';
 import { useAppDispatch, useAppSelector } from '../store/hooks';
[... 74 context line(s) omitted ...]
 const AUTOCOMPLETE_POLL_DEBOUNCE_MS = 320;
 const AUTOCOMPLETE_MIN_CONTEXT_CHARS = 3;
 const debug = debugFactory('conversations');
+const SAFE_IMAGE_DATA_URI_RE = /^data:image\/(?:png|jpe?g|gif|webp|bmp);base64,[a-z0-9+/=\s]+$/i;
+
+function isSafeAttachmentImageSrc(src: string): boolean {
+  return SAFE_IMAGE_DATA_URI_RE.test(src);
+}
 
 interface ConversationsProps {
   /**
[... 74 context line(s) omitted ...]
   }
   return String(err);
 }
@@ -2244,163 +2249,165 @@ const Conversations = ({
                                           className="flex items-center rounded-full border border-primary-200 bg-primary-100 px-1.5 text-xs leading-[1.5] shadow-sm transition-colors hover:bg-primary-200 dark:border-prim...
                                           title={t('chat.removeReaction').replace(
                                             '{emoji}',
[... 74 context line(s) omitted ...]
                           <div className="flex flex-col items-end gap-1">
                             {(() => {
                               const displayText = parsedContent.text;
-                              const dataUris = Array.isArray(msg.extraMetadata?.attachmentDataUris)
-                                ? (msg.extraMetadata.attachmentDataUris as string[])
-                                : parsedContent.dataUris;
+                              const dataUris = (

```

