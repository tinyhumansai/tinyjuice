<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/src/core/all.rs:651:            Some("Hierarchical time-based summarization tree for background knowledge compression.")
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 토큰으로 전달됩니다. 이것 없이는 이만큼 큰 두뇌를 감당할 수 없을 것입니다.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: وہی معلومات، 80% تک کم ٹوکنز۔ اتنا بڑا دماغ اس کے بغیر ناقابلِ برداشت مہنگا ہوتا۔
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。没有它，这么大的一颗大脑将贵得用不起。
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu 80% weniger Tokens. Ein so großes Gehirn wäre ohne es unbezahlbar.
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少ないトークンで扱えます。これがなければ、これほど大きな脳は維持できません。
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:31:| 20:1 compression engine | `orchestration/graph/compress.rs` + `ProductionRuntime::compress` |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:57:- No change to the orchestration wake graph, compression ratio, context-guard
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so sweeping through your last six months of email costs single-digit dollars. [Automatic model routing](features/model-routing/) sends each task to the right model - `hint:reasoning` to a frontier model, `hint:fast` to a cheap one, vision to vision - all under one subscription. Optional [local AI via Ollama or LM Studio](features/model-routing/local-ai.md) keeps supported workloads on-device.
<OPENHUMAN_ROOT>/gitbooks/features/privacy-and-security.md:65:Compression and locality together become the privacy architecture.
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:40:| `hint:summarize` | A model good at compression | Memory tree summary builders |
<OPENHUMAN_ROOT>/gitbooks/features/model-routing/README.md:98:- [Smart Token Compression](../token-compression.md). what makes large reasoning calls affordable.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:100:## Cost & token compression
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:102:Because cost tracks **real token counts**, anything that shrinks the prompt directly lowers spend. OpenHuman's [TokenJuice token compression](token-compression.md) reduces the tokens sent on each call, and [model routing](model-routing/README.md) sends work to the cheapest model that can handle it. Both show up as lower bars in the dashboard and slower budget burn.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:108:- [Token compression (TokenJuice)](token-compression.md)
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:3:  TokenJuice - a multi-stage compression router that compacts verbose tool
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:8:# Smart Token Compression
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:12:OpenHuman ships with **TokenJuice**, a compression router wired directly into the agent's tool-execution path. Before any tool result reaches a model, TokenJuice classifies it, routes it to a specialized compressor, optionally offloads the full original to a recoverable cache, and records how many tokens (and dollars) it saved.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:53:3. **Compressor selection.** Each kind routes to a dedicated compressor, honoring per-kind toggles (`search_enabled`, `code_enabled`, `html_enabled`, `ml_compression_enabled`).
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:54:4. **Compression.** The compressor runs. If it declines or its output is no smaller than the input, TokenJuice falls back to the generic compressor or passes the original through. It never makes things bigger.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:55:5. **CCR offload.** For **lossy** compressions where the original is large enough (`ccr_min_tokens`, default ~500 tokens), the full original is stowed in the **Compress-Cache-Retrieve** store so nothing is permanently lost.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:73:| **MlText**       | PlainText   | Opt-in ML salience compression (see below).                                                              |
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:80:## ML compression (opt-in)
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:84:* **Off by default.** Enable with `ml_compression_enabled = true` in `[tokenjuice]`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and leaves a breadcrumb (`vendor/tinyjuice/src/cache/`).
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:106:Every compression is metered by an OpenHuman savings callback (`src/openhuman/tokenjuice/savings.rs`). TokenJuice reports events and token deltas; OpenHuman applies per-model input pricing, aggregates `total`, `by_model`, and `by_compressor`, and persists stats to `<workspace>/state/tokenjuice_savings.json`.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/scoring.md:109:Embeddings run on the background workers, not the ingest hot path, so a burst of new sources never blocks the UI. Trees give compression and navigation; embeddings keep similarity search working underneath them.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/scoring.md:118:* [Token Compression](../token-compression.md) - why keeping the tree dense matters.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-diff.md:101:Checkpoints are cheap to prune: `cleanup` deletes tags older than N days, but **snapshot commits are never deleted** - git history _is_ the ledger, and git's delta compression keeps it compact.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/auto-fetch.md:60:* [Smart Token Compression](../token-compression.md). what keeps "fetch everything" cheap.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-tree.md:74:Trees give you compression _and_ navigation. Embeddings still live inside so semantic search keeps working, but the structure on top is what makes the memory feel like a brain instead of a bag of fragments.
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:28:Inbound sessions run through a **split-brain wake graph**: a fast reflex agent triages each message in seconds (reply immediately, or hand the deep reasoning core a concise brief), while the reasoning core does the real multi-step work and delegates to sub-agent workers. Long sessions stay bounded via 20:1 history compression and a rolling world-state diff, and your [subconscious loop](subconscious.md) periodically reviews the whole picture and injects a short steering directive to keep the layer aligned with *your* priorities.
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:196:* Long sessions stay bounded by **20:1 history compression** plus a rolling world-state diff with utilization-based eviction.
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/dispatch/mod.rs:128:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:14:**TinyJuice is a Rust token-compression engine for agent context.** It gives
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:26:auditable compression layer with conservative pass-through behavior, recovery
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:74:## Compression Surfaces
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:88:  compression; disabled by default.
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:92:TinyJuice does not publish compression percentage claims yet. Throughput
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:109:use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:114:        CompressionInput::new("Keep this text unchanged for now."),
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:115:        &CompressionConfig::default(),
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:145:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:153:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:181:The interface accepts metadata-oriented compression records. Do not feed raw
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:196:  config/            Small public CompressionConfig scaffold
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:225:claim compression percentages until benchmark fixtures exist.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/mod.rs:5:    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
<OPENHUMAN_ROOT>/vendor/tinyjuice/examples/passthrough.rs:1:use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};
<OPENHUMAN_ROOT>/vendor/tinyjuice/examples/passthrough.rs:5:    let input = CompressionInput::new("Keep this text unchanged for now.");
<OPENHUMAN_ROOT>/vendor/tinyjuice/examples/passthrough.rs:6:    let output = compressor.compress(input, &CompressionConfig::default())?;
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-scraper.md:31:* [Smart Token Compression](../token-compression.md) - what trims long pages before they hit the model.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:3:use crate::{CompressionConfig, TinyJuiceError, TinyJuiceResult};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:6:pub struct CompressionInput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:10:impl CompressionInput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:25:pub struct CompressionOutput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:27:    pub report: CompressionReport,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:31:pub struct CompressionReport {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:42:        input: CompressionInput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:43:        config: &CompressionConfig,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:44:    ) -> TinyJuiceResult<CompressionOutput>;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:57:        input: CompressionInput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:58:        config: &CompressionConfig,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:59:    ) -> TinyJuiceResult<CompressionOutput> {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:65:        Ok(CompressionOutput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/types.rs:66:            report: CompressionReport {
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:14:use tinyjuice::types::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:43:        AgentTokenjuiceCompression::Off,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:60:            AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:77:        AgentTokenjuiceCompression::Light,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:102:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:138:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:25:TinyJuice should make token compression explicit and inspectable. Prefer:
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:29:- deterministic compression behavior around model-facing context
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:31:- reports that make compression ratios and strategy choices visible
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:32:- examples that show concrete compression behavior rather than abstract promises
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:54:Add compression target validation tests
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/types.ts:1:export type CompressionStatus = "compressed" | "passthrough" | "declined" | "error";
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/types.ts:3:export type CompressionRun = {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/types.ts:8:  status: CompressionStatus;
<OPENHUMAN_ROOT>/vendor/tinyjuice/ROADMAP.md:3:TinyJuice is pre-1.0, but the core compression path is no longer blank
<OPENHUMAN_ROOT>/vendor/tinyjuice/ROADMAP.md:11:- core compression trait, input model, output report, and error taxonomy
<OPENHUMAN_ROOT>/vendor/tinyjuice/ROADMAP.md:23:1. Build benchmark fixtures for compression ratio, latency, retained facts, and
<OPENHUMAN_ROOT>/vendor/tinyjuice/ROADMAP.md:29:4. Expand source-code compression coverage and tree-sitter behavior tests.
<OPENHUMAN_ROOT>/vendor/tinyjuice/ROADMAP.md:31:   compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/scaffold.rs:1:use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/scaffold.rs:8:            CompressionInput::new("public api smoke test"),
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/scaffold.rs:9:            &CompressionConfig::default(),
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/README.md:21:* All output passes through [Smart Token Compression](../token-compression.md) for free.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/README.md:41:* [Smart Token Compression](../token-compression.md) - what keeps tool output costs bounded.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:4:        CompressionConfig, CompressionInput, Compressor, PassthroughCompressor, TinyJuiceError,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:10:        let input = CompressionInput::new("alpha beta gamma");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:13:            .compress(input, &CompressionConfig::default())
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:14:            .expect("passthrough compression should succeed");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:24:        let input = CompressionInput::new("   ");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressor/test.rs:27:            .compress(input, &CompressionConfig::default())
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/batched-edit-validation-spec.md:6:tool-round-trip reduction strategy, not text compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/batched-edit-validation-spec.md:74:  report: CompressionReport,
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/charts.tsx:1:import type { AlgorithmSummary, CompressionRun } from "./types";
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/charts.tsx:8:export function TokenTrend({ runs }: { runs: CompressionRun[] }) {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/charts.tsx:82:function toTrend(runs: CompressionRun[]): TrendPoint[] {
<OPENHUMAN_ROOT>/vendor/tinyjuice/SUPPORT.md:13:- proposals for compression strategies, OpenHuman adapters, or reporting APIs
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:8:The goal is one shared compression engine with thin adapters:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:209:- `light`: only lossless or non-CCR reductions; no ML text compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:337:- add benchmark fixtures before making compression percentage claims
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/web-search.md:51:* [Smart Token Compression](../token-compression.md) - search snippets are compressed before they hit the model.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/error.rs:7:    #[error("compression input was empty")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/error.rs:11:    #[error("compression strategy is not implemented: {0}")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/sql-introspection-reduction-spec.md:66:  report: CompressionReport,
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/App.tsx:26:import type { AlgorithmSummary, CompressionRun } from "./types";
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/App.tsx:37:  const [runs, setRuns] = useState<CompressionRun[]>([]);
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/App.tsx:296:function RunTable({ runs }: { runs: CompressionRun[] }) {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/App.tsx:332:function summarizeTotals(runs: CompressionRun[], summaries: AlgorithmSummary[]) {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/App.tsx:350:function applyFilters(runs: CompressionRun[], range: Range, algorithm: string, kind: string) {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/mod.rs:4:pub use types::CompressionConfig;
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:28:  output, and optional ML text compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:31:- per-agent compression profiles (`auto`, `full`, `light`, `off`)
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:336:- Agent `light` profile must disable lossy CCR-backed compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:338:- Compression percentage claims remain forbidden until benchmark fixtures exist.
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:1:import type { AlgorithmSummary, CompressionRun, CompressionStatus } from "./types";
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:3:const KNOWN_STATUSES = new Set<CompressionStatus>([
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:12:export async function loadSampleRuns(): Promise<CompressionRun[]> {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:20:export function loadStoredRuns(): CompressionRun[] | null {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:33:export function storeRuns(runs: CompressionRun[]) {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:41:export function normalizeRuns(input: unknown): CompressionRun[] {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:45:    .filter((row): row is CompressionRun => row !== null)
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:49:function normalizeRun(row: unknown, index: number): CompressionRun | null {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:67:  const status = typeof value.status === "string" && KNOWN_STATUSES.has(value.status as CompressionStatus)
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:68:    ? (value.status as CompressionStatus)
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:92:export function summarizeByAlgorithm(runs: CompressionRun[]): AlgorithmSummary[] {
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/src/data.ts:93:  const map = new Map<string, CompressionRun[]>();
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:6:pub struct CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:11:impl CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/types.rs:21:impl Default for CompressionConfig {
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:21:| `priority`   | `critical`, `high`, or `normal`. Drives retrieval + compression. |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:30:| Priority   | Where it lives | Compression-resistant? |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:34:| `normal`   | Stored in the namespace; retrieved on demand via `memory_recall`. | No - eligible for compression like any other namespaced memory. |
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:36:The compression-resistance property is structural: critical and high rules ride in the *system prompt*, which the inference backend's prefix cache keeps frozen for the entire session. There is no way for token compression to silently drop a `critical` rule.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:75:4. The agent sees `### \`send_email\`` followed by `- **[critical]** Never email Sarah at sarah@example.com.` before ever choosing a tool, and the rule survives any mid-session token compression.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/tool-memory.md:82:- [Smart Token Compression](../token-compression.md) - what the system prompt is protected against.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/ranked-search-read-spec.md:46:  report: CompressionReport,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:3:    use crate::{CompressionConfig, TinyJuiceError};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:6:    fn default_config_targets_aggressive_compression() {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:7:        let config = CompressionConfig::default();
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:15:        let config = CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/config/test.rs:17:            ..CompressionConfig::default()
<OPENHUMAN_ROOT>/vendor/tinyjuice/SECURITY.md:3:TinyJuice is a token compression library. Security-sensitive areas include
<OPENHUMAN_ROOT>/vendor/tinyjuice/SECURITY.md:48:TinyJuice should treat prompt and context input as sensitive. Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:1:# TurboQuant-Style Vector Compression Spec
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:5:Design reference for a possible TinyJuice vector-compression strategy. This is
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:6:not an implementation commitment, benchmark claim, or prompt compression claim.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:42:  config: CompressionConfig,
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:47:CompressionConfig {
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:79:## Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:98:## Decompression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/turboquant-vector-spec.md:120:compression_ratio(config) -> f32
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:31:Inbound traffic hits a **fast reflex agent** that triages in seconds and hands a deep **reasoning core** a concise brief; the core does the multi-step work and delegates to workers. The [subconscious loop](subconscious.md) reviews compressed session history and injects steering directives, keeping the always-on layer aligned with your goals, while 20:1 compression keeps week-long sessions bounded.
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:3:TinyJuice is a Rust crate for pluggable token compression in OpenHuman. Keep the
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:4:scaffold small until real compression strategies are ready.
<OPENHUMAN_ROOT>/vendor/tinyjuice/AGENTS.md:19:- Do not claim compression percentages until benchmark fixtures exist.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/pipeline-and-ccr-plan.md:104:- Log template compression will be a reformat.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/pipeline-and-ccr-plan.md:105:- Signal-log compression is an offload.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/pipeline-and-ccr-plan.md:127:- Reformat-only compression works with CCR disabled.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/pipeline-and-ccr-plan.md:128:- Offload compression declines if CCR cannot retain the original.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/README.md:15:- [TurboQuant-style vector compression](turboquant-vector-spec.md)
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/README.md:20:- [Hermes compression algorithms and techniques](hermes-compression-algorithms-spec.md)
<OPENHUMAN_ROOT>/src/openhuman/config/schema/context.rs:109:    /// byte cap and before they enter history. The compression never drops the
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:5:Integrate TinyJuice into OpenHuman as a shared Rust compression engine while
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:20:  plus per-agent `AgentTokenjuiceCompression` overrides and
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:71:- `AgentTokenjuiceCompression` profiles
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:88:- wiring optional ML compression through the existing callback
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:94:- deterministic classification and compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:201:`ContextCompressionMiddleware` (`src/openhuman/tinyagents/summarize.rs`, live
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:226:- Applying code compression to exact reads would damage trust. Default should be
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:233:- ML text compression can remove workflow tags or instructions. It should stay
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:1://! Criterion benchmarks for the compression hot paths.
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:4://! rule engine on deterministic synthetic payloads. They are NOT compression-
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:14:    AgentTokenjuiceCompression, CompressOptions, ReduceOptions, ToolExecutionInput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:95:                    AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/subagent-summary-spec.md:53:## Compression Contract
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/rule-cli-and-safety-parity-plan.md:43:- Content-router extension hints can still trigger code or JSON compression.
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:44:* [Smart Token Compression](features/token-compression.md)
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:1:# Hermes Compression Algorithms and Techniques Spec
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:5:This spec documents the compression and prompt-size management techniques found
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:7:It is a design reference only. It does not claim TinyJuice compression savings;
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:19:- `agent/conversation_compression.py`
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:23:- `agent/manual_compression_feedback.py`
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:55:- manual partial compression boundaries
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:66:- JSON table compression with head/tail, error-row, and outlier preservation
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:69:- optional tree-sitter code compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:70:- optional ML text compression through an adapter boundary
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:78:## Important Distinction: Compression vs Prompt Caching
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:80:Hermes keeps prompt caching separate from compression:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:82:- `agent/context_compressor.py` and `agent/conversation_compression.py` mutate
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:90:- compression APIs may mutate text or return CCR markers
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:96:## Hermes Production Compression Pipeline
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:113:  -> post-compression accounting
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:119:pub struct ConversationCompressionInput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:128:pub struct ConversationCompressionOutput {
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:130:    pub report: CompressionReport,
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:157:- Include output reservation in compression trigger decisions.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:197:- Record each digest in the compression report.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:276:the first compression. After the first compression, the early non-system head
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:319:  ceiling but compression was explicitly requested
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:331:- if a compression-start boundary lands on a tool result, move it forward
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:332:- if a compression-end boundary lands after tool results, move it backward to
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:492:- repeated ineffective compression: back off
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:501:- Add cooldown state outside stateless core compression, owned by the adapter.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:529:Compression reports should include:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:543:- A no-op compression does not require session persistence changes.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:547:## Compression Locks and Race Avoidance
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:549:Hermes uses a state-backed per-session compression lock with a TTL and refresher
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:578:## Manual Partial Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:589:- If the split leaves no head, return a no-op or fall back to full compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:650:- Cache hints are reported separately from compression steps.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:676:This helps hosts explain why compression triggered and where savings came from.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:682:- The report separates conversation compression from static tool/schema cost.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:698:TinyJuice should separate model-facing compression from UI-facing rendering, but
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:714:## Offline Trajectory Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:727:This is less relevant to TinyJuice runtime compression but useful for benchmark
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:739:- Fixture compression can be reproduced deterministically with a stub summary
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:848:- docs showing that these are separate from compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:883:- repeated no-op compression anti-thrash
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:889:- Do not merge prompt caching with lossy compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:909:- Compression failure policy aborts on auth and network failures.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:916:core Rust reducer should continue to focus on content-aware compression and CCR.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:1:# Conversation Compression Plan
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:11:Conversation compression is not the same as tool-output compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:14:`ContextCompressionMiddleware` (live history summarization,
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:107:- Cache hints are reported separately from compression steps.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:152:- Hosts can show why compression triggered without logging raw prompt text.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:160:- Compression leases implemented against OpenHuman storage inside core.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/conversation-compression-plan.md:161:- Prompt cache mutation mixed into lossy compression APIs.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/mod.rs:4:pub use types::OpenHumanCompressionContext;
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:128:Source spec: `hermes-compression-algorithms-spec.md`. Design detail exists in
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:129:`plan/conversation-compression-plan.md`; this section maps it onto OpenHuman's
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:135:- `src/openhuman/tinyagents/summarize.rs` — `ContextCompressionMiddleware`
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:165:- `src/openhuman/tinyagents/summarize.rs` — `ContextCompressionMiddleware`
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:256:compression behind `tokenjuice-treesitter` (OpenHuman enables it by default in
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:301:enabling `ml_compression_enabled` (the Kompress bridge is already wired in
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:370:Source spec: `hermes-compression-algorithms-spec.md` (P2 portions). Builds on
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:376:`ContextCompressionMiddleware` implements `SummaryProvider` over its existing
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:390:  value (fewer retry loops) but it is mutation tooling, not compression, and
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:398:- **TurboQuant vectors** (`turboquant-vector-spec.md`): storage compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:399:  for embedding indexes, not prompt compression. OpenHuman does have an
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:22:- Do not claim Headroom's public compression percentages for TinyJuice.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:54:- Reformat-only compression works when CCR is disabled.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:55:- Offload compression declines when the CCR store cannot retain the original.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:84:- Frozen bytes are byte-identical before and after compression in adapter
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:104:- Keep backend construction outside the hot compression path.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:210:ML text compression should not be the only useful plain-text path. Add a
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:232:## P1: Tag Protection for ML Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:234:Before expanding ML compression, TinyJuice should protect custom workflow tags.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:242:- Restore exact original tag text after ML compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:246:- Custom tags survive ML compression byte-for-byte.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:270:- Exact identifiers receive enough score to survive compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:274:Headroom centralizes mode-specific decisions in a compression policy. TinyJuice
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:279:- Add `CompressionPolicy` with fields such as:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:284:  - ML compression allowed
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:334:7. Add tag protection around ML compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/interface/README.md:3:Self-hostable analytics UI for TinyJuice compression runs. It follows the same
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/types.rs:4:pub struct OpenHumanCompressionContext {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:18:    /// Whether lossy compressions offload the original to the CCR store and emit
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:57:    pub ml_compression_enabled: bool,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:61:    /// Target compression ratio (0–1) hint for the ML compressor.
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:121:            ml_compression_enabled: false,
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:141:        assert!(!c.ml_compression_enabled);
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:150:            ml_compression_enabled = true
<OPENHUMAN_ROOT>/src/openhuman/config/schema/tokenjuice.rs:157:        assert!(c.ml_compression_enabled);
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:8:  `CompressionInput`, `CompressionOutput`, `CompressionReport`, and
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:23:- host callback slots for ML compression and savings accounting
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:31:The crate boundary is mostly clean. ML text compression is a host callback, and
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:141:- Abstractive prose compression as the default fallback.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:142:- Vector compression in the prompt-compression path.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:145:- Compression percentage marketing claims.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/savings-accounting-spec.md:6:accounting infrastructure, not a compression algorithm.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/savings-accounting-spec.md:72:enough `CompressionReport` metadata for aggregation:
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/test.rs:3:    use crate::openhuman::OpenHumanCompressionContext;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/openhuman/test.rs:7:        let context = OpenHumanCompressionContext::default();
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:41:## Diff Compression And DiffNoise
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:69:## Log Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:86:- Signal compression for data that has no log signal.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:92:- Logs with no useful template pass to signal compression or decline.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:123:## Search Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:189:- Tag protection around ML compression before enabling broader use.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:194:- Default ML text compression without tag-protection fixtures.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:199:- Custom workflow tags survive ML compression byte-for-byte.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/web-extract-truncate-store-spec.md:54:- Do not make compression percentage, speedup, or cost claims until TinyJuice has
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:311:pub enum AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:324:impl AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:529:    /// CCR only fires (offload original + lossy compression) when the input is
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/adapter-and-product-surfaces-plan.md:111:- Keep host-specific mutation code out of core compression modules.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:3:## Deferred: TurboQuant Vector Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:5:TurboQuant-style vector compression is useful for embedding indexes and recall
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:6:storage, not for immediate prompt compression. It should be deferred until
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:21:ML text compression and embedding relevance should stay optional.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:49:Installers are product surface, not core compression. They should wait until the
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:73:- it crosses from compression into interpretation
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:78:## Rejected: Compression Percentage Claims
<OPENHUMAN_ROOT>/vendor/tinyjuice/Cargo.toml:6:description = "Pluggable token compression for OpenHuman."
<OPENHUMAN_ROOT>/vendor/tinyjuice/Cargo.toml:9:keywords = ["llm", "tokens", "compression", "openhuman", "context"]
<OPENHUMAN_ROOT>/vendor/tinyjuice/Cargo.toml:49:name = "compression"
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:11://! Both are **pass-through safe**: if compression doesn't meaningfully shrink
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:23:use super::types::{AgentTokenjuiceCompression, CompressInput, CompressOptions, ContentHint};
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:55:fn options_for_agent(profile: AgentTokenjuiceCompression) -> Option<CompressOptions> {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:57:        AgentTokenjuiceCompression::Off => None,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:58:        AgentTokenjuiceCompression::Auto | AgentTokenjuiceCompression::Full => {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:61:        AgentTokenjuiceCompression::Light => {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:133:    profile: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:210:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:220:    profile: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:304:            AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:325:            AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:343:            AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:370:            AgentTokenjuiceCompression::Light,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs:381:            compact_output_with_policy(big.clone(), "grep", true, AgentTokenjuiceCompression::Off)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:145:**One engine, three entry points.** The loop lives in one place (the tinyagents `AgentHarness`, entered via `run_turn_via_tinyagents_shared` in `src/openhuman/tinyagents/mod.rs`) and every caller drives it: the chat turn (`harness/session/turn/core.rs` → `session/turn/graph.rs`), the channel/CLI bus turn (`harness/graph.rs`), and spawned sub-agents (`harness/subagent_runner/ops/graph.rs`). What varies per caller is supplied through the adapter seam: OpenHuman's provider wrapped as a `ChatModel` (`tinyagents/model.rs`), tools wrapped as tinyagents `Tool`s (`tinyagents/tools.rs`), an event bridge that projects harness `AgentEvent`s into `AgentProgress` + cost telemetry (`tinyagents/observability.rs`), `RunPolicy::unknown_tool` for hallucinated tool recovery, and a named middleware stack (`tinyagents/middleware.rs`) carrying the OpenHuman cross-cuts: approval/security gating (`ApprovalSecurityMiddleware`), tool policy and CLI/RPC-only denial (`ToolPolicyMiddleware`, `CliRpcOnlyMiddleware`), malformed-argument recovery (`ArgRecoveryMiddleware`), cost budget pre-checks (`CostBudgetMiddleware`), the repeated-tool-failure circuit breaker (`RepeatedToolFailureMiddleware`), and context trimming/compression. Policy stop hooks fire through `StopHookMiddleware` (`tinyagents/stop_hooks.rs`). The surviving OpenHuman-owned seams, `CheckpointStrategy` (error vs. summarize at the model-call cap) and `TurnProgress`, live in `harness/engine/`. Because all three entry points assemble the same harness, they can't drift.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:164:* **Microcompact / autocompact** - when total history is creeping toward the context window, tinyagents middleware (message trimming + the compression hooks in `tinyagents/summarize.rs`) compacts older turns into summaries before the next provider call. The compacted history keeps the system prompt and the most recent turns intact (KV-cache stability) and rewrites the middle.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effectively lossless: the agent calls `tokenjuice_retrieve` (token + optional byte/line range) to fetch the full original on demand. The same engine is exposed as a universal `compress_content(content, hint, opts)` for any large payload (file reads, web fetches), and as read-only `tokenjuice.*` debug RPCs. Configured via the `[tokenjuice]` block / `OPENHUMAN_TOKENJUICE_*` env. Agent definitions can override tool-result compression with `tokenjuice_compression = "auto" | "full" | "light" | "off"`; `auto` resolves coding-model agents (`[model] hint = "coding"`) to `light`, which disables CCR-backed lossy compression so coding agents keep raw build/test/diff/search text unless a reduction is truly lossless. Other agents default to `full`. The ML (Kompress) path runs as a `kompress` backend of the shared [`runtime_python_server`](../../../src/openhuman/runtime_python_server/) (torch + ModernBERT pip-installed at runtime), gated by the `ml_compression_enabled` flag and degrading gracefully to a native compressor when the Python runtime is unavailable.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/README.md:21:- [Conversation Compression Plan](conversation-compression-plan.md)
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/README.md:33:5. Add OpenHuman conversation-level compression as a separate adapter layer.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/README.md:44:- No compression-percentage claims should be made until fixture benchmarks
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:16:- `cargo bench` — criterion benchmarks (`benches/compression.rs`). For a
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:18:  `cargo bench --bench compression -- --warm-up-time 0.3 --measurement-time 0.5 --sample-size 10`.
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:33:benches/compression.rs            criterion throughput benches (harness=false)
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:74:`benches/compression.rs` measures **throughput of hot paths** on
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:76:decline, rule reduction). It is NOT a compression-ratio benchmark — ratio and
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:121:- No compression-percentage claims anywhere until a fixture-backed benchmark
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:5:This spec documents the compression algorithms and product strategies found in
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:25:- signal-preserving log compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:26:- optional tree-sitter code compression behind a feature boundary
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:27:- optional ML text compression through the existing OpenHuman runtime boundary
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:35:Headroom's Rust core defines a two-mechanism compression pipeline:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:52:- record applied steps and cache keys in the compression report
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:156:compression is disabled or unavailable.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:174:Headroom separates ordinary diff compression from a dedicated offload that
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:193:- signal compression is an offload because it drops low-priority lines and needs
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:201:Headroom protects workflow tags before ML compression:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:205:- replace custom-tag spans with placeholders before ML compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:206:- restore exact tags after compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:212:compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:221:- compression happens only in the live zone after the frozen prefix
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:229:from Headroom because it prevents compression from invalidating prompt caches or
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:248:## Compression Policy
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:250:Headroom centralizes compression policy by auth mode and request economics:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:259:TinyJuice can generalize this as a `CompressionPolicy` independent of auth mode.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:31:- protect custom workflow tags before ML compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:54:not be folded into `compress_content` or made mandatory for core compression.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:92:TinyJuice already has tree-sitter-backed code compression behind a feature. The
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:98:This is not text compression. It reduces retry loops:
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:134:Savings accounting is infrastructure, not compression:
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:143:## TurboQuant Vector Compression
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:145:TurboQuant-style vector compression applies to embedding/index storage:
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:151:This is not prompt compression. It should be deferred or isolated behind a
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:9:- **TinyJuice** (this repo): Rust compression engine crate. Core work happens
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:23:3. `plan/pipeline-and-ccr-plan.md`, `plan/conversation-compression-plan.md`,
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:69:- No compression-percentage claims anywhere until fixture benchmarks exist in
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:107:- `AgentTokenjuiceCompression::Auto` is resolved host-side
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:113:- `ml_compression_enabled` (Kompress bridge) stays default-off until tag
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/architecture.md:9:compression is unsafe or not smaller, and stores exact originals in CCR when a
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:28:│ • TokenJuice compression │
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:57:9. **Compress**. Tool output and large source data go through [TokenJuice](../../features/token-compression.md) before entering LLM context.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Development.md:28:- Hot-path performance benches live in `benches/compression.rs`.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Development.md:63:- compression percentage claims without benchmark fixtures
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:45:core (`hint:reasoning`, spawns worker sub-agents), 20:1 compression, the
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:17:    profile: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:70:Plain text compression is host-provided:
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:101:`OpenHumanCompressionContext` currently carries:
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/_Sidebar.md:3:Token compression for agent context.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Analytics-Interface.md:4:compression run metadata.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compress.rs:1://! Universal content-aware compression entry point.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compress.rs:28:/// compressor, or compression wouldn't shrink it.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:30:- Do not claim compression percentages without benchmark fixtures.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:110:benches/compression.rs      hot-path throughput benches
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:126:- "semantic compression is always safe"
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/ml_text.rs:4://! compression needs a learned model (Headroom uses ModernBERT token
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/ml_text.rs:7://! `tokenjuice.ml_compression_enabled` config flag.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:3:**TinyJuice is a Rust token-compression engine for agent context:** a small,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:26:3. decline when compression would be unsafe, too small, disabled, or not smaller
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:47:   `Compressor` trait, `CompressionInput`, `CompressionOutput`, and
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:48:   `CompressionConfig` surface.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:50:   UI for metadata-only compression run records.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:58:- Use `AgentTokenjuiceCompression::Light` when exact coding output matters more
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:53:Use this when you need a stable compression strategy boundary and report shape:
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:56:use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:60:    let input = CompressionInput::new("Keep this text unchanged for now.");
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:61:    let output = compressor.compress(input, &CompressionConfig::default())?;
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:102:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:111:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:184:  compression behavior.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/diff.rs:39:/// work with or compression wouldn't shrink it.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Capabilities.md:43:| `light` | Disables CCR-backed lossy compression and ML plain-text compression |
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:491:        if let Some(flag) = env.get("OPENHUMAN_TOKENJUICE_ML_COMPRESSION_ENABLED") {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:492:            if let Some(v) = parse_env_bool("OPENHUMAN_TOKENJUICE_ML_COMPRESSION_ENABLED", &flag) {
<OPENHUMAN_ROOT>/src/openhuman/config/schema/load/env_overlay.rs:493:                self.tokenjuice.ml_compression_enabled = v;
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Architecture.md:3:TinyJuice is a **token-compression boundary for Rust agent hosts**. It keeps the
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Architecture.md:113:- `AgentTokenjuiceCompression`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Architecture.md:114:- optional ML compression callback
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/log.rs:118:/// Signal-based log compression for non-command blobs detected as logs.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:17:use tinyjuice::{CompressionConfig, CompressionInput, Compressor, PassthroughCompressor};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:21:    let input = CompressionInput::new("Keep this text unchanged for now.");
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:22:    let output = compressor.compress(input, &CompressionConfig::default())?;
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:33:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:41:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/gitbooks/developing/architecture.md:250:| Sessions           | JSONL transcripts with compaction and tool compression |
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:1://! Pluggable token compression for OpenHuman and other Rust hosts.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:3://! TinyJuice owns the reusable TokenJuice compression engine. Hosts provide
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:28:    CompressionInput, CompressionOutput, CompressionReport, Compressor, PassthroughCompressor,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:31:pub use config::CompressionConfig;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:41:    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:146:/// AST-aware code compression via tree-sitter (Rust/TS/JS/Python). Keeps full
<OPENHUMAN_ROOT>/Cargo.lock:261:name = "async-compression"
<OPENHUMAN_ROOT>/Cargo.lock:266: "compression-codecs",
<OPENHUMAN_ROOT>/Cargo.lock:267: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:279: "async-compression",
<OPENHUMAN_ROOT>/Cargo.lock:1042:name = "compression-codecs"
<OPENHUMAN_ROOT>/Cargo.lock:1047: "compression-core",
<OPENHUMAN_ROOT>/Cargo.lock:1052:name = "compression-core"
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:5://! dry-run a compression and show the marker/stats), inspect CCR occupancy, and
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:284:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinycortex/README.md:17:The human brain is a master at compression. It doesn't try to remember every passing detail; instead it aggressively prunes noise to keep a sharp, focused, easily accessible recall of what truly matters. Traditional AI memory systems do the opposite — they try to remember _everything_ and retrieve whatever is _similar_. But similar doesn't mean important. The result? Your AI drowns in stale, irrelevant context that degrades every response.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:4://! compression needs a learned model (ModernBERT token/sentence salience). That
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:9://! Opt-in at runtime via `config.tokenjuice.ml_compression_enabled` (default
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:13://! too large — the agent loop never fails because ML compression is missing.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:24:/// `ml_compression_enabled` on from Settings — is picked up without a restart.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:52:    if !tj.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/README.md:30:| `image_processing.rs` | PNG→resized JPEG compression for the vision LLM (`compress_screenshot`, defaults 1024px / quality 72). |
<OPENHUMAN_ROOT>/vendor/tinycortex/docs/openhuman-memory-engine-spec.md:487:sections so they survive compression. The current TinyCortex module provides
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:1://! OpenHuman adapter for the vendored TinyJuice compression engine.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:27:/// Note: toggling `ml_compression_enabled` and the live compressor/CCR flags
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:38:        ml_text_enabled: tj.ml_compression_enabled,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:101:    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:1://! Image compression and resizing for screenshot intelligence.
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:152:    // ── Basic compression ───────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/image_processing.rs:344:    // ── Multicolored image (more realistic compression ratio) ───────────
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/README.md:3:The reusable compression engine now lives in the vendored `tinyjuice` crate at
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:26:    pub ml_compression_enabled: Option<bool>,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:70:        if let Some(v) = self.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:71:            cfg.ml_compression_enabled = v;
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:302:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:268:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/processing_worker.rs:232:    // ── Image compression (always runs — used by vision LLM and/or storage) ──
<OPENHUMAN_ROOT>/src/openhuman/screen_intelligence/processing_worker.rs:234:        .map_err(|e| format!("image compression failed: {e}"))?;
<OPENHUMAN_ROOT>/vendor/tinycortex/docs/openhuman-memory/agent-tool-goals-memory.md:159:- Tool memory rules must survive prompt compression through critical/high prompt
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:7://! | Compression + image processing      | ✅        | ✅           |
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:403:/// Compression pipeline handles various image sizes without panicking.
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:405:fn compression_handles_various_sizes() {
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:519:/// Verify that compression produces significant savings on realistic images.
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:521:fn compression_savings_on_realistic_screenshot() {
<OPENHUMAN_ROOT>/tests/screen_intelligence_vision_e2e.rs:541:        "compression ratio should be under 50%, got {:.1}%",
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:192:| `Critical` | `critical` | Pinned into the (compression-resistant) system prompt |
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:196:survives mid-session context compression.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:238:in the **system prompt** specifically because mid-session compression rewrites
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/goals-and-tool-memory.md:268:compression rewrites the rolling chat buffer but never the frozen system prompt,
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/README.md:61:| See how memories are compressed into a tree | [Memory Tree & Compression](memory-tree.md) |
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:5:# Memory Tree & Compression
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:7:The summary tree is TinyCortex's compression mechanism. Above the raw chunk leaves, the engine folds material into a tree of immutable summary nodes: many leaves seal into one L1 summary, many L1 summaries seal into one L2 summary, and so on. This keeps recall over long histories cheap — a read can answer from a handful of high-level summaries instead of scanning thousands of leaves — while the original markdown content files remain the authoritative source of truth.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:9:## The compression pipeline
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:11:Think of the tree as a decay/compression funnel. Fresh material arrives as leaves and accumulates in an unsealed frontier buffer. Once a buffer crosses its gate, the engine **seals** that bucket into a single, immutable summary one level up, then cascades: that new summary becomes a leaf for the next level, which may itself seal, and so on. The detail "decays" upward into progressively terser summaries, but nothing is destroyed — the leaves remain on disk.
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/memory-tree.md:20:![Compression pipeline](.gitbook/assets/compression-pipeline@2x.png)
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/how-memory-works.md:23:## Compression, Not Accumulation → Noise Pruning
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/how-memory-works.md:43:| Compression of experiences       | Noise pruning and memory decay           |
<OPENHUMAN_ROOT>/vendor/tinycortex/gitbooks/SUMMARY.md:17:* [Memory Tree & Compression](memory-tree.md)
<OPENHUMAN_ROOT>/src/openhuman/learning/extract/heuristics.rs:513:            "emitted set must record the compression emission"
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:429:    use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:792:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:793:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:804:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:805:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:816:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:817:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:45:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:416:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/mod.rs:33://!   system prompt so they survive mid-session compression.
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:180:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:1603:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/render.rs:5://! Mid-session compression rewrites the rolling chat buffer but never the
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/render.rs:8://! be **compression-resistant** therefore has to live in the system prompt.
<OPENHUMAN_ROOT>/src/openhuman/memory/schemas/tool_memory.rs:68:                `priority='critical'` for safety-critical rules that must survive context compression.",
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:372:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:13://!   therefore not subject to mid-session context compression.
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:37:    /// Safety-critical rule — pinned into the (compression-resistant)
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:51:    /// prefetched at session start, so they survive context compression).
<OPENHUMAN_ROOT>/vendor/tinycortex/src/memory/tool_memory/types.rs:93:    /// Criticality level for retrieval and compression behaviour.
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:329:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/about_app/catalog_data.rs:405:            compression. Critical-priority rules (e.g. 'never email Sarah') are pinned into the \
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:88:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:306:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/context/manager_tests.rs:4://! (`ContextCompressionMiddleware` + `tinyagents::summarize::ProviderModelSummarizer`),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:5://! startup. The actual compression runs inside the shared `server.py` and is
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:108:    if !config.tokenjuice.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:109:        bail!("tokenjuice.ml_compression_enabled is false");
<OPENHUMAN_ROOT>/src/openhuman/context/manager.rs:16://!    tinyagents graph (`ContextCompressionMiddleware` +
<OPENHUMAN_ROOT>/src/openhuman/context/manager.rs:61:    /// now runs in the tinyagents graph (`ContextCompressionMiddleware` +
