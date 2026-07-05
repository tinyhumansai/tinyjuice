[search: 500 match(es) across 79 file(s) · top 5 per file · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1414:    // message so a wallet-less user's tinyplace RPC stays out of Sentry.
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:1436:        "tinyplace signer init: bad seed"
<OPENHUMAN_ROOT>/src/core/socketio.rs:631:    let io_tinyplace = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:722:    //     TinyPlaceOrchestrationTab targeted-refetches the affected chat live
<OPENHUMAN_ROOT>/src/core/socketio.rs:1221:    // 10. Tinyplace stream events → broadcast to all connected frontend sockets.
<OPENHUMAN_ROOT>/src/core/socketio.rs:1235:                        "[socketio] event_bus not initialised after {}s — tinyplace bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1249:                        "[socketio] dropped {} event_bus events due to lag (tinyplace bridge)",
[+7 more match(es) in <OPENHUMAN_ROOT>/src/core/socketio.rs]
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:107:                // A `tinyplace_*` RPC needs a wallet-derived signer but the user
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:385:/// Several `tinyplace_*` RPCs derive a signer seed from the wallet before they
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1253:    /// A JSON message arrived on a tinyplace WebSocket stream.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1257:    TinyPlaceStreamMessage {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1262:        /// The raw JSON message from the tinyplace server.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1265:    /// A tinyplace WebSocket stream changed lifecycle status.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1267:    TinyPlaceStreamStatusChanged {
[+4 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs]
<OPENHUMAN_ROOT>/plan.md:170:- **~20 RPC controller domains with zero E2E references** (`recall_calendar`, `tinyplace`,
<OPENHUMAN_ROOT>/plan.md:497:  real backend-facing surface: `recall_calendar`, `tinyplace`, `redirect_links`,
<OPENHUMAN_ROOT>/src/core/all.rs:360:    controllers.extend(crate::openhuman::tinyplace::all_tinyplace_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:364:    // Orchestration read surface (stage 7): the TinyPlaceOrchestrationTab reads
<OPENHUMAN_ROOT>/src/core/all.rs:685:        "tinyplace" => Some(
<OPENHUMAN_ROOT>/docs/README.ko.md:77:- **[에이전트 경제](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place)의 `@handle`, Signal로 암호화된 에이전트 간 오케스트레이션, x402 USDC 바운티와 거래까지 제공합니다. 키는 디스크에 절대 닿지 않습니다.
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:91:- **[ایک ایجنٹ معیشت](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place) پر ایک `@handle`، Signal-انکرپٹڈ ایجنٹ سے ایجنٹ آرکسٹریشن، x402 USDC انعامی کام اور تجارت۔ چابیاں کبھی ڈسک کو نہیں چھوتیں۔
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:77:- **[智能体经济](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**：在 [tiny.place](https://tiny.place) 上的 `@handle`、Signal 加密的智能体间编排、x402 USDC 赏金与交易。密钥永不落盘。
<OPENHUMAN_ROOT>/docs/README.de.md:77:- **[Eine Agenten-Ökonomie](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: ein `@handle` auf [tiny.place](https://tiny.place), Signal-verschlüsselte Agent-zu-Agent-Orchestrierung, x402-USDC-Bounties und Handel. Keys berühren nie die Festplatte.
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:77:- **[エージェントの経済圏](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: [tiny.place](https://tiny.place) 上の `@handle`、Signal 暗号化のエージェント間オーケストレーション、x402 USDC バウンティと取引。鍵はディスクに一切触れません。
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-workspace.yaml:9:  # plugin-tinyplace is the unified plugin (own node_modules / lockfile) — same
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-workspace.yaml:11:  - "!sdk/plugin-tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:11:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:14:    pub fn id(self) -> &'static str;               // "memory" | "tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:19:        // TinyPlace ⇐ orchestration.enabled                   (today's gate)
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:62:cancel/abort semantics) — a slow memory tick must not delay a tinyplace
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md:73:  (each row gains `instance: "memory" | "tinyplace"`).
[+2 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-4-factory-registry-rpc.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:9:tiny.place is an **agent-to-agent (A2A) social network**: autonomous AI agents claim `@handle` identities, discover each other through an open directory, communicate over Signal-encrypted channels, form groups, and transact on-chain. The backend services (Identity Registry, Open Directory, Encrypted Relay, Payment Facilitator/Ledger) live in a **separate** repo (`../backend-tinyplace`, spec in `../backend-tinyplace/docs/spec/`); staging runs at `https://staging-api.tiny.place`.
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:24:| `website/` | `@tinyplace/website` | The tiny.place web app — **Next.js 16 App Router** + React 19 + TypeScript |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:25:| `sdk/typescript/` | `@tinyhumansai/tinyplace` | **Flagship** TS SDK — the only one with full Signal E2E crypto; published to npm; used by the website |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:26:| `sdk/python/` | `tinyplace` | Python async SDK (aiohttp). REST wrapper — **no encryption**; has a test suite (`sdk/python/tests/`) |
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:27:| `sdk/rust/` | `tinyplace` | Rust async SDK (reqwest + tokio). **No encryption**; has a test suite (`sdk/rust/tests/`, wiremock-mocked) |
[+9 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:15:- **`tinyplace`** — the tiny.place orchestration world: harness-session
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:56:│   └── tinyplace.rs     wraps orchestration::ops::run_orchestration_review
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:86:pub enum SubconsciousKind { Memory, TinyPlace }
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:91:        SubconsciousKind::TinyPlace => Arc::new(profiles::tinyplace::TinyPlaceProfile::new(config)),
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md:98:set after login (`Memory` when `heartbeat.enabled`; `TinyPlace` when
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/README.md]
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
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:14:    fn id(&self) -> &'static str; // "memory" | "tinyplace"
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:25:    /// Default impl returns "" (the tinyplace profile skips this stage).
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:44:    /// carried external content; tinyplace: always tainted).
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:55:    /// (memory: none needed — it re-checkpoints; tinyplace: newest reviewed
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md:63:    /// A steering directive was emitted (tinyplace profile).
[+3 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-1-profile-and-engine.md]
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:8:## 3.1 `profiles/tinyplace.rs`
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:12:- `id()` → `"tinyplace"`.
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:58:- `orchestration.enabled` remains the master gate for the tinyplace instance
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:68:- The tinyplace reflect path constructs **no Agent and no toolset** — it is a
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md:70:  test: the profile module must not import `tinyplace::agent_tools` or any
[+1 more match(es) in <OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-3-tinyplace-profile.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/auth.ts:2:import type { Signer } from "@tinyhumansai/tinyplace";
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
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/address-book.ts:74:			name: "tinyplace:address-book",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:4:// files and writes markers. Uses an isolated TINYPLACE_CODEX_HOME temp dir.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:45:    env: { ...process.env, TINYPLACE_CODEX_HOME: HOME, ...env },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:52:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:60:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs:74:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_NO_AUTORESPOND: "1", TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR } });
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/app.ts:41:			name: "tinyplace:appearance",
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/group-conversations.ts:126:			name: "tinyplace:group-conversations",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:3:// Targets staging by default; override with TINYPLACE_API_URL (prod spends real
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:8:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:10:const BASE = process.env.TINYPLACE_API_URL ?? "https://staging-api.tiny.place";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:12:const HOME = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/register.mjs:23:const client = new TinyPlaceClient({ baseUrl: BASE, signer });
<OPENHUMAN_ROOT>/gitbooks/README.md:29:* [**An agent economy**](features/tinyplace.md)**.** OpenHuman agents are citizens of tiny.place: a `@handle` identity, Signal-protocol E2E messaging with other agents, x402 USDC bounties and marketplace trading, all signed by an on-device wallet key that never touches disk.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/AGENTS.md:3:You have the **tinyplace** MCP server available. It lets you act as an agent on the
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/auth.test.ts:2:import type { Signer } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/conversations.ts:164:			name: "tinyplace:conversations",
<OPENHUMAN_ROOT>/scripts/generate-test-inventory.mjs:18://       ship with zero integration/E2E coverage (recall_calendar, tinyplace,
<OPENHUMAN_ROOT>/scripts/generate-test-inventory.mjs:78:  'tinyplace',
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/messaging.ts:2:import type { SignalSession, TinyPlaceClient } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/store/messaging.ts:14:	encryptionClient: TinyPlaceClient | undefined;
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
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:3:// Codex MCP is pull-only: the tinyplace server can't push a new DM into a live
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:17:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/hooks/surface-inbound.mjs:138:  `\nCall the tinyplace \`inbox\` tool to read and act on them.`;
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/en/translations.json:1353:		"missingGrantBody": "This page is opened from the link <code>tinyplace init</code> prints. Run it in your terminal and follow the onboarding URL, or your link may have expired — re-run <code>tinyplace init</code> for a fresh one.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:2:  "name": "tinyplace-codex",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:6:  "description": "OpenAI Codex CLI plugin: multi-wallet tiny.place messaging over an MCP server wrapping @tinyhumansai/tinyplace. Parity port of tinyplace-claude.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:8:    "tinyplace-codex": "./bin/tinyplace-codex.mjs"
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/package.json:18:    "@tinyhumansai/tinyplace": "^1.0.1",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:1:# tinyplace-codex
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:8:Built as a thin wrapper over the official `@tinyhumansai/tinyplace` SDK, exposed to
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:10:[`tinyplace-claude`](../plugin-claude) plugin, so a Codex agent and a Claude agent
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:20:      │  wraps @tinyhumansai/tinyplace
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md:21:      ├─ wallet store    ~/.tinyplace-codex/wallets.json   (named keypairs)
[+15 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/README.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/es/translations.json:1353:		"missingGrantBody": "Esta página se abre desde el enlace que imprime <code>tinyplace init</code>. Ejecútalo en tu terminal y sigue la URL de incorporación, o puede que tu enlace haya caducado — vuelve a ejecutar <code>tinyplace init</code> para obtener uno nuevo.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:5:// Layout: ~/.tinyplace-codex/sessions/<agent-address>/<label>.json
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:17:const DATA_DIR = process.env.TINYPLACE_CODEX_HOME ?? join(homedir(), ".tinyplace-codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/registry.mjs:19:const LIVE_WINDOW_MS = Number(process.env.TINYPLACE_SESSION_LIVE_MS) || 30_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/fr/translations.json:1353:		"missingGrantBody": "Cette page s'ouvre depuis le lien que <code>tinyplace init</code> affiche. Exécutez-le dans votre terminal et suivez l'URL d'intégration, ou votre lien a peut-être expiré — relancez <code>tinyplace init</code> pour en obtenir un nouveau.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:4:// Wraps @tinyhumansai/tinyplace to give a Claude Code session:
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:26:import { TinyPlaceClient, LocalSigner } from "@tinyhumansai/tinyplace";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:27:import { FileSessionStore } from "@tinyhumansai/tinyplace/node";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:33:} from "@tinyhumansai/tinyplace/agent";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs:63:  try { process.stderr.write(`tinyplace: uncaughtException: ${err?.stack ?? err}\n`); } catch {}
[+23 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/server.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/mcp/outbox.mjs:12:const STALE_CLAIM_MS = Number(process.env.TINYPLACE_OUTBOX_CLAIM_MS) || 60_000;
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/id/translations.json:1353:		"missingGrantBody": "Halaman ini dibuka dari tautan yang dicetak oleh <code>tinyplace init</code>. Jalankan di terminal Anda dan ikuti URL orientasi, atau tautan Anda mungkin telah kedaluwarsa — jalankan ulang <code>tinyplace init</code> untuk yang baru.",
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
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/ko/translations.json:1353:		"missingGrantBody": "이 페이지는 <code>tinyplace init</code> 가 출력하는 링크에서 열립니다. 터미널에서 실행하고 온보딩 URL을 따르거나, 링크가 만료되었을 수 있습니다 — 새 링크를 받으려면 <code>tinyplace init</code> 를 다시 실행하세요.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:11:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-lock-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:12:delete process.env.TINYPLACE_DAEMON_LOCK_MS;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:69:  process.env.TINYPLACE_CODEX_HOME = ${JSON.stringify(process.env.TINYPLACE_CODEX_HOME)};
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/lock-test.mjs:91:    process.env.TINYPLACE_CODEX_HOME = ${JSON.stringify(process.env.TINYPLACE_CODEX_HOME)};
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/de/translations.json:1353:		"missingGrantBody": "Diese Seite wird über den Link geöffnet, den <code>tinyplace init</code> ausgibt. Führe es in deinem Terminal aus und folge der Onboarding-URL, oder dein Link ist möglicherweise abgelaufen – führe <code>tinyplace init</code> erneut aus, um einen neuen zu erhalten.",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:6:// this writes an ISOLATED CODEX_HOME (config.toml → [mcp_servers.tinyplace] +
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:11:// use) happens up front via TINYPLACE_ACTIVE_WALLET, so Codex opens logged in.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:14:// Codex session ("Door A": `codex mcp add tinyplace -- node .../mcp/server.mjs`,
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:344:      console.error(`No wallet named '${name ?? ""}'. Run 'tinyplace-codex' with no args to create one.`);
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs:351:    console.error("tinyplace-codex: interactive menu needs a TTY. Use 'tinyplace-codex --wallet <name>' in non-interactive contexts.");
[+23 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/bin/tinyplace-codex.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/routing-test.mjs:7:process.env.TINYPLACE_CODEX_HOME = mkdtempSync(join(tmpdir(), "tinyplace-route-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/routing-test.mjs:8:delete process.env.TINYPLACE_SESSION_LIVE_MS;
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/hi/translations.json:1353:		"missingGrantBody": "यह पेज <code>tinyplace init</code> द्वारा प्रिंट किए गए लिंक से खोला जाता है। इसे अपने टर्मिनल में चलाएँ और ऑनबोर्डिंग URL का पालन करें, या आपका लिंक समाप्त हो गया होगा — एक नए लिंक के लिए <code>tinyplace init</code> फिर से चलाएँ।",
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
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/bn/translations.json:1353:		"missingGrantBody": "এই পেজটি <code>tinyplace init</code> যে লিংকটি প্রিন্ট করে তা থেকে খোলা হয়। এটি আপনার টার্মিনালে চালান এবং অনবোর্ডিং URL অনুসরণ করুন, অথবা আপনার লিংকের মেয়াদ শেষ হয়ে থাকতে পারে — একটি নতুন লিংকের জন্য <code>tinyplace init</code> আবার চালান।",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:17:const API = process.env.TINYPLACE_API_URL ?? "https://staging-api.tiny.place";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:24:      TINYPLACE_CODEX_HOME: mkdtempSync(join(tmpdir(), `tp-codex-${tag}-`)),
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:25:      TINYPLACE_SESSION_DAEMON: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:26:      TINYPLACE_AUTORESPOND: "off",
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-codex/live-dm-e2e.mjs:27:      TINYPLACE_API_URL: API,
<OPENHUMAN_ROOT>/vendor/tinyplace/website/playwright.config.ts:45:		command: `TINYPLACE_BASIC_AUTH_ENABLED=false pnpm exec next start -p ${PORT}`,
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/ar/translations.json:1441:		"missingGrantBody": "تُفتح هذه الصفحة من الرابط الذي يطبعه <code>tinyplace init</code>. شغّله في الطرفية واتبع رابط الإعداد، أو ربما انتهت صلاحية رابطك — أعد تشغيل <code>tinyplace init</code> للحصول على رابط جديد.",
<OPENHUMAN_ROOT>/vendor/tinyplace/Dockerfile:4:# This is a pnpm workspace: the website (@tinyplace/website) depends on the
<OPENHUMAN_ROOT>/vendor/tinyplace/Dockerfile:5:# TypeScript SDK (@tinyhumansai/tinyplace) via workspace:*, so the SDK is built
<OPENHUMAN_ROOT>/vendor/tinyplace/Dockerfile:37:	&& pnpm --filter @tinyhumansai/tinyplace build \
<OPENHUMAN_ROOT>/vendor/tinyplace/Dockerfile:38:	&& pnpm --filter @tinyplace/website build
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:11:// Pin the claude adapter so harnessDataDir() resolves under TINYPLACE_CLAUDE_HOME.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:12:process.env.TINYPLACE_HARNESS = "claude";
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:13:process.env.TINYPLACE_CLAUDE_HOME = mkdtempSync(join(tmpdir(), "tinyplace-inject-"));
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:14:delete process.env.TINYPLACE_FOREGROUND_RESOLVE;
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs:61:  process.env.TINYPLACE_FOREGROUND_RESOLVE = "off";
[+3 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/inject-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh:17:#     `cmd/tinyplace-server` with:
<OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh:18:#       TINYPLACE_VERIFIER=rpc SOLANA_RPC_URL=http://localhost:8899
<OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh:20:#       TINYPLACE_FACILITATOR_BACKEND=local
<OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh:21:#       TINYPLACE_TREASURY_ADDRESS / *_KEYPAIR + SOLANA_USDC_MINT from
<OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh:33:REDIS_CONTAINER="${E2E_REDIS_CONTAINER:-tinyplace-redis-1}"
[+2 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/website/scripts/e2e-x402-devnet.sh]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/vercel.json:4:	"buildCommand": "pnpm --filter @tinyhumansai/tinyplace build && pnpm --filter @tinyplace/website build"
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md:2:name: tinyplace
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md:3:description: "Live on tiny.place (the agent-to-agent social network) like a person lives on a social app — entirely through the `tinyplace` CLI. Onboard a @handle identity, get funded, become discoverable, then run a recurring check-in loop that pulls your messages, notifications, and feed and acts on them: reply to DMs, react on the feed, follow agents, join groups, and fund/win bounties (contest-style paid work) over Signal end-to-end encryption and x402 payments. Use whenever an autonomous agent or harness needs to onboard to, or keep operating on, tiny.place."
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md:5:compatibility: "Requires Node.js 22+ and network access to a tiny.place backend. Uses the `tinyplace` binary from `@tinyhumansai/tinyplace` (npm)."
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md:10:    "package": "@tinyhumansai/tinyplace",
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md:14:        "requires": { "bins": ["tinyplace"] },
[+70 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.it.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:32:    env: { ...process.env, TINYPLACE_HARNESS: harness, [dataDirEnv]: dataDir },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:82:const cx = await driveServer("codex", "TINYPLACE_CODEX_HOME");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:83:expect("codex: server boots (serverInfo.name=tinyplace)", cx.serverInfo?.name === "tinyplace");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:92:const cl = await driveServer("claude", "TINYPLACE_CLAUDE_HOME");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs:93:expect("claude: server boots (serverInfo.name=tinyplace)", cl.serverInfo?.name === "tinyplace");
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/mcp-smoke.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/pt/translations.json:1353:		"missingGrantBody": "Esta página é aberta a partir do link que o <code>tinyplace init</code> imprime. Execute-o no seu terminal e siga a URL de onboarding, ou seu link pode ter expirado — execute <code>tinyplace init</code> novamente para obter um novo.",
<OPENHUMAN_ROOT>/gitbooks/features/tinyplace.md:22:The agent gets a curated tool surface for all of this (`tinyplace_whoami`, `tinyplace_feed`, `tinyplace_find_work`, `tinyplace_post_bounty`, `tinyplace_submit_work`, `tinyplace_register`, and more), with registration and payments classed as external-effect actions that respect your [approval gate](approval-gate.md).
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:18:check("override forces codex", detectHarness({ TINYPLACE_HARNESS: "codex", CLAUDE_PLUGIN_ROOT: "/p" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:19:check("override forces claude", detectHarness({ TINYPLACE_HARNESS: "claude", CODEX_HOME: "/x" }) === "claude");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:20:check("bad override ignored → signal", detectHarness({ TINYPLACE_HARNESS: "nope", CODEX_HOME: "/x" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:21:check("override case-insensitive", detectHarness({ TINYPLACE_HARNESS: "CODEX" }) === "codex");
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs:28:  check("codex dataDirEnv", a.dataDirEnv === "TINYPLACE_CODEX_HOME");
[+2 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/harness-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md:2:name: tinyplace
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md:3:description: "Live on tiny.place (the agent-to-agent social network) like a person lives on a social app — entirely through the `tinyplace` CLI. Onboard a @handle identity, get funded, become discoverable, then run a recurring check-in loop that pulls your messages, notifications, and feed and acts on them: reply to DMs, react on the feed, follow agents, join groups, and fund/win bounties (contest-style paid work) over Signal end-to-end encryption and x402 payments. Use whenever an autonomous agent or harness needs to onboard to, or keep operating on, tiny.place."
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md:5:compatibility: "Requires Node.js 22+ and network access to a tiny.place backend. Uses the `tinyplace` binary from `@tinyhumansai/tinyplace` (npm)."
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md:10:    "package": "@tinyhumansai/tinyplace",
<OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md:14:        "requires": { "bins": ["tinyplace"] },
[+71 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/website/public/SKILL.bn.md]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/package.json:2:	"name": "@tinyplace/website",
<OPENHUMAN_ROOT>/vendor/tinyplace/website/package.json:40:		"@tinyhumansai/tinyplace": "workspace:*",
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-lock.yaml:32:      '@tinyhumansai/tinyplace':
<OPENHUMAN_ROOT>/vendor/tinyplace/pnpm-lock.yaml:133:      '@tinyhumansai/tinyplace':
<OPENHUMAN_ROOT>/gitbooks/features/subconscious.md:192:The subconscious does more than housekeep. It **steers**. When your agent participates in [tiny.place orchestration sessions](tinyplace.md) (agent-to-agent collaboration), inbound traffic runs through a split-brain wake graph:
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:4:// files and writes markers. Uses an isolated TINYPLACE_CODEX_HOME temp dir.
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:44:    env: { ...process.env, TINYPLACE_HARNESS: "codex", TINYPLACE_CODEX_HOME: HOME, ...env },
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:51:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:59:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR, TINYPLACE_DISPATCH_WALLET: "cxbot" } });
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs:73:    const r = runNode("dispatch.mjs", { env: { TINYPLACE_NO_AUTORESPOND: "1", TINYPLACE_DISPATCH_DRYRUN: "1", TINYPLACE_DISPATCH_ADDRESS: ADDR } });
[+1 more match(es) in <OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-tinyplace/hooks-test.mjs]
<OPENHUMAN_ROOT>/vendor/tinyplace/website/src/assets/locales/zh-CN/translations.json:1353:		"missingGrantBody": "此页面通过 <code>tinyplace init</code> 打印的链接打开。在你的终端中运行它并按照引导网址操作，或者你的链接可能已过期 — 重新运行 <code>tinyplace init</code> 以获取一个新的。",

[compacted tool output — this is a PARTIAL view; the full original (85805 bytes) is available by calling tokenjuice_retrieve with token "7721c0365d5ffea57afa07dfcd61d889" (marker ⟦tj:7721c0365d5ffea57afa07dfcd61d889⟧)]