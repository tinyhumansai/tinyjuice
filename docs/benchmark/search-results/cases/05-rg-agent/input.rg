<OPENHUMAN_ROOT>/src/api/config.rs:14://! caused every `/auth/*`, `/agent-integrations/*`, and `/voice/*` request to
<OPENHUMAN_ROOT>/src/api/config.rs:103:/// so `/auth/*`, `/voice/*`, and `/agent-integrations/*` never accidentally
<OPENHUMAN_ROOT>/src/api/config.rs:460:/// (`/auth/me`, `/agent-integrations/…`) which then land on
<OPENHUMAN_ROOT>/src/api/config.rs:502:/// | `https://api.tinyhumans.ai/openai/v1/…`   | `/agent-integrations/foo` | `https://api.tinyhumans.ai/agent-integrations/foo`  ← path replaced   |
<OPENHUMAN_ROOT>/src/api/config.rs:678:             /agent-integrations/* requests don't 404 against your local LLM"
<OPENHUMAN_ROOT>/src/api/config.rs:800:        // /agent-integrations/* calls.
<OPENHUMAN_ROOT>/src/api/config.rs:804:                "/agent-integrations/composio/toolkits",
<OPENHUMAN_ROOT>/src/api/config.rs:806:            "https://api.tinyhumans.ai/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:812:        let expected = "https://api.tinyhumans.ai/agent-integrations/composio/toolkits";
<OPENHUMAN_ROOT>/src/api/config.rs:816:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:823:                "/agent-integrations/composio/toolkits"
<OPENHUMAN_ROOT>/src/api/config.rs:834:                "/agent-integrations/composio/tools?toolkits=gmail"
<OPENHUMAN_ROOT>/src/api/config.rs:836:            "https://api.tinyhumans.ai/agent-integrations/composio/tools?toolkits=gmail"
<OPENHUMAN_ROOT>/src/api/config.rs:852:            api_url("http://localhost:1234/v1", "/agent-integrations/foo"),
<OPENHUMAN_ROOT>/src/api/config.rs:853:            "http://localhost:1234/agent-integrations/foo"
<OPENHUMAN_ROOT>/src/api/rest.rs:919:    /// Signals "the agent is typing…" on a channel that supports it
<OPENHUMAN_ROOT>/src/main.rs:111:            // the agent re-report routes through `TransientUpstreamHttp`, but the
<OPENHUMAN_ROOT>/src/main.rs:120:            // `agent::harness::session::runtime::run_single`,
<OPENHUMAN_ROOT>/src/main.rs:123:            // deterministic agent-state outcome surfaced to the user via
<OPENHUMAN_ROOT>/src/lib.rs:6://! - Domain-specific logic for the OpenHuman agent runtime.
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:7://! - `--mode harness` (default): build a real `Agent::from_config()`
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:24://!   RUST_LOG=info,openhuman_core::openhuman::agent=debug,openhuman_core::openhuman::inference=debug \
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:33:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:74:        "[probe] config.agent.tool_dispatcher = {:?}",
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:75:        config.agent.tool_dispatcher
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:89:    let mut agent = Agent::from_config(config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:94:    agent.fetch_connected_integrations().await;
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:95:    let refreshed = agent.refresh_delegation_tools();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:96:    let conn_count = agent.connected_integrations().len();
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:101:    eprintln!("[probe] visible tool count = {}", agent.tools().len());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:102:    eprintln!("[probe] model = {}", agent.model_name());
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:104:    eprintln!("[probe] >>> agent.run_single() ...");
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:106:    let response = agent
<OPENHUMAN_ROOT>/src/bin/inference_probe.rs:109:        .context("agent.run_single failed")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:1://! Live harness audit for reusable async sub-agent delegation.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:12://! scripts/debug/harness-subagent-audit.sh --turns 2
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:24:use openhuman_core::openhuman::agent::progress::AgentProgress;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:25:use openhuman_core::openhuman::agent::Agent;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:26:use openhuman_core::openhuman::agent_orchestration::harness_audit::{
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:27:    self, AuditSteerError, AuditSubagentSessionStore, DurableSubagentSession, DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:34:#[command(name = "harness-subagent-audit")]
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:36:    /// Sub-agent archetype to request from the orchestrator.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:38:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:40:    /// Stable reusable task key. Defaults to audit-subagent-<unix-seconds>.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:68:    /// After the first async sub-agent spawn, steer the running child through its run queue.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:72:    /// Delay after SubagentSpawned before attempting the steer.
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:89:    subagent_spawned: Vec<SubagentSpawnedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:90:    subagent_completed: Vec<SubagentCompletedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:91:    subagent_failed: Vec<SubagentFailedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:92:    subagent_tool_started: Vec<SubagentToolEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:93:    subagent_tool_completed: Vec<SubagentToolCompletedEvent>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:119:struct SubagentSpawnedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:121:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:133:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:135:    subagent_session_id: Option<String>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:145:    store: AuditSubagentSessionStore,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:154:struct SubagentCompletedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:156:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:164:struct SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:166:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:172:struct SubagentToolEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:174:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:182:struct SubagentToolCompletedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:184:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:196:    subagent_session_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:200:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:204:    status: DurableSubagentStatus,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:214:    agent_id: String,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:233:        eprintln!("[harness_subagent_audit] ERROR: {err:#}");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:244:        .unwrap_or_else(|| format!("audit-subagent-{}", unix_seconds()));
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:246:    eprintln!("[harness_subagent_audit] loading live OpenHuman config");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:250:    let store = AuditSubagentSessionStore::new(config.workspace_dir.clone());
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:252:        "[harness_subagent_audit] workspace_dir={} session_store={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:257:        "[harness_subagent_audit] default_model={:?} dispatcher={:?}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:258:        config.default_model, config.agent.tool_dispatcher
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:262:        .context("loading existing matching durable subagent sessions")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:265:            "[harness_subagent_audit] found {} pre-existing session(s) for task_key={}; reuse checks may include prior state",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:271:    let mut agent = Agent::from_config(&config).context("Agent::from_config failed")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:272:    eprintln!("[harness_subagent_audit] fetching connected integrations");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:273:    agent.fetch_connected_integrations().await;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:274:    let refreshed = agent.refresh_delegation_tools();
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:276:        "[harness_subagent_audit] connected_integrations={} delegation_tools_refreshed={} visible_tools={} model={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:277:        agent.connected_integrations().len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:279:        agent.tools().len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:280:        agent.model_name()
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:286:    agent.set_on_progress(Some(tx));
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:312:                .unwrap_or_else(|| first_turn_prompt(&args.agent_id, &task_key))
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:316:                .unwrap_or_else(|| second_turn_prompt(&args.agent_id, &task_key))
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:319:            "[harness_subagent_audit] >>> parent_turn={} prompt_chars={} task_key={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:325:        let reply = agent
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:328:            .with_context(|| format!("agent.run_single failed on turn {turn}"))?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:330:            "[harness_subagent_audit] <<< parent_turn={} elapsed_ms={} assistant_reply_chars={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:345:    .context("polling durable subagent sessions")?;
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:347:    agent.set_on_progress(None);
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:348:    drop(agent);
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:353:        &args.agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:362:        agent_id: args.agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:383:    mut rx: mpsc::Receiver<AgentProgress>,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:391:            AgentProgress::ToolCallStarted {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:400:                    "[harness_subagent_audit] progress turn={} parent_tool_started tool={} call_id={} iteration={} argument_keys={:?}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:415:            AgentProgress::ToolCallCompleted {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:425:                    "[harness_subagent_audit] progress turn={} parent_tool_completed tool={} call_id={} success={} output_chars={} elapsed_ms={} iteration={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:442:            AgentProgress::SubagentSpawned {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:443:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:453:                    "[harness_subagent_audit] progress turn={} subagent_spawned agent_id={} task_id={} mode={} dedicated_thread={} prompt_chars={} worker_thread_id={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:455:                    agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:462:                let spawned = SubagentSpawnedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:464:                    agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:475:                    .subagent_spawned
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:488:                                "[harness_subagent_audit] steer_attempt turn={} task_id={} delivered={} attempts={} elapsed_ms={} message_chars={} error={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:506:            AgentProgress::SubagentCompleted {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:507:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:515:                    "[harness_subagent_audit] progress turn={} subagent_completed agent_id={} task_id={} elapsed_ms={} iterations={} output_chars={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:516:                    turn, agent_id, task_id, elapsed_ms, iterations, output_chars
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:521:                    .subagent_completed
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:522:                    .push(SubagentCompletedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:524:                        agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:531:            AgentProgress::SubagentFailed {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:532:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:537:                    "[harness_subagent_audit] progress turn={} subagent_failed agent_id={} task_id={} error_chars={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:539:                    agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:546:                    .subagent_failed
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:547:                    .push(SubagentFailedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:549:                        agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:554:            AgentProgress::SubagentToolCallStarted {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:555:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:564:                    "[harness_subagent_audit] progress turn={} subagent_tool_started agent_id={} task_id={} tool={} call_id={} iteration={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:565:                    turn, agent_id, task_id, tool_name, call_id, iteration
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:570:                    .subagent_tool_started
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:571:                    .push(SubagentToolEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:573:                        agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:580:            AgentProgress::SubagentToolCallCompleted {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:581:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:593:                    "[harness_subagent_audit] progress turn={} subagent_tool_completed agent_id={} task_id={} tool={} call_id={} success={} output_chars={} elapsed_ms={} iteration={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:594:                    turn, agent_id, task_id, tool_name, call_id, success, output_chars, elapsed_ms, iteration
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:599:                    .subagent_tool_completed
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:600:                    .push(SubagentToolCompletedEvent {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:602:                        agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:612:            AgentProgress::TurnCompleted { .. } => {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:618:            AgentProgress::SubagentAwaitingUser {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:619:                agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:625:                    "[harness_subagent_audit] progress turn={} subagent_awaiting_user agent_id={} task_id={} question_chars={} worker_thread_id={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:627:                    agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:633:            AgentProgress::IterationStarted { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:634:            | AgentProgress::SubagentIterationStarted { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:635:            | AgentProgress::TextDelta { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:636:            | AgentProgress::ThinkingDelta { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:637:            | AgentProgress::SubagentTextDelta { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:638:            | AgentProgress::SubagentThinkingDelta { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:639:            | AgentProgress::ToolCallArgsDelta { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:640:            | AgentProgress::TaskBoardUpdated { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:641:            | AgentProgress::TurnCostUpdated { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:642:            | AgentProgress::ModelCallCompleted { .. }
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:643:            | AgentProgress::TurnStarted
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:644:            | AgentProgress::TurnContent { .. } => {}
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:651:    spawned: SubagentSpawnedEvent,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:661:                match harness_audit::steer_running_subagent(
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:671:                            agent_id: spawned.agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:673:                            subagent_session_id: Some(session.subagent_session_id),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:717:    spawned: SubagentSpawnedEvent,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:725:        agent_id: spawned.agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:727:        subagent_session_id: None,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:737:    store: &AuditSubagentSessionStore,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:757:fn first_turn_prompt(agent_id: &str, task_key: &str) -> String {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:759:        "Harness audit run. Call spawn_subagent exactly once with agent_id `{agent_id}`, \
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:761:         the sub-agent to return a concise confirmation for audit marker `{task_key}` without \
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:763:         async reusable worker was started. Do not call wait_subagent in this turn."
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:767:fn second_turn_prompt(agent_id: &str, task_key: &str) -> String {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:769:        "Harness audit follow-up. Continue the same reusable sub-agent by calling spawn_subagent \
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:770:         exactly once with agent_id `{agent_id}`, the same task_key `{task_key}`, blocking false, \
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:773:         wait_subagent in this turn."
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:784:    store: &AuditSubagentSessionStore,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:793:            !require_completion || !matches!(session.status, DurableSubagentStatus::Running)
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:806:    store: &AuditSubagentSessionStore,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:811:            "loading durable subagent store at {}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:822:impl From<DurableSubagentSession> for SessionSummary {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:823:    fn from(session: DurableSubagentSession) -> Self {
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:825:            subagent_session_id: session.subagent_session_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:829:            agent_id: session.agent_id,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:843:    agent_id: &str,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:859:            "observed {parent_spawn_calls} spawn_subagent/spawn_async_subagent start event(s)"
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:893:        .subagent_spawned
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:895:        .filter(|event| event.agent_id == agent_id && event.mode == "async")
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:898:        name: "async_subagent_registered",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:901:            "observed {spawned_events} async SubagentSpawned event(s), {} persisted matching session(s)",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:914:        .map(|session| session.subagent_session_id.as_str())
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:920:            "unique matching subagent_session_id count={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:925:    let session_agent_ok = sessions
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:927:        .all(|session| session.agent_id == agent_id && session.reusable);
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:929:        name: "session_agent_and_reusable_flag_match",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:930:        passed: !sessions.is_empty() && session_agent_ok,
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:932:            "all sessions match agent_id={agent_id} and reusable=true: {session_agent_ok}"
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:938:        .filter(|session| matches!(session.status, DurableSubagentStatus::Running))
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:954:    tool_name == "spawn_subagent" || tool_name == "spawn_async_subagent"
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:965:    println!("=== Harness Subagent Audit ===");
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:967:    println!("agent_id: {}", summary.agent_id);
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:971:        "progress: parent_spawn_started={} parent_spawn_completed={} subagent_spawned={} subagent_completed={} subagent_failed={} subagent_tool_started={} subagent_tool_completed={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:984:        summary.progress.subagent_spawned.len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:985:        summary.progress.subagent_completed.len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:986:        summary.progress.subagent_failed.len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:987:        summary.progress.subagent_tool_started.len(),
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:988:        summary.progress.subagent_tool_completed.len()
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:1009:                "  subagent_session_id={} task_id={} status={:?} reusable={} worker_thread_id={} updated_at={}",
<OPENHUMAN_ROOT>/src/bin/harness_subagent_audit.rs:1010:                session.subagent_session_id,
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:93:/// semaphore, `GLOBAL_REGISTRY` agent.run_turn handler, `STARTED`
<OPENHUMAN_ROOT>/src/core/jsonrpc_tests.rs:697:    let msg = r#"[composio] list_connections failed: Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 401 {"error":{"message":"Invalid API key: ak_o1Og5*****","code":10401,"slug":"HTTP_Unauthorized","status":401}}"#;
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:7:For deeper architecture and subsystem references, use the GitBook under [`gitbooks/developing/`](gitbooks/developing/). For coding-agent and repository-specific implementation rules, see [`AGENTS.md`](AGENTS.md) and [`CLAUDE.md`](CLAUDE.md).
<OPENHUMAN_ROOT>/CONTRIBUTING.md:201:If you only changed docs in a normal local workflow, `pnpm format:check` is usually the only validation you need. AI-authored or remote-agent PRs must still fill in the AI Authored PR Metadata section of the PR template and report any blocked commands with the exact command and error.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:220:- For AI-authored or remote-agent PRs, also fill in the AI Authored PR Metadata section of the PR template.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:249:├── AGENTS.md               # Coding-agent repo rules
<OPENHUMAN_ROOT>/CONTRIBUTING.md:303:If you are contributing through a coding agent or remote environment, include the metadata required by the PR template and the Codex PR checklist.
<OPENHUMAN_ROOT>/CONTRIBUTING.md:309:- Use the controller registry and domain module structure described in [`AGENTS.md`](AGENTS.md) for new Rust functionality.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:48:    // ── Agent ───────────────────────────────────────────────────────────
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:49:    /// An agent turn has started processing.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:50:    AgentTurnStarted { session_id: String, channel: String },
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:51:    /// An agent turn completed with a final response.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:52:    AgentTurnCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:57:    /// An error occurred during agent processing.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:58:    AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:63:    /// A sub-agent was dispatched via `spawn_subagent`.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:64:    SubagentSpawned {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:65:        /// Parent agent's session id.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:67:        /// Sub-agent definition id (e.g. `researcher`, `notion_specialist`, `fork`).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:68:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:76:    /// A sub-agent finished successfully.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:77:    SubagentCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:80:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:85:    /// A sub-agent failed (max iterations, provider error, missing
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:88:    SubagentFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:91:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:94:    /// A sub-agent called `ask_user_clarification` and paused, waiting
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:96:    /// `continue_subagent`.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:97:    SubagentAwaitingUser {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:100:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:103:    /// High-level orchestration accepted a child agent for execution.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:104:    AgentOrchestrationSpawned {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:107:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:108:        parent_agent_id: Option<String>,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:110:    /// High-level orchestration observed a child agent completion.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:111:    AgentOrchestrationCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:114:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:119:    /// High-level orchestration observed a child agent failure.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:120:    AgentOrchestrationFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:123:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:126:    /// High-level orchestration closed or cancelled a child agent.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:127:    AgentOrchestrationClosed {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:136:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:145:        agent_id: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:186:    /// `RunQueueMessageDelivered` event provided before the TinyAgents migration
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:343:    /// `channel` so the agent loop can derive per-sender conversation keys
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:492:    /// create). Lets a live agent session refresh its `## Installed Skills`
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:512:    /// A TinyAgents workspace descriptor was prepared for an isolated run.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:519:    /// A TinyAgents workspace descriptor blocked an out-of-root path.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:524:    /// A TinyAgents workspace descriptor was cleaned up.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:533:    /// Agent attempted a tool call that produces an external side
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:629:        /// cron / sub-agent paths — no client to fan out to.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:680:        /// sub-agent paths — no client to fan out to.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:776:    // Published by `crate::openhuman::agent::triage` when an external
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:778:    // later) has been classified by the trigger-triage agent. The
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:781:    // `agent::triage::run_triage` will publish these.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:782:    /// A trigger event was evaluated by the triage agent and assigned
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:800:    /// Triage decided to hand the trigger off to another agent
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:808:        /// Agent definition id the trigger was handed off to.
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:809:        target_agent: String,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:901:        /// `"conversations:agent"`).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:976:    /// The MCP setup agent asked the user for a secret value. The UI
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:979:    /// returned to the agent; the raw secret value never traverses this
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:988:    /// the registry before reaching the agent LLM context. Surfaced for
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1036:    /// The `[autonomy]` block (agent access mode / filesystem permissions) was
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1040:    /// The agent's filesystem roots (currently the `action_dir` sandbox) were
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1041:    /// changed at runtime via `config.update_agent_paths`. The live
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1044:    AgentPathsChanged,
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1285:            Self::AgentTurnStarted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1286:            | Self::AgentTurnCompleted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1287:            | Self::AgentError { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1288:            | Self::SubagentSpawned { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1289:            | Self::SubagentCompleted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1290:            | Self::SubagentFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1291:            | Self::SubagentAwaitingUser { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1292:            | Self::AgentOrchestrationSpawned { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1293:            | Self::AgentOrchestrationCompleted { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1294:            | Self::AgentOrchestrationFailed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1295:            | Self::AgentOrchestrationClosed { .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1302:            | Self::RunQueueSteerRequeued { .. } => "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1386:            | Self::AgentPathsChanged
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1400:            Self::TaskPlanAwaitingApproval { .. } | Self::TaskRunReclaimed { .. } => "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1402:            Self::ThreadGoalUpdated { .. } | Self::ThreadGoalCleared { .. } => "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1439:            | Self::MeetingSummaryGenerated { .. } => "agent_meetings",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1450:            Self::AgentTurnStarted { .. } => "AgentTurnStarted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1451:            Self::AgentTurnCompleted { .. } => "AgentTurnCompleted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1452:            Self::AgentError { .. } => "AgentError",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1453:            Self::SubagentSpawned { .. } => "SubagentSpawned",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1454:            Self::SubagentCompleted { .. } => "SubagentCompleted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1455:            Self::SubagentFailed { .. } => "SubagentFailed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1456:            Self::SubagentAwaitingUser { .. } => "SubagentAwaitingUser",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1457:            Self::AgentOrchestrationSpawned { .. } => "AgentOrchestrationSpawned",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1458:            Self::AgentOrchestrationCompleted { .. } => "AgentOrchestrationCompleted",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1459:            Self::AgentOrchestrationFailed { .. } => "AgentOrchestrationFailed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1460:            Self::AgentOrchestrationClosed { .. } => "AgentOrchestrationClosed",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1538:            Self::AgentPathsChanged => "AgentPathsChanged",
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1590:    /// Best-effort agent/session hint for display (not all events carry one).
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1591:    pub fn agent_hint(&self) -> Option<&str> {
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1593:            Self::AgentTurnStarted { session_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1594:            | Self::AgentTurnCompleted { session_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1595:            | Self::AgentError { session_id, .. } => Some(session_id.as_str()),
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1596:            Self::SubagentSpawned { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1597:            | Self::SubagentCompleted { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1598:            | Self::SubagentFailed { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1599:            | Self::SubagentAwaitingUser { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1600:            | Self::AgentOrchestrationSpawned { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1601:            | Self::AgentOrchestrationCompleted { agent_id, .. }
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1602:            | Self::AgentOrchestrationFailed { agent_id, .. } => Some(agent_id.as_str()),
<OPENHUMAN_ROOT>/src/core/event_bus/events.rs:1603:            Self::AgentOrchestrationClosed {
<OPENHUMAN_ROOT>/src/core/observability.rs:12://! being logged at error level never reach Sentry. The agent-turn path is the
<OPENHUMAN_ROOT>/src/core/observability.rs:13://! canonical example — `run_single` used to publish a `DomainEvent::AgentError`
<OPENHUMAN_ROOT>/src/core/observability.rs:101:    /// error is raised again by `agent.run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:102:    /// `web_channel.run_chat_task` under `domain=agent` / `web_channel`.
<OPENHUMAN_ROOT>/src/core/observability.rs:114:    /// component (frontend RPC relay, agent-integrations client) sees a TCP
<OPENHUMAN_ROOT>/src/core/observability.rs:185:    /// (`text_chars=0 thinking_chars=0 tool_calls=0`), so the agent harness
<OPENHUMAN_ROOT>/src/core/observability.rs:188:    /// (`agent::harness::session::turn`). This is a model/user-config
<OPENHUMAN_ROOT>/src/core/observability.rs:194:    /// `agent::run_single` already suppresses the **agent-layer** Sentry
<OPENHUMAN_ROOT>/src/core/observability.rs:196:    /// `AgentError::EmptyProviderResponse` + `AgentError::skips_sentry()`
<OPENHUMAN_ROOT>/src/core/observability.rs:287:    /// `domain=web_channel` / `agent` (the path
<OPENHUMAN_ROOT>/src/core/observability.rs:509:    // re-report at the agent / RPC boundary (`provider_chat` →
<OPENHUMAN_ROOT>/src/core/observability.rs:510:    // `report_error_or_expected`, the `domain=agent` half of the flood). Routed
<OPENHUMAN_ROOT>/src/core/observability.rs:580:    // Context-window-exceeded re-report from a higher layer (agent /
<OPENHUMAN_ROOT>/src/core/observability.rs:613:    // the two-emit-site rationale (agent layer is handled by the typed
<OPENHUMAN_ROOT>/src/core/observability.rs:614:    // `AgentError::skips_sentry()` in PR #2790; this covers the
<OPENHUMAN_ROOT>/src/core/observability.rs:737:/// backup agent), or a OneDrive "files on demand" placeholder that won't
<OPENHUMAN_ROOT>/src/core/observability.rs:908:/// backend-touching call site (agent, web channel, cron, integrations).
<OPENHUMAN_ROOT>/src/core/observability.rs:919:///   OpenHuman backend and re-raised through `agent::run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:1061:/// (frontend RPC relay, agent-integrations / composio HTTP clients) tried to
<OPENHUMAN_ROOT>/src/core/observability.rs:1100:/// error sending request for url (http://127.0.0.1:18474/agent-integrations/composio/connections)
<OPENHUMAN_ROOT>/src/core/observability.rs:1355:/// provider layer and into higher-level domains (`agent`, `web_channel`, …).
<OPENHUMAN_ROOT>/src/core/observability.rs:1362:/// `agent.run_single` (OPENHUMAN-TAURI-5Z), `web_channel.run_chat_task`,
<OPENHUMAN_ROOT>/src/core/observability.rs:1460:/// `"Backend returned 400 Bad Request for POST https://api.tinyhumans.ai/agent-integrations/composio/authorize: Composio authorization failed: 400 …"`
<OPENHUMAN_ROOT>/src/core/observability.rs:1514:/// classifier survives caller wrapping (rpc.invoke_method, agent.run_single,
<OPENHUMAN_ROOT>/src/core/observability.rs:1598:    // endpoint when requests are not sent from an approved coding-agent
<OPENHUMAN_ROOT>/src/core/observability.rs:1600:    // "currently only available for Coding Agents ...".
<OPENHUMAN_ROOT>/src/core/observability.rs:1602:        || lower.contains("currently only available for coding agents")
<OPENHUMAN_ROOT>/src/core/observability.rs:1644:    // `/agent-integrations/composio/connections`) wraps an upstream
<OPENHUMAN_ROOT>/src/core/observability.rs:1785:/// Detect the agent harness's empty-provider-response bail.
<OPENHUMAN_ROOT>/src/core/observability.rs:1788:/// `agent::harness::session::turn` —
<OPENHUMAN_ROOT>/src/core/observability.rs:1798:/// `AgentError::EmptyProviderResponse` was flattened to a `String` at the
<OPENHUMAN_ROOT>/src/core/observability.rs:1799:/// native-bus boundary (so the agent-layer `skips_sentry()` suppression
<OPENHUMAN_ROOT>/src/core/observability.rs:1806:/// returning empty extraction"` (`subagent_runner::extract_tool`) are
<OPENHUMAN_ROOT>/src/core/observability.rs:1962:            // again by agent.run_single / web_channel.run_chat_task. The
<OPENHUMAN_ROOT>/src/core/observability.rs:2013:            // upstream call site (agent.run_single, web_channel.run_chat_task)
<OPENHUMAN_ROOT>/src/core/observability.rs:2133:            // completely empty body and the agent harness bailed with the
<OPENHUMAN_ROOT>/src/core/observability.rs:2134:            // user-facing retry message. The agent layer already suppresses
<OPENHUMAN_ROOT>/src/core/observability.rs:2135:            // this via the typed `AgentError::skips_sentry()` (PR #2790);
<OPENHUMAN_ROOT>/src/core/observability.rs:2139:            // the other deterministic agent-state outcome surfaced to the
<OPENHUMAN_ROOT>/src/core/observability.rs:2509:/// `openhuman::agent::error::MAX_ITERATIONS_ERROR_PREFIX`).
<OPENHUMAN_ROOT>/src/core/observability.rs:2512:/// suppression lives at the call sites in `agent::harness::session::
<OPENHUMAN_ROOT>/src/core/observability.rs:2532:        .any(crate::openhuman::agent::error::is_max_iterations_error)
<OPENHUMAN_ROOT>/src/core/observability.rs:3028:/// `api_error` emit sites, and the agent re-report is demoted via
<OPENHUMAN_ROOT>/src/core/observability.rs:3196:                "agent.provider_chat failed: ollama API key not set. Configure via the web UI"
<OPENHUMAN_ROOT>/src/core/observability.rs:3748:        // OPENHUMAN-TAURI-140: ~1 480 events from `openhuman.agent_chat` where
<OPENHUMAN_ROOT>/src/core/observability.rs:3791:        // error is re-raised by `agent.run_single` / `web_channel.
<OPENHUMAN_ROOT>/src/core/observability.rs:3823:        // re-raised by the agent/web_channel, `report_error_or_expected` must
<OPENHUMAN_ROOT>/src/core/observability.rs:3858:        // by `agent.run_single` under `domain=agent`, `report_error_or_expected`
<OPENHUMAN_ROOT>/src/core/observability.rs:3994:        // TAURI-RUST-4Z1: the web-channel re-report of the agent harness's
<OPENHUMAN_ROOT>/src/core/observability.rs:3997:        // agent-layer typed suppression (PR #2790) can't reach it, so this
<OPENHUMAN_ROOT>/src/core/observability.rs:4027:            // subagent_runner/extract_tool.rs:379 — graceful empty extraction.
<OPENHUMAN_ROOT>/src/core/observability.rs:4032:            // through report_error_or_expected; subject is "agent", not "model").
<OPENHUMAN_ROOT>/src/core/observability.rs:4033:            "[channel-inbound] agent returned empty response — finalizing draft with fallback",
<OPENHUMAN_ROOT>/src/core/observability.rs:4038:            // agent/harness/session/turn.rs:811 — "provider returned an empty
<OPENHUMAN_ROOT>/src/core/observability.rs:4040:            "[agent_loop] provider returned an empty final response (i=2, no text, no tool calls)",
<OPENHUMAN_ROOT>/src/core/observability.rs:4149:                 https://api.tinyhumans.ai/agent-integrations/composio/list: \
<OPENHUMAN_ROOT>/src/core/observability.rs:4158:                 https://api.tinyhumans.ai/agent-integrations/composio/list: \
<OPENHUMAN_ROOT>/src/core/observability.rs:4171:                 https://api.tinyhumans.ai/agent-integrations/composio/list: \
<OPENHUMAN_ROOT>/src/core/observability.rs:4253:        // backup agent).
<OPENHUMAN_ROOT>/src/core/observability.rs:4641:        // `providers::ops::api_error` and re-raised through `agent.run_single`.
<OPENHUMAN_ROOT>/src/core/observability.rs:4664:        // Wrapped in an anyhow chain (as it reaches the agent layer) must
<OPENHUMAN_ROOT>/src/core/observability.rs:4668:                "agent turn failed: OpenHuman API error (504 Gateway Timeout): \
<OPENHUMAN_ROOT>/src/core/observability.rs:4724:                     (https://api.tinyhumans.ai/agent-integrations/composio/execute) → \
<OPENHUMAN_ROOT>/src/core/observability.rs:4808:        // agent re-raised the error, and `channels::runtime::dispatch`
<OPENHUMAN_ROOT>/src/core/observability.rs:4816:        // The wrapping shape at the dispatch site is the agent error
<OPENHUMAN_ROOT>/src/core/observability.rs:4825:            "agent.provider_chat failed: OpenHuman API error (503 Service Unavailable): retry budget exhausted",
<OPENHUMAN_ROOT>/src/core/observability.rs:4943:                  https://api.tinyhumans.ai/agent-integrations/composio/authorize: \
<OPENHUMAN_ROOT>/src/core/observability.rs:4979:        // bubbles to the agent tool-execute loop
<OPENHUMAN_ROOT>/src/core/observability.rs:4980:        // (`agent::harness::engine::tools::run_one_tool`'s `Ok(Err(e))` arm),
<OPENHUMAN_ROOT>/src/core/observability.rs:4990:        // the agent arm renders it with `{e:#}`, then `report_error_or_expected`
<OPENHUMAN_ROOT>/src/core/observability.rs:4995:             https://api.tinyhumans.ai/agent-integrations/parallel/search: Invalid token"
<OPENHUMAN_ROOT>/src/core/observability.rs:5070:                 https://api.tinyhumans.ai/agent-integrations/composio/triggers: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5081:                 for POST /agent-integrations/composio/triggers: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5103:                   https://api.tinyhumans.ai/agent-integrations/composio/execute: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5110:        // Wrapped variant (anyhow chain through the agent runtime).
<OPENHUMAN_ROOT>/src/core/observability.rs:5114:                 /agent-integrations/composio/execute: Toolkit \"linear\" is not enabled \
<OPENHUMAN_ROOT>/src/core/observability.rs:5132:        // Wrapped by higher-level callers (`agent.run_single`,
<OPENHUMAN_ROOT>/src/core/observability.rs:5136:                "agent.run_single failed: custom_openai API error (400 Bad Request): \
<OPENHUMAN_ROOT>/src/core/observability.rs:5179:                 https://api.tinyhumans.ai/agent-integrations/composio/authorize: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5226:                "custom_openai API error (403 Forbidden): {\"error\":{\"message\":\"Kimi For Coding is currently only available for Coding Agents such as Kimi CLI, Claude Code, Roo Code, Kilo Code, etc.\",\"type\":\"access_terminated_error\"}}"
<OPENHUMAN_ROOT>/src/core/observability.rs:5233:                "agent turn failed: custom_openai API error (403): currently only available for coding agents"
<OPENHUMAN_ROOT>/src/core/observability.rs:5246:                 /agent-integrations/composio/triggers: random panic in handler"
<OPENHUMAN_ROOT>/src/core/observability.rs:5274:        // provider; raised again by `agent.run_single` /
<OPENHUMAN_ROOT>/src/core/observability.rs:5279:                "agent.run_single failed: custom_openai API error (400 Bad Request): \
<OPENHUMAN_ROOT>/src/core/observability.rs:5395:                   /agent-integrations/composio/execute: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5485:                           https://api.tinyhumans.ai/agent-integrations/composio/connections: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5527:        let msg = r#"Backend returned 500 Internal Server Error for GET https://api.tinyhumans.ai/agent-integrations/composio/connections: 403 <!DOCTYPE html><html lang="en-US"><head><title>Just a moment...</title><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><meta name="robots" content="noindex,nofollow"><meta name="viewport" content="width=device-width,initial-scale=1"><link href="/cdn-cgi/styles/challenges.css" rel="stylesheet"></head><body class="no-js"><div class="main-wrapper" role="main"><div class="main-content"><h1 class="zone-name-title h1"><img class="heading-favicon" src="/favicon.ico" onerror="this.onerror=null;this.parentNode.removeChild(this)" alt="Icon for api.tinyhumans.ai">api.tinyhumans.ai</h1>...Powered by Cloudflare..."#;
<OPENHUMAN_ROOT>/src/core/observability.rs:5581:                   https://api.tinyhumans.ai/agent-integrations/composio/connections: \
<OPENHUMAN_ROOT>/src/core/observability.rs:5765:        // OPENHUMAN-TAURI-26: the canonical wire shape that `agent.run_single`
<OPENHUMAN_ROOT>/src/core/observability.rs:5779:        // Wrapped by the agent / web-channel report sites in production —
<OPENHUMAN_ROOT>/src/core/observability.rs:5817:    /// `agent.run_single`. PR #1763 (1fb0bef5) wired the `SessionExpired`
<OPENHUMAN_ROOT>/src/core/observability.rs:5886:        // appears at provider/agent layers; the substring matcher must
<OPENHUMAN_ROOT>/src/core/observability.rs:5959:        // Caller-wrapped (agent.run_single / web_channel.run_chat_task
<OPENHUMAN_ROOT>/src/core/observability.rs:6019:        // NOT be classified as session-expired at the agent layer — the
<OPENHUMAN_ROOT>/src/core/observability.rs:6333:                &format!("GET /agent-integrations/tools failed: {phrase}"),
<OPENHUMAN_ROOT>/src/core/observability.rs:6373:            "GET /agent-integrations/tools failed: invalid certificate",
<OPENHUMAN_ROOT>/src/core/observability.rs:6489:                 for GET /agent-integrations/composio/connections"
<OPENHUMAN_ROOT>/src/core/observability.rs:6508:                 for GET /agent-integrations/composio/connections"
<OPENHUMAN_ROOT>/src/core/observability.rs:6614:            "backend request to /agent-integrations failed with status code 500",
<OPENHUMAN_ROOT>/src/core/observability.rs:6728:            "agent",
<OPENHUMAN_ROOT>/src/core/observability.rs:6734:        // skip-log arm (the agent/web-channel re-report demotion path).
<OPENHUMAN_ROOT>/src/core/observability.rs:6736:            "agent.run_single failed: custom_openai API error (400 Bad Request): \
<OPENHUMAN_ROOT>/src/core/observability.rs:6739:            "agent",
<OPENHUMAN_ROOT>/src/core/observability.rs:6858:        // call site (`domain=cron`, `operation=agent_job`): the message-level
<OPENHUMAN_ROOT>/src/core/observability.rs:6915:        // demote to `TransientUpstreamHttp` when re-reported at the agent / RPC
<OPENHUMAN_ROOT>/src/core/observability.rs:6917:        // `domain=agent` half of the flood is suppressed too.
<OPENHUMAN_ROOT>/src/core/observability.rs:6993:        let event = event_with_message("Agent exceeded maximum tool iterations (8)");
<OPENHUMAN_ROOT>/src/core/observability.rs:7003:            "agent.run_single failed: Agent exceeded maximum tool iterations (10)",
<OPENHUMAN_ROOT>/src/core/observability.rs:7100:        (http://127.0.0.1:18474/agent-integrations/composio/connections) \
<OPENHUMAN_ROOT>/src/core/observability.rs:7108:        GET http://127.0.0.1:18474/agent-integrations/composio/connections failed: \
<OPENHUMAN_ROOT>/src/core/observability.rs:7110:        (http://127.0.0.1:18474/agent-integrations/composio/connections) \
<OPENHUMAN_ROOT>/src/core/observability.rs:7171:                   http://127.0.0.1:8080/agent-integrations/composio/connections: \
<OPENHUMAN_ROOT>/src/core/observability.rs:7188:                   (https://api.tinyhumans.ai/agent-integrations/composio/connections) \
<OPENHUMAN_ROOT>/src/core/observability.rs:7197:    /// agent job hits a local LM Studio server (`localhost:1234`) that isn't
<OPENHUMAN_ROOT>/src/core/observability.rs:7285:                ("path", "/agent-integrations/composio/connections"),
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:13:- `pub enum DomainEvent` — `events.rs` — `#[non_exhaustive]` catalog of events; current variants cover Agent (`AgentTurnStarted/Completed`, `AgentError`), Memory (`MemoryStored`, `MemoryRecalled`), Channels (`ChannelInboundMessage`, `ChannelMessageReceived/Processed`, `ChannelReactionReceived/Sent`, `ChannelConnected/Disconnected`), Cron (`CronJobTriggered/Completed`, `CronDeliveryRequested`), Skills, Tools, Webhooks, and System.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:31:- `src/openhuman/agent/bus.rs`, `agent/triage/{events,evaluator,escalation}.rs`, `agent_registry/tools/{dispatch,spawn_subagent}.rs` — agent + sub-agent events.
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:41:## Emission policy (tinyagents migration, 05.3)
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:43:The canonical run record is the TinyAgents event journal + status store
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:44:(`StoreEventJournal` / `FileStatusStore`, wired in `tinyagents/journal.rs`),
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:45:not this bus. When adding agent-run instrumentation:
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:48:  compression, tool-exposure, and steering signals ride the TinyAgents
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:49:  `AgentEvent` stream and are persisted to the journal; a UI reconstructs a
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:55:  module boundary the crate stream does not serve. Subagent lifecycle
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:57:  `agent_orchestration::subagent_events` (05.2), never hand-rolled
<OPENHUMAN_ROOT>/src/core/event_bus/README.md:58:  `publish_global(DomainEvent::Subagent*)`.
<OPENHUMAN_ROOT>/package.json:51:    "agent-batch": "node scripts/agent-batch/cli.mjs",
<OPENHUMAN_ROOT>/package.json:52:    "agent-batch:test": "node --test scripts/agent-batch/__tests__/lib.test.mjs scripts/agent-batch/__tests__/cli.test.mjs",
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:6:        // Agent
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:8:            DomainEvent::AgentTurnStarted {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:12:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:15:            DomainEvent::AgentTurnCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:20:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:23:            DomainEvent::AgentError {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:28:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:31:            DomainEvent::SubagentSpawned {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:33:                agent_id: "researcher".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:38:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:41:            DomainEvent::SubagentCompleted {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:44:                agent_id: "researcher".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:49:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:52:            DomainEvent::SubagentFailed {
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:55:                agent_id: "researcher".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:58:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:67:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:74:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:81:            "agent",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:377:                target_agent: "orchestrator".into(),
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:506:        // Agent meetings (issue #3507 contract events)
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:514:            "agent_meetings",
<OPENHUMAN_ROOT>/src/core/event_bus/events_tests.rs:523:            "agent_meetings",
<OPENHUMAN_ROOT>/src/core/mod.rs:9:pub mod agent_cli;
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:19:- [Optional — Let an AI coding agent guide you](#optional--let-an-ai-coding-agent-guide-you)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:402:## Optional — Let an AI coding agent guide you
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:404:If you use Claude Code, Cursor, AmpCode, Codex, or another coding agent, you can paste this prompt after cloning the repo:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:416:The agent should still ask before destructive actions like deleting files, resetting branches, or force-pushing. You are responsible for reviewing the final diff before opening a PR.
<OPENHUMAN_ROOT>/plan.md:3:Multi-agent audit of the OpenHuman test surface (2,367 files / ~25,900 test declarations per
<OPENHUMAN_ROOT>/plan.md:32:  frontend E2E (WDIO + Playwright), Rust unit (agent/memory; channels/providers/platform;
<OPENHUMAN_ROOT>/plan.md:52:| ✅ | `src/openhuman/agent/harness/harness_gap_tests.rs` | `datetime_section_is_static_grounding_rule_not_a_volatile_timestamp` | Strict subset of `agent/prompts/mod_tests.rs::datetime_section_is_static_grounding_rule_without_volatile_timestamp`; the file's own header lists item 6 as covered elsewhere. |
<OPENHUMAN_ROOT>/plan.md:91:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::grounding_contract_requires_exact_numeric_evidence` | Pins 5 verbatim prose substrings of the grounding contract — breaks on any copywriting pass. | Behavioral guarantee ("contract appended on every build path") already covered by the marker-based test; convert this to a single explicitly-labeled wording-lock, or assert stable structural markers. |
<OPENHUMAN_ROOT>/plan.md:92:| ✅ | `src/openhuman/agent/prompts/mod_tests.rs::identity_section_creates_missing_workspace_files` | Also string-matches SOUL.md brand-voice prose (`"Don't validate FUD"`). | Split: (a) files created + seeded from the checked-in template (compare against template file content); (b) a narrow, labeled brand-voice lock if the phrase must stay pinned. |
<OPENHUMAN_ROOT>/plan.md:141:- **Approval gate × agent turn**: harness-level test that a Write/Destructive-class turn parks
<OPENHUMAN_ROOT>/plan.md:164:- **Approval-gate Playwright mirror**: `agent-harness-behaviors.spec.ts` exists only in the slower
<OPENHUMAN_ROOT>/plan.md:166:- **AgentAccessPanel tier cross-check**: which sub-controls are enabled/hidden per autonomy tier.
<OPENHUMAN_ROOT>/plan.md:427:  onboarding pages, AgentAccessPanel (23 tests), and every P2 slice reducer
<OPENHUMAN_ROOT>/plan.md:499:  `council_registry`, `audio_toolkit`, `agent_experience`, `http_host`, `skill_runtime`,
<OPENHUMAN_ROOT>/README.md:64:OpenHuman is three things most assistants aren't: **a brain** that builds a persistent, local memory of your world; **a fantastic orchestrator** that runs fleets of agents on durable graphs; and **a deep researcher** that sweeps your data and the web before you finish asking. Every bullet links to the deeper writeup in the [docs](https://tinyhumans.gitbook.io/openhuman/).
<OPENHUMAN_ROOT>/README.md:76:- **[Workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows)**: the agent proposes the automation; you review it on a canvas and save. Durable, trigger-driven, approval-gated runs on open-source [tinyflows](https://github.com/tinyhumansai/tinyflows).
<OPENHUMAN_ROOT>/README.md:77:- **[A harness that finishes the job](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: checkpointed graph runs on open-source [tinyagents](https://github.com/tinyhumansai/tinyagents). Stuck agents get steered, halted ones return a root cause, and every run replays with real per-call costs.
<OPENHUMAN_ROOT>/README.md:78:- **[A split brain, always on](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**: a fast reflex agent triages inbound traffic while a deep reasoning core delegates to worker fleets, steered by the subconscious.
<OPENHUMAN_ROOT>/README.md:79:- **[An agent economy](https://tinyhumans.gitbook.io/openhuman/features/tinyplace)**: a `@handle` on [tiny.place](https://tiny.place), Signal-encrypted agent-to-agent orchestration, x402 USDC bounties and trading. Keys never touch disk.
<OPENHUMAN_ROOT>/README.md:85:- **[Meeting agents](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: joins **Meet, Zoom, Teams, and Webex** with a face and a voice. It auto-joins from your calendar, streams a live transcript, answers by name, and files a summary with action items.
<OPENHUMAN_ROOT>/README.md:87:- **[17 messaging channels](https://tinyhumans.gitbook.io/openhuman/features/channels)**: Telegram, Discord, Slack, WhatsApp, Signal, iMessage… plus **native email** (IMAP IDLE + SMTP). Your agent reaches you where you already are.
<OPENHUMAN_ROOT>/README.md:91:- **Simple, UI-first & Human**: install to working agent in a few clicks, with no config files and no terminal. And it has [a face](https://tinyhumans.gitbook.io/openhuman/features/mascot): a mascot that speaks, reacts, and remembers you.
<OPENHUMAN_ROOT>/README.md:97:OpenHuman is the first agent harness that gets to know you in minutes. Inspired by [Karpathy's LLM Knowledgebase](https://x.com/karpathy/status/2039805659525644595). Most agents start cold. Hermes learns by watching you work; OpenClaw waits for plugins to ferry context in. Either way, you spend days or weeks before the agent knows enough about your stack to be genuinely useful.
<OPENHUMAN_ROOT>/README.md:103:> OpenHuman summarizes and compresses all your documents, emails & chats; and creates a memory graph that lets your agent remember everything about you.
<OPENHUMAN_ROOT>/README.md:107:In just one sync pass, the agent has full (compressed) context of your inbox, your calendar, your repos, your docs, your messages. No training period. No "give it a few weeks.". It becomes you, controlled by you.
<OPENHUMAN_ROOT>/README.md:109:Already self-host [agentmemory](https://github.com/rohitg00/agentmemory) across other coding agents? OpenHuman ships an optional `Memory` backend that proxies to it. Set `memory.backend = "agentmemory"` in `config.toml` and the same durable store powers OpenHuman alongside Claude Code, Cursor, Codex, and OpenCode. See the [agentmemory backend](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) page for setup.
<OPENHUMAN_ROOT>/README.md:113:Most agent harnesses run one agent in one loop. OpenHuman is an **[orchestrator](https://tinyhumans.gitbook.io/openhuman/features/orchestration)**:
<OPENHUMAN_ROOT>/README.md:119:> Agent-to-agent messaging runs over Signal-protocol end-to-end encryption, so you can connect anything (Claude Code, Codex, OpenClaw, Hermes) and use OpenHuman to orchestrate all of your agents and tools.
