[search: 500 match(es) across 162 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 토큰으로 전달됩니다. 이것 없이는 이만큼 큰 두뇌를 감당할 수 없을 것입니다.
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。没有它，这么大的一颗大脑将贵得用不起。
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu 80% weniger Tokens. Ein so großes Gehirn wäre ohne es unbezahlbar.
<OPENHUMAN_ROOT>/src/core/all.rs:651:            Some("Hierarchical time-based summarization tree for background knowledge compression.")
<OPENHUMAN_ROOT>/src/openhuman/context/manager_tests.rs:4://! (`ContextCompressionMiddleware` + `tinyagents::summarize::ProviderModelSummarizer`),
<OPENHUMAN_ROOT>/src/openhuman/context/manager.rs:16://!    tinyagents graph (`ContextCompressionMiddleware` +
<OPENHUMAN_ROOT>/src/openhuman/context/manager.rs:61:    /// now runs in the tinyagents graph (`ContextCompressionMiddleware` +
<OPENHUMAN_ROOT>/src/openhuman/context/manager.rs:90:    /// (`ContextCompressionMiddleware`). Gated by both `[context].enabled` and
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: وہی معلومات، 80% تک کم ٹوکنز۔ اتنا بڑا دماغ اس کے بغیر ناقابلِ برداشت مہنگا ہوتا۔
<OPENHUMAN_ROOT>/src/openhuman/context/README.md:5:> **Status (#4249): live history reduction/summarization moved to the tinyagents graph.** The in-turn compaction that used to live here — `ContextManager::reduce_before_call`, the `Summarizer` trait, `ProviderSummarizer`, `SegmentRecapSummarizer`, `context/microcompact.rs`, `context/pipeline.rs`, and `context/guard.rs` — has been **removed**. Folding an over-budget transcript into a summary now runs as `ContextCompressionMiddleware` (+ `MessageTrimMiddleware` backstop) inside `run_turn_via_tinyagents_shared`, backed by `tinyagents::summarize::ProviderModelSummarizer`. Tool-result body clearing now runs in TinyAgents `MicrocompactMiddleware`; `context/stats.rs` keeps the data model behind `ContextManager::stats()` (the utilisation footer) and session-memory bookkeeping.
<OPENHUMAN_ROOT>/src/openhuman/context/README.md:24:| `src/openhuman/context/guard.rs` | **Removed (#4249).** The live 0.90 compression threshold is mirrored by `tinyagents::summarize::SUMMARIZE_THRESHOLD_FRACTION`. |
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少ないトークンで扱えます。これがなければ、これほど大きな脳は維持できません。
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:31:| 20:1 compression engine | `orchestration/graph/compress.rs` + `ProductionRuntime::compress` |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:57:- No change to the orchestration wake graph, compression ratio, context-guard
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/README.md:30:| `image_processing.rs` | PNG→resized JPEG compression for the vision LLM (`compress_screenshot`, defaults 1024px / quality 72). |
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:1://! Image compression and resizing for screenshot intelligence.
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:152:    // ── Basic compression ───────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:344:    // ── Multicolored image (more realistic compression ratio) ───────────
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/processing_worker.rs:232:    // ── Image compression (always runs — used by vision LLM and/or storage) ──
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/processing_worker.rs:234:        .map_err(|e| format!("image compression failed: {e}"))?;
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/dispatch/mod.rs:128:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so sweeping through your last six months of email costs single-digit dollars. [Automatic model routing](features/model-routing/) sends each task to the right model - `hint:reasoning` to a frontier model, `hint:fast` to a cheap one, vision to vision - all under one subscription. Optional [local AI via Ollama or LM Studio](features/model-routing/local-ai.md) keeps supported workloads on-device.
<OPENHUMAN_ROOT>/src/openhuman/memory/schemas/tool_memory.rs:68:                `priority='critical'` for safety-critical rules that must survive context compression.",
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:65:Compression and locality together become the privacy architecture.
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:429:    use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:792:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:793:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:804:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:805:            AgentTokenjuiceCompression::Light
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs]
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:40:| `hint:summarize` | A model good at compression | Memory tree summary builders |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:98:- [Smart Token Compression](../token-compression.md). what makes large reasoning calls affordable.
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:284:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:100:## Cost & token compression
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:102:Because cost tracks **real token counts**, anything that shrinks the prompt directly lowers spend. OpenHuman's [TokenJuice token compression](token-compression.md) reduces the tokens sent on each call, and [model routing](model-routing/README.md) sends work to the cheapest model that can handle it. Both show up as lower bars in the dashboard and slower budget burn.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:108:- [Token compression (TokenJuice)](token-compression.md)
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:3:  TokenJuice - a multi-stage compression router that compacts verbose tool
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:12:OpenHuman ships with **TokenJuice**, a compression router wired directly into the agent's tool-execution path. Before any tool result reaches a model, TokenJuice classifies it, routes it to a specialized compressor, optionally offloads the full original to a recoverable cache, and records how many tokens (and dollars) it saved.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:54:4. **Compression.** The compressor runs. If it declines or its output is no smaller than the input, TokenJuice falls back to the generic compressor or passes the original through. It never makes things bigger.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:84:* **Off by default.** Enable with `ml_compression_enabled = true` in `[tokenjuice]`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and leaves a breadcrumb (`vendor/tinyjuice/src/cache/`).
[+6 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/token-compression.md]
<OPENHUMAN_ROOT>/src/openhuman/about_app/catalog_data.rs:405:            compression. Critical-priority rules (e.g. 'never email Sarah') are pinned into the \
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/scoring.md:109:Embeddings run on the background workers, not the ingest hot path, so a burst of new sources never blocks the UI. Trees give compression and navigation; embeddings keep similarity search working underneath them.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/scoring.md:118:* [Token Compression](../token-compression.md) - why keeping the tree dense matters.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-diff.md:101:Checkpoints are cheap to prune: `cleanup` deletes tags older than N days, but **snapshot commits are never deleted** - git history _is_ the ledger, and git's delta compression keeps it compact.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/auto-fetch.md:60:* [Smart Token Compression](../token-compression.md). what keeps "fetch everything" cheap.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-tree.md:74:Trees give you compression _and_ navigation. Embeddings still live inside so semantic search keeps working, but the structure on top is what makes the memory feel like a brain instead of a bag of fragments.
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:28:Inbound sessions run through a **split-brain wake graph**: a fast reflex agent triages each message in seconds (reply immediately, or hand the deep reasoning core a concise brief), while the reasoning core does the real multi-step work and delegates to sub-agent workers. Long sessions stay bounded via 20:1 history compression and a rolling world-state diff, and your [subconscious loop](subconscious.md) periodically reviews the whole picture and injects a short steering directive to keep the layer aligned with *your* priorities.
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:196:* Long sessions stay bounded by **20:1 history compression** plus a rolling world-state diff with utilization-based eviction.
<OPENHUMAN_ROOT>/vendor/tinycortex/README.md:17:The human brain is a master at compression. It doesn't try to remember every passing detail; instead it aggressively prunes noise to keep a sharp, focused, easily accessible recall of what truly matters. Traditional AI memory systems do the opposite — they try to remember _everything_ and retrieve whatever is _similar_. But similar doesn't mean important. The result? Your AI drowns in stale, irrelevant context that degrades every response.
<OPENHUMAN_ROOT>/vendor/tinycortex/docs/openhuman-memory-engine-spec.md:487:sections so they survive compression. The current TinyCortex module provides
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/mod.rs:33://!   system prompt so they survive mid-session compression.
<OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs:469:    use flate2::{write::GzEncoder, Compression};
<OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs:472:    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
<OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs:496:    use flate2::{write::GzEncoder, Compression};
<OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs:500:    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
<OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs:522:async fn prepare_messages_bounds_gzipped_data_uri_decompression() {
[+5 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/multimodal_tests.rs]
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-scraper.md:31:* [Smart Token Compression](../token-compression.md) - what trims long pages before they hit the model.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/render.rs:5://! Mid-session compression rewrites the rolling chat buffer but never the
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/render.rs:8://! be **compression-resistant** therefore has to live in the system prompt.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/README.md:21:* All output passes through [Smart Token Compression](../token-compression.md) for free.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/README.md:41:* [Smart Token Compression](../token-compression.md) - what keeps tool output costs bounded.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:13://!   therefore not subject to mid-session context compression.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:37:    /// Safety-critical rule — pinned into the (compression-resistant)
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:51:    /// prefetched at session start, so they survive context compression).
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:93:    /// Criticality level for retrieval and compression behaviour.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-search.md:51:* [Smart Token Compression](../token-compression.md) - search snippets are compressed before they hit the model.
<OPENHUMAN_ROOT>/vendor/tinycortex/docs/openhuman-memory/agent-tool-goals-memory.md:159:- Tool memory rules must survive prompt compression through critical/high prompt
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:21:| `priority`   | `critical`, `high`, or `normal`. Drives retrieval + compression. |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:30:| Priority   | Where it lives | Compression-resistant? |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:34:| `normal`   | Stored in the namespace; retrieved on demand via `memory_recall`. | No - eligible for compression like any other namespaced memory. |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:36:The compression-resistance property is structural: critical and high rules ride in the *system prompt*, which the inference backend's prefix cache keeps frozen for the entire session. There is no way for token compression to silently drop a `critical` rule.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:75:4. The agent sees `### \`send_email\`` followed by `- **[critical]** Never email Sarah at sarah@example.com.` before ever choosing a tool, and the rule survives any mid-session token compression.
[+1 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md]
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:49:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:350:    /// Set the per-agent TokenJuice tool-output compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:351:    pub fn tokenjuice_compression(
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:353:        profile: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:355:        self.tokenjuice_compression = profile;
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs]
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:31:Inbound traffic hits a **fast reflex agent** that triages in seconds and hands a deep **reasoning core** a concise brief; the core does the multi-step work and delegates to workers. The [subconscious loop](subconscious.md) reviews compressed session history and injects steering directives, keeping the always-on layer aligned with your goals, while 20:1 compression keeps week-long sessions bounded.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:999:        // pin into the (compression-resistant) system prompt for the whole
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1091:        let effective_tokenjuice_compression = target_def
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1092:            .map(|def| def.effective_tokenjuice_compression())
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1093:            .unwrap_or(crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Full);
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1215:            .tokenjuice_compression(effective_tokenjuice_compression);
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:44:* [Smart Token Compression](features/token-compression.md)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:145:**One engine, three entry points.** The loop lives in one place (the tinyagents `AgentHarness`, entered via `run_turn_via_tinyagents_shared` in `src/openhuman/tinyagents/mod.rs`) and every caller drives it: the chat turn (`harness/session/turn/core.rs` → `session/turn/graph.rs`), the channel/CLI bus turn (`harness/graph.rs`), and spawned sub-agents (`harness/subagent_runner/ops/graph.rs`). What varies per caller is supplied through the adapter seam: OpenHuman's provider wrapped as a `ChatModel` (`tinyagents/model.rs`), tools wrapped as tinyagents `Tool`s (`tinyagents/tools.rs`), an event bridge that projects harness `AgentEvent`s into `AgentProgress` + cost telemetry (`tinyagents/observability.rs`), `RunPolicy::unknown_tool` for hallucinated tool recovery, and a named middleware stack (`tinyagents/middleware.rs`) carrying the OpenHuman cross-cuts: approval/security gating (`ApprovalSecurityMiddleware`), tool policy and CLI/RPC-only denial (`ToolPolicyMiddleware`, `CliRpcOnlyMiddleware`), malformed-argument recovery (`ArgRecoveryMiddleware`), cost budget pre-checks (`CostBudgetMiddleware`), the repeated-tool-failure circuit breaker (`RepeatedToolFailureMiddleware`), and context trimming/compression. Policy stop hooks fire through `StopHookMiddleware` (`tinyagents/stop_hooks.rs`). The surviving OpenHuman-owned seams, `CheckpointStrategy` (error vs. summarize at the model-call cap) and `TurnProgress`, live in `harness/engine/`. Because all three entry points assemble the same harness, they can't drift.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:164:* **Microcompact / autocompact** - when total history is creeping toward the context window, tinyagents middleware (message trimming + the compression hooks in `tinyagents/summarize.rs`) compacts older turns into summaries before the next provider call. The compacted history keeps the system prompt and the most recent turns intact (KV-cache stability) and rewrites the middle.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effectively lossless: the agent calls `tokenjuice_retrieve` (token + optional byte/line range) to fetch the full original on demand. The same engine is exposed as a universal `compress_content(content, hint, opts)` for any large payload (file reads, web fetches), and as read-only `tokenjuice.*` debug RPCs. Configured via the `[tokenjuice]` block / `OPENHUMAN_TOKENJUICE_*` env. Agent definitions can override tool-result compression with `tokenjuice_compression = "auto" | "full" | "light" | "off"`; `auto` resolves coding-model agents (`[model] hint = "coding"`) to `light`, which disables CCR-backed lossy compression so coding agents keep raw build/test/diff/search text unless a reduction is truly lossless. Other agents default to `full`. The ML (Kompress) path runs as a `kompress` backend of the shared [`runtime_python_server`](../../../src/openhuman/runtime_python_server/) (torch + ModernBERT pip-installed at runtime), gated by the `ml_compression_enabled` flag and degrading gracefully to a native compressor when the Python runtime is unavailable.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:28:│ • TokenJuice compression │
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:57:9. **Compress**. Tool output and large source data go through [TokenJuice](../../features/token-compression.md) before entering LLM context.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:192:| `Critical` | `critical` | Pinned into the (compression-resistant) system prompt |
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:196:survives mid-session context compression.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:238:in the **system prompt** specifically because mid-session compression rewrites
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:268:compression rewrites the rolling chat buffer but never the frozen system prompt,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/turn/core.rs:895:            tokenjuice_compression: self.tokenjuice_compression,
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:45:core (`hint:reasoning`, spawns worker sub-agents), 20:1 compression, the
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/README.md:61:| See how memories are compressed into a tree | [Memory Tree & Compression](memory-tree.md) |
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:179:    /// when oversized tool results need summarizer-subagent compression before
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:189:    pub(super) tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:362:    /// Per-agent TokenJuice tool-output compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:363:    pub(super) tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:78:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:125:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:165:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:5:# Memory Tree & Compression
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:7:The summary tree is TinyCortex's compression mechanism. Above the raw chunk leaves, the engine folds material into a tree of immutable summary nodes: many leaves seal into one L1 summary, many L1 summaries seal into one L2 summary, and so on. This keeps recall over long histories cheap — a read can answer from a handful of high-level summaries instead of scanning thousands of leaves — while the original markdown content files remain the authoritative source of truth.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:9:## The compression pipeline
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:11:Think of the tree as a decay/compression funnel. Fresh material arrives as leaves and accumulates in an unsealed frontier buffer. Once a buffer crosses its gate, the engine **seals** that bucket into a single, immutable summary one level up, then cascades: that new summary becomes a leaf for the next level, which may itself seal, and so on. The detail "decays" upward into progressively terser summaries, but nothing is destroyed — the leaves remain on disk.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:20:![Compression pipeline](.gitbook/assets/compression-pipeline@2x.png)
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/how-memory-works.md:23:## Compression, Not Accumulation → Noise Pruning
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/how-memory-works.md:43:| Compression of experiences       | Noise pruning and memory decay           |
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:28:use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:213:    /// Per-agent TokenJuice tool-result compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:219:    pub tokenjuice_compression: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:485:    pub fn effective_tokenjuice_compression(&self) -> AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:486:        match self.tokenjuice_compression {
[+5 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs]
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/SUMMARY.md:17:* [Memory Tree & Compression](memory-tree.md)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture.md:250:| Sessions           | JSONL transcripts with compaction and tool compression |
<OPENHUMAN_ROOT>/Cargo.lock:261:name = "async-compression"
<OPENHUMAN_ROOT>/Cargo.lock:266: "compression-codecs",
<OPENHUMAN_ROOT>/Cargo.lock:267: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:279: "async-compression",
<OPENHUMAN_ROOT>/Cargo.lock:1042:name = "compression-codecs"
[+2 more match(es) in <OPENHUMAN_ROOT>/Cargo.lock]
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops_tests.rs:68:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/runner.rs:996:                    definition.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/runner.rs:1026:                    tokenjuice_compression: definition.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:40:use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:84:        tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:111:            tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:169:    // Agent-level TokenJuice profile (`definition.effective_tokenjuice_compression()`,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:583:                "[subagent_runner:graph] config load failed building sub-agent context mw; using defaults + compression profile"
[+11 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs]
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/events/types.rs:349:    /// [`ContextCompressionMiddleware`][crate::harness::middleware::ContextCompressionMiddleware]
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/events/types.rs:353:        /// Estimated total tokens of the transcript before compression.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/events/types.rs:355:        /// Estimated total tokens of the transcript after compression.
<OPENHUMAN_ROOT>/src/openhuman/learning/extract/heuristics.rs:513:            "emitted set must record the compression emission"
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:302:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:268:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/agent_graph.rs:66:    /// (`definition.effective_tokenjuice_compression()`), threaded into the
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/agent_graph.rs:69:    pub tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition_tests.rs:29:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/library/ops.rs:151:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:7://! | Compression + image processing      | ✅        | ✅           |
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:403:/// Compression pipeline handles various image sizes without panicking.
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:405:fn compression_handles_various_sizes() {
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:519:/// Verify that compression produces significant savings on realistic images.
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:521:fn compression_savings_on_realistic_screenshot() {
[+1 more match(es) in <OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs]
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:5://! dry-run a compression and show the marker/stats), inspect CCR occupancy, and
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:4://! compression needs a learned model (ModernBERT token/sentence salience). That
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:9://! Opt-in at runtime via `config.tokenjuice.ml_compression_enabled` (default
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:13://! too large — the agent loop never fails because ML compression is missing.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:24:/// `ml_compression_enabled` on from Settings — is picked up without a restart.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:52:    if !tj.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:1://! OpenHuman adapter for the vendored TinyJuice compression engine.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:27:/// Note: toggling `ml_compression_enabled` and the live compressor/CCR flags
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:38:        ml_text_enabled: tj.ml_compression_enabled,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:101:    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/README.md:3:The reusable compression engine now lives in the vendored `tinyjuice` crate at
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:26:    pub ml_compression_enabled: Option<bool>,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:70:        if let Some(v) = self.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:71:            cfg.ml_compression_enabled = v;
<OPENHUMAN_ROOT>/src/openhuman/harness_init/registry.rs:173:        && config.tokenjuice.ml_compression_enabled
<OPENHUMAN_ROOT>/src/openhuman/runtime_python/bootstrap_tests.rs:112:    use flate2::Compression;
<OPENHUMAN_ROOT>/src/openhuman/runtime_python/bootstrap_tests.rs:117:        let encoder = GzEncoder::new(&mut tar_bytes, Compression::default());
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:5://! startup. The actual compression runs inside the shared `server.py` and is
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:108:    if !config.tokenjuice.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:109:        bail!("tokenjuice.ml_compression_enabled is false");
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/registry.rs:28:    if config.tokenjuice.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/registry.rs:58:        config.tokenjuice.ml_compression_enabled = true;
<OPENHUMAN_ROOT>/vendor/tinyagents/examples/subconscious_loop/autonomous_loop.rs:195:        .with_node_kind("summarization_gate", "compression_gate")
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:45:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:416:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent_orchestration/tools/spawn_parallel_agents_tests.rs:358:        tokenjuice_compression: Default::default(),
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:180:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:1603:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:372:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:329:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/mod.rs:1://! Explicit message trimming, summarization, and compression policies.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/mod.rs:212:        let provenance = CompressionProvenance {
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs:1://! Types for explicit message trimming, summarization, and compression policies.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs:62:// Compression provenance
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs:71:pub struct CompressionProvenance {
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs:77:    /// Estimated token count of the original messages before compression.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs:84:    /// compression (e.g. `"token budget exceeded threshold 4096"`).
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/types.rs]
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/summarization/test.rs:1://! Tests for trimming, summarization, and compression policies.
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:88:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:306:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md:95:   pre-call compression.
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md:98:10. Run `after_model` middleware, including post-call compression and summary
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md:201:- context compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md:202:- transcript compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md:203:- retrieval compression middleware
[+2 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/runtime.md]
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/README.md:253:| Built-in middleware          | `libs/langchain_v1/langchain/agents/middleware/*.py`                                                | Ship focused middleware for summarization, context compression, transcript compression, retrieval compression, output compression, prompt cache layout guards, context editing, PII redaction, model/tool limits, retries, fallback, tool selection, human-in-the-loop, shell/file-search style privileged tools, and todo/task state. |
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md:177:- context compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md:178:- transcript compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md:179:- retrieval compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md:180:- streaming delta compression middleware
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md:181:- output compression middleware
[+6 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/middleware.md]
<OPENHUMAN_ROOT>/src/openhuman/memory_tree/retrieval/benchmarks.rs:13://! | 5 | Long-source compression | Large source retrieves exact relevant leaf chunk |
<OPENHUMAN_ROOT>/src/openhuman/memory_tree/retrieval/benchmarks.rs:238:// Scenario 5 — Long-source compression
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/modules/harness/embeddings.md:4:semantic search, deduplication, document compression, reranking, and
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:23:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:262:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/inference_local_ops_piper_raw_coverage_e2e.rs:17:use flate2::Compression;
<OPENHUMAN_ROOT>/tests/inference_local_ops_piper_raw_coverage_e2e.rs:258:    let encoder = GzEncoder::new(Vec::new(), Compression::none());
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/README.md:85:- `ContextCompressionMiddleware` — consults a `SummarizationPolicy` in
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/README.md:104:| `library/` | Constructors and impls for every built-in middleware, split by concern: `resilience.rs` (retry/timeout/fallback/rate-limit), `budget.rs` (token/cost tracking and enforcement), `tool_policy.rs` (allowlisting, policy, dynamic/contextual selection, human approval), `context.rs` (message trim, summarization-based compression, prompt-cache guard), `observe.rs` (structured-output validation, dynamic prompt, redaction, tracing, logging, usage accounting). |
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/tests.rs:447:        Some(200_000), // known context window → compression + trim install
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/tests.rs:517:    // context compression + message trim (window known + autocompact on), SDK
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/tests.rs:560:/// one, neither compression nor trim installs (and no early-exit hook without
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/tests.rs:592:        "compression + trim must not install without a window"
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs:2://! compression, and prompt-cache-layout guarding.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs:10:    ContextCompressionMiddleware, DEFAULT_CACHE_GUARD_EVENT_CAP, DEFAULT_COMPRESSION_RECORD_CAP,
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs:44:// ── ContextCompressionMiddleware ──────────────────────────────────────────────
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs:52:impl ContextCompressionMiddleware {
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs:53:    /// Creates a compression middleware backed by the default
[+6 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/library/context.rs]
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs:53:use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs:78:    pub(crate) tokenjuice_compression: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs:287:            tokenjuice_compression: AgentTokenjuiceCompression::Off,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs:362:                tokenjuice_compression: self.tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs:731:    tokenjuice_compression: AgentTokenjuiceCompression,
[+7 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tinyagents/middleware.rs]
<OPENHUMAN_ROOT>/src/openhuman/config/schema/context.rs:109:    /// byte cap and before they enter history. The compression never drops the
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs:51:    BudgetLimits, BudgetMiddleware, ContextCompressionMiddleware, PromptCacheGuardMiddleware,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs:482:        compression_mw,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs:863:    // Context-compression provenance (issue #4249, 03.1 item 6): the harness's
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs:865:    // compression middleware's `records()` here — each carries the full
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs:866:    // `CompressionProvenance` (source ids + before/after token estimates + policy
[+11 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tinyagents/mod.rs]
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs:467:// ── ContextCompressionMiddleware ──────────────────────────────────────────────
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs:481:/// resulting [`SummaryRecord`] (with its compression provenance) is recorded,
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs:487:/// [`ContextCompressionMiddleware::with_summarizer`].
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs:489:/// [`ContextCompressionMiddleware`] retains before evicting the oldest.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs:490:pub const DEFAULT_COMPRESSION_RECORD_CAP: usize = 1024;
[+2 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/types.rs]
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs:299:async fn context_compression_is_noop_below_window_threshold() {
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs:306:    let mw = Arc::new(ContextCompressionMiddleware::new(policy));
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs:338:async fn context_compression_compresses_at_or_above_threshold() {
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs:347:    let mw = Arc::new(ContextCompressionMiddleware::new(policy));
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs:401:async fn context_compression_records_are_bounded_by_max_records() {
[+5 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/middleware/test.rs]
<OPENHUMAN_ROOT>/app/src/services/webviewAccountService.ts:896:    // hour of a 4h meeting — the compression pass still works on tail.
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/payload_summarizer.rs:1://! Oversized-tool-result compression via the `summarizer` sub-agent.
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/payload_summarizer.rs:470:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:18:    /// Whether lossy compressions offload the original to the CCR store and emit
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:57:    pub ml_compression_enabled: bool,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:61:    /// Target compression ratio (0–1) hint for the ML compressor.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:121:            ml_compression_enabled: false,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:141:        assert!(!c.ml_compression_enabled);
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs]
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/convert.rs:343:/// *appends* (assistant/tool rounds + applied steers); the compression/trim
<OPENHUMAN_ROOT>/tests/agent_session_turn_raw_coverage_e2e.rs:25:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_session_turn_raw_coverage_e2e.rs:1063:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/memory_tree/tree/bucket_seal.rs:1352:    // needs compression).
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs:13://! [`ContextCompressionMiddleware`][tinyagents::harness::middleware::ContextCompressionMiddleware]
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs:20://! Layering: a graph installs the compression middleware **before** the
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs:33:    estimate_tokens, CompressionProvenance, SummarizationPolicy, Summarizer, SummaryRecord,
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs:149:            provenance: CompressionProvenance {
<OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs:188:/// crate [`ContextCompressionMiddleware`][tinyagents::harness::middleware::ContextCompressionMiddleware]
[+2 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tinyagents/summarize.rs]
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/capture.rs:449:        //    can pin it into the (compression-resistant) system prompt.
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/prompt.rs:6://! Mid-session compression rewrites the rolling chat buffer but never
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/prompt.rs:11://! Anything we want to be **compression-resistant** therefore has to
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md:44:- Provide middleware hooks during streaming model calls so compression,
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md:237:   compression state, and provider options.
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md:240:6. Run streaming middleware while model deltas arrive, including compression,
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md:279:compression algorithm often needs to wrap the entire model operation so it can
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md:291:- context compression
[+9 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyagents/docs/spec/harness-spec.md]
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/mod.rs:30://!   mid-session compression.
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/README.md:20:| [`prompt.rs`](prompt.rs) | `ToolMemoryRulesSection` + `render_tool_memory_rules` — prompt section that pins Critical / High rules into the system prompt so they survive compression. `TOOL_MEMORY_HEADING` + `TOOL_MEMORY_PROMPT_CAP` constants. |
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/types.rs:14://!   therefore not subject to mid-session context compression.
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/types.rs:33:    /// Safety-critical rule — pinned into the (compression-resistant)
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/types.rs:47:    /// prefetched at session start, so they survive context compression).
<OPENHUMAN_ROOT>/src/openhuman/memory_tools/types.rs:89:    /// Criticality level for retrieval and compression behaviour.
<OPENHUMAN_ROOT>/vendor/tinyagents/wiki/Harness.md:157:`LoggingMiddleware`, `MessageTrimMiddleware`, `ContextCompressionMiddleware`,
<OPENHUMAN_ROOT>/vendor/tinyagents/wiki/Harness.md:309:`CompressionProvenance` record what was compressed so the compaction is
<OPENHUMAN_ROOT>/vendor/tinyagents/wiki/Harness.md:318:limit-reached (`LimitKind`), memory, and compression. Events flow
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:491:        if let Some(flag) = env.get("OPENHUMAN_TOKENJUICE_ML_COMPRESSION_ENABLED") {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:492:            if let Some(v) = parse_env_bool("OPENHUMAN_TOKENJUICE_ML_COMPRESSION_ENABLED", &flag) {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:493:                self.tokenjuice.ml_compression_enabled = v;
<OPENHUMAN_ROOT>/app/src/pages/onboarding/steps/ContextGatheringStep.tsx:164: * First-launch profile compression on slow hardware (#2156) can run past the
<OPENHUMAN_ROOT>/src/openhuman/inference/local/install_piper.rs:32:/// is ~60 MB; allow some slack for CDN compression differences.
<OPENHUMAN_ROOT>/src/openhuman/inference/local/install_whisper.rs:44:/// some slack for HF mirror compression differences). Anything below this
<OPENHUMAN_ROOT>/src/openhuman/orchestration/ops.rs:24:use super::graph::compress::{compression_budget, count_tokens, enforce_budget};
<OPENHUMAN_ROOT>/src/openhuman/orchestration/ops.rs:521:/// agents, the compression summarizer, the world-diff + compressed-history store
<OPENHUMAN_ROOT>/src/openhuman/orchestration/ops.rs:607:        // The trace the compression node condenses. `run_single` surfaces the
<OPENHUMAN_ROOT>/src/openhuman/orchestration/ops.rs:621:        let budget = compression_budget(input_tokens);
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs:1://! 20:1 compression mechanics for the `compress` node (stage 5).
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs:13:/// The strict compression ratio (spec §3): 20 input tokens per output token.
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs:14:pub const COMPRESSION_RATIO: u64 = 20;
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs:18:pub const COMPRESSION_FLOOR_TOKENS: u64 = 200;
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs:30:pub fn compression_budget(input_tokens: u64) -> u64 {
[+8 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/compress.rs]
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/mod.rs:22://! reasoning core + sub-agent spawning, 20:1 compression, the world-diff append,
<OPENHUMAN_ROOT>/src/openhuman/memory_diff/ops.rs:461:/// delta compression keeps it compact — so cleanup only prunes named baselines.
<OPENHUMAN_ROOT>/src/openhuman/orchestration/graph/build.rs:75:    /// Compression node appended a compressed-history entry.
<OPENHUMAN_ROOT>/src/openhuman/inference/provider/compatible_stream_native.rs:256:        // root Cargo.toml), so no Content-Encoding decompression happens
<OPENHUMAN_ROOT>/app/src-tauri/src/meet_audio/listen_capture.rs:254:            // overshoot a touch on heavy compression.
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:327:# AST-aware code compression (tree-sitter Rust/TS/Python grammars; C build).
<OPENHUMAN_ROOT>/app/src/lib/i18n/id.ts:420:  'settings.tokenUsage.compressionTitle': 'Kompresi',
<OPENHUMAN_ROOT>/app/src/lib/i18n/id.ts:421:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/ko.ts:415:  'settings.tokenUsage.compressionTitle': '압축',
<OPENHUMAN_ROOT>/app/src/lib/i18n/ko.ts:416:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/pl.ts:422:  'settings.tokenUsage.compressionTitle': 'Kompresja',
<OPENHUMAN_ROOT>/app/src/lib/i18n/pl.ts:423:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/pt.ts:424:  'settings.tokenUsage.compressionTitle': 'Compressão',
<OPENHUMAN_ROOT>/app/src/lib/i18n/pt.ts:425:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/mod.rs:5:    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:3:use crate::{CompressionConfig, TinyJuiceError, TinyJuiceResult};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:6:pub struct CompressionInput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:10:impl CompressionInput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:25:pub struct CompressionOutput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:27:    pub report: CompressionReport,
[+9 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:4:        CompressionConfig, CompressionInput, Compressor, PassthroughCompressor, TinyJuiceError,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:10:        let input = CompressionInput::new("alpha beta gamma");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:13:            .compress(input, &CompressionConfig::default())
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:14:            .expect("passthrough compression should succeed");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:24:        let input = CompressionInput::new("   ");
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/error.rs:7:    #[error("compression input was empty")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/error.rs:11:    #[error("compression strategy is not implemented: {0}")]
<OPENHUMAN_ROOT>/app/src/lib/i18n/ru.ts:424:  'settings.tokenUsage.compressionTitle': 'Сжатие',
<OPENHUMAN_ROOT>/app/src/lib/i18n/ru.ts:425:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/mod.rs:4:pub use types::CompressionConfig;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:6:pub struct CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:11:impl CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:21:impl Default for CompressionConfig {
<OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts:413:    'Paramètres de compression et combien de tokens et de dollars ils ont économisés',
<OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts:426:  'settings.tokenUsage.compressionTitle': 'Compression',
<OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts:427:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts:429:  'settings.tokenUsage.routerEnabled': 'Activer la compression',
<OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts:982:  'onboarding.runtimeChoice.cloud.f2': 'Compression de tokens pour aller plus loin',
[+1 more match(es) in <OPENHUMAN_ROOT>/app/src/lib/i18n/fr.ts]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:3:    use crate::{CompressionConfig, TinyJuiceError};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:6:    fn default_config_targets_aggressive_compression() {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:7:        let config = CompressionConfig::default();
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:15:        let config = CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:17:            ..CompressionConfig::default()
<OPENHUMAN_ROOT>/app/src/lib/i18n/hi.ts:416:  'settings.tokenUsage.compressionTitle': 'संपीड़न',
<OPENHUMAN_ROOT>/app/src/lib/i18n/hi.ts:417:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/en.ts:307:    'Compression settings and how many tokens and dollars they have saved',
<OPENHUMAN_ROOT>/app/src/lib/i18n/en.ts:320:  'settings.tokenUsage.compressionTitle': 'Compression',
<OPENHUMAN_ROOT>/app/src/lib/i18n/en.ts:321:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/en.ts:323:  'settings.tokenUsage.routerEnabled': 'Enable compression',
<OPENHUMAN_ROOT>/app/src/lib/i18n/en.ts:1119:  'onboarding.runtimeChoice.cloud.f2': 'Token compression to stretch your usage further',
<OPENHUMAN_ROOT>/app/src/lib/i18n/es.ts:423:  'settings.tokenUsage.compressionTitle': 'Compresión',
<OPENHUMAN_ROOT>/app/src/lib/i18n/es.ts:424:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/mod.rs:4:pub use types::OpenHumanCompressionContext;
<OPENHUMAN_ROOT>/app/src/lib/i18n/bn.ts:417:  'settings.tokenUsage.compressionTitle': 'সংকোচন',
<OPENHUMAN_ROOT>/app/src/lib/i18n/bn.ts:418:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/types.rs:4:pub struct OpenHumanCompressionContext {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/test.rs:3:    use crate::openhuman::OpenHumanCompressionContext;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/test.rs:7:        let context = OpenHumanCompressionContext::default();
<OPENHUMAN_ROOT>/app/src/lib/i18n/it.ts:412:    'Impostazioni di compressione e quanti token e dollari hanno fatto risparmiare',
<OPENHUMAN_ROOT>/app/src/lib/i18n/it.ts:425:  'settings.tokenUsage.compressionTitle': 'Compressione',
<OPENHUMAN_ROOT>/app/src/lib/i18n/it.ts:426:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/it.ts:428:  'settings.tokenUsage.routerEnabled': 'Abilita compressione',
<OPENHUMAN_ROOT>/app/src/lib/i18n/it.ts:979:  'onboarding.runtimeChoice.cloud.f2': 'Compressione token per allungare il tuo utilizzo',
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:311:pub enum AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:324:impl AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:529:    /// CCR only fires (offload original + lossy compression) when the input is
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:23:use super::types::{AgentTokenjuiceCompression, CompressInput, CompressOptions, ContentHint};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:55:fn options_for_agent(profile: AgentTokenjuiceCompression) -> Option<CompressOptions> {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:57:        AgentTokenjuiceCompression::Off => None,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:58:        AgentTokenjuiceCompression::Auto | AgentTokenjuiceCompression::Full => {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:61:        AgentTokenjuiceCompression::Light => {
[+9 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs]
<OPENHUMAN_ROOT>/app/src/lib/i18n/ar.ts:408:  'settings.tokenUsage.compressionTitle': 'الضغط',
<OPENHUMAN_ROOT>/app/src/lib/i18n/ar.ts:409:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/app/src/lib/i18n/zh-CN.ts:395:  'settings.tokenUsage.compressionTitle': '压缩',
<OPENHUMAN_ROOT>/app/src/lib/i18n/zh-CN.ts:396:  'settings.tokenUsage.compressionDesc': '在大型工具输出进入模型上下文之前，对其进行内容感知压缩。',
<OPENHUMAN_ROOT>/app/src/lib/i18n/de.ts:427:  'settings.tokenUsage.compressionTitle': 'Komprimierung',
<OPENHUMAN_ROOT>/app/src/lib/i18n/de.ts:428:  'settings.tokenUsage.compressionDesc':
<OPENHUMAN_ROOT>/vendor/tinyjuice/SECURITY.md:3:TinyJuice is a token compression library. Security-sensitive areas include
<OPENHUMAN_ROOT>/vendor/tinyjuice/SECURITY.md:48:TinyJuice should treat prompt and context input as sensitive. Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:3:TinyJuice is a Rust crate for pluggable token compression in OpenHuman. Keep the
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:4:scaffold small until real compression strategies are ready.
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:19:- Do not claim compression percentages until benchmark fixtures exist.
<OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock:338:name = "async-compression"
<OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock:343: "compression-codecs",
<OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock:344: "compression-core",
<OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock:370: "async-compression",
<OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock:1355:name = "compression-codecs"
[+2 more match(es) in <OPENHUMAN_ROOT>/app/src-tauri/Cargo.lock]
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:1://! Criterion benchmarks for the compression hot paths.
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:4://! rule engine on deterministic synthetic payloads. They are NOT compression-
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:14:    AgentTokenjuiceCompression, CompressOptions, ReduceOptions, ToolExecutionInput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:95:                    AgentTokenjuiceCompression::Full,

[compacted tool output — this is a PARTIAL view; the full original (73893 bytes) is available by calling tokenjuice_retrieve with token "bb5fcd0f9f8f5609a3e39bf3a060762d" (marker ⟦tj:bb5fcd0f9f8f5609a3e39bf3a060762d⟧)]