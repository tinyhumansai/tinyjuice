[search: 500 match(es) across 113 file(s) · top 5-12 per file (adaptive) · full set via retrieve footer]
<OPENHUMAN_ROOT>/Cargo.toml:51:# TinyJuice — host-agnostic TokenJuice compression engine. OpenHuman keeps
<OPENHUMAN_ROOT>/Cargo.toml:52:# config/RPC/tool/runtime adapters in `src/openhuman/tokenjuice/` and patches
<OPENHUMAN_ROOT>/Cargo.toml:79:# TokenJuice code compressor — AST-aware signature extraction. Optional (C build)
<OPENHUMAN_ROOT>/Cargo.toml:80:# behind the default `tokenjuice-treesitter` feature; disabling it falls back to
<OPENHUMAN_ROOT>/Cargo.toml:81:# the language-agnostic brace-depth heuristic. See src/openhuman/tokenjuice/compressors/code.rs.
<OPENHUMAN_ROOT>/Cargo.toml:326:default = ["tokenjuice-treesitter"]
<OPENHUMAN_ROOT>/Cargo.toml:329:tokenjuice-treesitter = [
<OPENHUMAN_ROOT>/Cargo.toml:330:    "tinyjuice/tokenjuice-treesitter",
<OPENHUMAN_ROOT>/README.md:72:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: tool output compressed before it hits the model: same information, up to 80% fewer tokens. A brain this big would be unaffordable without it.
<OPENHUMAN_ROOT>/README.md:145:| **Cost**               | ⚠️ Sub + add-ons  | ⚠️ BYO models     | ⚠️ BYO models     | ✅ One sub + TokenJuice                                                                                  |
<OPENHUMAN_ROOT>/docs/README.ko.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: 도구 출력은 모델에 닿기 전에 압축되어, 동일한 정보가 최대 80% 적은 토큰으로 전달됩니다. 이것 없이는 이만큼 큰 두뇌를 감당할 수 없을 것입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:143:| **비용**           | ⚠️ 구독 + 애드온  | ⚠️ 모델 직접 제공 | ⚠️ 모델 직접 제공 | ✅ 단일 구독 + TokenJuice                                                                            |
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ツール出力はモデルに届く前に圧縮され、同じ情報を最大 80% 少ないトークンで扱えます。これがなければ、これほど大きな脳は維持できません。
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:143:| **コスト**                 | ⚠️ サブスク + アドオン | ⚠️ モデル持ち込み   | ⚠️ モデル持ち込み   | ✅ 1 つのサブスク + TokenJuice                                                                                     |
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**：工具输出在触达模型之前先被压缩：信息不变，token 最多减少 80%。没有它，这么大的一颗大脑将贵得用不起。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:143:| **成本**       | ⚠️ 订阅 + 附加项 | ⚠️ 自带模型 | ⚠️ 自带模型  | ✅ 单一订阅 + TokenJuice                                                                    |
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:166:1. Delete transitional shims (`ToolAdapter` test-only wrapper, `subagent_graph.rs` no-op skeleton once the graph path is the real one, `retrieve_tool_output` vs tokenjuice duplication).
<OPENHUMAN_ROOT>/docs/README.de.md:70:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: Tool-Ausgaben werden komprimiert, bevor sie das Modell erreichen: dieselbe Information, bis zu 80% weniger Tokens. Ein so großes Gehirn wäre ohne es unbezahlbar.
<OPENHUMAN_ROOT>/docs/README.de.md:143:| **Kosten**             | ⚠️ Abo + Zusatzkosten | ⚠️ BYO-Modelle     | ⚠️ BYO-Modelle     | ✅ Ein Abo + TokenJuice                                                                                  |
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:23:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_archivist_debug_round21_raw_coverage_e2e.rs:262:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:84:- **[TokenJuice](https://tinyhumans.gitbook.io/openhuman/features/token-compression)**: ٹول آؤٹ پٹ ماڈل تک پہنچنے سے پہلے کمپریس ہوتا ہے: وہی معلومات، 80% تک کم ٹوکنز۔ اتنا بڑا دماغ اس کے بغیر ناقابلِ برداشت مہنگا ہوتا۔
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:185:| **لاگت**           | ⚠️ سبسکرپشن + ایڈ آنز    | ⚠️ اپنے ماڈل        | ⚠️ اپنے ماڈل        | ✅ ایک سبسکرپشن + TokenJuice                                                                           |
<OPENHUMAN_ROOT>/src/core/all.rs:296:    // TokenJuice content-router debug controllers (detect / compress / cache_stats / retrieve)
<OPENHUMAN_ROOT>/src/core/all.rs:297:    controllers.extend(crate::openhuman::tokenjuice::all_tokenjuice_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:471:    // TokenJuice content-router debug controllers
<OPENHUMAN_ROOT>/src/core/all.rs:472:    schemas.extend(crate::openhuman::tokenjuice::all_tokenjuice_controller_schemas());
<OPENHUMAN_ROOT>/gitbooks/README.md:23:* **An agent built for big data.** [Smart token compression (TokenJuice)](features/token-compression.md) compacts verbose tool output before it ever enters the model's context, so sweeping through your last six months of email costs single-digit dollars. [Automatic model routing](features/model-routing/) sends each task to the right model - `hint:reasoning` to a frontier model, `hint:fast` to a cheap one, vision to vision - all under one subscription. Optional [local AI via Ollama or LM Studio](features/model-routing/local-ai.md) keeps supported workloads on-device.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2315:        // Install the TokenJuice content-router runtime config (compressor
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2319:        crate::openhuman::tokenjuice::install_from_config(&config);
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:216:        // even after tokenjuice's generic/fallback reducer runs. The reducer
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:224:        // No HTML markup: clean_tool_output runs after tokenjuice and would
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:302:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:364:    // the oversized-result path with payloads that survive tokenjuice's
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:425:    // payload (tokenjuice-compacted to ~1200 chars) fits within the handoff
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:268:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/agent_session_turn_raw_coverage_e2e.rs:25:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_session_turn_raw_coverage_e2e.rs:1063:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:102:Because cost tracks **real token counts**, anything that shrinks the prompt directly lowers spend. OpenHuman's [TokenJuice token compression](token-compression.md) reduces the tokens sent on each call, and [model routing](model-routing/README.md) sends work to the cheapest model that can handle it. Both show up as lower bars in the dashboard and slower budget burn.
<OPENHUMAN_ROOT>/gitbooks/features/billing-and-usage.md:108:- [Token compression (TokenJuice)](token-compression.md)
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:3:  TokenJuice - a multi-stage compression router that compacts verbose tool
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:12:OpenHuman ships with **TokenJuice**, a compression router wired directly into the agent's tool-execution path. Before any tool result reaches a model, TokenJuice classifies it, routes it to a specialized compressor, optionally offloads the full original to a recoverable cache, and records how many tokens (and dollars) it saved.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:14:It began as a port of [vincentkoc/tokenjuice](https://github.com/vincentkoc/tokenjuice). That JSON rule overlay is still in here as the log/command compressor, but it has since grown into a multi-stage, content-aware pipeline.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:20:Every blob that flows through the policy-aware TokenJuice tool-output adapters
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:54:4. **Compression.** The compressor runs. If it declines or its output is no smaller than the input, TokenJuice falls back to the generic compressor or passes the original through. It never makes things bigger.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:82:Beyond the deterministic compressors, TokenJuice can route plain text through a **ModernBERT** token-salience model that scores and drops low-information spans. The TinyJuice compressor exposes the optional ML slot, and OpenHuman bridges it to Kompress in `src/openhuman/tokenjuice/ml/`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:84:* **Off by default.** Enable with `ml_compression_enabled = true` in `[tokenjuice]`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:93:Lossy compression would normally mean throwing data away. TokenJuice instead **offloads** the full original into the **Compress-Cache-Retrieve (CCR)** store and leaves a breadcrumb (`vendor/tinyjuice/src/cache/`).
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:96:* **On-disk tier** (optional): `<workspace>/.tokenjuice/ccr/`, enabled with `ccr_disk_enabled`, survives memory eviction. Optional TTL via `ccr_ttl_secs`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:97:* **The marker:** compacted output ends with a footer like `[compacted tool output — PARTIAL view; full original available via tokenjuice_retrieve with token "…"]` carrying the `⟦tj:<hash>⟧` token.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:106:Every compression is metered by an OpenHuman savings callback (`src/openhuman/tokenjuice/savings.rs`). TokenJuice reports events and token deltas; OpenHuman applies per-model input pricing, aggregates `total`, `by_model`, and `by_compressor`, and persists stats to `<workspace>/state/tokenjuice_savings.json`.
<OPENHUMAN_ROOT>/gitbooks/features/token-compression.md:135:* **Agent tool:** `tokenjuice_retrieve` (read-only) recovers offloaded originals.
[+10 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/token-compression.md ⟦tj:759984c17c9aa9ed53074dbce9dd586e⟧]
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:18:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_raw_coverage_e2e.rs:329:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1055:async fn json_rpc_tokenjuice_detect_and_cache_stats() {
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1064:        "openhuman.tokenjuice_detect",
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1068:    let detect_result = assert_no_jsonrpc_error(&detect, "tokenjuice_detect");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1078:        "openhuman.tokenjuice_cache_stats",
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1082:    let stats_result = assert_no_jsonrpc_error(&stats, "tokenjuice_cache_stats");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1093:async fn json_rpc_tokenjuice_settings_and_savings() {
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1098:    // settings_get: returns the [tokenjuice] block with the configurable
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1103:        "openhuman.tokenjuice_settings_get",
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1107:    let get_result = assert_no_jsonrpc_error(&get, "tokenjuice_settings_get");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1128:    let updated_result = assert_no_jsonrpc_error(&updated, "tokenjuice_settings_update");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1145:    let savings_result = assert_no_jsonrpc_error(&savings, "tokenjuice_savings_stats");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:1162:    let reset_result = assert_no_jsonrpc_error(&reset, "tokenjuice_savings_reset");
[+3 more match(es) in <OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs ⟦tj:b946ec8eb4b729bc6c66d76401dac3c0⟧]
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:88:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_approval_channels_raw_coverage_e2e.rs:306:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/features/platform.md:63:│ • Model router, TokenJuice, native tools │
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:45:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/tools_agent_credentials_state_raw_coverage_e2e.rs:416:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/tests/tokenjuice_integration.rs:1://! Integration tests for the TokenJuice module.
<OPENHUMAN_ROOT>/tests/tokenjuice_integration.rs:7:use openhuman_core::openhuman::tokenjuice::{
<OPENHUMAN_ROOT>/tests/tokenjuice_integration.rs:77:        "\ntokenjuice integration: {} passed, {} skipped, {} failed",
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/dispatch/mod.rs:128:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:172:### TokenJuice - content-aware tool-output compaction (Stage 1a)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:174:Before a fresh tool result enters history (and ahead of the byte-budget backstop), it passes through the **TokenJuice content router** in the vendored TinyJuice crate (`vendor/tinyjuice`), with OpenHuman adapters in `src/openhuman/tokenjuice/`. Inspired by Headroom, the router *detects the content kind* (JSON, code, log, search, diff, HTML, plain text) from the bytes and/or a hint derived from the tool name and arguments, then dispatches to a specialised compressor:
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:184:Every lossy compression offloads the original to the **CCR (Compress-Cache-Retrieve)** store behind a `⟦tj:<hash>⟧` marker, so compaction is effectively lossless: the agent calls `tokenjuice_retrieve` (token + optional byte/line range) to fetch the full original on demand. The same engine is exposed as a universal `compress_content(content, hint, opts)` for any large payload (file reads, web fetches), and as read-only `tokenjuice.*` debug RPCs. Configured via the `[tokenjuice]` block / `OPENHUMAN_TOKENJUICE_*` env. Agent definitions can override tool-result compression with `tokenjuice_compression = "auto" | "full" | "light" | "off"`; `auto` resolves coding-model agents (`[model] hint = "coding"`) to `light`, which disables CCR-backed lossy compression so coding agents keep raw build/test/diff/search text unless a reduction is truly lossless. Other agents default to `full`. The ML (Kompress) path runs as a `kompress` backend of the shared [`runtime_python_server`](../../../src/openhuman/runtime_python_server/) (torch + ModernBERT pip-installed at runtime), gated by the `ml_compression_enabled` flag and degrading gracefully to a native compressor when the Python runtime is unavailable.
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:180:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/inference_agent_raw_coverage_e2e.rs:1603:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:28:│ • TokenJuice compression │
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/README.md:57:9. **Compress**. Tool output and large source data go through [TokenJuice](../../features/token-compression.md) before entering LLM context.
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:24:use openhuman_core::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/tests/agent_harness_leftovers_raw_coverage_e2e.rs:372:        tokenjuice_compression: AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:2: * TokenJuice content-router client.
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:4: * Thin wrapper around the `openhuman.tokenjuice_*` JSON-RPC methods exposed by
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:5: * the Rust core (`src/openhuman/tokenjuice/schemas.rs`): read/update the
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:6: * `[tokenjuice]` settings block and read/reset compaction savings statistics.
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:10:/** The `[tokenjuice]` config block (snake_case, matching the Rust config keys). */
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:11:export interface TokenjuiceSettings {
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:32:export type TokenjuiceSettingsPatch = Partial<TokenjuiceSettings>;
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:50:export async function getTokenjuiceSettings(): Promise<TokenjuiceSettings> {
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:51:  const res = await callCoreRpc<{ settings: TokenjuiceSettings }>({
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:52:    method: 'openhuman.tokenjuice_settings_get',
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:58:export async function updateTokenjuiceSettings(
<OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts:59:  patch: TokenjuiceSettingsPatch
[+7 more match(es) in <OPENHUMAN_ROOT>/app/src/utils/tauriCommands/tokenjuice.ts ⟦tj:8da31d59bb9948408fa401f84028e22c⟧]
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:230:        // TokenJuice 2.0 content-router retrieval: fetches the original (full or
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:233:        Box::new(crate::openhuman::tokenjuice::TokenjuiceRetrieveTool::new()),
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:284:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/mod.rs:77:        for name in crate::openhuman::tokenjuice::RECOVERY_TOOL_NAMES {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:49:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:350:    /// Set the per-agent TokenJuice tool-output compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:351:    pub fn tokenjuice_compression(
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:353:        profile: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:355:        self.tokenjuice_compression = profile;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/setters.rs:527:            tokenjuice_compression: self.tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1091:        let effective_tokenjuice_compression = target_def
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1092:            .map(|def| def.effective_tokenjuice_compression())
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1093:            .unwrap_or(crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Full);
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/factory.rs:1215:            .tokenjuice_compression(effective_tokenjuice_compression);
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/builder/builder_tests.rs:19:    use crate::openhuman::tokenjuice::RETRIEVE_TOOL_NAME as RECOVERY_TOOL_NAME;
<OPENHUMAN_ROOT>/src/openhuman/tools/impl/system/retrieve_tool_output.rs:5://! stashing the original in the TokenJuice store. This tool hands the original
<OPENHUMAN_ROOT>/src/openhuman/tools/impl/system/retrieve_tool_output.rs:71:        match crate::openhuman::tokenjuice::cache::retrieve(hash) {
<OPENHUMAN_ROOT>/src/openhuman/tools/impl/system/retrieve_tool_output.rs:91:    use crate::openhuman::tokenjuice::cache::store;
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
[+10 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/tool_integration.rs ⟦tj:6b2a27a56f24649644328de121fcf50b⟧]
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/turn/core.rs:894:            tokenjuice_compaction_enabled: self.context.compaction_enabled(),
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/turn/core.rs:895:            tokenjuice_compression: self.tokenjuice_compression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/text_tests.rs:1://! Additional unit tests for the `tokenjuice::text` sub-modules.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/text_tests.rs:4://! ("TokenJuice Rust port for tool-output compaction"):
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/detect/mod.rs:1://! Content-kind detection + tool-name priors for the TokenJuice content router.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/detect/kind.rs:1://! Content-kind detection for the TokenJuice content router.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:187:    /// Per-agent TokenJuice profile for tool results entering this session's
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:189:    pub(super) tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:362:    /// Per-agent TokenJuice tool-output compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/session/types.rs:363:    pub(super) tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:14:    AgentTokenjuiceCompression, CompressOptions, ReduceOptions, ToolExecutionInput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/benches/compression.rs:95:                    AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:78:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:125:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/builtin_definitions.rs:165:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:34:  a `tokenjuice_retrieve` footer whenever data is dropped.
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:145:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:153:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:28:use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:213:    /// Per-agent TokenJuice tool-result compression profile.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:219:    pub tokenjuice_compression: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:485:    pub fn effective_tokenjuice_compression(&self) -> AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:486:        match self.tokenjuice_compression {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:487:            AgentTokenjuiceCompression::Auto => match &self.model {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:489:                    AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs:491:                _ => AgentTokenjuiceCompression::Full,
[+1 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition.rs ⟦tj:7cdaaed514cf46b5cfb658eeaa1ce5e0⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/text/ansi.rs:39:        "[tokenjuice] strip_ansi in_len={} out_len={}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/text/mod.rs:1://! Text-processing utilities for the TokenJuice engine.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compress.rs:130:        "[tokenjuice] compacted kind={} compressor={} lossy={} {}->{} bytes (~{}->{} tok)",
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:48:    agent_tokenjuice_profile,
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:56:- expose a `tokenjuice_retrieve` tool for CCR recovery
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:180:[tokenjuice]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:188:[tokenjuice.ccr]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:196:[tokenjuice.compressors]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:227:- `tokenjuice_retrieve(token)`
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs:76:                    "[tokenjuice] failed to parse builtin rule '{}': {}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs:106:            log::debug!("[tokenjuice] read_dir failed at {}: {}", dir.display(), err);
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs:119:                    "[tokenjuice] file_type failed at {}: {}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs:153:                        "[tokenjuice] read_to_string failed for {:?} rule at {}: {}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs:173:                        "[tokenjuice] failed to parse {:?} rule at {}: {}",
[+11 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader.rs ⟦tj:ce5f2d66a97c864c41be39dec9aea8e7⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:1:# TokenJuice Improvement Ingestion Spec
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:6:`vincentkoc/tokenjuice` codebase and what TinyJuice has already improved in the
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:11:- Original: `https://github.com/vincentkoc/tokenjuice`, commit
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:19:The original TokenJuice repository is a TypeScript CLI and adapter suite. It
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tokenjuice-improvement-spec.md:342:The original TokenJuice repository is MIT licensed. TinyJuice is GPL-3.0-only.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/tool_prep.rs:191:            if crate::openhuman::tokenjuice::is_recovery_tool(name) {
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/tool_prep.rs:277:    use crate::openhuman::tokenjuice::LEGACY_RETRIEVE_TOOL_NAME as RECOVERY_TOOL_NAME;
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/README.md:17:- [TokenJuice improvement ingestion](tokenjuice-improvement-spec.md)
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/compiler.rs:38:                "[tokenjuice] rule compiler: invalid regex '{}' (flags={:?}): {}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/compiler.rs:57:        "[tokenjuice] compiling rule '{}' from {:?} path={}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/compiler.rs:310:            "/home/user/.config/tokenjuice/rules/test.json".to_owned(),
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/compiler.rs:315:            "/home/user/.config/tokenjuice/rules/test.json"
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops_tests.rs:68:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader_tests.rs:265:    // Should end in .config/tokenjuice/rules
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader_tests.rs:266:    assert!(path.to_string_lossy().contains("tokenjuice"));
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/loader_tests.rs:272:    assert!(path.to_string_lossy().contains(".tokenjuice"));
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/hermes-compression-algorithms-spec.md:68:- log signal preservation and TokenJuice rule support
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/builtin_tests.rs:26:                eprintln!("[tokenjuice/builtin] PARSE FAIL '{}': {}", id, e);
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/builtin_tests.rs:35:                "[tokenjuice/builtin] DUPLICATE id '{}' in entries: {:?}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/rules/builtin_tests.rs:88:            "[tokenjuice/builtin] {} compile issues (lenient — not failing test):",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/ml_text.rs:7://! `tokenjuice.ml_compression_enabled` config flag.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/ml_text.rs:41:                log::debug!("[tokenjuice][ml] unavailable, falling back: {e:#}");
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/web-extract-truncate-store-spec.md:90:[tokenjuice.web_extract]
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/web-extract-truncate-store-spec.md:172:`tokenjuice-improvement-spec.md` rather than writing arbitrary filesystem paths
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/web-extract-truncate-store-spec.md:268:- The host exposes retrieval through `tokenjuice_retrieve`, `read_file`, or a
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/search.rs:147:        "[tokenjuice][search] {} matches -> {} bytes (from {} bytes)",
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/runner.rs:994:                    // Agent-level TokenJuice profile → sub-agent context middleware
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/runner.rs:996:                    definition.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/runner.rs:1026:                    tokenjuice_compression: definition.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/diff.rs:113:        "[tokenjuice][diff] {} -> {} bytes ({} input lines)",
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/architecture.md:6:2. the TokenJuice content router for real tool-output compaction
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/json.rs:138:        "[tokenjuice][json] {} rows × {} cols, lossy={} ({} -> {} bytes)",
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:40:use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:84:        tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:111:            tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:169:    // Agent-level TokenJuice profile (`definition.effective_tokenjuice_compression()`,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:173:    tokenjuice_compression: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:246:    let context_mw = build_subagent_context_mw(tokenjuice_compression).await;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:548:    tokenjuice_compression: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:553:    mw.tokenjuice_compression = tokenjuice_compression;
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:573:                compression = ?mw.tokenjuice_compression,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:1093:            AgentTokenjuiceCompression::Off,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:1182:            AgentTokenjuiceCompression::Off,
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs:1329:            AgentTokenjuiceCompression::Off,
[+9 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/ops/graph.rs ⟦tj:491ab8260b1646392e5f91145e2db076⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/log.rs:7://!   is TokenJuice's original behaviour — git/cargo/npm/docker-aware rules with
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/log.rs:109:        "[tokenjuice][log] command rule={} kind={} {} -> {} bytes",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/log.rs:217:        "[tokenjuice][log] signal kept {} of {} line(s)",
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Rule-Engine.md:21:2. user rules from `~/.config/tokenjuice/rules/`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Rule-Engine.md:22:3. project rules from `<cwd>/.tokenjuice/rules/`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Rule-Engine.md:143:- Prefer a project rule in `.tokenjuice/rules/` when behavior is specific to a
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Rule-Engine.md:145:- Prefer a user rule in `~/.config/tokenjuice/rules/` for local operator
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/html.rs:70:        "[tokenjuice][html] {} -> {} bytes",
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:17:    profile: AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:114:- canonical name: `tokenjuice_retrieve`
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:44:        #[cfg(feature = "tokenjuice-treesitter")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:108:        "[tokenjuice][code] heuristic {} -> {} bytes ({} lines)",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:150:#[cfg(feature = "tokenjuice-treesitter")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:236:            "[tokenjuice][code] tree-sitter ext={} {} -> {} bytes",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:302:    #[cfg(feature = "tokenjuice-treesitter")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/compressors/code.rs:329:    #[cfg(feature = "tokenjuice-treesitter")]
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:3://! TinyJuice owns the reusable TokenJuice compression engine. Hosts provide
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/lib.rs:41:    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/CCR-Recovery.md:22:router appends footer with tokenjuice_retrieve token
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/CCR-Recovery.md:40:available by calling tokenjuice_retrieve with token "..."]
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/CCR-Recovery.md:51:- `tokenjuice_retrieve`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/CCR-Recovery.md:115:2. Call `tokenjuice_retrieve` with the token when exact omitted data matters.
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/subagent_runner/handoff.rs:142:    // survive tokenjuice's compaction cap. Never consulted in production
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:29:- Do not compact output from `tokenjuice_retrieve` or `retrieve_tool_output`.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:52:   - user: `~/.config/tokenjuice/rules/`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:53:   - project: `.tokenjuice/rules/`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Agent-Guide.md:119:- "exact original via `tokenjuice_retrieve`"
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce.rs:252:/// convention (see `tokenjuice::text::ansi`).
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce.rs:643:        "[tokenjuice] apply_rule '{}': {} lines → head={} tail={} failure={}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce.rs:767:        "[tokenjuice] reduce_execution: tool='{}' raw_chars={} family='{}'",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce.rs:845:        "[tokenjuice] reduce_execution complete: rule='{}' raw={} reduced={} ratio={:.2}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:58:- Use `AgentTokenjuiceCompression::Light` when exact coding output matters more
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Home.md:60:- Never re-compact `tokenjuice_retrieve` output.
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:20:The default feature set enables the `tokenjuice-treesitter` feature, which
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:102:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:111:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Quick-Start.md:148:When a compressed output includes a footer with `tokenjuice_retrieve`, the
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Security-and-Privacy.md:47:`tokenjuice_retrieve` returns exact original content. It should follow the host
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Capabilities.md:85:5. Ensure `tokenjuice_retrieve` is always available if the model can see CCR
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Architecture.md:113:- `AgentTokenjuiceCompression`
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:33:use tinyjuice::{AgentTokenjuiceCompression, compact_tool_output_with_policy};
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/Examples.md:41:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:13:  `[patch.crates-io]`) and exposes it as the "TokenJuice" feature through the
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:14:  adapter at `src/openhuman/tokenjuice/`.
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:61:  consumes the crate via `src/openhuman/tokenjuice/` re-exports and
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:76:- Recovery-tool output (`tokenjuice_retrieve`, legacy `retrieve_tool_output`)
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:104:  HandoffMiddleware (sees raw first) → PayloadSummarizer → TokenJuice →
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:105:  per-tool char cap → byte-cap backstop. TokenJuice may receive summarizer
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:107:- `AgentTokenjuiceCompression::Auto` is resolved host-side
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/agent_graph.rs:65:    /// Agent-level TokenJuice compaction profile
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/agent_graph.rs:66:    /// (`definition.effective_tokenjuice_compression()`), threaded into the
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/agent_graph.rs:69:    pub tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/marker.rs:11:pub const RETRIEVE_TOOL_NAME: &str = "tokenjuice_retrieve";
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/marker.rs:79:    // Legacy: retrieve_tool_output("HASH") or tokenjuice_retrieve("HASH")
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/marker.rs:82:        "tokenjuice_retrieve(\"",
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:14:use tinyjuice::types::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:43:        AgentTokenjuiceCompression::Off,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:60:            AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:77:        AgentTokenjuiceCompression::Light,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:102:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/tests/e2e_tool_output.rs:138:        AgentTokenjuiceCompression::Full,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:6://! `tokenjuice_retrieve` tool to get the original back on demand — so even
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:32:/// Tunable limits, settable once at startup from the `[tokenjuice]` config.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:68:/// Enable the on-disk tier rooted at `root` (e.g. `<workspace>/.tokenjuice/ccr`).
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:74:        log::warn!("[tokenjuice][ccr] could not create disk tier at {root:?}");
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:197:            Err(e) => log::debug!("[tokenjuice][ccr] disk write failed for {hash}: {e}"),
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/cache/store.rs:240:        log::debug!("[tokenjuice][ccr] disk entry expired for {hash}");
<OPENHUMAN_ROOT>/src/openhuman/agent/harness/definition_tests.rs:29:        tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:151:            "[tokenjuice] forced classification: rule='{}' family='{}'",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:170:            "[tokenjuice] no rule matched tool='{}' argv={:?} — using generic fallback",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/classify.rs:199:        "[tokenjuice] classified tool='{}' → rule='{}' family='{}' confidence={}",
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:1://! Core type definitions for the TokenJuice reduction engine.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:304:/// Per-agent TokenJuice profile.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:306:/// `Auto` is resolved by the agent definition layer. TokenJuice itself treats
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:307:/// `Auto` like `Full` so non-agent callers keep the global `[tokenjuice]`
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:311:pub enum AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:316:    /// Use the process-global TokenJuice configuration unchanged.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:320:    /// Bypass TokenJuice for this agent's tool results.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:324:impl AgentTokenjuiceCompression {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:336:// Content Router (TokenJuice 2.0) — content-kind detection + compressor dispatch
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/types.rs:511:/// `[tokenjuice]` config block. TokenJuice stays decoupled from the config
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:12:`[patch.crates-io]`) and ships it as the "TokenJuice" product feature. The
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:15:- Adapter module: `src/openhuman/tokenjuice/` re-exports the crate API and owns
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:19:- Config: `[tokenjuice]` block in `src/openhuman/config/schema/tokenjuice.rs`
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:20:  plus per-agent `AgentTokenjuiceCompression` overrides and
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:21:  `tokenjuice/config_patch.rs` partial updates.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:25:  (2) TokenJuice compaction, (3) per-tool char cap, (4) shared 16 KiB byte-cap
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:32:  `tokenjuice_retrieve` (`src/openhuman/tokenjuice/tools.rs`) are registered.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:52:   TokenJuice and keep the head, so an output that is compacted but still over
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:71:- `AgentTokenjuiceCompression` profiles
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:72:- `tokenjuice_retrieve` marker and recovery-tool bypass constants
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:84:- exposing `tokenjuice_retrieve(token, range?)`
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:121:  first, then summarizer, then TokenJuice, then caps. TokenJuice must tolerate
[+3 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md ⟦tj:fde0141f1dd1bcb6c2a07e450ef2d5c7⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/rule-cli-and-safety-parity-plan.md:5:Bring TinyJuice to TokenJuice product parity where it matters for safe
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/rule-cli-and-safety-parity-plan.md:14:- The reference spec says upstream TokenJuice has 130 non-fixture rules.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/rule-cli-and-safety-parity-plan.md:23:- Handle licensing in the sync: upstream TokenJuice is MIT while TinyJuice is
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:17:  in the crate; OpenHuman consumes them through `src/openhuman/tokenjuice/`
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:33:| savings-accounting | Port | P1 | Core + `tokenjuice/savings.rs` |
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:43:| tokenjuice reduce-json protocol + CLI + installers | Defer for OpenHuman | — | OpenHuman uses the crate directly |
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:68:- `src/openhuman/tokenjuice/mod.rs` — re-export `CcrStore`, `PipelineReport`;
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:71:- `src/openhuman/tokenjuice/schemas.rs` — surface `PipelineReport` fields
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:84:`tokenjuice-improvement-spec.md` (P0 Safety Policy Port). Prerequisite: the
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:111:- `src/openhuman/config/schema/tokenjuice.rs` — add `shell_policy` key
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:113:- `src/openhuman/tokenjuice/config_patch.rs` — partial-update support for the
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:183:(`src/openhuman/tokenjuice/savings.rs`, dashboard-facing) and the README
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:196:- `src/openhuman/tokenjuice/savings.rs` — persist the new fields; feed
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:199:- Dashboard schema (`src/openhuman/tokenjuice/schemas.rs`) — expose the class
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md:256:compression behind `tokenjuice-treesitter` (OpenHuman enables it by default in
[+6 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-algorithm-port-plan.md ⟦tj:dd5d593a76a3f35b3dd07ee84125f9f8⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:10:- The newer TokenJuice-style engine, exported from `lib.rs`, centered on
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/current-state-and-critique.md:101:backstop after the TokenJuice stage) keep the head and cut the tail, which
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:3:## TokenJuice
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:5:TokenJuice contributes the deterministic command-output reducer model:
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:16:important remaining TokenJuice work is parity hardening: missing rules, fixture
<OPENHUMAN_ROOT>/vendor/tinyjuice/Cargo.toml:53:default = ["tokenjuice-treesitter"]
<OPENHUMAN_ROOT>/vendor/tinyjuice/Cargo.toml:54:tokenjuice-treesitter = [
<OPENHUMAN_ROOT>/src/openhuman/agent/library/ops.rs:151:            tokenjuice_compression: crate::openhuman::tokenjuice::AgentTokenjuiceCompression::Auto,
<OPENHUMAN_ROOT>/AGENTS.md:178:Domains: `about_app`, `accessibility`, `agent`, `app_state`, `approval`, `autocomplete`, `billing`, `channels`, `composio`, `config`, `context`, `cost`, `credentials`, `cron`, `doctor`, `embeddings`, `encryption`, `health`, `heartbeat`, `integrations`, `learning`, `local_ai`, `meet`, `meet_agent`, `memory`, `migration`, `node_runtime`, `notifications`, `overlay`, `people`, `prompt_injection`, `provider_surfaces`, `providers`, `redirect_links`, `referral`, `routing`, `scheduler_gate`, `screen_intelligence`, `security`, `service`, `skills`, `socket`, `subconscious`, `team`, `text_input`, `threads`, `tokenjuice`, `tool_timeout`, `tools`, `tree_summarizer`, `update`, `voice`, `wallet`, `webhooks`, `webview_accounts`, `webview_apis`, `webview_notifications`.
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:429:    use crate::openhuman::tokenjuice::AgentTokenjuiceCompression;
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:792:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:793:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:804:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:805:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:816:            def.effective_tokenjuice_compression(),
<OPENHUMAN_ROOT>/src/openhuman/agent_registry/agents/loader.rs:817:            AgentTokenjuiceCompression::Light
<OPENHUMAN_ROOT>/vendor/tinyagents/docs/sdk-gaps.md:17:  `src/openhuman/tokenjuice/*`.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:1://! Read-only RPC controller for inspecting the TokenJuice content router.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:73:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:105:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:130:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:142:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:159:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:161:            description: "Get the current [tokenjuice] configuration block.",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:166:                comment: "The tokenjuice config (router/CCR/compressor toggles + ML fields).",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:171:            namespace: "tokenjuice",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:173:            description: "Patch the [tokenjuice] config (any subset of fields), persist, and apply live.",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs:177:                comment: "Partial tokenjuice settings; only present fields are changed.",
[+13 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tokenjuice/schemas.rs ⟦tj:bdf09de6bbc7d40976efcae70480883a⟧]
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:10://! `workspace_dir/state/tokenjuice_savings.json` so the dashboard survives
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:20:use crate::openhuman::tokenjuice::types::{CompressorKind, ContentKind};
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:130:/// snapshot. Called once at startup from [`crate::openhuman::tokenjuice::install_config`].
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:132:    let path = workspace_dir.join("state").join("tokenjuice_savings.json");
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:192:                log::debug!("[tokenjuice][savings] snapshot write failed: {e}");
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/savings.rs:195:        Err(e) => log::debug!("[tokenjuice][savings] snapshot serialize failed: {e}"),
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:1://! TokenJuice ML plain-text compressor ("Kompress").
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:9://! Opt-in at runtime via `config.tokenjuice.ml_compression_enabled` (default
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:20:use crate::openhuman::tokenjuice::types::CompressOptions;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:31:/// `tokenjuice.settings_update` so the runtime sees current values.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:48:            None => anyhow::bail!("tokenjuice ml not configured"),
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:51:    let tj = &config.tokenjuice;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/ml/mod.rs:65:        "[tokenjuice::ml] kompress {} -> {} chars",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:3://! TinyJuice owns the host-agnostic TokenJuice engine: detection, compressors,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:22:/// Install the full TokenJuice runtime from a [`Config`] in one call: router /
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:31:    let tj = &config.tokenjuice;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:45:        .then(|| config.workspace_dir.join(".tokenjuice").join("ccr"));
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:75:/// All read-only TokenJuice debug controllers (detect / compress / cache_stats
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:77:pub fn all_tokenjuice_registered_controllers() -> Vec<crate::core::all::RegisteredController> {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:81:/// Declared schemas for the TokenJuice debug controllers.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:82:pub fn all_tokenjuice_controller_schemas() -> Vec<crate::core::ControllerSchema> {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:99:pub use tools::TokenjuiceRetrieveTool;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/mod.rs:101:    AgentTokenjuiceCompression, CompactResult, CompressInput, CompressOptions, CompressOutput,
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/README.md:1:# OpenHuman TokenJuice Adapter
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/README.md:13:| `config_patch.rs` | Partial update shape for the `[tokenjuice]` config block. |
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/README.md:14:| `tools.rs` | OpenHuman agent tool implementation for `tokenjuice_retrieve`. |
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:1://! Agent tool: `tokenjuice_retrieve` — fetch the original of a compacted result.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:5://! ([`crate::openhuman::tokenjuice::cache::store`]). This tool hands the
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:14:use crate::openhuman::tokenjuice::cache::{self, store::RangeUnit};
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:17:pub struct TokenjuiceRetrieveTool;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:19:impl TokenjuiceRetrieveTool {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:25:impl Default for TokenjuiceRetrieveTool {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:32:impl Tool for TokenjuiceRetrieveTool {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:83:                "tokenjuice_retrieve: missing required 'token' argument".to_string(),
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:98:                        "[tokenjuice][ccr] retrieved range token={token} {start}..{end} {} bytes",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:110:                    "[tokenjuice][ccr] retrieved token={token} bytes={}",
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:122:        "tokenjuice_retrieve: no cached original for token '{token}' \
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs:130:    use crate::openhuman::tokenjuice::cache::store;
[+4 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tokenjuice/tools.rs ⟦tj:ddbbafa6ef89474f4bf38e57757cf9c5⟧]
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:1://! Partial-update patch for the `[tokenjuice]` config block, used by the
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:2://! `tokenjuice.settings_update` RPC. Only fields present in the JSON are
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:7:use crate::openhuman::config::TokenjuiceConfig;
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:9:// Field names are snake_case to match the `[tokenjuice]` config keys that
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:10:// `tokenjuice.settings_get` returns, so the UI reads and writes the same shape.
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:13:pub struct TokenjuiceSettingsPatch {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:34:impl TokenjuiceSettingsPatch {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:36:    pub fn apply(&self, cfg: &mut TokenjuiceConfig) {
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:103:        let mut cfg = TokenjuiceConfig::default();
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:104:        let patch: TokenjuiceSettingsPatch =
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:116:        let mut cfg = TokenjuiceConfig::default();
<OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs:117:        let set: TokenjuiceSettingsPatch =
[+1 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tokenjuice/config_patch.rs ⟦tj:4abed0c92ec389d2d5581b16668f3b27⟧]
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:296:            let idle_timeout = Duration::from_secs(config.tokenjuice.ml_sidecar_idle_timeout_secs);
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:435:        config.tokenjuice.ml_model_id.clone(),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:439:        config.tokenjuice.ml_device.clone(),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:443:        config.tokenjuice.ml_target_ratio.to_string(),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:447:        config.tokenjuice.ml_max_input_chars.to_string(),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:568:                "target_ratio": config.tokenjuice.ml_target_ratio,
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/server.rs:569:                "max_input_chars": config.tokenjuice.ml_max_input_chars,
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:1://! Kompress backend — TokenJuice ML plain-text compressor (ModernBERT/torch).
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:99:    marker_path(&venv, &config.tokenjuice.ml_model_id).exists() && venv_python_path(&venv).exists()
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:108:    if !config.tokenjuice.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:109:        bail!("tokenjuice.ml_compression_enabled is false");
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:116:    if marker_path(&venv_dir, &config.tokenjuice.ml_model_id).exists() && venv_python.exists() {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:130:        config.tokenjuice.ml_model_id
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:153:    install_deps_and_model(&venv_python, &hf, &config.tokenjuice.ml_model_id).await?;
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:156:        marker_path(&venv_dir, &config.tokenjuice.ml_model_id),
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:177:        .map(|d| marker_path(d, &config.tokenjuice.ml_model_id))
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:178:        .unwrap_or_else(|| marker_path(Path::new("."), &config.tokenjuice.ml_model_id));
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:189:    install_deps_and_model(venv_python, &hf, &config.tokenjuice.ml_model_id).await?;
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/kompress.rs:190:    let _ = tokio::fs::write(&shared_marker, config.tokenjuice.ml_model_id.as_bytes()).await;
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/registry.rs:6:    /// TokenJuice ML plain-text compressor ("Kompress", ModernBERT via torch).
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/registry.rs:28:    if config.tokenjuice.ml_compression_enabled {
<OPENHUMAN_ROOT>/src/openhuman/runtime_python_server/registry.rs:58:        config.tokenjuice.ml_compression_enabled = true;
<OPENHUMAN_ROOT>/src/openhuman/mod.rs:133:pub mod tokenjuice;
<OPENHUMAN_ROOT>/src/openhuman/harness_init/registry.rs:160:        label: "TokenJuice ML compressor (torch)",
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (71423 bytes): call tinyjuice_retrieve with token "256e5d7fc313b1f608c415c7979c0c84"]