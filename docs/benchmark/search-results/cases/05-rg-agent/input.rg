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
<OPENHUMAN_ROOT>/src/core/logging.rs:96:                Level::TRACE => Style::new().fg(Color::Magenta).dimmed().paint(tag),
<OPENHUMAN_ROOT>/src/core/logging.rs:518:        std::env::set_var("OPENHUMAN_LOG_FILE_CONSTRAINTS", "rpc, , agent ,memory");
<OPENHUMAN_ROOT>/src/core/logging.rs:520:        assert_eq!(parsed, vec!["rpc", "agent", "memory"]);
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
<OPENHUMAN_ROOT>/README.md:121:- **Graphs, not loops**: turns run as checkpointed graphs on [tinyagents](https://github.com/tinyhumansai/tinyagents). They pause for a human, survive a restart, and resume mid-run.
<OPENHUMAN_ROOT>/README.md:122:- **Sub-agent fleets**: specialists spawn three levels deep; stuck agents become root-cause reports.
<OPENHUMAN_ROOT>/README.md:123:- **Agent-to-agent, encrypted**: instances orchestrate each other over Signal-protocol E2E sessions with x402 payments. No server ever sees plaintext.
<OPENHUMAN_ROOT>/README.md:127:Heavily inspired by n8n and Zapier, [workflows](https://tinyhumans.gitbook.io/openhuman/features/workflows) bring the same visual, trigger-driven automation to your agent, except the agent builds them for you. Ask for an automation and it proposes one: a [tinyflows](https://github.com/tinyhumansai/tinyflows) graph you review on a visual canvas before saving.
<OPENHUMAN_ROOT>/README.md:133:> The agent proposes the workflow; you review it on a canvas and save it.
<OPENHUMAN_ROOT>/README.md:137:## OpenHuman vs Other Agent Harnesses
<OPENHUMAN_ROOT>/README.md:139:High-level comparison (products evolve, so verify against each vendor). OpenHuman is built to **minimize vendor sprawl**, keep **workflow knowledge on-device**, and give the agent a **persistent memory** of your data, not only chat.
<OPENHUMAN_ROOT>/README.md:141:|                        | Claude Cowork     | OpenClaw          | Hermes Agent      | OpenHuman                                                                                                |
<OPENHUMAN_ROOT>/README.md:146:| **Memory**             | ✅ Chat-scoped    | ⚠️ Plugin-reliant | ✅ Self-learning  | 🚀 Memory Tree + Obsidian vault, optional [agentmemory](https://github.com/rohitg00/agentmemory) backend |
<OPENHUMAN_ROOT>/README.md:149:| **Orchestration**      | ⚠️ Sub-tasks      | ⚠️ Single loop    | ⚠️ Single loop    | 🚀 Agent graphs + checkpoints + E2E-encrypted A2A                                                        |
<OPENHUMAN_ROOT>/README.md:150:| **Workflows**          | 🚫 None           | ⚠️ Scripts        | ⚠️ Scripts        | 🚀 Visual, durable, agent-proposed, approval-gated                                                       |
<OPENHUMAN_ROOT>/README.md:161:New contributor? Start with [`CONTRIBUTING.md`](./CONTRIBUTING.md) for the fork/PR workflow and local validation commands, or use the copy-paste AI-agent prompt in [`CONTRIBUTING-BEGINNERS.md`](./CONTRIBUTING-BEGINNERS.md#optional--let-an-ai-coding-agent-guide-you). The short path is:
<OPENHUMAN_ROOT>/src/core/runtime.rs:3://! A single agent turn is a very large async state machine (system prompt +
<OPENHUMAN_ROOT>/src/core/runtime.rs:5://! to a sub-agent runs another full turn one level down. Even with the inner
<OPENHUMAN_ROOT>/src/core/runtime.rs:6://! sub-agent future boxed, that nesting overflows tokio's default 2 MiB
<OPENHUMAN_ROOT>/src/core/runtime.rs:12://! an agent turn (the desktop Tauri host's runtime, `agent_cli`, the rest of
<OPENHUMAN_ROOT>/src/core/runtime.rs:14://! in sync; downstream call sites should set `.thread_stack_size(AGENT_WORKER_STACK_BYTES)`
<OPENHUMAN_ROOT>/src/core/runtime.rs:15://! on every multi-thread runtime that may host an agent turn.
<OPENHUMAN_ROOT>/src/core/runtime.rs:16:pub const AGENT_WORKER_STACK_BYTES: usize = 16 * 1024 * 1024;
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:19:- [Optional — Let an AI coding agent guide you](#optional--let-an-ai-coding-agent-guide-you)
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:402:## Optional — Let an AI coding agent guide you
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:404:If you use Claude Code, Cursor, AmpCode, Codex, or another coding agent, you can paste this prompt after cloning the repo:
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:410:AGENTS.md: https://raw.githubusercontent.com/tinyhumansai/openhuman/main/AGENTS.md
<OPENHUMAN_ROOT>/CONTRIBUTING-BEGINNERS.md:416:The agent should still ask before destructive actions like deleting files, resetting branches, or force-pushing. You are responsible for reviewing the final diff before opening a PR.
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:130:        "openhuman.local_ai_agent_chat",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:131:        "openhuman.inference_agent_chat",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:134:        "openhuman.local_ai_agent_chat_simple",
<OPENHUMAN_ROOT>/src/core/legacy_aliases.rs:135:        "openhuman.inference_agent_chat_simple",
<OPENHUMAN_ROOT>/src/core/cli.rs:17:/// Debug/e2e agent paths can build deep async poll stacks while assembling
<OPENHUMAN_ROOT>/src/core/cli.rs:18:/// prompts, provider requests, and sub-agent tool loops.
<OPENHUMAN_ROOT>/src/core/cli.rs:83:        "agent" => {
<OPENHUMAN_ROOT>/src/core/cli.rs:85:                "[cli] dispatching to agent subcommand, args={:?}",
<OPENHUMAN_ROOT>/src/core/cli.rs:88:            crate::core::agent_cli::run_agent_command(&args[1..])
<OPENHUMAN_ROOT>/src/core/cli.rs:289:    // A single agent turn is a very large async state machine (system prompt +
<OPENHUMAN_ROOT>/src/core/cli.rs:291:    // to a sub-agent runs another full turn one level down. Even with the inner
<OPENHUMAN_ROOT>/src/core/cli.rs:292:    // sub-agent future boxed (`subagent_runner::ops`), that nesting overflows
<OPENHUMAN_ROOT>/src/core/cli.rs:298:        .thread_stack_size(crate::core::runtime::AGENT_WORKER_STACK_BYTES)
<OPENHUMAN_ROOT>/src/core/cli.rs:348:    // (e.g. `agent.chat`), so it needs the same roomy stack as the server.
<OPENHUMAN_ROOT>/src/core/cli.rs:351:        .thread_stack_size(crate::core::runtime::AGENT_WORKER_STACK_BYTES)
<OPENHUMAN_ROOT>/src/core/cli.rs:432:        .thread_stack_size(crate::core::runtime::AGENT_WORKER_STACK_BYTES)
<OPENHUMAN_ROOT>/src/core/cli.rs:566:    println!("  openhuman agent <subcommand> [options]    (inspect agent definitions & prompts)");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:110:                // here so the message is left untouched for direct (agent-tool)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:386:/// can run (the feed, signal/messaging, etc. — backend `GraphQLAuth::Agent`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:395:/// sentinel string to agent tools that call those handlers directly.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1143:        // AgentBox are merged below) so the outer router becomes
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1149:    // Mount AgentBox marketplace routes when explicitly enabled.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1155:    if crate::openhuman::agentbox::agentbox_mode_enabled() {
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1156:        let store = crate::openhuman::agentbox::JobStore::new(std::time::Duration::from_secs(3600));
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1157:        let invoker: std::sync::Arc<dyn crate::openhuman::agentbox::invoker::AgentInvoker> =
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1158:            std::sync::Arc::new(crate::openhuman::agentbox::invoker::CoreAgentInvoker);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1159:        let job_timeout = std::env::var("OPENHUMAN_AGENTBOX_JOB_TIMEOUT_SECS")
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1173:                    log::info!("[agentbox] sweep evicted {} terminal jobs", evicted);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1178:        log::info!("[agentbox] enabled; public routes: POST /run, GET /jobs/{{id}}, GET /health");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1179:        router = router.merge(crate::openhuman::agentbox::agentbox_router(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1520:/// set to the domain name (agent, tool, memory, etc.).
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1591:            let agent = event.agent_hint().unwrap_or("").to_string();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1595:                "agent": agent,
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1755:    // AgentBox GMI MaaS provider bridge — no-op when env vars absent.
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1756:    // Must run BEFORE `build_core_http_router` mounts the AgentBox routes so
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1760:    crate::openhuman::agentbox::register_gmi_provider_if_present();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:1829:                crate::openhuman::agent::multimodal::init_attachments_dir(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2133:                // Ensure proactive agent jobs (e.g. the autonomous bounty job)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2137:                if let Err(e) = crate::openhuman::cron::seed::seed_proactive_agents_on_boot(&config)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2139:                    log::warn!("[cron] boot seed of proactive agent jobs failed: {e}");
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2285:        crate::openhuman::agent_meetings::calendar::register_meet_calendar_subscriber();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2286:        crate::openhuman::agent_meetings::bus::register_meeting_event_subscriber();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2302:        crate::openhuman::agent::task_dispatcher::start_board_poller();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2317:        // on every agent's tool output, so this must be set before any agent
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2388:        // The agent `agent.run_turn` handler is what channel dispatch
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2390:        crate::openhuman::agent::bus::register_agent_handlers();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2392:        // Background-completion delivery: when a detached sub-agent
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2393:        // (spawn_async_subagent) finishes, surface its result back into the
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2395:        crate::openhuman::agent_orchestration::background_delivery::register_background_delivery();
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2397:        // Run-ledger finalizer: detached `spawn_async_subagent` runs outlive
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2398:        // their parent turn, so their terminal `AgentProgress` never reaches the
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2400:        // subscriber settles `agent_runs` from `DomainEvent::Subagent{Completed,
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2403:        crate::openhuman::agent_orchestration::run_ledger_finalize::register_run_ledger_finalize_subscriber(&config);
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2413:            "[event_bus] domain subscribers registered (webhook, channel, health, conversation, composio, restart, proactive, agent, session_expired, mcp_client)"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2490:    // Detached sub-agent runs (`spawn_async_subagent`) from a previous process
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2491:    // are gone with that process. Any `agent_runs` row still marked `running`
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2495:    match crate::openhuman::session_db::run_ledger::interrupt_orphaned_agent_runs(&cfg) {
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2497:        Ok(count) => log::info!("[runtime] settled {count} orphaned agent run(s) on startup"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2498:        Err(err) => log::warn!("[runtime] failed to settle orphaned agent runs: {err}"),
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2501:    // --- Detached sub-agent TaskStore reconciliation -------------------
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2503:    // orchestration_tasks.jsonl`) can hold non-terminal sub-agent records left
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2511:            crate::openhuman::agent_orchestration::running_subagents::reconcile_orphaned_tasks_on_boot(
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2516:                "[runtime] reconciled {reconciled} orphaned detached sub-agent task(s) on startup"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2536:    // --- Sub-agent definition registry bootstrap ---
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2538:    // under `<workspace>/agents/*.toml`. Idempotent — safe to call
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2541:        crate::openhuman::agent::harness::AgentDefinitionRegistry::init_global(&workspace_dir)
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2544:            "[runtime] AgentDefinitionRegistry::init_global failed: {err} — \
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2545:             spawn_subagent will be unavailable until restart"
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2549:    // --- Agent sandbox + projects dirs ---
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2558:    crate::openhuman::config::ensure_agent_dirs(&mut cfg).await;
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2599:    // on now that the release surface exists (ApprovalRequestCard + the Agent
<OPENHUMAN_ROOT>/src/core/jsonrpc.rs:2702:    // tools are available to the agent as soon as the core is ready.
<OPENHUMAN_ROOT>/src/core/event_bus/bus.rs:59:/// (e.g., an agent turn completed, a memory was stored).
<OPENHUMAN_ROOT>/src/core/mod.rs:9:pub mod agent_cli;
<OPENHUMAN_ROOT>/docs/README.ko.md:75:- **[일을 끝까지 마무리하는 하네스](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**: 오픈 소스 [tinyagents](https://github.com/tinyhumansai/tinyagents) 기반의 체크포인트 그래프 실행입니다. 막힌 에이전트는 방향을 조정받고, 중단된 에이전트는 근본 원인을 돌려주며, 모든 실행은 호출별 실제 비용과 함께 재생됩니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:83:- **[미팅 에이전트](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**: 얼굴과 목소리를 가지고 **Meet, Zoom, Teams, Webex**에 참여합니다. 캘린더에서 자동으로 참여하고, 실시간 자막을 스트리밍하며, 이름이 불리면 대답하고, 요약과 액션 아이템을 정리합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:107:이미 다른 코딩 에이전트에서 [agentmemory](https://github.com/rohitg00/agentmemory)를 자체 호스팅하고 있나요? OpenHuman은 이를 프록시하는 선택적 `Memory` 백엔드를 제공합니다. `config.toml`에서 `memory.backend = "agentmemory"`를 설정하면 동일한 내구성 있는 저장소가 Claude Code, Cursor, Codex, OpenCode와 함께 OpenHuman을 구동합니다. 설정 방법은 [agentmemory 백엔드](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend) 페이지를 참조하세요.
<OPENHUMAN_ROOT>/docs/README.ko.md:119:- **루프가 아닌 그래프**: 턴은 [tinyagents](https://github.com/tinyhumansai/tinyagents) 기반의 체크포인트가 있는 그래프로 실행되어, 사람을 위해 일시 정지하고, 재시작에도 살아남고, 실행 중간에 재개합니다.
<OPENHUMAN_ROOT>/docs/README.ko.md:139:|                    | Claude Cowork     | OpenClaw          | Hermes Agent      | OpenHuman                                                                                            |
<OPENHUMAN_ROOT>/docs/README.ko.md:144:| **메모리**         | ✅ 채팅 범위 한정 | ⚠️ 플러그인 의존  | ✅ 자기 학습      | 🚀 메모리 트리 + Obsidian 볼트, 선택적 [agentmemory](https://github.com/rohitg00/agentmemory) 백엔드 |
<OPENHUMAN_ROOT>/docs/README.ko.md:159:새로운 기여자인가요? 포크/PR 워크플로우 및 로컬 검증 명령에 대해서는 [`CONTRIBUTING.md`](../CONTRIBUTING.md)에서 시작하거나, [`CONTRIBUTING-BEGINNERS.md`](../CONTRIBUTING-BEGINNERS.md#optional--let-an-ai-coding-agent-guide-you)의 복사-붙여넣기 AI 에이전트 프롬프트를 사용하세요. 빠른 경로는 다음과 같습니다.
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:20://! [`crate::openhuman::agent::bus::mock_agent_run_turn`]) compose on top of
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:61:/// [`crate::openhuman::agent::bus::use_real_agent_handler`] that need the
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:62:/// real agent handler installed without racing against a stub-installing
<OPENHUMAN_ROOT>/src/core/event_bus/testing.rs:110:/// [`crate::openhuman::agent::bus::mock_agent_run_turn`]) should compose
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:1://! `openhuman agent` — developer CLI for inspecting agent definitions and
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:6://! agent definitions / tool registry and printing something.
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:9://!   openhuman agent dump-prompt --agent <id> [--toolkit <slug>] [--workspace <path>] [--json] [--with-tools] [-v]
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:10://!     (--toolkit is REQUIRED when --agent is `integrations_agent`.)
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:11://!   openhuman agent dump-all --out <dir> [--workspace <path>] [--model <name>] [-v]
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:12://!   openhuman agent list [--json] [-v]
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:15://! context engine would hand to the LLM when that agent is spawned. The
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:16://! dump routes through [`Agent::from_config_for_agent`] and calls
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:17://! [`Agent::build_system_prompt`] on the live session, so the output is
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:19://! `--agent orchestrator` for the orchestrator prompt; otherwise pass
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:20://! any built-in or workspace-custom agent id (e.g. `integrations_agent`,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:26:use crate::openhuman::agent::debug::{
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:27:    dump_agent_prompt, dump_all_agent_prompts, write_prompt_dumps, DumpPromptOptions, DumpedPrompt,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:29:use crate::openhuman::agent::harness::definition::AgentDefinitionRegistry;
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:31:/// Entry point for `openhuman agent <subcommand>`.
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:32:pub fn run_agent_command(args: &[String]) -> Result<()> {
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:34:        print_agent_help();
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:43:            "unknown agent subcommand '{other}'. Run `openhuman agent --help`."
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:94:                println!("Usage: openhuman agent dump-all --out <dir> [--workspace <path>] [--model <name>] [-v]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:96:                println!("Render every registered agent's turn-1 system prompt into <dir>.");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:97:                println!("`integrations_agent` is expanded into one file per currently-connected");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:117:        "[agent-cli] run_dump_all entry: out={} workspace={:?} model={:?}",
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:125:        .thread_stack_size(crate::core::runtime::AGENT_WORKER_STACK_BYTES)
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:127:    log::debug!("[agent-cli] run_dump_all: calling dump_all_agent_prompts");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:129:        dump_all_agent_prompts(flags.workspace.clone(), flags.model.clone()).await
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:132:        "[agent-cli] run_dump_all: dump_all_agent_prompts returned {} prompt(s)",
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:138:        "[agent-cli] run_dump_all exit: wrote {} prompt(s) + SUMMARY.txt",
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:149:    agent: Option<String>,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:160:        agent: None,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:171:            "--agent" | "-a" => {
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:172:                out.agent = Some(
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:174:                        .ok_or_else(|| anyhow!("missing value for --agent"))?
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:227:    let agent = flags.agent.clone().ok_or_else(|| {
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:228:        anyhow!("--agent <id> is required (e.g. `orchestrator`, `integrations_agent`, `welcome`)")
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:231:    if agent == "integrations_agent" && flags.toolkit.is_none() {
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:233:            "--toolkit <slug> is required when --agent is `integrations_agent` \
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:241:        "[agent-cli] run_dump_prompt entry: agent={} toolkit={:?} workspace={:?} model={:?}",
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:242:        agent,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:249:        agent_id: agent,
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:257:        .thread_stack_size(crate::core::runtime::AGENT_WORKER_STACK_BYTES)
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:259:    log::debug!("[agent-cli] run_dump_prompt: calling dump_agent_prompt");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:260:    let dumped = rt.block_on(async { dump_agent_prompt(options).await })?;
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:262:        "[agent-cli] run_dump_prompt: dump returned (tools={}, skill_tools={}, prompt_bytes={})",
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:277:    // Banner on stderr so `openhuman agent dump-prompt ... > file.md` stays
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:281:    eprintln!("# Agent prompt dump");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:282:    eprintln!("agent:          {}", dumped.agent_id);
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:305:    // DumpedPrompt (which would pull the agent harness types into our
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:309:        "agent_id".into(),
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:310:        serde_json::Value::String(dumped.agent_id.clone()),
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:390:                println!("Usage: openhuman agent list [--workspace <path>] [--json] [-v]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:392:                println!("  List every built-in agent plus any custom `<workspace>/agents/*.toml` overrides.");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:399:    // Silence the logger so Config::load_or_init and AgentDefinitionRegistry::load
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:408:        AgentDefinitionRegistry::load(&ws)?
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:413:        rt.block_on(AgentDefinitionRegistry::load_for_default_workspace())?
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:454:        println!("{} agent(s) registered.", registry.len());
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:463:fn print_agent_help() {
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:464:    println!("openhuman agent — inspect agents and the prompts they receive");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:467:    println!("  openhuman agent list [--workspace <path>] [--json]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:468:    println!("  openhuman agent dump-prompt --agent <id> [--workspace <path>] [--model <name>] [--with-tools] [--json] [-v]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:469:    println!("  openhuman agent dump-all --out <dir> [--workspace <path>] [--model <name>] [-v]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:471:    println!("Run `openhuman agent <subcommand> --help` for details.");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:475:    println!("openhuman agent dump-prompt — render the exact system prompt an agent receives");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:478:    println!("  openhuman agent dump-prompt --agent <id> [options]");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:481:    println!("  --agent, -a <id>     Target agent id — any built-in or workspace-custom id");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:482:    println!("                       (e.g. `orchestrator`, `integrations_agent`, `welcome`).");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:485:    println!("  --toolkit, -t <slug> REQUIRED when `--agent integrations_agent`. Names the");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:493:    println!("  --with-tools         Also print the full list of tool names the agent sees.");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:499:    println!("  openhuman agent dump-prompt --agent orchestrator --json");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:501:    println!("  # integrations_agent bound to the user's gmail connection.");
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:503:        "  openhuman agent dump-prompt --agent integrations_agent --toolkit gmail --with-tools"
<OPENHUMAN_ROOT>/src/core/agent_cli.rs:513:/// `agent dump-prompt` is designed to be redirected into a file, and
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:31://! register_native_global::<AgentTurnRequest, AgentTurnResponse, _, _>(
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:32://!     "agent.run_turn",
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:34://!         let text = run_agent_turn(/* ... */).await
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:36://!         Ok(AgentTurnResponse::new(text))
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:41://! let resp: AgentTurnResponse = request_native_global(
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:42://!     "agent.run_turn",
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:43://!     AgentTurnRequest { /* owned + Arc fields */ },
<OPENHUMAN_ROOT>/src/core/event_bus/native_request.rs:120:/// Handlers are keyed by a method name (e.g., `"agent.run_turn"`) and store the
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:9:- **HOST-OWNED** — the change lives in a layer that stays in OpenHuman (RPC, agent tools,
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:64:| D3 | `c43f79641` (07-03) (within TinyAgents migration) | `memory_store/vectors/store.rs` | `count()` reads `COUNT(*)` as `i64` and converts via `usize::try_from(...).context(...)` instead of `row.get::<usize>` directly — robustness against platform `usize`/`i64` mismatch. | **ABSENT.** `vendor/tinycortex/src/memory/store/vectors/store.rs:370–380` still does `let count: usize = ... row.get(0)` then `Ok(count)`. | `store::vectors::store` — small; port the `i64` + `try_from` guard. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:77:| `0304d145f` (07-03) | `memory/tools/store.rs`, `memory/tools/forget.rs` | Agent tools | Tool contract/prompt text; agent tools stay host (plan §1). |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:82:| `c43f79641` (07-03) | `memory_search/{vector,tools}/*`, `memory_sync/composio/*` | Agent tools / live sync | Import-path churn from the TinyAgents cutover + live-sync; not engine semantics. |
<OPENHUMAN_ROOT>/docs/tinycortex-drift-ledger.md:120:its `tools/` stay host (agent tools), its `vector`/`scoring` are engine (W5) — flagged for the gap audit.
<OPENHUMAN_ROOT>/src/core/event_bus/subscriber.rs:17:    /// or `Some(&["agent", "cron"])` to receive only matching domains.
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:75:- **[能把事情做完的执行框架](https://tinyhumans.gitbook.io/openhuman/developing/architecture/agent-harness)**：基于开源 [tinyagents](https://github.com/tinyhumansai/tinyagents) 的检查点式图运行：卡住的智能体会被引导回正轨，中止的会交回根因，每次运行都可回放并附带真实的每次调用成本。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:83:- **[会议智能体](https://tinyhumans.gitbook.io/openhuman/features/mascot/meeting-agents)**：带着一张脸和一副嗓音加入 **Meet、Zoom、Teams 和 Webex**：根据日历自动入会、实时输出转写字幕、被点名时回答、归档摘要和行动项。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:107:已经在其他编码智能体中自托管 [agentmemory](https://github.com/rohitg00/agentmemory)？OpenHuman 提供可选的 `Memory` 后端来代理它：在 `config.toml` 中设置 `memory.backend = "agentmemory"`，同一个持久化存储将同时服务于 OpenHuman 和 Claude Code、Cursor、Codex、OpenCode。详见 [agentmemory 后端](https://tinyhumans.gitbook.io/openhuman/features/obsidian-wiki/agentmemory-backend)页面。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:119:- **图，而非循环**：每一轮对话都作为带检查点的图在 [tinyagents](https://github.com/tinyhumansai/tinyagents) 上运行，可暂停等待人工介入、可在重启后存活、可从运行中途恢复。
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:139:|                | Claude Cowork    | OpenClaw    | Hermes Agent | OpenHuman                                                                                   |
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:144:| **记忆**       | ✅ 对话范围      | ⚠️ 依赖插件 | ✅ 自学习    | 🚀 记忆树 + Obsidian 仓库，可选 [agentmemory](https://github.com/rohitg00/agentmemory) 后端 |
<OPENHUMAN_ROOT>/docs/README.zh-CN.md:159:新贡献者？从 [`CONTRIBUTING.md`](../CONTRIBUTING.md) 了解 fork/PR 工作流和本地验证命令，或使用 [`CONTRIBUTING-BEGINNERS.md`](../CONTRIBUTING-BEGINNERS.md#optional--let-an-ai-coding-agent-guide-you) 中可直接复制粘贴的 AI 智能体提示词。快速路径：
<OPENHUMAN_ROOT>/src/core/event_bus/mod.rs:5://! modules (like memory, skills, and agents) to communicate without
<OPENHUMAN_ROOT>/src/core/socketio.rs:123:    /// `"provider"` | `"openhuman_budget"` | `"agent_loop"`
<OPENHUMAN_ROOT>/src/core/socketio.rs:139:    /// agent loop) and for transport failures without a provider prefix.
<OPENHUMAN_ROOT>/src/core/socketio.rs:197:    /// Sub-agent specific progress detail. Populated on
<OPENHUMAN_ROOT>/src/core/socketio.rs:198:    /// `subagent_spawned`, `subagent_completed`, `subagent_iteration_start`,
<OPENHUMAN_ROOT>/src/core/socketio.rs:199:    /// `subagent_tool_call`, and `subagent_tool_result` events so the UI
<OPENHUMAN_ROOT>/src/core/socketio.rs:200:    /// can attribute child activity to the parent's live subagent row
<OPENHUMAN_ROOT>/src/core/socketio.rs:202:    /// non-subagent event.
<OPENHUMAN_ROOT>/src/core/socketio.rs:204:    pub subagent: Option<SubagentProgressDetail>,
<OPENHUMAN_ROOT>/src/core/socketio.rs:209:    /// `subagent_tool_call`), e.g. "Reading messages". The frontend renders
<OPENHUMAN_ROOT>/src/core/socketio.rs:215:    /// `subagent_tool_call`), e.g. "steven@gmail.com" — the bracketed target
<OPENHUMAN_ROOT>/src/core/socketio.rs:220:    /// sub-agents), carried on `chat_done`. Lets the UI footer show session
<OPENHUMAN_ROOT>/src/core/socketio.rs:222:    /// per-sub-agent hover breakdown. `None` for every non-`chat_done` event and
<OPENHUMAN_ROOT>/src/core/socketio.rs:230:/// Every numeric is a turn total (parent agent **plus** any sub-agents spawned
<OPENHUMAN_ROOT>/src/core/socketio.rs:231:/// during the turn); the `subagents` list breaks the same spend down per child
<OPENHUMAN_ROOT>/src/core/socketio.rs:243:    /// Per-sub-agent spend, omitted from the wire when no sub-agents ran.
<OPENHUMAN_ROOT>/src/core/socketio.rs:245:    pub subagents: Vec<SubagentUsagePayload>,
<OPENHUMAN_ROOT>/src/core/socketio.rs:248:/// One sub-agent's token/cost contribution within a turn (hover breakdown).
<OPENHUMAN_ROOT>/src/core/socketio.rs:251:pub struct SubagentUsagePayload {
<OPENHUMAN_ROOT>/src/core/socketio.rs:253:    pub agent_id: String,
<OPENHUMAN_ROOT>/src/core/socketio.rs:259:/// Per-event subagent progress detail attached to `WebChannelEvent`.
<OPENHUMAN_ROOT>/src/core/socketio.rs:262:/// subagent block — child iteration counters, mode, child task/agent
<OPENHUMAN_ROOT>/src/core/socketio.rs:264:/// the agent id on top-level subagent events but not on nested
<OPENHUMAN_ROOT>/src/core/socketio.rs:265:/// `subagent_tool_*` events where `tool_name` is the *child's* tool),
<OPENHUMAN_ROOT>/src/core/socketio.rs:266:/// and final-run statistics on `subagent_completed`.
<OPENHUMAN_ROOT>/src/core/socketio.rs:269:/// absent — this keeps the wire format compact for non-subagent events
<OPENHUMAN_ROOT>/src/core/socketio.rs:274:pub struct SubagentProgressDetail {
<OPENHUMAN_ROOT>/src/core/socketio.rs:281:    /// Character length of the delegation prompt (on `subagent_spawned`).
<OPENHUMAN_ROOT>/src/core/socketio.rs:284:    /// Sub-agent's child iteration counter (on `subagent_iteration_start`,
<OPENHUMAN_ROOT>/src/core/socketio.rs:285:    /// `subagent_tool_call`, `subagent_tool_result`). 1-based.
<OPENHUMAN_ROOT>/src/core/socketio.rs:288:    /// Sub-agent's configured iteration cap.
<OPENHUMAN_ROOT>/src/core/socketio.rs:291:    /// Child agent id (on nested `subagent_tool_*` events where the flat
<OPENHUMAN_ROOT>/src/core/socketio.rs:292:    /// `tool_name` is the child's tool, not the agent).
<OPENHUMAN_ROOT>/src/core/socketio.rs:294:    pub agent_id: Option<String>,
<OPENHUMAN_ROOT>/src/core/socketio.rs:295:    /// Spawn task id (on nested `subagent_tool_*` events).
<OPENHUMAN_ROOT>/src/core/socketio.rs:301:    /// Total iterations the sub-agent used (on `subagent_completed`).
<OPENHUMAN_ROOT>/src/core/socketio.rs:304:    /// Character length of the sub-agent's final assistant text
<OPENHUMAN_ROOT>/src/core/socketio.rs:305:    /// (on `subagent_completed`) or the tool result
<OPENHUMAN_ROOT>/src/core/socketio.rs:306:    /// (on `subagent_tool_result`).
<OPENHUMAN_ROOT>/src/core/socketio.rs:310:    /// `subagent_spawned`). The frontend stores it on the subagent row and
<OPENHUMAN_ROOT>/src/core/socketio.rs:311:    /// uses it to reopen the full parent↔subagent conversation from memory.
<OPENHUMAN_ROOT>/src/core/socketio.rs:314:    /// Human-readable display name from the agent registry (e.g.
<OPENHUMAN_ROOT>/src/core/socketio.rs:315:    /// "Researcher", "Coding Agent"). The frontend uses this for
<OPENHUMAN_ROOT>/src/core/socketio.rs:316:    /// consistent agent labels across timeline, sub-mascots, and drawer.
<OPENHUMAN_ROOT>/src/core/socketio.rs:320:    /// (on `subagent_completed`, when the worker ran with
<OPENHUMAN_ROOT>/src/core/socketio.rs:326:    /// after the run (on `subagent_completed`). Absent for non-isolated
<OPENHUMAN_ROOT>/src/core/socketio.rs:331:    /// (on `subagent_completed`). A dirty worktree must not be auto-removed —
<OPENHUMAN_ROOT>/src/core/socketio.rs:433:            // (welcome agent, morning briefing, cron-driven announcements)
<OPENHUMAN_ROOT>/src/core/socketio.rs:630:    let io_agent_meetings = io.clone();
<OPENHUMAN_ROOT>/src/core/socketio.rs:1095:                        "[socketio] event_bus not initialised after {}s — agent_meetings bridge giving up",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1109:                        "[socketio] dropped {} event_bus events due to lag (agent_meetings bridge)",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1122:                    log::debug!("[socketio] broadcast agent_meetings:joined");
<OPENHUMAN_ROOT>/src/core/socketio.rs:1123:                    let _ = io_agent_meetings.emit("agent_meetings:joined", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1131:                    log::debug!("[socketio] broadcast agent_meetings:left reason={}", reason);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1132:                    let _ = io_agent_meetings.emit("agent_meetings:left", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1147:                        "[socketio] broadcast agent_meetings:reply reply_len={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1150:                    let _ = io_agent_meetings.emit("agent_meetings:reply", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1165:                        "[socketio] broadcast agent_meetings:harness instruction_len={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1168:                    let _ = io_agent_meetings.emit("agent_meetings:harness", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1181:                        "[socketio] broadcast agent_meetings:transcript turns={} duration_ms={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1185:                    let _ = io_agent_meetings.emit("agent_meetings:transcript", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1200:                        "[socketio] broadcast agent_meetings:transcript_delta index={} is_partial={}",
<OPENHUMAN_ROOT>/src/core/socketio.rs:1204:                    let _ = io_agent_meetings.emit("agent_meetings:transcript_delta", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1212:                    log::debug!("[socketio] broadcast agent_meetings:error");
<OPENHUMAN_ROOT>/src/core/socketio.rs:1213:                    let _ = io_agent_meetings.emit("agent_meetings:error", &payload);
<OPENHUMAN_ROOT>/src/core/socketio.rs:1218:        log::debug!("[socketio] agent_meetings bridge stopped");
