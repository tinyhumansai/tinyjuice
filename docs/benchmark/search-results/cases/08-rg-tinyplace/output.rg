[search: 500 match(es) across 90 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1414:    // message so a wallet-less user's tinyplace RPC stays out of Sentry.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1436:        "tinyplace signer init: bad seed"
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
<OPENHUMAN_ROOT>/Cargo.lock:4446: "tinyplace",
<OPENHUMAN_ROOT>/Cargo.lock:6910:name = "tinyplace"
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1253:    /// A JSON message arrived on a tinyplace WebSocket stream.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1257:    TinyPlaceStreamMessage {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1262:        /// The raw JSON message from the tinyplace server.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1265:    /// A tinyplace WebSocket stream changed lifecycle status.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1267:    TinyPlaceStreamStatusChanged {
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/plan.md:170:- **~20 RPC controller domains with zero E2E references** (`recall_calendar`, `tinyplace`,
<OPENHUMAN_ROOT>/plan.md:497:  real backend-facing surface: `recall_calendar`, `tinyplace`, `redirect_links`,
<OPENHUMAN_ROOT>/docs/README.ko.md:77:- **[에이전트 경제](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place)의 `@handle`, Signal로 암호화된 에이전트 간 오케스트레이션, x402 USDC 바운티와 거래까지 제공합니다. 키는 디스크에 절대 닿지 않습니다.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:107:                // A `tinyplace_*` RPC needs a wallet-derived signer but the user
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:385:/// Several `tinyplace_*` RPCs derive a signer seed from the wallet before they
<OPENHUMAN_ROOT>/Cargo.toml:44:tinyplace = "2.0"
<OPENHUMAN_ROOT>/Cargo.toml:359:# TinyFlows, TinyCortex, TinyJuice, TinyChannels, and TinyPlace are vendored beside
<OPENHUMAN_ROOT>/Cargo.toml:366:tinyplace = { path = "vendor/tinyplace/sdk/rust" }
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:77:- **[智能体经济](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**：在 [tiny.place](https://tiny.place) 上的 `@handle`、Signal 加密的智能体间编排、x402 USDC 赏金与交易。密钥永不落盘。
<OPENHUMAN_ROOT>/src/core/all.rs:360:    controllers.extend(crate::openhuman::tinyplace::all_tinyplace_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:364:    // Orchestration read surface (stage 7): the TinyPlaceOrchestrationTab reads
<OPENHUMAN_ROOT>/src/core/all.rs:685:        "tinyplace" => Some(
<OPENHUMAN_ROOT>/src/core/socketio.rs:631:    let io_tinyplace = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:722:    //     TinyPlaceOrchestrationTab targeted-refetches the affected chat live
<OPENHUMAN_ROOT>/src/core/socketio.rs:1221:    // 10. Tinyplace stream events → broadcast to all connected frontend sockets.
<OPENHUMAN_ROOT>/src/core/socketio.rs:1235:                        "[socketio] event_bus not initialised after {}s — tinyplace bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1249:                        "[socketio] dropped {} event_bus events due to lag (tinyplace bridge)",
[+7 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/docs/README.de.md:77:- **[Eine Agenten-Ökonomie](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: ein `@handle` auf [tiny.place](https://tiny.place), Signal-verschlüsselte Agent-zu-Agent-Orchestrierung, x402-USDC-Bounties und Handel. Keys berühren nie die Festplatte.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:91:- **[ایک ایجنٹ معیشت](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place) پر ایک `@handle`، Signal-انکرپٹڈ ایجنٹ سے ایجنٹ آرکسٹریشن، x402 USDC انعامی کام اور تجارت۔ چابیاں کبھی ڈسک کو نہیں چھوتیں۔
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:77:- **[エージェントの経済圏](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place) 上の `@handle`、Signal 暗号化のエージェント間オーケストレーション、x402 USDC バウンティと取引。鍵はディスクに一切触れません。
<OPENHUMAN_ROOT>/gitbooks/README.md:29:* [**An agent economy**](features/tinyplace.md)**.** OpenHuman agents are citizens of tiny.place: a `@handle` identity, Signal-protocol E2E messaging with other agents, x402 USDC bounties and marketplace trading, all signed by an on-device wallet key that never touches disk.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:18:  └─ tinyplace harness wrapper — tails the session JSONL → SessionEnvelopeV1
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/orchestration.md:81:chat message. The Brain → Orchestration tab (`TinyPlaceOrchestrationTab.tsx` +
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:11:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:14:    pub fn id(self) -> &'static str;               // "memory" | "tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:19:        // TinyPlace ⇐ orchestration.enabled                   (today's gate)
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:62:cancel/abort semantics) — a slow memory tick must not delay a tinyplace
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:73:  (each row gains `instance: "memory" | "tinyplace"`).
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:15:- **`tinyplace`** — the tiny.place orchestration world: harness-session
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:56:│   └── tinyplace.rs     wraps orchestration::ops::run_orchestration_review
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:86:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:91:        SubconsciousKind::TinyPlace => Arc::new(profiles::tinyplace::TinyPlaceProfile::new(config)),
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:98:set after login (`Memory` when `heartbeat.enabled`; `TinyPlace` when
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:8:## 3.1 `profiles/tinyplace.rs`
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:12:- `id()` → `"tinyplace"`.
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:58:- `orchestration.enabled` remains the master gate for the tinyplace instance
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:68:- The tinyplace reflect path constructs **no Agent and no toolset** — it is a
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:70:  test: the profile module must not import `tinyplace::agent_tools` or any
[+1 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:5:controls) and in the **TinyPlace Orchestration tab** (the tinyplace kind in
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:10:(`chat.kind === 'subconscious'` in `TinyPlaceOrchestrationTab.tsx`), which is
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:17:  shape plus `instance: 'memory' | 'tinyplace'`. Legacy top-level fields keep
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:19:- `subconsciousTrigger(kind?: 'memory' | 'tinyplace' | 'all')` — optional
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md:35:instance — mode semantics don't apply to tinyplace):
[+10 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-7-ui.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:11:| tinyplace profile | observe/reflect/commit against scripted provider; idle-NONE advances cursor | 3 |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:18:| isolation | tinyplace source-scan (no agent/toolset imports); agent.toml scan (exists) | 3 |
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:26:  generic tick skeleton, the profile table (memory / tinyplace), per-instance
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-5-tests-and-docs.md:31:  driven by the tinyplace subconscious instance, not inlined in the memory
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts:28:    agentId: 'tinyplace_agent',
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts:29:    metadata: { displayName: 'Tinyplace Agent' },
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts:339:            id: 't1:subagent:s1:tinyplace_agent',
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts:340:            name: 'subagent:tinyplace_agent',
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts:367:            subagent: { taskId: 's3', agentId: 'tinyplace_agent', status: 'failed', toolCalls: [] },
[+8 more match(es) in <OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.test.ts]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:14:    fn id(&self) -> &'static str; // "memory" | "tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:25:    /// Default impl returns "" (the tinyplace profile skips this stage).
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:44:    /// carried external content; tinyplace: always tainted).
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:55:    /// (memory: none needed — it re-checkpoints; tinyplace: newest reviewed
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:63:    /// A steering directive was emitted (tinyplace profile).
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md]
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5622:        json!({ "kind": "tinyplace" }),
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:5634:        Some("tinyplace"),
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:22:The agent gets a curated tool surface for all of this (`tinyplace_whoami`, `tinyplace_feed`, `tinyplace_find_work`, `tinyplace_post_bounty`, `tinyplace_submit_work`, `tinyplace_register`, and more), with registration and payments classed as external-effect actions that respect your [approval gate](approval-gate.md).
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:192:The subconscious does more than housekeep. It **steers**. When your agent participates in [tiny.place orchestration sessions](tinyplace.md) (agent-to-agent collaboration), inbound traffic runs through a split-brain wake graph:
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:35:OpenHuman instances collaborate through [tiny.place](tinyplace.md) sessions secured with the **Signal protocol**: real end-to-end encryption, with keys derived on-device and never persisted. Pairing is consent-based and fails closed: an unlinked agent's message is just a message, never an instruction. Your agent can orchestrate other agents (and be orchestrated) without a server ever seeing plaintext.
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:57:* [Workflows](workflows.md) · [Subconscious Loop](subconscious.md) · [tiny.place Agent Economy](tinyplace.md)
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:65:* [tiny.place Agent Economy](features/tinyplace.md)
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx:141:// ── Mock useTinyplaceStream hook ──────────────────────────────────────────────
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx:145:const { mockUseTinyplaceStream } = vi.hoisted(() => ({
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx:146:  mockUseTinyplaceStream: vi.fn((_streamId?: string) => ({
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx:153:vi.mock('../hooks/useTinyplaceStream', () => ({
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx:154:  useTinyplaceStream: (streamId?: string) => mockUseTinyplaceStream(streamId),
[+2 more match(es) in <OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.test.tsx]
<OPENHUMAN_ROOT>/app/src/agentworld/pages/DirectorySection.tsx:92:        debug('[tinyplace][ui] DirectorySection: loaded %d GraphQL agents', data.agents.length);
<OPENHUMAN_ROOT>/app/src/agentworld/pages/DirectorySection.tsx:98:          debug('[tinyplace][ui] DirectorySection: 402 payment_required');
<OPENHUMAN_ROOT>/app/src/agentworld/pages/DirectorySection.tsx:101:          debug('[tinyplace][ui] DirectorySection: error: %s', String(err));
<OPENHUMAN_ROOT>/app/src/agentworld/pages/AgentWorld.test.tsx:66:  test('defaults /agent-world to the TinyPlace world section', () => {
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.tsx:35:import { useTinyplaceStream } from '../hooks/useTinyplaceStream';
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MessagingSection.tsx:874:  const { messages: streamMessages, status: streamStatus } = useTinyplaceStream('inbox');
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MarketplaceSection.tsx:6: * invoke API client bridge (`openhuman.tinyplace_graphql_*`,
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MarketplaceSection.tsx:7: * `openhuman.tinyplace_marketplace_*`,
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MarketplaceSection.tsx:8: * `openhuman.tinyplace_artifacts_*`, `openhuman.tinyplace_escrow_*`,
<OPENHUMAN_ROOT>/app/src/agentworld/pages/MarketplaceSection.tsx:9: * `openhuman.tinyplace_jobs_*`) via `apiClient` from `AgentWorldShell`.
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts:4:import { useTinyplaceStream } from './useTinyplaceStream';
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts:35:describe('useTinyplaceStream', () => {
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts:37:    const { result } = renderHook(() => useTinyplaceStream('inbox'));
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts:42:  test('updates status on tinyplace:stream_status event', () => {
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts:43:    const { result } = renderHook(() => useTinyplaceStream('inbox'));
[+17 more match(es) in <OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.test.ts]
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts:5:export interface TinyplaceStreamMessage {
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts:11:export interface TinyplaceStreamStatus {
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts:17: * Subscribe to tinyplace WebSocket stream events via the core's Socket.IO
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts:18: * bridge. The hook listens for `tinyplace:stream_message` and
<OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts:19: * `tinyplace:stream_status` events, optionally filtered by `streamId`.
[+8 more match(es) in <OPENHUMAN_ROOT>/app/src/agentworld/hooks/useTinyplaceStream.ts]
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-workspace.yaml:9:  # plugin-tinyplace is the unified plugin (own node_modules / lockfile) — same
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-workspace.yaml:11:  - "!sdk/plugin-tinyplace"
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:9:tiny.place is an **agent-to-agent (A2A) social network**: autonomous AI agents claim `@handle` identities, discover each other through an open directory, communicate over Signal-encrypted channels, form groups, and transact on-chain. The backend services (Identity Registry, Open Directory, Encrypted Relay, Payment Facilitator/Ledger) live in a **separate** repo (`../backend-tinyplace`, spec in `../backend-tinyplace/docs/spec/`); staging runs at `https://staging-api.tiny.place`.
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:24:| `website/` | `@tinyplace/website` | The tiny.place web app — **Next.js 16 App Router** + React 19 + TypeScript |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:25:| `sdk/typescript/` | `@tinyhumansai/tinyplace` | **Flagship** TS SDK — the only one with full Signal E2E crypto; published to npm; used by the website |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:26:| `sdk/python/` | `tinyplace` | Python async SDK (aiohttp). REST wrapper — **no encryption**; has a test suite (`sdk/python/tests/`) |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:27:| `sdk/rust/` | `tinyplace` | Rust async SDK (reqwest + tokio). **No encryption**; has a test suite (`sdk/rust/tests/`, wiremock-mocked) |
[+9 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/xplugin-e2e.mjs:15:    TINYPLACE_SESSION_DAEMON: "off", TINYPLACE_AUTORESPOND: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/xplugin-e2e.mjs:16:    TINYPLACE_API_URL: "https://staging-api.tiny.place" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/xplugin-e2e.mjs:34:const A = agent(CLAUDE_SERVER, "xp-claude-", "TINYPLACE_CLAUDE_HOME"); // sender
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/xplugin-e2e.mjs:35:const B = agent(CODEX_SERVER, "xp-codex-", "TINYPLACE_CODEX_HOME");   // receiver
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/daemon-test.mjs:19:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-daemon-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/daemon-test.mjs:20:const dataDir = process.env.TINYPLACE_CODEX_HOME;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/daemon-test.mjs:32:    env: { ...process.env, TINYPLACE_CODEX_HOME: dataDir, TINYPLACE_API_URL: DEAD_BACKEND, TINYPLACE_HARNESS_SESSION_ID: sessionId, ...extraEnv },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/daemon-test.mjs:59:let s = server("setup", { TINYPLACE_SESSION_DAEMON: "off" });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/daemon-test.mjs:70:  env: { ...process.env, TINYPLACE_DAEMON_WALLET: "d1", TINYPLACE_CODEX_HOME: dataDir, TINYPLACE_API_URL: DEAD_BACKEND, TINYPLACE_DAEMON_IDLE_MS: "500", TINYPLACE_DAEMON_POLL_MS: "120", TINYPLACE_DAEMON_HEARTBEAT_MS: "120" },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp-smoke.mjs:22:    TINYPLACE_CODEX_HOME: mkdtempSync(join(tmpdir(), "tinyplace-codex-smoke-")),
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp-smoke.mjs:23:    TINYPLACE_SESSION_DAEMON: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp-smoke.mjs:24:    TINYPLACE_AUTORESPOND: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp-smoke.mjs:25:    TINYPLACE_API_URL: "https://staging-api.tiny.place",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp-smoke.mjs:72:  check("initialize returns serverInfo.name=tinyplace", init.result?.serverInfo?.name === "tinyplace");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:4:// files and writes markers. Uses an isolated TINYPLACE_CODEX_HOME temp dir.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:45:    env: { ...process.env, TINYPLACE_CODEX_HOME: HOME, ...env },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:52:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:60:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:74:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_NO_AUTORESPOND: "1", TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR } });
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:3:// Targets staging by default; override with TINYPLACE_API_URL (prod spends real
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:8:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:10:const BASE = process.env.TINYPLACE_API_URL ?? "https://staging-api.tiny.place";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:12:const HOME = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:23:const client = new TinyPlaceClient({ baseUrl: BASE, signer });
<OPENHUMAN_ROOT>/src/openhuman/approval/redact.rs:249:            "associatedData": { "topic": "tinyplace dm" }
<OPENHUMAN_ROOT>/src/openhuman/approval/redact.rs:269:        assert_eq!(red["associatedData"]["topic"], "tinyplace dm");
<OPENHUMAN_ROOT>/src/openhuman/approval/redact.rs:301:    fn tinyplace_write_content_fields_are_redacted() {
<OPENHUMAN_ROOT>/src/openhuman/approval/redact.rs:328:    fn tinyplace_profile_update_fields_are_redacted() {
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/AGENTS.md:3:You have the **tinyplace** MCP server available. It lets you act as an agent on the
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:590:    // so the dedicated tinyplace subagent can register identities, inspect
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:593:    let tinyplace_tools = crate::openhuman::tinyplace::tools::all_tinyplace_agent_tools();
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:595:        "[tools::ops][tinyplace] registering tinyplace agent tools count={}",
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:596:        tinyplace_tools.len()
<OPENHUMAN_ROOT>/src/openhuman/tools/ops.rs:598:    tools.extend(tinyplace_tools);
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs:12://   - The tinyplace MCP server is reached via the isolated CODEX_HOME the launcher
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs:13://     wrote (config.toml → [mcp_servers.tinyplace]); we forward CODEX_HOME so the
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs:14://     responder loads the same tools. TINYPLACE_ACTIVE_WALLET pins its identity.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs:26:const rawPool = Number(process.env.TINYPLACE_AUTORESPOND_POOL);
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs:28:const MODEL = process.env.TINYPLACE_AUTORESPOND_MODEL ?? "gpt-5.4";
[+4 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/respond-batch.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/hooks.json:12:            "command": "node \"${TINYPLACE_PLUGIN_ROOT}/hooks/surface-inbound.mjs\""
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/hooks.json:22:            "command": "node \"${TINYPLACE_PLUGIN_ROOT}/hooks/surface-inbound.mjs\""
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/hooks.json:32:            "command": "node \"${TINYPLACE_PLUGIN_ROOT}/hooks/dispatch.mjs\""
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs:11:// Env: TINYPLACE_DAEMON_WALLET (required) — the wallet name to serve.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs:18:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs:19:import { FileSessionStore } from "@tinyhumansai/tinyplace/node";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs:35:  try { process.stderr.write(`tinyplace-daemon: uncaughtException: ${err?.stack ?? err}\n`); } catch {}
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs:55:  console.error("agent-daemon: TINYPLACE_DAEMON_WALLET is required");
[+11 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/agent-daemon.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs:8:// hook too — TINYPLACE_NO_AUTORESPOND (set on responders) makes it a no-op there.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs:18:if (process.env.TINYPLACE_NO_AUTORESPOND || process.env.TINYPLACE_AUTORESPOND === "off") process.exit(0);
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs:21:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs:51:const active = process.env.TINYPLACE_DISPATCH_ADDRESS
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs:52:  ? { address: process.env.TINYPLACE_DISPATCH_ADDRESS, wallet: process.env.TINYPLACE_DISPATCH_WALLET ?? "" }
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/dispatch.mjs]
<OPENHUMAN_ROOT>/src/openhuman/tools/mod.rs:51:pub use crate::openhuman::tinyplace::tools::*;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:3:// Codex MCP is pull-only: the tinyplace server can't push a new DM into a live
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:17:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:138:  `\nCall the tinyplace \`inbox\` tool to read and act on them.`;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:2:  "name": "tinyplace-codex",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:6:  "description": "OpenAI Codex CLI plugin: multi-wallet tiny.place messaging over an MCP server wrapping @tinyhumansai/tinyplace. Parity port of tinyplace-claude.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:8:    "tinyplace-codex": "./bin/tinyplace-codex.mjs"
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:18:    "@tinyhumansai/tinyplace": "^1.0.1",
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:461:    fn tinyplace_agent_subagent_synthesises_use_tinyplace_delegate() {
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:463:        orch.subagents = vec![SubagentEntry::AgentId("tinyplace_agent".into())];
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:466:            "tinyplace_agent",
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:468:            Some("use_tinyplace"),
<OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs:474:            vec!["use_tinyplace"],
[+3 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/tools/orchestrator_tools.rs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:1:# tinyplace-codex
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:8:Built as a thin wrapper over the official `@tinyhumansai/tinyplace` SDK, exposed to
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:10:[`tinyplace-claude`](../plugin-claude) plugin, so a Codex agent and a Claude agent
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:20:      │  wraps @tinyhumansai/tinyplace
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:21:      ├─ wallet store    ~/.tinyplace-codex/wallets.json   (named keypairs)
[+15 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:5:// Layout: ~/.tinyplace-codex/sessions/<agent-address>/<label>.json
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:17:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:19:const LIVE_WINDOW_MS = Number(process.env.TINYPLACE_SESSION_LIVE_MS) || 30_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:4:// Wraps @tinyhumansai/tinyplace to give a Claude Code session:
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:26:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:27:import { FileSessionStore } from "@tinyhumansai/tinyplace/node";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:33:} from "@tinyhumansai/tinyplace/agent";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:63:  try { process.stderr.write(`tinyplace: uncaughtException: ${err?.stack ?? err}\n`); } catch {}
[+23 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/outbox.mjs:12:const STALE_CLAIM_MS = Number(process.env.TINYPLACE_OUTBOX_CLAIM_MS) || 60_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/format.mjs:4:// `tinyplace.harness.session.v1`, see sdk/typescript/src/types/harness.ts) plus
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/format.mjs:20:export const SESSION_ENVELOPE_VERSION = "tinyplace.harness.session.v1";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/format.mjs:52:    process.env.TINYPLACE_HARNESS_SESSION_ID?.trim() ||
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/format.mjs:60:  return process.env.TINYPLACE_SESSION_LABEL?.trim() || "codex:1";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/format.mjs:101:    harness: { provider: "codex", command: "tinyplace-codex-plugin", argv: [] },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/daemon-lock.mjs:2:// Signal ratchet for an agent. Lock file: ~/.tinyplace-codex/daemon/<agent>.lock
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/daemon-lock.mjs:15:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/daemon-lock.mjs:18:const LOCK_WINDOW_MS = Number(process.env.TINYPLACE_DAEMON_LOCK_MS) || 30_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/routing.mjs:11:const STALE_CLAIM_MS = Number(process.env.TINYPLACE_INBOX_CLAIM_MS) || 60_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/routing.mjs:13:// No-target delivery policy (TINYPLACE_UNROUTED_POLICY): primary (default),
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/routing.mjs:16:  const p = process.env.TINYPLACE_UNROUTED_POLICY?.trim();
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:11:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-lock-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:12:delete process.env.TINYPLACE_DAEMON_LOCK_MS;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:69:  process.env.TINYPLACE_CODEX_HOME = ${JSON.stringify(process.env.TINYPLACE_CODEX_HOME)};
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:91:    process.env.TINYPLACE_CODEX_HOME = ${JSON.stringify(process.env.TINYPLACE_CODEX_HOME)};
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:6:// this writes an ISOLATED CODEX_HOME (config.toml → [mcp_servers.tinyplace] +
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:11:// use) happens up front via TINYPLACE_ACTIVE_WALLET, so Codex opens logged in.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:14:// Codex session ("Door A": `codex mcp add tinyplace -- node .../mcp/server.mjs`,
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:344:      console.error(`No wallet named '${name ?? ""}'. Run 'tinyplace-codex' with no args to create one.`);
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:351:    console.error("tinyplace-codex: interactive menu needs a TTY. Use 'tinyplace-codex --wallet <name>' in non-interactive contexts.");
[+23 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs]
<OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs:31:/// Generic on purpose: it runs `tinyplace_agent`, which can do anything on
<OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs:33:const TINYPLACE_AUTOPILOT_JOB_NAME: &str = "tinyplace_autopilot";
<OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs:83:    if !has(TINYPLACE_AUTOPILOT_JOB_NAME) {
<OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs:85:            "[cron::seed] creating autonomous tiny.place autopilot job (tinyplace_agent, disabled — opt-in)"
<OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs:87:        seed_tinyplace_autopilot(config)?;
[+27 more match(es) in <OPENHUMAN_ROOT>/src/openhuman/cron/seed.rs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/routing-test.mjs:7:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-route-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/routing-test.mjs:8:delete process.env.TINYPLACE_SESSION_LIVE_MS;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md:31:  (`[mcp_servers.tinyplace]` + inline `[hooks]` or bundled `hooks.json`), launches
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md:34:- **Path A (later)**: package as a real Codex marketplace plugin (`codex plugin add tinyplace@...`).
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md:56:| Config dir | `~/.tinyplace-claude` | `~/.tinyplace-codex` |
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md:64:  Recursion guard `TINYPLACE_NO_AUTORESPOND` on responders. Offline-tested
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md:72:- **hooks.json** uses `${TINYPLACE_PLUGIN_ROOT}` placeholders → the P5 launcher
[+3 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/SPIKE.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/registry-test.mjs:9:// registry.mjs reads TINYPLACE_CODEX_HOME at import time — set it first.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/registry-test.mjs:10:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-reg-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/registry-test.mjs:11:delete process.env.TINYPLACE_SESSION_LIVE_MS; // use the default 30s window
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/registry-test.mjs:101:    process.env.TINYPLACE_CODEX_HOME = ${JSON.stringify(process.env.TINYPLACE_CODEX_HOME)};
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:17:const API = process.env.TINYPLACE_API_URL ?? "https://staging-api.tiny.place";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:24:      TINYPLACE_CODEX_HOME: mkdtempSync(join(tmpdir(), `tp-codex-${tag}-`)),
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:25:      TINYPLACE_SESSION_DAEMON: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:26:      TINYPLACE_AUTORESPOND: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:27:      TINYPLACE_API_URL: API,
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:11:// Pin the claude adapter so harnessDataDir() resolves under TINYPLACE_CLAUDE_HOME.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:12:process.env.TINYPLACE_HARNESS = "claude";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:13:process.env.TINYPLACE_CLAUDE_HOME = mkdtempSync(join(tmpdir(), "tinyplace-inject-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:14:delete process.env.TINYPLACE_FOREGROUND_RESOLVE;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:61:  process.env.TINYPLACE_FOREGROUND_RESOLVE = "off";
[+3 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:32:    env: { ...process.env, TINYPLACE_HARNESS: harness, [dataDirEnv]: dataDir },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:82:const cx = await driveServer("codex", "TINYPLACE_CODEX_HOME");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:83:expect("codex: server boots (serverInfo.name=tinyplace)", cx.serverInfo?.name === "tinyplace");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:92:const cl = await driveServer("claude", "TINYPLACE_CLAUDE_HOME");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:93:expect("claude: server boots (serverInfo.name=tinyplace)", cl.serverInfo?.name === "tinyplace");
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:18:check("override forces codex", detectHarness({ TINYPLACE_HARNESS: "codex", CLAUDE_PLUGIN_ROOT: "/p" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:19:check("override forces claude", detectHarness({ TINYPLACE_HARNESS: "claude", CODEX_HOME: "/x" }) === "claude");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:20:check("bad override ignored → signal", detectHarness({ TINYPLACE_HARNESS: "nope", CODEX_HOME: "/x" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:21:check("override case-insensitive", detectHarness({ TINYPLACE_HARNESS: "CODEX" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:28:  check("codex dataDirEnv", a.dataDirEnv === "TINYPLACE_CODEX_HOME");
[+2 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:4:// files and writes markers. Uses an isolated TINYPLACE_CODEX_HOME temp dir.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:44:    env: { ...process.env, TINYPLACE_HARNESS: "codex", TINYPLACE_CODEX_HOME: HOME, ...env },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:51:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:59:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:73:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_NO_AUTORESPOND: "1", TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR } });
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs]
<OPENHUMAN_ROOT>/src/openhuman/cron/scheduler_tests.rs:825:        !visible.contains("use_tinyplace"),
<OPENHUMAN_ROOT>/src/openhuman/cron/scheduler_tests.rs:829:        !visible.iter().any(|name| name.starts_with("tinyplace_")),
<OPENHUMAN_ROOT>/src/openhuman/cron/scheduler_tests.rs:830:        "morning briefing cron jobs must preserve tinyplace_* disallowlist"
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/envelope-test.mjs:5:// Runs the unified module under the codex adapter (TINYPLACE_HARNESS=codex) so
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/envelope-test.mjs:7:process.env.TINYPLACE_HARNESS = "codex";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/envelope-test.mjs:43:expect("harness.provider stamped from active adapter (codex)", parsedRaw.harness.provider === "codex" && parsedRaw.harness.command === "tinyplace-codex-plugin");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/register.mjs:3:// Targets staging by default; override with TINYPLACE_API_URL (prod spends real
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/register.mjs:8:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/register.mjs:12:const BASE = process.env.TINYPLACE_API_URL ?? "https://staging-api.tiny.place";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/register.mjs:26:const client = new TinyPlaceClient({ baseUrl: BASE, signer });
<OPENHUMAN_ROOT>/scripts/generate-test-inventory.mjs:18://       ship with zero integration/E2E coverage (recall_calendar, tinyplace,
<OPENHUMAN_ROOT>/scripts/generate-test-inventory.mjs:78:  'tinyplace',
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md:1:# tinyplace — one plugin, any harness
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md:9:1333 loc — ~95% shared) with a single `@tinyhumansai/tinyplace-plugin`. The 5% that
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md:16:   install once  ───▶  │  @tinyhumansai/tinyplace-*   │
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md:37:| `TINYPLACE_HARNESS` set | that value (explicit escape hatch) |
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md:48:  dataDirEnv, dataDirDefault,          // ~/.tinyplace-codex vs -claude (or unified ~/.tinyplace)
[+18 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/DESIGN.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-inbox/SKILL.md:7:Requires an active agent (`/tinyplace:use`). The background listener decrypts inbound
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-inbox/SKILL.md:17:the next inbound. See the `tinyplace-await` skill.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-send/SKILL.md:7:Requires an active agent (`/tinyplace:use`). Recipient may be a `@handle`, a base58
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-send/SKILL.md:22:fire-and-forget `send` plus checking `/tinyplace:inbox` periodically.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-wallet/SKILL.md:17:`~/.tinyplace-claude/wallets.json` under Claude Code (`~/.tinyplace-codex` under
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/skills/tinyplace-wallet/SKILL.md:21:To actually message, the user must select a wallet as active with the `/tinyplace:use`
<OPENHUMAN_ROOT>/src/openhuman/mcp_server/resources.rs:89:        uri: "openhuman://prompts/agents/tinyplace_agent",
<OPENHUMAN_ROOT>/src/openhuman/mcp_server/resources.rs:90:        name: "tinyplace_agent",
<OPENHUMAN_ROOT>/src/openhuman/mcp_server/resources.rs:92:        content: include_str!("../tinyplace/agent/prompt.md"),
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/unassign.md:5:Clear the persistent tiny.place agent assignment for this session. Call the tinyplace `unassign` tool and report what was cleared. Note that this does not change the currently active agent for the running session — it only stops a future run from auto-adopting it.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/reset.md:6:Recover a tiny.place channel where messages have stopped decrypting. Call the tinyplace `reset_session` tool with peer="$ARGUMENTS" (if a peer was given) — that clears the local Signal session with them and sends a fresh handshake so the next message re-runs X3DH. If no peer was given, call `reset_session` with no arguments to republish your own key bundle. Report what was reset.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/agents.md:5:List the available tiny.place agents. Call the tinyplace `wallet_list` tool and present each wallet's name and address, clearly marking which one (if any) is the active agent for this session. If none is active, tell me I can activate one with `/tinyplace:use <name>` or `/tinyplace:assign <name>`.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/sessions.md:5:List the live sessions registered for the active tiny.place agent. Call the tinyplace `sessions` tool and report each session's label (e.g. `claude:1`), whether it is this session (`self`), and its pid/cwd. Explain that a peer can direct a message to a specific session by passing `to_session` to the `send` tool. If no agent is active, tell me to select one with `use` first.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/contacts.md:6:Show the active agent's tiny.place contacts. If "$ARGUMENTS" starts with "accept", call the tinyplace `contact_accept` tool with `from` set to the address that follows. Otherwise: call `contact_requests` to list pending INCOMING requests (peers waiting for you to approve so they can DM you) and `contacts` to list accepted contacts. Present both, and for each pending request note that it can be approved with `/tinyplace:contacts accept <address>` (or the `contact_accept` tool). Accepting a contact is a trust decision — do not auto-approve.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/use.md:6:Activate the tiny.place agent named "$ARGUMENTS" for this session. Call the tinyplace `use` tool with name="$ARGUMENTS", then confirm the active agent, its session label (e.g. `claude:1`), and whether its keys published. To pin a specific label, pass `label:"claude:2"` in the arguments. If no name was given, call `wallet_list` first and ask me which one to activate.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/autorespond.md:6:Control the tiny.place auto-responder for this session. Call the tinyplace `autorespond` tool with state="$ARGUMENTS" (use "status" if no argument was given), then report whether the auto-responder is on or off. When on, inbound DMs are answered automatically by a background responder even while this session is idle.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/whoami.md:5:Show which tiny.place agent is currently active in this session. Call the tinyplace `whoami` tool and report the active wallet name and address, this session's label (e.g. `claude:1`), the list of live sessions for this agent, plus the scope and any persisted assignment.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/commands/assign.md:6:Assign the tiny.place agent named "$ARGUMENTS" as the active agent for this session AND persist the assignment, so future runs of this session auto-adopt it without re-selecting. Call the tinyplace `assign` tool with name="$ARGUMENTS", then confirm the active agent and the scope it was assigned to. If no name was given, call `wallet_list` first and ask me which one.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs:13:// tinyplace `auto_reply` MCP tool (never the shell or filesystem):
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs:14://   - Codex → `codex exec --sandbox read-only … <prompt>`; the tinyplace MCP
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs:18://     --tools "" --allowedTools mcp__tinyplace__auto_reply …`.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs:19:// TINYPLACE_ACTIVE_WALLET pins the responder's identity either way.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs:30:const rawPool = Number(process.env.TINYPLACE_AUTORESPOND_POOL);
[+4 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks/respond-batch.mjs]

[compacted tool output — this is a PARTIAL view; the full original (76032 bytes) is available by calling tokenjuice_retrieve with token "6442f7183b24a21bc1d886a436f2497d" (marker ⟦tj:6442f7183b24a21bc1d886a436f2497d⟧)]