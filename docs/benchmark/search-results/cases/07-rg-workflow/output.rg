[search: 500 match(es) across 118 file(s) · top 5-12 per file (adaptive) · full set via retrieve footer]
<OPENHUMAN_ROOT>/src/main.rs:139:            // suppression lives at the `install_workflow_from_url_with_home`
<OPENHUMAN_ROOT>/CONTRIBUTING.md:15:- [Git Workflow](#git-workflow)
<OPENHUMAN_ROOT>/CONTRIBUTING.md:105:- **Windows 10 WSL + classic X11 forwarding** is unsupported for the desktop app. The Tauri/CEF stack can hang, render blank windows, or crash before useful app logs are available. Use native Windows development, or Windows 11 WSLg if you need a Linux GUI workflow. OpenHuman logs a startup warning when it detects WSL with `DISPLAY` set but no `WAYLAND_DISPLAY`/WSLg markers.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:164:These commands cover the most common local workflows from the repository root:
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR Metadata section of the PR template and report any blocked commands with the exact command and error.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:246:├── docs/                   # Internal and workflow docs
<OPENHUMAN_ROOT>/CONTRIBUTING.md:250:└── CLAUDE.md               # Additional contributor and workflow guidance
<OPENHUMAN_ROOT>/CONTRIBUTING.md:259:## Git Workflow
<OPENHUMAN_ROOT>/CONTRIBUTING.md:287:4. Update docs with code whenever behavior, commands, or contributor workflow changes.
[+1 more match(es) in <OPENHUMAN_ROOT>/CONTRIBUTING.md ⟦tj:26956379d6b2d7028575cd0be50d5909⟧]
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:213:        // Workflow
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:215:            DomainEvent::WorkflowLoaded {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:219:            "workflow",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:222:            DomainEvent::WorkflowStopped {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:228:            DomainEvent::WorkflowStartFailed {
[+8 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs ⟦tj:63b95799cc3c184d2de2ffe18426c489⟧]
<OPENHUMAN_ROOT>/src/core/observability.rs:2764:/// `install_workflow_from_url_with_home` fetches a user/catalog-supplied
<OPENHUMAN_ROOT>/AGENTS.md:66:**CI build topology**: full-suite E2E is **build-once-then-fanout** on all three OSes — `build-{linux,macos,windows}-full` compile/bundle the app once and upload it as a per-run workflow artifact, and the shard jobs (`e2e-*-full`) `needs:` that job and download it instead of each shard rebuilding on a cold cache (`.github/workflows/e2e-reusable.yml`). Linux desktop packaging (`build-desktop.yml`) does a **single** `cargo tauri build`: libcef.so is resolved from the restored CEF cache (or a targeted `cargo build -p cef-dll-sys` prewarm on a cold cache) rather than a throwaway `--no-bundle` full build. The root core crate and the Tauri shell are still **separate Cargo worlds** (two `Cargo.lock`, two `target/`); converging them into one workspace is tracked as follow-up in #3877.
<OPENHUMAN_ROOT>/AGENTS.md:88:PRs need **≥ 80% coverage on changed lines** via `diff-cover` over Vitest + `cargo-llvm-cov` lcov. Enforced by the coverage jobs (`frontend-coverage`/`rust-core-coverage`/`rust-tauri-coverage`/`coverage-gate`) in `.github/workflows/ci-lite.yml`.
<OPENHUMAN_ROOT>/AGENTS.md:257:## Feature design workflow
<OPENHUMAN_ROOT>/AGENTS.md:272:## Git workflow
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:125:## Workflows you can see
<OPENHUMAN_ROOT>/README.md:127:Heavily inspired by n8n and Zapier, [workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) bring the same visual, trigger-driven automation to your agent, except the agent builds them for you. Ask for an automation and it proposes one: a [tinyflows](https://github.com/tinyhumansai/tinyflows) graph you review on a visual canvas before saving.
<OPENHUMAN_ROOT>/README.md:130: <img src="./gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman workflow canvas">
<OPENHUMAN_ROOT>/README.md:133:> The agent proposes the workflow; you review it on a canvas and save it.
<OPENHUMAN_ROOT>/README.md:135:Saved workflows are durable and trigger-driven. They fire on schedules, webhooks, or channel events, survive restarts, and gate side effects behind approvals.
<OPENHUMAN_ROOT>/README.md:139:High-level comparison (products evolve, so verify against each vendor). OpenHuman is built to **minimize vendor sprawl**, keep **workflow knowledge on-device**, and give the agent a **persistent memory** of your data, not only chat.
<OPENHUMAN_ROOT>/README.md:150:| **Workflows**          | 🚫 None           | ⚠️ Scripts        | ⚠️ Scripts        | 🚀 Visual, durable, agent-proposed, approval-gated                                                       |
<OPENHUMAN_ROOT>/README.md:161:New contributor? Start with [`CONTRIBUTING.md`](./CONTRIBUTING.md) for the fork/PR workflow and local validation commands, or use the copy-paste AI-agent prompt in [`CONTRIBUTING-BEGINNERS.md`](./CONTRIBUTING-BEGINNERS.md#optional--let-an-ai-coding-agent-guide-you). The short path is:
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:460:    /// non-trigger node settles, so the Workflows UI can show a run advancing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:477:    WorkflowLoaded { skill_id: String, runtime: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:479:    WorkflowStopped { skill_id: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:481:    WorkflowStartFailed { skill_id: String, error: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:483:    WorkflowExecuted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:491:    /// The set of installed skills/workflows changed (install / uninstall /
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:495:    WorkflowsChanged { reason: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1335:            Self::WorkflowLoaded { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1336:            | Self::WorkflowStopped { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1337:            | Self::WorkflowStartFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1338:            | Self::WorkflowExecuted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1497:            Self::WorkflowStartFailed { .. } => "WorkflowStartFailed",
[+5 more match(es) in <OPENHUMAN_ROOT>/src/core/event_bus/events.rs ⟦tj:3eee264a045fca5f59af0f295f9d459b⟧]
<OPENHUMAN_ROOT>/plan.md:20:   most of `scripts/__tests__/`, and the Pester Windows-install test are invoked by no workflow.
<OPENHUMAN_ROOT>/plan.md:139:### P1 — core workflows
<OPENHUMAN_ROOT>/plan.md:197:**Holes found (verified by exhaustive grep of package.json + workflows):**
<OPENHUMAN_ROOT>/plan.md:240:- Legacy `e2e.yml` / `e2e-playwright.yml` / `test.yml` are now `workflow_dispatch`-only — the
<OPENHUMAN_ROOT>/plan.md:243:### Findings from §5 that are STILL open (re-verified by grep on the new workflows)
<OPENHUMAN_ROOT>/plan.md:247:| Orphaned harness self-tests (`scripts/mock-api/socket.{auth,transport}.test.mjs`, most of `scripts/__tests__/`, Pester `test:install-ps1`) | **Still orphaned.** No workflow references them in either lane (`pnpm docs:test` remains the only `scripts/__tests__` entry point; no `pwsh` anywhere). |
<OPENHUMAN_ROOT>/plan.md:251:| `check-domain-e2e-coverage.mjs` | Still not wired into any workflow (`check-coverage-matrix.mjs` runs in `pr-quality.yml`). |
<OPENHUMAN_ROOT>/plan.md:360:  ≥1 package.json script or workflow.
<OPENHUMAN_ROOT>/plan.md:458:*(Re-audited separately after the original workflow auditor returned an unusable result. These
<OPENHUMAN_ROOT>/src/core/socketio.rs:1053:                // truth and the Workflows UI keeps a 2s poller as fallback, so
<OPENHUMAN_ROOT>/docs/README.ko.md:74:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: 에이전트가 자동화를 제안하면 캔버스에서 검토하고 저장하면 됩니다. 내구성 있고, 트리거 기반이며, 승인 게이트를 거치는 실행이 오픈 소스 [tinyflows](https://github.com/tinyhumansai/tinyflows) 위에서 동작합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:125:n8n과 Zapier에서 깊은 영감을 받은 [워크플로우](https://tinyhumans.gitbook.io/openhuman/features/workflows)는 동일한 시각적, 트리거 기반 자동화를 에이전트에 가져옵니다. 다만 에이전트가 대신 만들어 준다는 점이 다릅니다. 자동화를 요청하면 에이전트가 하나를 제안합니다: 저장하기 전에 시각적 캔버스에서 검토하는 [tinyflows](https://github.com/tinyhumansai/tinyflows) 그래프입니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:128: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman 워크플로우 캔버스">
<OPENHUMAN_ROOT>/e2e/docker-compose.yml:4:# This mirrors the `e2e-linux` job in `.github/workflows/e2e.yml` so any
<OPENHUMAN_ROOT>/src/core/all.rs:122:    // Saved automation workflows (tinyflows graphs): create/get/list/update/delete/run
<OPENHUMAN_ROOT>/src/core/all.rs:159:    // Interactive approval workflow (#1339 — gate external-effect tool calls)
<OPENHUMAN_ROOT>/src/core/all.rs:217:    controllers.extend(crate::openhuman::workflows::all_workflows_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:225:    // Workflow tool registry
<OPENHUMAN_ROOT>/src/core/all.rs:330:    // Durable dynamic workflow runs — definitions + read surface over the run ledger
<OPENHUMAN_ROOT>/src/core/all.rs:332:        .extend(crate::openhuman::agent_orchestration::all_workflow_run_registered_controllers());
<OPENHUMAN_ROOT>/src/core/all.rs:430:    schemas.extend(crate::openhuman::workflows::all_workflows_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:499:    // Durable dynamic workflow runs
<OPENHUMAN_ROOT>/src/core/all.rs:500:    schemas.extend(crate::openhuman::agent_orchestration::all_workflow_run_controller_schemas());
<OPENHUMAN_ROOT>/src/core/all.rs:548:        "flows" => Some("Create, store, and run automation workflows."),
<OPENHUMAN_ROOT>/src/core/all.rs:574:        "workflows" => Some("Discovered workflows (WORKFLOW.md/SKILL.md bundles) and their resources."),
<OPENHUMAN_ROOT>/src/core/all.rs:600:            "Durable agent and workflow run state, child lineage, events, telemetry, and checkpoint references.",
[+2 more match(es) in <OPENHUMAN_ROOT>/src/core/all.rs ⟦tj:6a53a6fb4320accc883b21e1a80a54cd⟧]
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1858:                // Prune legacy bundled skills (dev-workflow / github-issue-crusher
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1862:                crate::openhuman::workflows::registry::prune_legacy_default_workflows(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2263:        // workflows still dispatch when no realtime channel is configured or
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2584:    // --- Triggered-workflow subscriber ---
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2587:    // `OPENHUMAN_DISABLE_CHANNEL_LISTENERS=1`). Without this, any workflow
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2592:    crate::openhuman::workflows::bus::ensure_triggered_workflow_subscriber(&workspace_dir);
<OPENHUMAN_ROOT>/e2e/run-local.sh:5:# Mirrors `.github/workflows/e2e.yml` `e2e-linux` step-for-step inside
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:74:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**：智能体提出自动化方案；你在画布上审阅并保存。持久化、触发器驱动、审批把关的运行，基于开源的 [tinyflows](https://github.com/tinyhumansai/tinyflows)。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:125:深受 n8n 和 Zapier 的启发，[工作流](https://tinyhumans.gitbook.io/openhuman/features/workflows)将同样的可视化、触发器驱动的自动化带给你的智能体。不同的是，智能体会替你构建它们。向智能体请求一个自动化，它就会提出方案：一张 [tinyflows](https://github.com/tinyhumansai/tinyflows) 图，你可以在可视化画布上审阅后再保存。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:128: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman 工作流画布">
<OPENHUMAN_ROOT>/docs/README.de.md:74:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: der Agent schlägt die Automatisierung vor; du prüfst sie auf einer Canvas und speicherst. Dauerhafte, trigger-gesteuerte, freigabe-gesicherte Läufe auf dem quelloffenen [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/docs/README.de.md:123:## Workflows, die du sehen kannst
<OPENHUMAN_ROOT>/docs/README.de.md:125:Stark inspiriert von n8n und Zapier bringen [Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) dieselbe visuelle, trigger-gesteuerte Automatisierung zu deinem Agenten, nur dass der Agent sie für dich baut. Bitte um eine Automatisierung, und er schlägt eine vor: einen [tinyflows](https://github.com/tinyhumansai/tinyflows)-Graphen, den du vor dem Speichern auf einer visuellen Canvas prüfst.
<OPENHUMAN_ROOT>/docs/README.de.md:128: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman-Workflow-Canvas">
<OPENHUMAN_ROOT>/docs/README.de.md:131:> Der Agent schlägt den Workflow vor; du prüfst ihn auf einer Canvas und speicherst ihn.
<OPENHUMAN_ROOT>/docs/README.de.md:133:Gespeicherte Workflows sind dauerhaft und trigger-gesteuert: sie feuern auf Zeitpläne, Webhooks oder Kanal-Events, überleben Neustarts und sichern Seiteneffekte hinter Freigaben ab.
<OPENHUMAN_ROOT>/docs/README.de.md:137:Übersichtsvergleich (Produkte entwickeln sich weiter, bitte beim jeweiligen Anbieter verifizieren). OpenHuman ist darauf ausgelegt, **Vendor-Wildwuchs zu reduzieren**, **Workflow-Wissen auf dem Gerät zu halten** und dem Agenten eine **persistente Erinnerung** an deine Daten zu geben, nicht nur an den Chat.
<OPENHUMAN_ROOT>/docs/README.de.md:148:| **Workflows**          | 🚫 keine              | ⚠️ Skripte         | ⚠️ Skripte         | 🚀 visuell, dauerhaft, agent-vorgeschlagen, freigabe-gesichert                                           |
<OPENHUMAN_ROOT>/docs/README.de.md:159:Neu hier? Beginne mit [`CONTRIBUTING.md`](../CONTRIBUTING.md) für den Fork-/PR-Workflow und die lokalen Prüfbefehle, oder nutze den Copy-Paste-Prompt für KI-Coding-Agenten in [`CONTRIBUTING-BEGINNERS.md`](../CONTRIBUTING-BEGINNERS.md#optional--let-an-ai-coding-agent-guide-you). Der kurze Weg:
<OPENHUMAN_ROOT>/e2e/docker-local-bootstrap.sh:8:# CI workflow runs (`.github/workflows/e2e.yml` → `e2e-linux`), but only
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:77:- **CI (`.github/workflows/ci.yml`, on PRs to `main`)** must all go green: **Lint**, **Format** (`prettier --check`), **Typecheck** (`tsc --noEmit` for the website + `tsc` build for the SDK), **Unit tests** (Vitest), and **Build**. A separate **E2E** workflow (`e2e.yml`) runs Playwright on PRs.
<OPENHUMAN_ROOT>/vendor/tinyplace/CLAUDE.md:78:- **Releasing is manual and consolidated** in **`.github/workflows/release.yml`** (`workflow_dispatch`): tick the checkboxes for the targets to ship (TypeScript / Python / Rust SDKs and/or the website) and choose a `patch|minor|major` bump. It bumps the selected versions (`scripts/release.sh` → `sdk/bump-versions.mjs --only …`), commits to `main`, publishes each selected SDK to npm/PyPI/crates.io, and for the website pushes a `website-vX.Y.Z` tag. **`deploy-website.yml`** (on that tag, or invoked by the release run) deploys the website to Vercel via the CLI and attaches a GitHub Release. Vercel's commit auto-deploy is **off** (`website/vercel.json` → `git.deploymentEnabled: false`); production only ships through the tag. The Vercel project's **Root Directory is `website`** (so `vercel.json` lives there and Next is detected from `website/package.json`). See [`docs/releasing.md`](docs/releasing.md).
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:28:- CI (`test-reusable.yml`, `build-ci-image.yml`, release workflows) already checks out submodules recursively. **No new CI plumbing needed.**
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:43:### 0.3 Submodule contribution workflow (how engine changes are made)
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:95:**0.4 Toolchain baseline.** Add `tinycortex = { version = "0.1" }` under `[dependencies]` (activating the existing `[patch.crates-io]` override); align `rusqlite` versions between host and crate (both must link one bundled SQLite); check edition (crate is 2021), feature flags, and that **both Cargo worlds** (root crate and `app/src-tauri`) compile with the dep active; confirm `GGML_NATIVE=OFF` macOS builds. Verify the release workflows' submodule-init covers `vendor/tinycortex` (the tinyagents wave needed +5-line fixes there).
<OPENHUMAN_ROOT>/docs/tinycortex-memory-migration-plan.md:129:## 4. Git / PR / submodule workflow
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:88:- **[ورک فلوز](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: ایجنٹ آٹومیشن تجویز کرتا ہے؛ آپ اسے کینوس پر جائزہ لے کر محفوظ کرتے ہیں۔ اوپن سورس [tinyflows](https://github.com/tinyhumansai/tinyflows) پر پائیدار، ٹرگر سے چلنے والے، منظوری سے محفوظ رنز۔
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:155:n8n اور Zapier سے گہرے متاثر، [ورک فلوز](https://tinyhumans.gitbook.io/openhuman/features/workflows) وہی بصری، ٹرگر سے چلنے والی آٹومیشن آپ کے ایجنٹ تک لاتے ہیں۔ فرق یہ ہے کہ ایجنٹ انہیں آپ کے لیے بناتا ہے۔ کسی آٹومیشن کی درخواست کریں اور ایجنٹ ایک تجویز کرتا ہے: ایک [tinyflows](https://github.com/tinyhumansai/tinyflows) گراف جس کا آپ محفوظ کرنے سے پہلے بصری کینوس پر جائزہ لیتے ہیں۔
<OPENHUMAN_ROOT>/docs/README.ur-pk.md:162: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman ورک فلو کینوس">
<OPENHUMAN_ROOT>/vendor/tinyplace/scripts/release.sh:13:# "Release" GitHub Actions workflow, but works locally too.
<OPENHUMAN_ROOT>/vendor/tinyplace/scripts/release.sh:166:# Emit machine-readable outputs for the GitHub Actions workflow.
<OPENHUMAN_ROOT>/gitbooks/legal/privacy-policy.md:30:• Provide contextual assistance across files, applications, or workflows
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:74:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: エージェントが自動化を提案し、あなたはキャンバス上でレビューして保存します。オープンソースの [tinyflows](https://github.com/tinyhumansai/tinyflows) 上で、永続的・トリガー駆動・承認ゲート付きの実行が行われます。
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:125:n8n と Zapier に強くインスパイアされた[ワークフロー](https://tinyhumans.gitbook.io/openhuman/features/workflows)は、同じビジュアルでトリガー駆動の自動化をあなたのエージェントにもたらします。ただし、それを構築するのはエージェント自身です。自動化を依頼すると、エージェントが提案してくれます: 保存する前にビジュアルキャンバス上でレビューできる [tinyflows](https://github.com/tinyhumansai/tinyflows) グラフです。
<OPENHUMAN_ROOT>/docs/README.ja-JP.md:128: <img src="../gitbooks/.gitbook/assets/workflows.png" alt="OpenHuman のワークフローキャンバス">
<OPENHUMAN_ROOT>/gitbooks/legal/terms-of-use.md:14:The Service is a system-level AI assistant designed to help users complete tasks, automate workflows, and interact with files, applications, and system resources based on explicit user instructions. The Service acts only on user requests and does not operate autonomously.
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:17:| `src/openhuman/agent_orchestration/` | ~25,800 | Product control plane over sub-agents: in-memory session, detached-run registry, workflow runs, agent teams, command center, worktree isolation, RPC/tools surface |
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:51:### 0.4 Contribution workflow (same convention as tinycortex)
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:77:| `agent_orchestration/workflow_runs/` phase-DAG validation + `agent_teams/` dependency-DAG/atomic-claim/quality-gate logic | evaluate upstreaming the *validation/scheduling slices* as graph extensions; durability (`session_db::run_ledger`) and RPC stay host-side | `graph/` |
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:152:2. Replace `AgentOrchestrationSession` (`ops.rs`, 679 L) with crate `SubAgentSession` + `TaskStore`; consumers (`workflow_runs/engine.rs`, `agent_teams/runtime.rs`) move onto the crate API.
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:159:### Phase 5 — Workflow/team generic slices (optional, evaluate after Phase 4)
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:161:1. Upstream the **validation** slices: workflow phase-DAG structural/cycle validation (`workflow_runs/{types,ops}`), team dependency validation + atomic CAS claim + quality-gate state machine (`agent_teams/ops.rs`) — as `graph/` extensions if they generalize cleanly; otherwise leave host-side. Durability (`session_db::run_ledger`) and `command_center/` stay host-side regardless.
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:194:- `tools/traits.rs:15` — `ToolScope::AgentOnly` is a dead variant; `ToolCategory::Workflow` is pinned to wire `"skill"` (documented tech-debt to resolve before porting the type).
<OPENHUMAN_ROOT>/docs/tinyagents-port-plan.md:197:- `user_filter.rs:168-188` — `skill_manage` and `workflow_manage` families carry identical `rust_names` (deliberate alias, duplication footgun); `web_search` → `"web_search_tool"` name mapping is drift-prone against the `search` domain.
<OPENHUMAN_ROOT>/vendor/tinychannels/SECURITY.md:43:- unsafe workflows caused by downstream applications granting broad authority
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:61:| U3  | **No authoring entry** — "New workflow" navigates to `/chat` with a TODO; empty-state copy promises canvas creation that doesn't exist                                   | `pages/FlowsPage.tsx`                    |
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:91:| Error workflow / retry per node                       | ✅ on_error/retry/backoff                                                    | —                 |
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:101:Our structural advantages to preserve: the agent can _author_ workflows conversationally (`propose_workflow` → proposal card → single human "Save & enable" persistence gate), everything runs under the security policy/approval-gate substrate, and secrets never enter the engine (opaque `connection_ref`).
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:182:- **3c. Validation UX**: new RPC `openhuman.flows_validate(graph)` (thin wrapper over `ops::validate_and_migrate_graph`, same path `propose_workflow` uses) → inline canvas errors (missing trigger, cycle, invalid config on node X) before save.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:193:- **4d. Import/export**: export flow JSON; import with `migrate()` + validate. **n8n importer** (host-side, best-effort): map n8n workflow JSON → `WorkflowGraph` for the overlapping vocabulary (IF→condition, Switch, Merge, SplitOut, HTTP Request, Code, Schedule/Webhook triggers; `={{...}}` → `=` jq where trivially translatable); unmapped nodes land as annotated placeholder nodes rather than failing the import.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:204:- New builtin definition `workflow-builder` (Worker tier): system prompt specialized for workflow design — knows the 12 node kinds, `=`/jq expression semantics, port/edge rules, trigger kinds and which ones are live, error-handling config (`on_error`/`retry`), and the "propose, never persist" invariant. Prompt ships in `src/openhuman/agent/prompts/` like the other bundled prompts.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:215:| `get_workflow_run` | new (read-only) | Read a failed run's steps so the agent can debug/repair a workflow from an error report |
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:225:- **Canvas copilot panel**: on `/flows/:id` (and on drafts), a side panel chat bound to the same agent with the current graph injected as context. Each agent proposal updates a **draft overlay** on the canvas (diff-style: added nodes highlighted, removed ones ghosted) — accept/reject applies it to the local draft from Phase 3d. This is `revise_workflow` in a loop: "add a Slack notification on failure", "make the schedule weekdays only", "split this into a sub-workflow".
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:226:- **Repair entry point**: from a failed run in `FlowRunInspectorDrawer`, "Fix with agent" opens the copilot with the run's failing step context preloaded (`get_workflow_run`).
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:260:- **Deprecate then delete**: mark `openhuman.workflows_*` deprecated for one release (RPC responses carry a deprecation notice), then remove `src/openhuman/workflows/`, its controllers from `src/core/all.rs`, the frontend clients/pages, and the `/workflows/new`//`workflows/run` routes (bare `/workflows` already redirects to `/settings/automations`).
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:270:Backend: ☐ Composio trigger-subscription lifecycle (auto-provision on enable) · ☐ trigger catalog RPC · ☐ loud validation warning for unfired trigger kinds · ☐ raw-webhook dispatch (deferred backlog) · ☐ `flows_cancel_run` · ☐ resume-with-rejection/deny · ☐ live `RunObserver` + incremental step persistence · ☐ `FlowRunProgress` socket events · ☐ `flows_validate` RPC · ☐ `flows_list_connections` · ☐ Composio connected-account resolution · ☐ HTTP credential resolution · ☐ toolkit allow-list fix · ☐ `chat_message` trigger dispatch · ☐ sub-workflow by id · ☐ parked-run TTL sweep · ☐ JSON-RPC E2E suite.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md:274:Agent authoring: ☐ `workflow-builder` builtin `AgentDefinition` + specialized prompt · ☐ `revise_workflow` tool · ☐ read-only `list_workflows`/`get_workflow`/`get_workflow_run` tools · ☐ `list_flow_connections` tool · ☐ `search_tool_catalog` tool · ☐ `dry_run_workflow` (sandboxed) · ☐ delegation routing from main agent · ☐ FlowsPage prompt bar · ☐ canvas copilot panel with draft diff overlay · ☐ "Fix with agent" from failed runs.
[+47 more match(es) in <OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/README.md ⟦tj:bb263bb8778c2f30275ff5bdd809b691⟧]
<OPENHUMAN_ROOT>/vendor/tinychannels/CONTRIBUTING.md:68:Feature requests should explain the channel workflow they unlock, the public
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:1:# Trigger taxonomy & the cron → workflows unification
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:3:> Companion to [README.md](README.md) (the tinyflows completion POA). This doc answers two questions: **what kinds of triggers can this platform offer**, and **how do we migrate cron so that "everything automated is a workflow"**.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:9:**End state**: *Workflows are the single user-facing automation concept.* A trigger is just a subscription that seeds a flow run; the trigger catalog is a curated projection of the event bus plus time and manual entry points.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:13:`tinyflows::model::TriggerKind` (declarative — the host fires them): `manual`, `schedule`, `webhook`, `app_event`, `form`, `execute_by_workflow`, `chat_message`, `evaluation`, `system`.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:23:| `chat_message`, `form`, `execute_by_workflow`, `evaluation`, `system` | ❌ no dispatcher |
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:42:- **Form**: a manual run with a typed input schema. Trigger config declares fields; the UI renders a form; submission becomes the trigger payload. This is n8n's Form trigger and gives every workflow a shareable "mini-app" entry point. Cheap to ship: validation + a generated form in the canvas/run dialog.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:73:Examples: "when a meeting summary is generated, extract action items → create tasks → email attendees" — the flagship demo workflow.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:85:Examples: self-healing/maintenance workflows ("on health degradation, run doctor and report"). These should default to `require_approval` off but notification-heavy.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:91:The subconscious domain is itself a mini automation engine (evaluate → escalate). Long-term convergence candidate: an escalation's *action* becomes "run flow X", making workflows the actuator layer for subconscious signals rather than a parallel system.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:93:### 2.11 Workflow-to-workflow (`execute_by_workflow`, `evaluation`)
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:95:- `execute_by_workflow`: fired when another flow's `sub_workflow`/by-id call targets this flow (README Phase 4b) — enables composition and shared "library" flows.
<OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md:96:- `evaluation`: reserved for eval-harness runs of a flow (regression-test a workflow against recorded inputs); pairs with `dry_run_workflow` (README Phase 5b).
[+5 more match(es) in <OPENHUMAN_ROOT>/docs/plans/tinyflows-integration/triggers.md ⟦tj:a55f8a3f735825a34275a45537bd3834⟧]
<OPENHUMAN_ROOT>/gitbooks/guides/doctor-assistant.md:10:**Goal:** tailor OpenHuman for a clinician's workflow: a persona that speaks the part, memory scoped to the right sources, and privacy settings appropriate for sensitive information.
<OPENHUMAN_ROOT>/gitbooks/guides/doctor-assistant.md:58:Add only the integrations relevant to the workflow (e.g. a reference/notes source), and **not** anything carrying data you're not cleared to process. Every integration is a separate, revocable OAuth grant.
<OPENHUMAN_ROOT>/vendor/tinychannels/README.md:10: <a href="https://github.com/tinyhumansai/tinychannels/actions/workflows/ci.yml"><img src="https://github.com/tinyhumansai/tinychannels/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
<OPENHUMAN_ROOT>/gitbooks/guides/README.md:38:* **Privacy implications**: what stays on your machine and what is sent to the OpenHuman backend or a model provider for this workflow, in plain language.
<OPENHUMAN_ROOT>/gitbooks/guides/README.md:40:* **Success checks**: how to confirm the workflow is actually working, not just "looks done".
<OPENHUMAN_ROOT>/gitbooks/README.md:5:  workflows, and a deep researcher across 118+ connected services.
<OPENHUMAN_ROOT>/gitbooks/README.md:13:OpenHuman is an open-source AI assistant built to be three things most assistants aren't: **a brain** (a persistent, local, readable memory of your world); **a fantastic orchestrator** (durable agent graphs, visual workflows, sub-agent fleets, and [end-to-end encrypted agent-to-agent sessions](features/orchestration.md)); and **a deep researcher** (it sweeps your data and the web before you finish asking). Built on Rust + Tauri, licensed under GNU GPL3.
<OPENHUMAN_ROOT>/gitbooks/README.md:20:* **An** [**Obsidian-style wiki**](features/obsidian-wiki/) **on top of it.** The same chunks the agent reasons over land as `.md` files in a vault you can open in [Obsidian](https://obsidian.md), browse, edit, and link by hand. Inspired by [Karpathy's obsidian-wiki workflow](https://x.com/karpathy/status/2039805659525644595). You can't trust a memory you can't read.
<OPENHUMAN_ROOT>/gitbooks/README.md:25:* [**Workflows**](features/workflows.md)**.** Durable, visual automations on the open-source tinyflows engine. Describe the automation in chat, the agent *proposes* a workflow graph, you review it on a canvas and save it. Flows fire on schedules or live app events, pause at approval gates, and resume exactly where they stopped, with full step-by-step run history.
<OPENHUMAN_ROOT>/packages/homebrew/openhuman.rb:2:# Placeholders replaced by .github/workflows/release-packages.yml before commit.
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-6-tinyagents-reuse.md:4:durable-workflow runtime the orchestration wake path uses. This phase tracks
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-6-tinyagents-reuse.md:9:merged commit (same workflow as `docs/plans/rlm-workflows/phase-2-tinyagents.md`).
<OPENHUMAN_ROOT>/docs/plans/subconscious-factory/phase-6-tinyagents-reuse.md:39:   from the rlm-workflows plan — lets a newer tick abort an in-flight one
<OPENHUMAN_ROOT>/vendor/tinychannels/docs/spec/openclaw-hermes-channel-porting.md:85:| `irc`            | `irc`            | `IRC_HOST`, `IRC_PORT`, `IRC_TLS`, `IRC_NICK`, `IRC_CHANNELS`, auth vars                | IRC channel and DM workflow.                                                  |
<OPENHUMAN_ROOT>/vendor/tinychannels/docs/spec/openclaw-hermes-channel-porting.md:100:| `tlon`           | `tlon`           | Tlon/Urbit setup                                                                        | Tlon/Urbit chat workflows.                                                    |
<OPENHUMAN_ROOT>/vendor/tinychannels/docs/spec/openclaw-hermes-channel-porting.md:101:| `twitch`         | `twitch`         | `OPENCLAW_TWITCH_ACCESS_TOKEN`                                                          | Twitch chat and moderation workflows.                                         |
<OPENHUMAN_ROOT>/gitbooks/features/notifications-and-activity.md:69:| **Automations**         | Workflows the agent runs on your behalf (the workflows panel)                                  |
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-3-rlm-domain.md:40:  (use `agent_query` instead), `run_workflow`/`await_workflow`. Because
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-3-rlm-domain.md:76:   `workflows::run_log::register_run_cancel`-style bookkeeping tied to the
<OPENHUMAN_ROOT>/gitbooks/features/cloud-deploy.md:557:To redeploy automatically on every push to `main`, add a workflow file at
<OPENHUMAN_ROOT>/gitbooks/features/cloud-deploy.md:558:`.github/workflows/fly-deploy.yml`:
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-2-tinyagents.md:14:(`workflows::run_log::cancel_run`) or the agent turn aborts.
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-6-tests.md:18:- excluded tools (`rlm`, `spawn_*`, `run_workflow`) absent from registry;
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:450:/** One step in a `WorkflowProposal`'s summary — a non-trigger node. */
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:451:export interface WorkflowProposalStep {
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:461: * A candidate automation workflow the agent proposed via the `propose_workflow`
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:462: * tool (issue B4 — agent-first Workflow authoring). VALIDATED but never
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:464: * user must click "Save & enable" on `WorkflowProposalCard` to actually
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:465: * persist it via `openhuman.flows_create`. Parsed from the `propose_workflow`
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:469:export interface WorkflowProposal {
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:472:  /** The validated tinyflows WorkflowGraph, ready to hand to `flows_create` as-is. */
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:480:    steps: WorkflowProposalStep[];
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:582:   * Thread-scoped candidate workflow proposed by the `propose_workflow` agent
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:584:   * on `WorkflowProposalCard`. Unlike `pendingApprovalByThread` /
<OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts:590:  pendingWorkflowProposalsByThread: Record<string, WorkflowProposal>;
[+13 more match(es) in <OPENHUMAN_ROOT>/app/src/store/chatRuntimeSlice.ts ⟦tj:9104b14d7015b87c52a9a5eea297a4bf⟧]
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-5-hardening.md:61:  `ToolAdapter` for inner calls; add coarse start/finish workflow events for
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/README.md:1:# RLM — Language-Based Workflows (Rhai/`.ragsh`) Integration Plan
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/README.md:6:OpenHuman Rust core, so the orchestrator agent can *write its own workflow
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/README.md:9:Workflows and Recursive Language Models (RLMs).
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/README.md:14:`spawn_subagent`, `spawn_parallel_agents`, `run_workflow` (WORKFLOW.md
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/README.md:48:   prompt/docs surfacing, and tests. Branch: `feat/rlm-language-workflows`,
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/README.md:187:Prior work including Titans/MIRAS, GraphRAG, and MemoryBank demonstrates the importance of structured memory and retention-aware mechanisms. TinyCortex extends this direction with an operational conscious-intelligence loop that integrates ingestion, interval recall, action policy, and write-back reweighting in one production workflow.
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-1-research.md:106:  `workflows::run_log::register_run_cancel(run_id) -> CancellationToken`.
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-1-research.md:113:  `Workflow*` events).
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-1-research.md:118:  beside `workflows/`, `flows/`, `tinyflows/`, `agent_orchestration/`.
<OPENHUMAN_ROOT>/vendor/tinycortex/paper/main.tex:372:reweighting in one production workflow.
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-4-rlm-tool.md:19:    "script":       { "type": "string",  "description": "Rhai workflow cell to evaluate" },
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-4-rlm-tool.md:50:- `display_label` → "running RLM workflow"; `display_detail` → first line of
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-4-rlm-tool.md:70:- Add a "Language workflows (rlm)" section to
<OPENHUMAN_ROOT>/src/openhuman/channels/tests/prompt.rs:167:    let skills = vec![crate::openhuman::workflows::Workflow {
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-7-delivery.md:15:- Repo: `tinyhumansai/openhuman`, branch `feat/rlm-language-workflows` off
<OPENHUMAN_ROOT>/docs/plans/rlm-workflows/phase-7-delivery.md:17:  `--head senamakel:feat/rlm-language-workflows` against upstream.
<OPENHUMAN_ROOT>/app/src/store/__tests__/chatRuntimeSlice.test.ts:376:            kind: 'workflow_child',
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/README.md:4:  vault you can open and edit. Inspired by Karpathy's obsidian-wiki workflow.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/README.md:14:The design is directly inspired by [Andrej Karpathy's obsidian-wiki workflow](https://x.com/karpathy/status/2039805659525644595): a personal wiki where every interesting thing in your life ends up as a linkable note.
<OPENHUMAN_ROOT>/gitbooks/features/obsidian-wiki/memory-tree.md:142:* **RPC** - `openhuman.memory_tree_ingest` for advanced workflows.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/registry/mod.rs:9://! plans safe to compile — a self-authored workflow can only bind to
<OPENHUMAN_ROOT>/vendor/tinyagents/src/registry/component/types.rs:2://! by-name handles a recursive workflow uses to address sub-capabilities.
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:159:    crate::openhuman::workflows::bus::register_workflow_cleanup_subscriber();
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:430:    let skills = crate::openhuman::workflows::load_workflow_metadata(&workspace);
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:432:    // Install the triggered-workflow subscriber now that workflows are
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:433:    // discovered — otherwise any workflow declaring `triggers:` is silently
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:437:    crate::openhuman::workflows::bus::ensure_triggered_workflow_subscriber(&workspace);
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:510:    // Filter out Workflow-category tools (e.g. Composio, Apify) from the
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:515:        .filter(|t| t.category() != crate::openhuman::tools::traits::ToolCategory::Workflow)
<OPENHUMAN_ROOT>/src/openhuman/channels/runtime/startup.rs:829:    // schedule/app-event workflows undispatched (issue B2 review).
<OPENHUMAN_ROOT>/gitbooks/features/rewards-and-referrals.md:91:> GitHub-based contributor rewards are a **separate** mechanism: a GitHub Actions workflow (`.github/workflows/contributor-rewards.yml`) that posts a Discord/merch invite comment when a contributor's first PR merges. It is not part of the in-app Rewards screen and uses no in-app GitHub OAuth.
<OPENHUMAN_ROOT>/vendor/tinyagents/src/registry/capability/types.rs:2://! addressable by name — the data structure that lets a workflow reference
<OPENHUMAN_ROOT>/vendor/tinyjuice/CONTRIBUTING.md:72:Feature requests should explain the agent workflow they unlock, the public API
<OPENHUMAN_ROOT>/vendor/tinyjuice/README.md:10: <a href="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml"><img src="https://github.com/tinyhumansai/tinyjuice/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
<OPENHUMAN_ROOT>/scripts/debug/agent-prepare-context-audit.mjs:444:  "list_workflows",
<OPENHUMAN_ROOT>/scripts/debug/agent-prepare-context-audit.mjs:552:  "list_workflows",
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/tinyjuice-integration-spec.md:97:Use the CLI in explicit workflows, not hidden build steps.
<OPENHUMAN_ROOT>/gitbooks/features/native-tools/system-and-utilities.md:28:* The bits of a workflow that don't fit a richer tool family.
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/openhuman-integration-plan.md:233:- ML text compression can remove workflow tags or instructions. It should stay
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce_tests.rs:926:    let json_line = r#"{"databaseId":999,"displayTitle":"My Workflow Run","status":"completed","conclusion":"success","headBranch":"main"}"#;
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce_tests.rs:935:        result.inline_text.contains("#999") || result.inline_text.contains("My Workflow Run"),
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce_tests.rs:1253:// --- gh json with workflowName field ---
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce_tests.rs:1256:fn gh_json_workflow_name_field() {
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce_tests.rs:1257:    let json_line = r#"{"databaseId":100,"workflowName":"CI/CD Pipeline","status":"in_progress"}"#;
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:234:Before expanding ML compression, TinyJuice should protect custom workflow tags.
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-improvement-ingestion-spec.md:308:  - tagged workflow text
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/content-compressor-roadmap.md:199:- Custom workflow tags survive ML compression byte-for-byte.
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:4:  workflows, sub-agent fleets, a split-brain always-on layer, and end-to-end
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:19:Every agent turn runs on [tinyagents](https://github.com/tinyhumansai/tinyagents), our open-source graph engine. Multi-step work compiles to **state-machine graphs with conditional routing**: `plan → execute ⇄ review → finalize` for delegation, phase DAGs for multi-agent workflow runs, and map-reduce fan-out for parallel workers. All of it has **durable checkpointing**. A graph can pause mid-run (for your answer, for an approval, for a restart) and resume exactly where it stopped.
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:25:## 3. Workflows you can see
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:27:[Workflows](workflows.md) lift orchestration out of the chat: the agent *proposes* a typed graph of triggers, agents, tools and conditions; you review it on a canvas and save it. Runs are durable, approval-gated, and fully inspectable step-by-step, powered by open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:41:The direction we're building toward: **RLM-style language-based workflows**. These are agents that express orchestration as small programs in a sandboxed REPL, rather than a fixed graph, so control flow itself becomes something the model writes, inspects, and repairs. The graph engine, checkpointing, and trust model above are the substrate for it.
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:51:| Automation | Scripts & cron | Visual, durable, approval-gated workflows |
<OPENHUMAN_ROOT>/gitbooks/features/orchestration.md:57:* [Workflows](workflows.md) · [Subconscious Loop](subconscious.md) · [tiny.place Agent Economy](tinyplace.md)
<OPENHUMAN_ROOT>/scripts/debug-agent-prompts.sh:10:# to stderr. Useful workflow:
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/deferred-and-rejected-work.md:27:- ML compressors can damage custom workflow tags and instructions without tag
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:201:Headroom protects workflow tags before ML compression:
<OPENHUMAN_ROOT>/vendor/tinyjuice/docs/references/headroom-algorithms-strategies-spec.md:204:- distinguish HTML tags from custom workflow tags
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:4:  agent proposes a workflow in chat; you review it on a canvas, save it, and it
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:10:# Workflows
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:12:<figure><img src="../.gitbook/assets/workflows.png" alt=""><figcaption><p>A workflow on the visual canvas. The agent proposes the graph; you review each step and save.</p></figcaption></figure>
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:14:Chat is great for one-off asks. **Workflows** are for the things you want done *every time*: triage every new support email, file every Linear ticket that mentions your team, post a digest every Monday at 9. Heavily inspired by [n8n](https://n8n.io) and [Zapier](https://zapier.com), a workflow is a saved, typed graph of steps you can see on a canvas and that runs without you. It is backed by the open-source [tinyflows](https://github.com/tinyhumansai/tinyflows) engine and the same trust and approval machinery as the rest of OpenHuman. The difference from n8n and Zapier: you don't build the graph, the agent does.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:18:You don't drag boxes to get started. Describe the automation in chat, for example *"whenever a new email arrives from a customer, summarize it and post to my Slack"*, and the agent uses its `propose_workflow` tool to draft a complete workflow graph. The proposal shows up in chat as a **Workflow Proposal Card** with a plain-English summary of every step.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:22:* The `propose_workflow` tool **only validates and describes** a candidate graph. It can never create or enable a flow by itself.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:23:* The **only** path from a proposal to a saved workflow is you clicking **Save & enable** on the card. That calls the `flows_create` RPC directly from the app, not from the agent.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:25:## What a workflow is made of
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:27:A workflow graph is composed of **12 node kinds**: exactly one `trigger`, plus any mix of `agent` (a full agent turn with tools), `tool_call`, `http_request`, `code` (JavaScript or Python), `condition`, `switch`, `transform`, `split_out`, `merge`, `output_parser`, and `sub_workflow`.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:33:* **Manual**: a Run button on the Workflows page or the `flows_run` RPC.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:40:Every flow run executes under a dedicated trust origin (`TrustedAutomation → Workflow`). The reasoning: the flow's *actions* (which tools it calls, which URLs it hits) are static graph configuration you approved at save time. The runtime trigger payload (a webhook body, an inbound event) stays **untrusted**: it can feed arguments into those pre-declared actions, but it can never introduce a new action.
<OPENHUMAN_ROOT>/gitbooks/features/workflows.md:48:* **`/flows`**: the Workflows hub, showing every flow with its enabled toggle, last-run status (`completed` / `pending approval` / `failed`), and a Run button.
[+4 more match(es) in <OPENHUMAN_ROOT>/gitbooks/features/workflows.md ⟦tj:27c2ff587362a9997079743514a96e2d⟧]
<OPENHUMAN_ROOT>/vendor/tinyjuice/plan/reference-algorithm-summary.md:31:- protect custom workflow tags before ML compression
<OPENHUMAN_ROOT>/gitbooks/SUMMARY.md:38:* [Workflows](features/workflows.md)
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:2:# Test the package-and-publish workflow locally using `act`.
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:12:#   ./scripts/test-ci-local.sh              # Run full workflow via act
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:22:WORKFLOW=".github/workflows/package-and-publish.yml"
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:82:    -W "$WORKFLOW"
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:93:    echo "Available jobs in $WORKFLOW:"
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:94:    act -W "$WORKFLOW" --list
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:99:    echo "Dry-run of workflow:"
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:144:# ── Default: run full workflow with act ────────────────────────────────────────
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:146:# We run the full workflow (not -j single-job) so act executes the dependency
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:151:echo "=== Testing package-and-publish workflow locally ==="
<OPENHUMAN_ROOT>/scripts/test-ci-local.sh:153:echo "Workflow: $WORKFLOW"
<OPENHUMAN_ROOT>/vendor/tinyjuice/wiki/OpenHuman-Integration.md:126:- If a coding workflow fails because a partial view hid relevant data, switch
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:13:- The **Tauri updater** endpoint (see `scripts/prepareTauriConfig.js` and release workflows) should point users at the current release artifacts.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:25:Configure these as **GitHub Actions variables**. They must be present on **both** the standalone **`pnpm build`** step and the **`tauri-apps/tauri-action`** step env in `.github/workflows/build-desktop.yml` (the reusable matrix invoked by `release-production.yml` / `release-staging.yml`) so the Vite bundle embedded in shipped installers includes the gate. Leave `VITE_MINIMUM_SUPPORTED_APP_VERSION` **unset** for local dev (gate disabled).
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:36:1. Bump `app/package.json` and `app/src-tauri/tauri.conf.json` (and root `Cargo.toml` / core) per existing version workflows.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:37:2. When dropping support for older installs, set **`VITE_MINIMUM_SUPPORTED_APP_VERSION`** to the new floor **before** or **with** that release (repo Actions variables + both workflow steps above).
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:46:- **`main`** — where all feature/fix PRs land. Every PR (and push to main) runs **CI Lite** ([`ci-lite.yml`](../../.github/workflows/ci-lite.yml)): quality checks per changed area plus unit tests scoped to the changed files, gated at ≥ 80% diff coverage by the `PR CI Gate` check.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:47:- **`release`** — a maintainer-promoted snapshot of `main` that releases are cut from. PRs targeting `release` and every push to `release` run **CI Full** ([`ci-full.yml`](../../.github/workflows/ci-full.yml)): complete unit suites, Rust mock-backend E2E, Playwright web E2E, and the full desktop E2E matrix on Linux/macOS/Windows. The `CI Full Gate` check aggregates every lane **except the Playwright spec run**, which is non-blocking signal for now (`continue-on-error`, flaky under CI contention — #3615): a green gate does not prove Playwright specs passed, so check that lane's result in the run before cutting. Only the Playwright artifact *build* is gated.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:51:1. A maintainer dispatches [`promote-main-to-release.yml`](../../.github/workflows/promote-main-to-release.yml), which pushes a **merge commit from `main` into `release`** (no PR). Re-dispatching refreshes `release` with main's latest while preserving fix commits already on `release`; when `release` already contains `main` it's a no-op.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:53:3. Once CI Full is green on `release` HEAD, cut a build with `release-staging.yml` or `release-production.yml`. Both workflows **enforce** this: `scripts/release/require-ci-full-gate.sh` fails the run unless the latest `CI Full Gate` check on the commit being cut (walking past `[skip ci]` bump commits) concluded success. The `skip_ci_gate` input overrides it for operator recovery only.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:56:Required GitHub settings for this model (repo **Settings → Rules**): `main` requires the `PR CI Gate` status check on PRs; `release` requires PRs for non-bypass actors with the `CI Full Gate` status check required (it runs on PRs targeting release); the release GitHub App's identity sits on the bypass list of both rulesets so the promote/release workflows can push directly.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:58:## Workflows: staging vs. production
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:60:Two first-class GitHub Actions workflows, one per environment. Pick by intent rather than toggling a flag. Both run from the `release` branch only.
<OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md:118:- **Collisions.** Both workflows fail fast if the target tag already exists locally or on `origin`. Resolve by deleting the stale tag (org maintainers only) or bumping past it.
[+12 more match(es) in <OPENHUMAN_ROOT>/gitbooks/developing/release-policy.md ⟦tj:70fd96334076c5139fa06ddd07dc1f77⟧]
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:36:> - the **workflow phase engine** fans each phase's agents out on the graph
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:37:>   (`with_max_concurrency`), keeping the durable `WorkflowRun` ledger as the resume
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:57:- **`repl` feature enabled for language workflows; `.rag` expressive language unused.** OpenHuman still drives *graphs* from Rust (`GraphBuilder`), not the declarative `.rag` language. But the `repl` feature (the imperative Rhai `.ragsh` session runtime) is enabled to power the `rlm` language-workflow tool ([`openhuman::rlm`](../../../src/openhuman/rlm/README.md), see "Language workflows (`rlm`)" below).
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:58:- **Adapter map (feature-gated SDK piece → OpenHuman replacement):** provider clients → `ProviderModel`; crate SQLite checkpointer rows not yet adopted → `SqlRunLedgerCheckpointer`; task/status stores not yet controller-canonical → OpenHuman SQL/JSON run ledgers (`running_subagents`, `workflow_runs`, `agent_teams`, `command_center`). The generic harness/graph/middleware/event primitives are used as-is.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:285:## Language workflows (`rlm`)
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:287:The fixed delegation primitives (`spawn_subagent`, `spawn_parallel_agents`, `run_workflow`) can't express *ad-hoc control flow* — "spawn N readers, dedupe their findings, verify each survivor with 3 refuters, loop until dry". The **`rlm` tool** closes that gap: it exposes TinyAgents' Rhai-backed `.ragsh` REPL (the `repl` cargo feature) so the orchestrator can author and run its own workflow scripts.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:294:- **`bridge.rs`** builds the `CapabilityRegistry`: the parent's visible tools (each re-wrapped so the **approval gate runs in the bridge** — it is *not* on the repl path, which bypasses the harness `wrap_tool` middleware), the turn's provider model, and a sub-agent capability per `allowed_subagent_ids`. Recursion/duplication hazards (`rlm`, `spawn_*`, workflow tools, `CliRpcOnly`-scoped tools) are excluded. Because `eval_cell` runs on `spawn_blocking` + `block_on`, the `agent_query` adapter re-installs the `PARENT_CONTEXT` task-local that `run_subagent` resolves.
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:464:- **Workflow phase DAG** (`workflow_runs/engine.rs`) runs on a `dispatch ⇄ run_phase → done` conditional-routing graph; each phase fans its agents out via `graph::parallel::map_reduce`. The durable `workflow_runs` row stays the source of truth (controllers + resume read it).
<OPENHUMAN_ROOT>/gitbooks/developing/architecture/agent-harness.md:472:- **Durable run ledgers** (`workflow_runs`, `agent_teams`, `command_center`, `subagent_sessions`) stay on openhuman SQLite/JSON until their controller projections and restart semantics are mapped onto TinyAgents task/status/journal records. The `agent_teams` race-safe SQL compare-and-swap task claim remains OpenHuman-owned.
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/vendor/rules/cloud__gh.json:4:  "description": "Compact GitHub CLI output while preserving issue, PR, and workflow result lines.",
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt.md:79:## Workflow Requirements
<OPENHUMAN_ROOT>/scripts/act-staging.sh:5:# .secrets / .vars files act consumes, and fakes a workflow_dispatch event
<OPENHUMAN_ROOT>/scripts/act-staging.sh:16:# - The workflow's `Enforce main branch` step compares `github.ref` against
<OPENHUMAN_ROOT>/scripts/act-staging.sh:142:exec act workflow_dispatch \
<OPENHUMAN_ROOT>/scripts/act-staging.sh:143:  -W "${ROOT}/.github/workflows/release-staging.yml" \
<OPENHUMAN_ROOT>/src/openhuman/channels/providers/presentation_tests.rs:126:        i'm reaching out to share what we're building at TinyHumans. we just launched OpenHuman, and the insight is simple: AI agents are incredibly powerful, but today they're locked behind developer workflows. setup, API keys, terminals. the 99% who can't code are completely left out.\n\n\
<OPENHUMAN_ROOT>/gitbooks/developing/integrations/polymarket.md:7:The `polymarket` tool now supports both market browsing and trading workflows over:
<OPENHUMAN_ROOT>/gitbooks/developing/README.md:61:* [**Workflows**](../features/workflows.md). The tinyflows-backed `flows` domain: triggers, trust origins, approval-gated runs, and the `flows_*` RPC surface.
<OPENHUMAN_ROOT>/src/openhuman/channels/providers/web/progress_bridge.rs:1415:    /// timeline result view and the `propose_workflow` proposal parser).
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/ops.rs:361:) -> Result<crate::openhuman::workflows::ops_install::InstallWorkflowFromUrlOutcome, String> {
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/ops.rs:381:    let params = crate::openhuman::workflows::ops_install::InstallWorkflowFromUrlParams {
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/ops.rs:386:    crate::openhuman::workflows::ops_install::install_workflow_from_url(workspace_dir, params).await
<OPENHUMAN_ROOT>/gitbooks/developing/e2e-testing.md:165:## CI workflows
<OPENHUMAN_ROOT>/gitbooks/developing/e2e-testing.md:169:The default pull-request gate is `.github/workflows/ci-lite.yml` (quick lane: quality checks + unit tests scoped to the changed files). E2E suites do not run on PRs to `main` — the full E2E matrix (Rust mock-backend, Playwright web, desktop on Linux/macOS/Windows) runs in `.github/workflows/ci-full.yml` on PRs targeting the `release` branch and on every push to it.
<OPENHUMAN_ROOT>/gitbooks/developing/e2e-testing.md:171:macOS and Windows desktop E2E do not run on every PR. Use the manually dispatched E2E workflow (`.github/workflows/e2e.yml`) when cross-platform desktop signal is needed before promotion.
<OPENHUMAN_ROOT>/gitbooks/developing/e2e-testing.md:175:macOS/Appium Chromium is available for local runs and through the manually dispatched E2E workflow:
<OPENHUMAN_ROOT>/gitbooks/developing/e2e-testing.md:187:For the default CEF runtime, this usually means a stale local runner is trying to drive a CEF-backed WebView through WebKitWebDriver. Current CI uses the Appium Chromium driver on Linux; use `app/scripts/e2e-run-session.sh` or the PR CI workflow for the supported Linux path.
<OPENHUMAN_ROOT>/vendor/tinyjuice/prompt-harness.md:108:- No CI workflow exists in this repo. If asked to add one: GitHub Actions,
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/flows/README.md:11:- **Workflows** — `tinyplace <verb>` — bundle several API calls into one result and
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/flows/README.md:14:  a workflow doesn't cover exactly what you need.
<OPENHUMAN_ROOT>/scripts/shortcuts/work/prompts/start.md:14:# Workflow
<OPENHUMAN_ROOT>/scripts/shortcuts/work/prompts/start.md:51:- Coverage gate: changed lines must hit ≥ 80% (`.github/workflows/coverage.yml`). Cover error/edge paths, not just the happy path.
<OPENHUMAN_ROOT>/gitbooks/developing/building-rust-core.md:16:If you want the full desktop app (`pnpm dev`, Tauri, CEF, frontend tooling), use [Getting Set Up](getting-set-up.md). That path has extra JavaScript, submodule, and desktop-runtime requirements that are **not** needed for a core-only `cargo` workflow.
<OPENHUMAN_ROOT>/gitbooks/developing/building-rust-core.md:142:**Ubuntu / Debian** (mirrored from [`.github/workflows/build-desktop.yml`](../../.github/workflows/build-desktop.yml)):
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/README.md:46:- Production installs still go through the hardened `workflows` URL installer.
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/flows/fulfilling-a-job.md:58:| Step | Workflow | Raw equivalent |
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/flows/groups-and-social.md:79:| Goal | Workflow | Raw equivalent |
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/schemas/wire_types.rs:7:use crate::openhuman::workflows::ops_types::WorkflowScope;
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/schemas/wire_types.rs:71:    pub(super) scope: WorkflowScope,
<OPENHUMAN_ROOT>/vendor/tinyjuice/src/reduce.rs:349:        .or_else(|| obj.get("workflowName").and_then(|v| v.as_str()))?
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/flows/posting-a-job.md:63:| Step | Workflow | Raw equivalent |
<OPENHUMAN_ROOT>/scripts/shortcuts/review/coverage.sh:44:  --workflow "PR CI" \
<OPENHUMAN_ROOT>/scripts/shortcuts/review/coverage.sh:51:    "No recent PR CI (coverage gate) workflow runs found for this branch."
<OPENHUMAN_ROOT>/scripts/shortcuts/review/coverage.sh:68:Recent PR CI (coverage gate) workflow runs for this branch:
<OPENHUMAN_ROOT>/scripts/shortcuts/review/coverage.sh:72:failing. Fix the coverage workflow or scripts if they are broken, improve test \
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/releasing.md:4:workflow: **Release** (`.github/workflows/release.yml`, `workflow_dispatch`).
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/releasing.md:8:1. GitHub → **Actions → Release → Run workflow**.
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/releasing.md:26:  is pushed. The **Deploy Website** workflow (`deploy-website.yml`) then runs
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/releasing.md:36:> cannot trigger the `push:` tag workflow, so the release run invokes
<OPENHUMAN_ROOT>/vendor/tinyplace/docs/releasing.md:37:> `deploy-website.yml` directly via `workflow_call`. Pushing a `website-v*` tag
<OPENHUMAN_ROOT>/vendor/tinyagents/src/harness/testkit/mod.rs:12://! possible to test model-and-tool workflows without live providers.
<OPENHUMAN_ROOT>/scripts/shortcuts/review/review.sh:5:# so the workflow is agent-agnostic (no reliance on Claude Code's named
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/schemas/handlers.rs:89:        let workspace = crate::openhuman::workflows::schemas::resolve_workspace_dir().await;
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/schemas/handlers.rs:123:        let workflow_params = crate::openhuman::workflows::ops_install::UninstallWorkflowParams {
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/schemas/handlers.rs:127:            crate::openhuman::workflows::ops_install::uninstall_workflow(workflow_params, None)?;
<OPENHUMAN_ROOT>/scripts/shortcuts/review/prompts/review.md:13:# Workflow
<OPENHUMAN_ROOT>/scripts/shortcuts/review/prompts/fix.md:18:# Workflow
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/agent/skill_setup/agent.toml:19:    "list_workflows",
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/agent/skill_setup/agent.toml:20:    "describe_workflow",
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/agent/skill_setup/agent.toml:26:    "install_workflow_from_url",
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/agent/skill_setup/agent.toml:27:    "uninstall_workflow",
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/agent/skill_setup/prompt.md:19:## Workflow
<OPENHUMAN_ROOT>/tests/monitor_agent_e2e.rs:290:        .workflows(Vec::new())
<OPENHUMAN_ROOT>/scripts/shortcuts/review/fix.sh:5:# so the workflow is agent-agnostic (no reliance on Claude Code's named
<OPENHUMAN_ROOT>/scripts/shortcuts/review/fix.sh:65:Only after conflicts are cleanly resolved should you proceed to the review/fix workflow below."
<OPENHUMAN_ROOT>/scripts/shortcuts/README.md:3:Workflow shortcuts — high-level pnpm commands that orchestrate routine
<OPENHUMAN_ROOT>/scripts/shortcuts/README.md:24:workflow **agent-agnostic** — works with `codex`, `gemini`, `cursor-agent`,
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/tools.rs:256:         installed workflows or when the user asks to remove a skill."
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/tools.rs:284:        let params = crate::openhuman::workflows::ops_install::UninstallWorkflowParams {
<OPENHUMAN_ROOT>/src/openhuman/skill_registry/tools.rs:287:        match crate::openhuman::workflows::ops_install::uninstall_workflow(params, None) {
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:2:# Test the Release workflow locally using act.
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:29:WORKFLOW=".github/workflows/release.yml"
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:97:  act -W "$WORKFLOW" --list
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:163:echo "Workflow: $WORKFLOW"
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:176:  workflow_dispatch
<OPENHUMAN_ROOT>/scripts/test-release-act.sh:177:  -W "$WORKFLOW"
<OPENHUMAN_ROOT>/tests/agent_harness_public.rs:142:        workflows: Arc::new(vec![]),
<OPENHUMAN_ROOT>/tests/agent_large_round25_raw_coverage_e2e.rs:333:        workflows: Arc::new(Vec::new()),
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:299:        workflows: Arc::new(Vec::new()),
<OPENHUMAN_ROOT>/tests/agent_prompts_subagent_raw_coverage_e2e.rs:322:        workflows: &[],
<OPENHUMAN_ROOT>/vendor/tinyplace/sdk/plugin-hermes/DEMO.md:12:  or the local Docker Compose stack (backend on `:8080`) from `workflow-tinyplace`.
<OPENHUMAN_ROOT>/scripts/agent-batch/__tests__/cli.test.mjs:19:  "agent-workflows",
<OPENHUMAN_ROOT>/scripts/agent-batch/launch.mjs:36:  - docs/agent-workflows/cursor-cloud-agents.md
<OPENHUMAN_ROOT>/scripts/agent-batch/launch.mjs:37:  - docs/agent-workflows/codex-pr-checklist.md
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3620:async fn json_rpc_workflow_run_definitions_and_runs_roundtrip() {
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3647:        "openhuman.workflow_run_list_definitions",
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3651:    let defs_outer = assert_no_jsonrpc_error(&defs, "workflow_run_list_definitions");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3665:    // Seed a durable workflow run, then list + get it.
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3690:    let list_outer = assert_no_jsonrpc_error(&list, "workflow_run_list");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:3711:    let get_outer = assert_no_jsonrpc_error(&get, "workflow_run_get");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:8727:    let ok_result = assert_no_jsonrpc_error(&ok, "workflows_uninstall success");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:12178:    let create_result = assert_no_jsonrpc_error(&create, "workflows_create");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:12189:    let list_result = assert_no_jsonrpc_error(&list, "workflows_list");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:12209:    let describe_result = assert_no_jsonrpc_error(&describe, "workflows_describe");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:12237:    let uninstall_result = assert_no_jsonrpc_error(&uninstall, "workflows_uninstall");
<OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs:12245:    let after_result = assert_no_jsonrpc_error(&after, "workflows_list");
[+39 more match(es) in <OPENHUMAN_ROOT>/tests/json_rpc_e2e.rs ⟦tj:c304757ddc4cb025b1ca47210573f803⟧]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (95475 bytes): call tinyjuice_retrieve with token "d8c09ca8668dbfbc772e95721d115384"]