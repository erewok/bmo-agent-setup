# Pi-Code Integration

This guide covers installing the bmo-dev-team pi extension, what it provides,
and how to use the dev-team workflow in pi-code.

## What You Get

Installing the extension gives you:

| Feature | Description |
|---------|-------------|
| **`bmo_*` tools** | Every bmo command available as a typed tool the LLM calls directly — no shell strings |
| **`subagent` tool** | Spawn any dev-team agent in an isolated pi subprocess with a single tool call |
| **`dev-team` skill** | `/skill:dev-team` — full orchestration: design → plan → implement → review → QA |
| **`documentation-driver` skill** | `/skill:documentation-driver` — generate all five `docs/spec/` files in parallel |
| **8 specialist agents** | `staff-engineer`, `project-manager`, `senior-engineer`, `qa-engineer`, `ux-designer`, `code-quality`, `documentation-writer`, `distributed-systems-expert` |
| **Board context injection** | Current board state silently injected into every agent turn — the LLM always knows what's in progress |
| **Status widget** | Live board counts (`○3 ●2 ◐1 ◎0 ✔5`) above the pi editor |
| **Slash commands** | `/bmo-board`, `/bmo-ls`, `/bmo-show`, `/bmo-next`, `/bmo-plan`, `/bmo-stats`, `/bmo-init`, `/bmo-widget` |

The `bmo_*` tools and widget come from the `pi-bmo` extension, which is bundled
as a dependency — you don't need to install it separately.

---

## Installation

### From git (recommended)

```bash
pi install git:github.com/erewok/bmo-agent-setup
```

pi clones the repo, runs `npm install` (which pulls in `pi-bmo`), and loads
both extension entry points on next start.

### Local development

If you have both repos checked out, add the extension paths directly to
`~/.pi/agent/settings.json`:

```json
{
  "extensions": [
    "/path/to/bmo/pi-extension",
    "/path/to/bmo-agent-setup/pi-extension"
  ]
}
```

Then type `/reload` in your running pi session to pick up the changes.

### Upgrading

```bash
pi update git:github.com/erewok/bmo-agent-setup
```

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

Any skill directories that contain no `SKILL.md` are inert and can be left or
removed at your preference.

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

## Using bmo Tools Directly

The LLM can call `bmo_*` tools at any time. You can also ask for bmo operations
in plain language:

> "Show me what's in progress"
> "Create a bmo issue for the login bug"
> "What's next on the board?"

Or use the slash commands for a quick read-only view:

| Command | Output |
|---------|--------|
| `/bmo-board` | Full kanban board |
| `/bmo-ls [status]` | List issues, optionally filtered by status |
| `/bmo-show <id>` | Show an issue with comments and relations |
| `/bmo-next` | Next work-ready issues |
| `/bmo-plan` | Phased execution plan |
| `/bmo-stats` | Issue counts by status and priority |
| `/bmo-init` | Initialize bmo in this project |
| `/bmo-widget` | Toggle the board status widget on or off |

---

## Agent Reference

### `staff-engineer`

Produces TDDs in `docs/tdd/`, reviews all implementation changes, and maintains
project specs in `docs/spec/`. The only agent that can sign off on closing an issue.
Never writes implementation code.

**Best used for:** "Write a TDD for X", "Review the changes in Y", "Generate docs/spec/"

### `project-manager`

Decomposes requests into bmo issues with dependency graphs, phases, and file scoping.
The only agent that creates bmo issues. Never writes code.

**Best used for:** "Plan this feature", "Break this work into issues"

### `senior-engineer`

Implements from a bmo issue within the scoped files. Claims the issue before starting,
moves to review when done, never closes or self-reviews.

**Best used for:** "Implement BMO-7", "Fix the bug in BMO-3"

### `qa-engineer`

Verifies acceptance criteria after implementation and review. Writes tests, runs the
suite, posts results as bmo comments. Never claims or closes issues.

**Best used for:** "Verify BMO-7 is complete", "Check BMO-3 acceptance criteria"

### `ux-designer`

Produces UX design specs in `docs/ux/` for user-facing work. Always runs before the
`staff-engineer` TDD in UX-Heavy workflows.

**Best used for:** "Design the onboarding flow", "Spec the new settings UI"

### `code-quality`

Focused code quality pass: naming, complexity, dead code, test mechanics, conventions.
Runs on Haiku for speed. Complements `staff-engineer` review rather than replacing it.

**Best used for:** A fast quality pass before the full staff-engineer review

### `documentation-writer`

Writes READMEs, API docs, guides, and changelogs. Documents what actually exists, not
what you wish existed.

**Best used for:** "Write a README for this module", "Document the public API"

### `distributed-systems-expert`

Reviews distributed systems correctness: safety/liveness properties, consistency models,
fault tolerance, and TLA+ formal verification. Triages itself — stands down immediately
if the work has no distributed systems dimension.

**Best used for:** Anything touching consensus, replication, shared mutable state across
nodes, CRDTs, or ordering constraints

---

## Documentation Driver

Generate all five `docs/spec/` files in parallel for an existing codebase:

```
/skill:documentation-driver
```

Or:

> "Document this project" / "Initialize the specs" / "Generate docs/spec/"

The skill checks for existing files first and asks whether to overwrite or skip them.
It then spawns five `staff-engineer` agents in parallel, one per spec file:

| File | Content |
|------|---------|
| `docs/spec/architecture.md` | Project structure, components, design patterns |
| `docs/spec/external-contracts.md` | APIs, data contracts, integration points |
| `docs/spec/security.md` | Auth, secrets, trust boundaries |
| `docs/spec/code-quality.md` | Linting, conventions, patterns in use |
| `docs/spec/testing.md` | Test pyramid, coverage tools, test patterns |

Each file documents what the codebase **actually contains**, not aspirational goals.

---

## Troubleshooting

**Skills not showing up after install**

Run `/reload`. If they still don't appear, check that conflicting copies in
`~/.pi/agent/skills/` have been removed (see [Removing Conflicting Skills](#removing-conflicting-skills)).

**`bmo_*` tools not available**

Check that `bmo` is on your `$PATH`:

```bash
which bmo
```

If it's missing, install it: `cargo install bmo`.

**"No bmo project found" on `/bmo-board`**

Run `/bmo-init` to initialize a bmo project in the current directory, or `cd` into
a directory that already has a `.bmo/` folder.

**Agent spawning fails with "Unknown agent"**

The `subagent` tool discovers agents from the extension's `agents/` directory,
`~/.pi/agent/agents/`, and `.pi/agents/` in the project. Run `/reload` after
any changes to agent files.

**Context window fills up during large orchestrations**

Each `senior-engineer` runs in its own isolated subprocess with a fresh context
window. The orchestrating session sees only summaries. For very large projects,
consider breaking the work into explicit phases and running one phase at a time.
