# Pi-Code Integration

This guide covers installing the pi-bmo-agents extension, what it provides,
and how to use the dev-team workflow in pi-code.

## What You Get

Installing the extension gives you:

| Feature | Description |
|---------|-------------|
| **`subagent` tool** | Spawn any dev-team agent in an isolated pi subprocess with a single tool call |
| **`dev-team` skill** | `/skill:dev-team` — full orchestration: design → plan → implement → review → QA |
| **`documentation-driver` skill** | `/skill:documentation-driver` — generate all five `docs/spec/` files in parallel |
| **8 specialist agents** | `staff-engineer`, `project-manager`, `senior-engineer`, `qa-engineer`, `ux-designer`, `code-quality`, `documentation-writer`, `distributed-systems-expert` |

The `bmo_*` tools, board context injection, status widget, and `/bmo-*` slash commands
come from the [pi-bmo](https://github.com/erewok/pi-bmo) extension, **which must be
installed separately**.

---

## Installation

```bash
pi install git:github.com/erewok/pi-bmo
pi install git:github.com/erewok/bmo-agent-setup
```

Both are required. `pi-bmo` provides the `bmo_*` tools that the dev-team agents
depend on; `pi-bmo-agents` provides the `subagent` tool and orchestration skills.

### Keeping up to date

```bash
pi update                    # updates pi + all unpinned packages
pi update --extensions       # packages only, skip pi itself
```

### Local development

Add both repo roots to `~/.pi/agent/settings.json`:

```json
{
  "extensions": [
    "/path/to/pi-bmo",
    "/path/to/bmo-agent-setup"
  ]
}
```

Then type `/reload` in your running pi session to pick up changes.

---

## Prerequisites

- [`bmo`](https://github.com/erewok/bmo) installed and on `$PATH`
- [pi-code](https://pi.dev) installed

---

## Removing Conflicting Skills

If you previously copied skills from this repo into `~/.pi/agent/skills/` manually,
remove any that conflict before reloading — pi keeps the first copy found and the
global directory is checked before extension-contributed paths:

```bash
rm -rf ~/.pi/agent/skills/dev-team
rm -rf ~/.pi/agent/skills/documentation-driver
```

---

## Using the Dev-Team Skill

Trigger the dev-team with natural language or the explicit skill command:

```
/skill:dev-team add Redis caching to the session store
```

Or just describe the work — the skill's trigger phrases are broad:

> "Use the agent team to add rate limiting to the API"
> "Plan and execute the auth refactor"
> "Have the team work on migrating the database schema"

### Orchestration Patterns

The skill picks a pattern automatically based on scope:

| Pattern | When | Agent sequence |
|---------|------|----------------|
| **Small** | Bug fix, config change, small feature | PM → SE → Staff review |
| **Medium** | Feature with architecture decisions | Staff (TDD) → PM → SE → Staff review → QA |
| **Large** | Multiple phases, user approval between each | Same as Medium, one phase at a time |
| **UX-Heavy** | Any user-facing work needing design first | UX → Staff (TDD) → PM → SE → Staff review → QA |

### What the skill does

1. **Initializes bmo** — runs `bmo_agent_init()` to see current board state
2. **Designs** (Medium+) — spawns `staff-engineer` to write a TDD in `docs/tdd/`
3. **Plans** — spawns `project-manager` to decompose work into bmo issues with phases,
   dependencies, and file attachments
4. **Presents the plan** — shows you the phase breakdown and waits for approval on
   non-trivial work
5. **Implements in parallel** — spawns `senior-engineer` agents in parallel within each
   phase, each claiming their issue before starting
6. **Reviews** — spawns `staff-engineer` to review all changes; posts findings as bmo
   comments; any blockers route back to the responsible engineer
7. **Verifies** (Medium+) — spawns `qa-engineer` to check acceptance criteria
8. **Reports** — summarizes what was done and reminds you nothing has been committed

### What to expect

- The skill never commits. When it finishes, run `git diff` to review everything.
- Each `senior-engineer` agent runs in its own isolated pi subprocess with its own
  context window. They work in parallel without seeing each other's output.
- `staff-engineer` review comments land directly on the bmo issue. The skill reads them
  and routes blockers back automatically.
- You'll see live streaming output from each agent as it runs.

---

## Using Individual Agents

You can spawn any agent directly with the `subagent` tool:

```
Use the staff-engineer agent to review my changes to src/auth/
```

```
Use the project-manager to plan this work: [describe what you want]
```

```
Use the distributed-systems-expert to review the TDD in docs/tdd/replication.md
```

Or using the tool directly:

```
subagent(agent="staff-engineer", task="Review all changes in git diff and post findings")
```

### Parallel spawning

```
subagent(tasks=[
  { agent: "senior-engineer", task: "Implement BMO-4: add retry logic to the HTTP client" },
  { agent: "senior-engineer", task: "Implement BMO-5: add connection pooling" }
])
```

### Chained workflows

```
subagent(chain=[
  { agent: "staff-engineer", task: "Explore the codebase and write a TDD for adding OAuth support. Save to docs/tdd/oauth.md." },
  { agent: "project-manager", task: "Decompose the TDD at docs/tdd/oauth.md into bmo issues. Prior output: {previous}" }
])
```

---

## Agent Reference

### `staff-engineer`

Produces TDDs in `docs/tdd/`, reviews all implementation changes, and maintains
project specs in `docs/spec/`. The only agent that can sign off on closing an issue.
Never writes implementation code.

### `project-manager`

Decomposes requests into bmo issues with dependency graphs, phases, and file scoping.
The only agent that creates bmo issues. Never writes code.

### `senior-engineer`

Implements from a bmo issue within the scoped files. Claims the issue before starting,
moves to review when done, never closes or self-reviews.

### `qa-engineer`

Verifies acceptance criteria after implementation and review. Writes tests, runs the
suite, posts results as bmo comments. Never claims or closes issues.

### `ux-designer`

Produces UX design specs in `docs/ux/` for user-facing work. Always runs before the
`staff-engineer` TDD in UX-Heavy workflows.

### `code-quality`

Focused code quality pass: naming, complexity, dead code, test mechanics, conventions.
Runs on Haiku for speed. Complements `staff-engineer` review rather than replacing it.

### `documentation-writer`

Writes READMEs, API docs, guides, and changelogs. Documents what actually exists.

### `distributed-systems-expert`

Reviews distributed systems correctness: safety/liveness properties, consistency models,
fault tolerance, and TLA+ formal verification. Stands down if work has no distributed
systems dimension.

---

## Documentation Driver

Generate all five `docs/spec/` files in parallel for an existing codebase:

```
/skill:documentation-driver
```

The skill checks for existing files first and asks whether to overwrite or skip them.
It then spawns five `staff-engineer` agents in parallel, one per spec file:

| File | Content |
|------|---------|
| `docs/spec/architecture.md` | Project structure, components, design patterns |
| `docs/spec/external-contracts.md` | APIs, data contracts, integration points |
| `docs/spec/security.md` | Auth, secrets, trust boundaries |
| `docs/spec/code-quality.md` | Linting, conventions, patterns in use |
| `docs/spec/testing.md` | Test pyramid, coverage tools, test patterns |

---

## Troubleshooting

**Skills not showing up after install**

Run `/reload`. If they still don't appear, check that conflicting copies in
`~/.pi/agent/skills/` have been removed.

**`bmo_*` tools not available**

Make sure `pi-bmo` is installed: `pi install git:github.com/erewok/pi-bmo`.
Also check that `bmo` is on your `$PATH`: `which bmo`.

**"No bmo project found" on `/bmo-board`**

Run `/bmo-init` to initialize a bmo project in the current directory.

**Agent spawning fails with "Unknown agent"**

The `subagent` tool discovers agents from the extension's `agents/` directory,
`~/.pi/agent/agents/`, and `.pi/agents/` in the project. Run `/reload` after
any changes to agent files.
