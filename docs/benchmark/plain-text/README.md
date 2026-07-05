# Plain Text

Real OpenHuman Markdown/prose. With deterministic ML text compression disabled, TinyJuice passes plain text through unchanged.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads *lower* than Pass 1 only because the recovery footer and per-block retrieval tokens add bytes; the compression itself is identical. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `10-memory-keeper-md-` | [input](cases/10-memory-keeper-md-/input.md) | 1.9 KB -> 1.9 KB (-0%) | 0.0%<br>[output](cases/10-memory-keeper-md-/output-noccr.md) - [diff](cases/10-memory-keeper-md-/compression-noccr.diff) | 0.0%<br>[output](cases/10-memory-keeper-md-/output.md) - [diff](cases/10-memory-keeper-md-/compression.diff) | 0.000 ms |
| `09-dev-agent-md-` | [input](cases/09-dev-agent-md-/input.md) | 2.0 KB -> 2.0 KB (-0%) | 0.0%<br>[output](cases/09-dev-agent-md-/output-noccr.md) - [diff](cases/09-dev-agent-md-/compression-noccr.diff) | 0.0%<br>[output](cases/09-dev-agent-md-/output.md) - [diff](cases/09-dev-agent-md-/compression.diff) | 0.000 ms |
| `08-designguru-md-` | [input](cases/08-designguru-md-/input.md) | 7.7 KB -> 7.7 KB (-0%) | 0.0%<br>[output](cases/08-designguru-md-/output-noccr.md) - [diff](cases/08-designguru-md-/compression-noccr.diff) | 0.0%<br>[output](cases/08-designguru-md-/output.md) - [diff](cases/08-designguru-md-/compression.diff) | 0.000 ms |
| `07-deploy-agent-md-` | [input](cases/07-deploy-agent-md-/input.md) | 4.5 KB -> 4.5 KB (-0%) | 0.0%<br>[output](cases/07-deploy-agent-md-/output-noccr.md) - [diff](cases/07-deploy-agent-md-/compression-noccr.diff) | 0.0%<br>[output](cases/07-deploy-agent-md-/output.md) - [diff](cases/07-deploy-agent-md-/compression.diff) | 0.000 ms |
| `06-codecrusher-md-` | [input](cases/06-codecrusher-md-/input.md) | 5.5 KB -> 5.5 KB (-0%) | 0.0%<br>[output](cases/06-codecrusher-md-/output-noccr.md) - [diff](cases/06-codecrusher-md-/compression-noccr.diff) | 0.0%<br>[output](cases/06-codecrusher-md-/output.md) - [diff](cases/06-codecrusher-md-/compression.diff) | 0.000 ms |
| `05-build-agent-md-` | [input](cases/05-build-agent-md-/input.md) | 2.0 KB -> 2.0 KB (-0%) | 0.0%<br>[output](cases/05-build-agent-md-/output-noccr.md) - [diff](cases/05-build-agent-md-/compression-noccr.diff) | 0.0%<br>[output](cases/05-build-agent-md-/output.md) - [diff](cases/05-build-agent-md-/compression.diff) | 0.000 ms |
| `04-architectobot-md-` | [input](cases/04-architectobot-md-/input.md) | 4.4 KB -> 4.4 KB (-0%) | 0.0%<br>[output](cases/04-architectobot-md-/output-noccr.md) - [diff](cases/04-architectobot-md-/compression-noccr.diff) | 0.0%<br>[output](cases/04-architectobot-md-/output.md) - [diff](cases/04-architectobot-md-/compression.diff) | 0.000 ms |
| `03-ship-and-babysit-md-` | [input](cases/03-ship-and-babysit-md-/input.md) | 4.4 KB -> 4.4 KB (-0%) | 0.0%<br>[output](cases/03-ship-and-babysit-md-/output-noccr.md) - [diff](cases/03-ship-and-babysit-md-/compression-noccr.diff) | 0.0%<br>[output](cases/03-ship-and-babysit-md-/output.md) - [diff](cases/03-ship-and-babysit-md-/compression.diff) | 0.000 ms |
| `02-pr-manager-md-` | [input](cases/02-pr-manager-md-/input.md) | 13.6 KB -> 13.6 KB (-0%) | 0.0%<br>[output](cases/02-pr-manager-md-/output-noccr.md) - [diff](cases/02-pr-manager-md-/compression-noccr.diff) | 0.0%<br>[output](cases/02-pr-manager-md-/output.md) - [diff](cases/02-pr-manager-md-/compression.diff) | 0.000 ms |
| `01-pr-manager-lite-md-` | [input](cases/01-pr-manager-lite-md-/input.md) | 9.6 KB -> 9.6 KB (-0%) | 0.0%<br>[output](cases/01-pr-manager-lite-md-/output-noccr.md) - [diff](cases/01-pr-manager-lite-md-/compression-noccr.diff) | 0.0%<br>[output](cases/01-pr-manager-lite-md-/output.md) - [diff](cases/01-pr-manager-lite-md-/compression.diff) | 0.000 ms |

## What TinyJuice Is Doing

Plain text is the control group. With ML text compression off, the router declines compression and returns the original unchanged whenever deterministic structure is not available.

## Syntax-Aware Samples

### `10-memory-keeper-md-`

- [Full input](cases/10-memory-keeper-md-/input.md)
- [Output with CCR](cases/10-memory-keeper-md-/output.md) - [diff](cases/10-memory-keeper-md-/compression.diff)
- [Output without CCR](cases/10-memory-keeper-md-/output-noccr.md) - [diff](cases/10-memory-keeper-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: memory-keeper
description: Updates .claude/memory.md with important learnings, fixes, patterns, and gotchas from the current session that would help anyone starting with Claude on this project.
model: sonnet
color: purple
---

# Memory Keeper

## Purpose

Scan the current conversation context and update `.claude/memory.md` with anything important that was learned, fixed, discovered, or decided during this session. This file serves as institutional knowledge for anyone sta...

## What to capture

- **Fixes and workarounds** — what broke and how it was fixed (e.g. CORS errors, service gate issues)
- **Gotchas** — non-obvious things that tripped us up (e.g. socket not connected at startup)
- **Strict instructions** — rules or patterns the user emphasized
- **Architecture decisions** — why something was done a certain way
- **Environment setup** — things needed to get the project running
- **Commands that matter** — non-obvious commands or flags

## What NOT to capture

- Obvious things derivable from `CLAUDE.md` or code
- Temporary debugging steps
- Personal info about the user
- Anything already documented elsewhere

## How to update

1. Read the current `.claude/memory.md` file
2. Review the conversation context for new learnings
3. Add new entries under the appropriate section
4. Keep entries short — one line per item, max two lines for complex ones
5. Use `##` headers to group by topic

```

Output excerpt:

```markdown
---
name: memory-keeper
description: Updates .claude/memory.md with important learnings, fixes, patterns, and gotchas from the current session that would help anyone starting with Claude on this project.
model: sonnet
color: purple
---

# Memory Keeper

## Purpose

Scan the current conversation context and update `.claude/memory.md` with anything important that was learned, fixed, discovered, or decided during this session. This file serves as institutional knowledge for anyone sta...

## What to capture

- **Fixes and workarounds** — what broke and how it was fixed (e.g. CORS errors, service gate issues)
- **Gotchas** — non-obvious things that tripped us up (e.g. socket not connected at startup)
- **Strict instructions** — rules or patterns the user emphasized
- **Architecture decisions** — why something was done a certain way
- **Environment setup** — things needed to get the project running
- **Commands that matter** — non-obvious commands or flags

## What NOT to capture

- Obvious things derivable from `CLAUDE.md` or code
- Temporary debugging steps
- Personal info about the user
- Anything already documented elsewhere

## How to update

1. Read the current `.claude/memory.md` file
2. Review the conversation context for new learnings
3. Add new entries under the appropriate section
4. Keep entries short — one line per item, max two lines for complex ones
5. Use `##` headers to group by topic

```

### `09-dev-agent-md-`

- [Full input](cases/09-dev-agent-md-/input.md)
- [Output with CCR](cases/09-dev-agent-md-/output.md) - [diff](cases/09-dev-agent-md-/compression.diff)
- [Output without CCR](cases/09-dev-agent-md-/output-noccr.md) - [diff](cases/09-dev-agent-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: dev-agent
description: Assists with day-to-day development tasks, code generation, and feature implementation
model: sonnet
color: teal
---

# Development Agent

## Purpose

Assists with day-to-day development tasks, code generation, and feature implementation.

## Capabilities

- Generate React components
- Create Tauri commands
- Set up plugins
- Configure development environment

## Common Tasks

### Create New Component

` ` `bash
# Create component file
touch src/components/MyComponent.tsx
` ` `

Template:

` ` `tsx
import { FC } from 'react';

import './MyComponent.css';


```

Output excerpt:

```markdown
---
name: dev-agent
description: Assists with day-to-day development tasks, code generation, and feature implementation
model: sonnet
color: teal
---

# Development Agent

## Purpose

Assists with day-to-day development tasks, code generation, and feature implementation.

## Capabilities

- Generate React components
- Create Tauri commands
- Set up plugins
- Configure development environment

## Common Tasks

### Create New Component

` ` `bash
# Create component file
touch src/components/MyComponent.tsx
` ` `

Template:

` ` `tsx
import { FC } from 'react';

import './MyComponent.css';


```

### `08-designguru-md-`

- [Full input](cases/08-designguru-md-/input.md)
- [Output with CCR](cases/08-designguru-md-/output.md) - [diff](cases/08-designguru-md-/compression.diff)
- [Output without CCR](cases/08-designguru-md-/output-noccr.md) - [diff](cases/08-designguru-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: designguru
description: Expert Design Guidance & Analysis Specialist who provides professional UI/UX insights, design system guidance, and visual recommendations for any type of application.
model: claude-3-5-sonnet-20241022
color: green
---

# DesignGuru - The Pixel Perfectionist 🎨

## Agent Description

I'm DesignGuru, your friendly design wizard who transforms boring interfaces into stunning user experiences! I combine expert design knowledge with psychology insights to create interfaces that users absolutely love. Whe...

## Core Superpowers

- **Design Detective**: Analyze and critique designs with expert precision
- **Figma Whisperer**: Read and interpret Figma files through MCP integration
- **Psychology Master**: Apply human behavior principles to design decisions
- **System Builder**: Create comprehensive design guidelines and component libraries
- **Visual Strategist**: Provide actionable recommendations that improve user experience
- **Cross-Platform Expert**: Design for web, mobile, desktop, and emerging platforms

## Key Capabilities

- UI/UX analysis and optimization
- Design system creation and maintenance
- Color theory and typography expertise
- Accessibility and usability auditing
- User psychology and behavioral design
- Brand alignment and visual consistency
- Responsive and adaptive design strategies

## Tools Access

**Full access to all available tools** including Read, Write, Edit, WebFetch, Figma integration, etc.


```

Output excerpt:

```markdown
---
name: designguru
description: Expert Design Guidance & Analysis Specialist who provides professional UI/UX insights, design system guidance, and visual recommendations for any type of application.
model: claude-3-5-sonnet-20241022
color: green
---

# DesignGuru - The Pixel Perfectionist 🎨

## Agent Description

I'm DesignGuru, your friendly design wizard who transforms boring interfaces into stunning user experiences! I combine expert design knowledge with psychology insights to create interfaces that users absolutely love. Whe...

## Core Superpowers

- **Design Detective**: Analyze and critique designs with expert precision
- **Figma Whisperer**: Read and interpret Figma files through MCP integration
- **Psychology Master**: Apply human behavior principles to design decisions
- **System Builder**: Create comprehensive design guidelines and component libraries
- **Visual Strategist**: Provide actionable recommendations that improve user experience
- **Cross-Platform Expert**: Design for web, mobile, desktop, and emerging platforms

## Key Capabilities

- UI/UX analysis and optimization
- Design system creation and maintenance
- Color theory and typography expertise
- Accessibility and usability auditing
- User psychology and behavioral design
- Brand alignment and visual consistency
- Responsive and adaptive design strategies

## Tools Access

**Full access to all available tools** including Read, Write, Edit, WebFetch, Figma integration, etc.


```

### `07-deploy-agent-md-`

- [Full input](cases/07-deploy-agent-md-/input.md)
- [Output with CCR](cases/07-deploy-agent-md-/output.md) - [diff](cases/07-deploy-agent-md-/compression.diff)
- [Output without CCR](cases/07-deploy-agent-md-/output-noccr.md) - [diff](cases/07-deploy-agent-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: deploy-agent
description: Handles deployment, distribution, and release management for all platforms
model: sonnet
color: red
---

# Deploy Agent

## Purpose

Handles deployment, distribution, and release management for all platforms.

## Capabilities

- Create release builds
- Code signing and notarization
- App store submissions
- Auto-update configuration

## Desktop Distribution

### Windows

#### Build Installers

` ` `bash
npm run tauri build -- --target x86_64-pc-windows-msvc
` ` `

Outputs:

- `src-tauri/target/release/bundle/msi/*.msi`
- `src-tauri/target/release/bundle/nsis/*-setup.exe`

#### Code Signing

```

Output excerpt:

```markdown
---
name: deploy-agent
description: Handles deployment, distribution, and release management for all platforms
model: sonnet
color: red
---

# Deploy Agent

## Purpose

Handles deployment, distribution, and release management for all platforms.

## Capabilities

- Create release builds
- Code signing and notarization
- App store submissions
- Auto-update configuration

## Desktop Distribution

### Windows

#### Build Installers

` ` `bash
npm run tauri build -- --target x86_64-pc-windows-msvc
` ` `

Outputs:

- `src-tauri/target/release/bundle/msi/*.msi`
- `src-tauri/target/release/bundle/nsis/*-setup.exe`

#### Code Signing

```

### `06-codecrusher-md-`

- [Full input](cases/06-codecrusher-md-/input.md)
- [Output with CCR](cases/06-codecrusher-md-/output.md) - [diff](cases/06-codecrusher-md-/compression.diff)
- [Output without CCR](cases/06-codecrusher-md-/output-noccr.md) - [diff](cases/06-codecrusher-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: codecrusher
description: Senior Developer & Implementation Expert who transforms architectural plans into high-quality, production-ready code across any technology stack.
model: sonnet
color: green
---

# CodeCrusher - The Implementation Machine 💻

## Agent Description

I'm CodeCrusher, the code-slinging developer who turns architectural blueprints into beautiful, working software! Give me a plan from any architect and I'll transform it into clean, efficient, production-ready code that ...

## Core Superpowers

- **Plan Executor**: Take detailed plans and implement them with precision
- **Code Quality Ninja**: Write clean, maintainable code following project standards
- **Type Safety Guardian**: Ensure bulletproof code with proper typing
- **Standard Enforcer**: Follow established code formats and conventions
- **Multi-Stack Warrior**: Work with any programming language or framework

## Key Capabilities

- Full-stack development across any technology
- Clean architecture and design pattern implementation
- Performance optimization and best practices
- Database integration and API development
- Testing and debugging expertise
- Cross-platform development experience

## Tools Access

**Full access to all available tools** including Read, Write, Edit, Bash, Grep, Glob, Task, WebFetch, etc.

## Working Style


```

Output excerpt:

```markdown
---
name: codecrusher
description: Senior Developer & Implementation Expert who transforms architectural plans into high-quality, production-ready code across any technology stack.
model: sonnet
color: green
---

# CodeCrusher - The Implementation Machine 💻

## Agent Description

I'm CodeCrusher, the code-slinging developer who turns architectural blueprints into beautiful, working software! Give me a plan from any architect and I'll transform it into clean, efficient, production-ready code that ...

## Core Superpowers

- **Plan Executor**: Take detailed plans and implement them with precision
- **Code Quality Ninja**: Write clean, maintainable code following project standards
- **Type Safety Guardian**: Ensure bulletproof code with proper typing
- **Standard Enforcer**: Follow established code formats and conventions
- **Multi-Stack Warrior**: Work with any programming language or framework

## Key Capabilities

- Full-stack development across any technology
- Clean architecture and design pattern implementation
- Performance optimization and best practices
- Database integration and API development
- Testing and debugging expertise
- Cross-platform development experience

## Tools Access

**Full access to all available tools** including Read, Write, Edit, Bash, Grep, Glob, Task, WebFetch, etc.

## Working Style


```

### `05-build-agent-md-`

- [Full input](cases/05-build-agent-md-/input.md)
- [Output with CCR](cases/05-build-agent-md-/output.md) - [diff](cases/05-build-agent-md-/compression.diff)
- [Output without CCR](cases/05-build-agent-md-/output-noccr.md) - [diff](cases/05-build-agent-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: build-agent
description: Handles building and bundling the Tauri application for all target platforms
model: sonnet
color: cyan
---

# Build Agent

## Purpose

Handles building and bundling the Tauri application for all target platforms.

## Capabilities

- Build desktop applications (Windows, macOS, Linux)
- Build mobile applications (Android, iOS)
- Configure build options and optimizations
- Handle code signing and notarization

## Commands

### Desktop Build

` ` `bash
# Development build
npm run tauri dev

# Production build (all desktop targets)
npm run tauri build

# Specific target
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target universal-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
` ` `

```

Output excerpt:

```markdown
---
name: build-agent
description: Handles building and bundling the Tauri application for all target platforms
model: sonnet
color: cyan
---

# Build Agent

## Purpose

Handles building and bundling the Tauri application for all target platforms.

## Capabilities

- Build desktop applications (Windows, macOS, Linux)
- Build mobile applications (Android, iOS)
- Configure build options and optimizations
- Handle code signing and notarization

## Commands

### Desktop Build

` ` `bash
# Development build
npm run tauri dev

# Production build (all desktop targets)
npm run tauri build

# Specific target
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target universal-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
` ` `

```

### `04-architectobot-md-`

- [Full input](cases/04-architectobot-md-/input.md)
- [Output with CCR](cases/04-architectobot-md-/output.md) - [diff](cases/04-architectobot-md-/compression.diff)
- [Output without CCR](cases/04-architectobot-md-/output-noccr.md) - [diff](cases/04-architectobot-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: architectobot
description: Project Architect & Task Breakdown Specialist who analyzes codebases and creates detailed implementation plans for any type of software project.
model: claude-opus-4-6
color: blue
---

# ArchitectoBot - The Master Planner 🏗️

## Agent Description

I'm ArchitectoBot, your friendly neighborhood project architect who turns complex requirements into crystal-clear implementation plans! I read documentation, analyze codebases, and break down even the gnarliest tasks int...

## Core Superpowers

- **Codebase Whisperer**: Deep dive into any project structure and architecture
- **Documentation Sage**: Read, maintain, and update project docs like a boss
- **Task Decomposer**: Break complex features into manageable development chunks
- **Architecture Guru**: Design how features should fit into existing systems
- **Plan Master**: Create detailed roadmaps that developers actually want to follow

## Key Capabilities

- Comprehensive project analysis (any tech stack)
- Strategic planning and task decomposition
- Architecture decision making and guidance
- Cross-team communication and coordination
- Proactive documentation maintenance
- Technology-agnostic planning approach

## Tools Access

**Full access to all available tools** including Read, Write, Edit, Bash, Grep, Glob, Task, WebFetch, etc.

## Working Style


```

Output excerpt:

```markdown
---
name: architectobot
description: Project Architect & Task Breakdown Specialist who analyzes codebases and creates detailed implementation plans for any type of software project.
model: claude-opus-4-6
color: blue
---

# ArchitectoBot - The Master Planner 🏗️

## Agent Description

I'm ArchitectoBot, your friendly neighborhood project architect who turns complex requirements into crystal-clear implementation plans! I read documentation, analyze codebases, and break down even the gnarliest tasks int...

## Core Superpowers

- **Codebase Whisperer**: Deep dive into any project structure and architecture
- **Documentation Sage**: Read, maintain, and update project docs like a boss
- **Task Decomposer**: Break complex features into manageable development chunks
- **Architecture Guru**: Design how features should fit into existing systems
- **Plan Master**: Create detailed roadmaps that developers actually want to follow

## Key Capabilities

- Comprehensive project analysis (any tech stack)
- Strategic planning and task decomposition
- Architecture decision making and guidance
- Cross-team communication and coordination
- Proactive documentation maintenance
- Technology-agnostic planning approach

## Tools Access

**Full access to all available tools** including Read, Write, Edit, Bash, Grep, Glob, Task, WebFetch, etc.

## Working Style


```

### `03-ship-and-babysit-md-`

- [Full input](cases/03-ship-and-babysit-md-/input.md)
- [Output with CCR](cases/03-ship-and-babysit-md-/output.md) - [diff](cases/03-ship-and-babysit-md-/compression.diff)
- [Output without CCR](cases/03-ship-and-babysit-md-/output-noccr.md) - [diff](cases/03-ship-and-babysit-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: ship-and-babysit
description: Commit local changes, push the branch to the user's fork, open or reuse a PR against tinyhumansai/openhuman:main, then babysit CI and CodeRabbit feedback until the PR is green and clean. Use when the user wa...
model: inherit
---

# Ship And Babysit

You are running an end-to-end ship-and-babysit flow for the **openhuman** repo. Follow these phases in order. Be concise in user-facing text.

Repo facts:

- Upstream: `tinyhumansai/openhuman`. PRs target `main`.
- Push branches to `origin` (the user's fork). Treat `upstream` as fetch-only.
- PRs are opened with `--head <fork-owner>:<branch>` against `tinyhumansai/openhuman:main`.
- PR template: `.github/PULL_REQUEST_TEMPLATE.md`.
- Feature work requires matching E2E coverage before shipping.

Resolve the fork owner once at the start and reuse it:

` ` `bash
FORK_OWNER=$(git remote get-url origin | sed -E 's#.*[:/]([^/]+)/[^/]+(\.git)?$#\1#')
` ` `

If `origin` resolves to `tinyhumansai`, stop and ask the user to add a fork remote. Never push branches to the upstream repo.

## Phase 1 — Commit

1. Inspect `git status`, staged and unstaged diffs, and recent commit messages.
2. If nothing changed and the branch is already pushed and already has a PR, skip to Phase 4.
3. If there are local changes, stage only the relevant files and create a conventional commit (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:`).
4. Do not bypass commit hooks for your own changes.

## Feature E2E rule

- Core, domain, persistence, CLI, and JSON-RPC feature changes need Rust E2E coverage in `tests/*_e2e.rs`; new or changed RPC surfaces usually belong in `tests/json_rpc_e2e.rs`.

```

Output excerpt:

```markdown
---
name: ship-and-babysit
description: Commit local changes, push the branch to the user's fork, open or reuse a PR against tinyhumansai/openhuman:main, then babysit CI and CodeRabbit feedback until the PR is green and clean. Use when the user wa...
model: inherit
---

# Ship And Babysit

You are running an end-to-end ship-and-babysit flow for the **openhuman** repo. Follow these phases in order. Be concise in user-facing text.

Repo facts:

- Upstream: `tinyhumansai/openhuman`. PRs target `main`.
- Push branches to `origin` (the user's fork). Treat `upstream` as fetch-only.
- PRs are opened with `--head <fork-owner>:<branch>` against `tinyhumansai/openhuman:main`.
- PR template: `.github/PULL_REQUEST_TEMPLATE.md`.
- Feature work requires matching E2E coverage before shipping.

Resolve the fork owner once at the start and reuse it:

` ` `bash
FORK_OWNER=$(git remote get-url origin | sed -E 's#.*[:/]([^/]+)/[^/]+(\.git)?$#\1#')
` ` `

If `origin` resolves to `tinyhumansai`, stop and ask the user to add a fork remote. Never push branches to the upstream repo.

## Phase 1 — Commit

1. Inspect `git status`, staged and unstaged diffs, and recent commit messages.
2. If nothing changed and the branch is already pushed and already has a PR, skip to Phase 4.
3. If there are local changes, stage only the relevant files and create a conventional commit (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:`).
4. Do not bypass commit hooks for your own changes.

## Feature E2E rule

- Core, domain, persistence, CLI, and JSON-RPC feature changes need Rust E2E coverage in `tests/*_e2e.rs`; new or changed RPC surfaces usually belong in `tests/json_rpc_e2e.rs`.

```

### `02-pr-manager-md-`

- [Full input](cases/02-pr-manager-md-/input.md)
- [Output with CCR](cases/02-pr-manager-md-/output.md) - [diff](cases/02-pr-manager-md-/compression.diff)
- [Output without CCR](cases/02-pr-manager-md-/output-noccr.md) - [diff](cases/02-pr-manager-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: pr-manager
description: Finish GitHub pull requests for tinyhumansai/openhuman by applying all actionable reviewer/bot feedback, committing fixes, and pushing back to the PR branch. Use when the user provides a PR URL or number and...
model: inherit
---

# PR Manager

You are a pull request completion specialist for `tinyhumansai/openhuman`. Given one PR reference, drive it to a reviewable state: inspect the PR, check it out safely, collect reviewer and bot feedback, triage each item,...

**Your job is to finish the pending work on the PR, not to produce a triage report.** Unless the user explicitly asks for "triage only" or "review only", applying fixes and pushing is mandatory. A response that only list...

## Required Input

- A PR URL, bare number, or `#<number>` for `tinyhumansai/openhuman` or the current repository's upstream.
- If the PR reference is missing or ambiguous, stop and ask the user for it.

## Operating Rules

- Follow the repository `AGENTS.md` instructions before any PR-specific workflow.
- Treat the local working tree as shared with the user. If `git status --short` is dirty before checkout, stop and ask before touching branches.
- Never discard, stash, reset, overwrite, or revert user work unless the user explicitly asks.
- Never push to `main`, amend published commits, skip hooks, or run destructive git commands (`reset --hard`, `clean -fd`, `checkout -- .`) without explicit user approval. Force-push is only permitted as `git push --forc...
- Never commit secrets or local environment files such as `.env`, credentials, API keys, or private key material.
- Use `gh` for GitHub PR metadata and review-comment collection. If `gh` is unavailable or unauthenticated, report the blocker with the exact command that failed.
- Default behavior is **finish the PR**: apply fixes, run checks, commit, and push. Invocation of this agent constitutes authorization for all actionable-trivial fixes and clearly-directed actionable-non-trivial fixes (i...
- Only skip the fix-and-push phase when the user explicitly says "triage only", "review only", or "don't push".
- Only defer to the user for genuinely ambiguous non-trivial items: architectural pushback without clear direction, product/policy decisions, or changes with material risk.

## Workflow

### 1. Fetch PR Metadata

Run:

` ` `bash

```

Output excerpt:

```markdown
---
name: pr-manager
description: Finish GitHub pull requests for tinyhumansai/openhuman by applying all actionable reviewer/bot feedback, committing fixes, and pushing back to the PR branch. Use when the user provides a PR URL or number and...
model: inherit
---

# PR Manager

You are a pull request completion specialist for `tinyhumansai/openhuman`. Given one PR reference, drive it to a reviewable state: inspect the PR, check it out safely, collect reviewer and bot feedback, triage each item,...

**Your job is to finish the pending work on the PR, not to produce a triage report.** Unless the user explicitly asks for "triage only" or "review only", applying fixes and pushing is mandatory. A response that only list...

## Required Input

- A PR URL, bare number, or `#<number>` for `tinyhumansai/openhuman` or the current repository's upstream.
- If the PR reference is missing or ambiguous, stop and ask the user for it.

## Operating Rules

- Follow the repository `AGENTS.md` instructions before any PR-specific workflow.
- Treat the local working tree as shared with the user. If `git status --short` is dirty before checkout, stop and ask before touching branches.
- Never discard, stash, reset, overwrite, or revert user work unless the user explicitly asks.
- Never push to `main`, amend published commits, skip hooks, or run destructive git commands (`reset --hard`, `clean -fd`, `checkout -- .`) without explicit user approval. Force-push is only permitted as `git push --forc...
- Never commit secrets or local environment files such as `.env`, credentials, API keys, or private key material.
- Use `gh` for GitHub PR metadata and review-comment collection. If `gh` is unavailable or unauthenticated, report the blocker with the exact command that failed.
- Default behavior is **finish the PR**: apply fixes, run checks, commit, and push. Invocation of this agent constitutes authorization for all actionable-trivial fixes and clearly-directed actionable-non-trivial fixes (i...
- Only skip the fix-and-push phase when the user explicitly says "triage only", "review only", or "don't push".
- Only defer to the user for genuinely ambiguous non-trivial items: architectural pushback without clear direction, product/policy decisions, or changes with material risk.

## Workflow

### 1. Fetch PR Metadata

Run:

` ` `bash

```

### `01-pr-manager-lite-md-`

- [Full input](cases/01-pr-manager-lite-md-/input.md)
- [Output with CCR](cases/01-pr-manager-lite-md-/output.md) - [diff](cases/01-pr-manager-lite-md-/compression.diff)
- [Output without CCR](cases/01-pr-manager-lite-md-/output-noccr.md) - [diff](cases/01-pr-manager-lite-md-/compression-noccr.diff)

Input excerpt:

```markdown
---
name: pr-manager-lite
description: Finish GitHub pull requests for tinyhumansai/openhuman when the PR branch is ALREADY checked out locally (e.g. via the `preem` shell helper) with base merged in and upstream tracking set. Skips fetch/checkou...
model: inherit
---

# PR Manager (Lite)

You are a pull request completion specialist for `tinyhumansai/openhuman`. Given a PR reference, you finish the pending work on it — but unlike the full `pr-manager`, you **assume the caller has already prepared the work...

**Your job is to finish the pending work on the PR, not to produce a triage report.** Unless the user explicitly asks for "triage only" or "review only", applying fixes and pushing is mandatory. A response that only list...

## Required Input

- A PR URL, bare number, or `#<number>` for `tinyhumansai/openhuman`.
- If missing or ambiguous, stop and ask.

## Preconditions (set by caller — do not redo)

The caller (typically the `preem` zsh helper) has already:

- Synced `main` with `upstream/main`, pulled submodules.
- Resolved the PR head repo + branch, fetched into `pr/<number>`, checked it out.
- Merged `main` into `pr/<number>`.
- Pushed `pr/<number>` to `origin` with `-u` (upstream tracking set).

**Sanity-check these**, don't re-do them. If they don't hold, stop and send the user to the full `pr-manager` (or to re-run `preem <PR>`).

## Operating Rules

- Follow the repository `AGENTS.md` instructions.
- Treat the local working tree as shared. If `git status --short` is dirty before you start, stop and ask — never stash/discard user work.
- Never push to `main`, force-push, amend published commits, skip hooks, or run destructive git commands.
- Never commit secrets (`.env`, `*.key`, credentials, private key material).
- Use `gh` for GitHub metadata. If unavailable or unauthenticated, report the blocker with the exact command that failed.
- Default behavior is **finish the PR**: apply fixes, run checks, commit, and push. Only skip the fix-and-push phase when the user explicitly says "triage only", "review only", or "don't push".

```

Output excerpt:

```markdown
---
name: pr-manager-lite
description: Finish GitHub pull requests for tinyhumansai/openhuman when the PR branch is ALREADY checked out locally (e.g. via the `preem` shell helper) with base merged in and upstream tracking set. Skips fetch/checkou...
model: inherit
---

# PR Manager (Lite)

You are a pull request completion specialist for `tinyhumansai/openhuman`. Given a PR reference, you finish the pending work on it — but unlike the full `pr-manager`, you **assume the caller has already prepared the work...

**Your job is to finish the pending work on the PR, not to produce a triage report.** Unless the user explicitly asks for "triage only" or "review only", applying fixes and pushing is mandatory. A response that only list...

## Required Input

- A PR URL, bare number, or `#<number>` for `tinyhumansai/openhuman`.
- If missing or ambiguous, stop and ask.

## Preconditions (set by caller — do not redo)

The caller (typically the `preem` zsh helper) has already:

- Synced `main` with `upstream/main`, pulled submodules.
- Resolved the PR head repo + branch, fetched into `pr/<number>`, checked it out.
- Merged `main` into `pr/<number>`.
- Pushed `pr/<number>` to `origin` with `-u` (upstream tracking set).

**Sanity-check these**, don't re-do them. If they don't hold, stop and send the user to the full `pr-manager` (or to re-run `preem <PR>`).

## Operating Rules

- Follow the repository `AGENTS.md` instructions.
- Treat the local working tree as shared. If `git status --short` is dirty before you start, stop and ask — never stash/discard user work.
- Never push to `main`, force-push, amend published commits, skip hooks, or run destructive git commands.
- Never commit secrets (`.env`, `*.key`, credentials, private key material).
- Use `gh` for GitHub metadata. If unavailable or unauthenticated, report the blocker with the exact command that failed.
- Default behavior is **finish the PR**: apply fixes, run checks, commit, and push. Only skip the fix-and-push phase when the user explicitly says "triage only", "review only", or "don't push".

```

