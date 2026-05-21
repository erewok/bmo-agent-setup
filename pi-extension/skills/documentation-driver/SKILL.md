---
name: documentation-driver
description: >
  Bootstrap project specification files in docs/spec/ by spawning staff-engineer
  agents in parallel. Use when the user wants to initialize, generate, or bootstrap
  project specs — phrases like "document the project", "initialize specs",
  "generate specs", "create project specifications", "bootstrap docs/spec",
  "populate specs", or "set up project documentation".
---

# Documentation Driver

You spawn `staff-engineer` agents in parallel using the `subagent` tool to populate
`docs/spec/` with the Five Spec Files. You coordinate and verify; you never write
spec files yourself.

> **CRITICAL: Do NOT commit ANY changes (no `git add`, `git commit`, `git push`) unless EXPLICITLY instructed. This applies to all spawned agents.**

---

## Pre-flight

Check for existing spec files before spawning:

```
ls docs/spec/   (via bash or Read)
```

- **Files exist** → ask the user: Overwrite all / Skip existing / Cancel
- **No files** → proceed to execution

---

## Execution

Spawn all agents **in a single parallel `subagent` call**. Parallelism is the entire point.

```
subagent(
  agentScope="all",
  tasks=[
    { agent: "staff-engineer", task: "<architecture prompt>" },
    { agent: "staff-engineer", task: "<external-contracts prompt>" },
    { agent: "staff-engineer", task: "<security prompt>" },
    { agent: "staff-engineer", task: "<code-quality prompt>" },
    { agent: "staff-engineer", task: "<testing prompt>" },
  ]
)
```

After completion, run `ls docs/spec/` to confirm all files exist and report results.

---

## Agent Task Templates

### architecture.md

```
Generate docs/spec/architecture.md for this project.

- Explore the codebase with Read and Bash
- Examine project structure, entry points, module boundaries, and the dependency graph
- Identify system components, design patterns, integration points, and key architectural decisions
- Look at package manifests, config files, and directory layout
- Document what ACTUALLY exists — not aspirational goals. Be honest about gaps.
- Save to docs/spec/architecture.md (create docs/spec/ if needed)
- Do NOT write implementation code. Do NOT commit.
```

### external-contracts.md

```
Generate docs/spec/external-contracts.md for this project.

- Explore the codebase with Read and Bash
- Identify all external interfaces, APIs, data contracts, and integration points
- Look for API client code, HTTP request patterns, serialization formats, and schema definitions
- Check for third-party services, message queues, event handlers, and inter-service communication (both inbound and outbound)
- Check config files and environment variables that specify external dependencies
- Document what ACTUALLY exists — not aspirational goals. Be honest about gaps.
- Save to docs/spec/external-contracts.md (create docs/spec/ if needed)
- Do NOT write implementation code. Do NOT commit.
```

### security.md

```
Generate docs/spec/security.md for this project.

- Explore the codebase with Read and Bash
- Examine authentication/authorization patterns, secret management, and environment variables
- Check .env files, credential handling, API key patterns, and trust boundaries
- Identify security-relevant dependencies and their configurations
- Document what ACTUALLY exists — not aspirational goals. Be honest about gaps.
- Save to docs/spec/security.md (create docs/spec/ if needed)
- Do NOT write implementation code. Do NOT commit.
```

### code-quality.md

```
Generate docs/spec/code-quality.md for this project.

- Explore the codebase with Read and Bash
- Check for linter configs (eslint, clippy, ruff, etc.), formatters, and editor settings
- Identify naming conventions, error handling patterns, and design patterns in use
- Look at existing code style, module organization, and project-specific conventions
- Document what ACTUALLY exists — not aspirational goals. Be honest about gaps.
- Save to docs/spec/code-quality.md (create docs/spec/ if needed)
- Do NOT write implementation code. Do NOT commit.
```

### testing.md

```
Generate docs/spec/testing.md for this project.

- Explore the codebase with Read and Bash
- Check for test directories, runners, configs, and CI test steps
- Identify the test pyramid breakdown: unit, integration, e2e, and their proportions
- Look at coverage tools, test utilities, fixtures, and mocking patterns
- Document what ACTUALLY exists — not aspirational goals. Be honest about gaps.
- Save to docs/spec/testing.md (create docs/spec/ if needed)
- Do NOT write implementation code. Do NOT commit.
```

---

## Rules

1. **Spawn all agents in a single `subagent` call** — parallelism is the point.
2. **Never write spec files yourself** — you are the orchestrator, not the author.
3. **Never commit** — no `git add`, `git commit`, `git push`.
4. **No bmo** — this skill does not use bmo for issue tracking.
5. **No cross-agent dependencies** — all five specs are independent.
6. **Respect the user's choice** on existing files (overwrite / skip / cancel).
7. **Fail loud** — if an agent fails, report it immediately with details.
