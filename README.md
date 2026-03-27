# bmo-agent-setup

A CLI tool to configure a Claude Code environment with [BMO](https://github.com/erewok/bmo).

---

## Attribution

This repository includes code and Markdown content adapted from
[ALT-F4-LLC/dotfiles.vorpal](https://github.com/ALT-F4-LLC/dotfiles.vorpal).

- Original project: `https://github.com/ALT-F4-LLC/dotfiles.vorpal`
- Upstream license: Apache License 2.0
- Attribution details for this repository: see [NOTICE](NOTICE)

Substantial credit for the agent, skill, and workflow design belongs to the
`dotfiles.vorpal` maintainers and contributors. This repository contains
modifications and simplifications focused on a standalone Claude Code setup tool,
and is not an official ALT-F4-LLC project.

The code in this repository was written by [Claude Code](https://docs.anthropic.com/en/docs/claude-code).

---

## Overview

`bmo-agent-setup` generates a directory containing everything you need to run a [BMO](https://github.com/erewok/bmo)-powered
Claude Code agent team:

- **agents/** — Five agent definition files (Markdown prompts consumed by Claude Code)
- **skills/** — Two orchestration skills (`dev-team` and `dev-init`)
- **settings.json** — Claude Code configuration
- **statusline.sh** — Status bar script for Claude Code

After the tool runs it prints step-by-step instructions showing where to place each generated
file so that Claude Code picks them up automatically.

Future support for GitHub Copilot CLI as an alternative target is planned.

The agent team is configured to use **[`bmo`](https://github.com/erewok/bmo)** for issue tracking.

---

## Agent Team

### Staff Engineer (`@staff-engineer`)

Technical architect and code reviewer. Produces Technical Design Documents (TDDs) in `docs/tdd/` for complex work, maintains project specifications in `docs/spec/`, and reviews all `@senior-engineer` implementation changes before they are considered complete. Never writes implementation code — outputs are design documents and review feedback only.

### Project Manager (`@project-manager`)

Plans and decomposes work into BMO issues. Explores the codebase, creates issue hierarchies with dependencies, attaches affected files to issues for collision detection, and runs `bmo plan` to verify the computed execution phase structure. The only agent that creates BMO issues. Never writes code.

### Senior Engineer (`@senior-engineer`)

Implements solutions from pre-planned BMO issues. Claims issues atomically with `bmo issue claim`, implements within the scoped files, then moves issues to `review` status when done. Does **not** close issues (closing requires `@staff-engineer` sign-off) and does **not** commit code unless explicitly instructed after review passes.

### QA Engineer (`@qa-engineer`)

Verifies implementation against acceptance criteria after the review phase. Writes and runs tests, checks for regressions, and reports results and defects as BMO comments. Does not claim or close issues — communicates via comments only.

### UX Designer (`@ux-designer`)

Produces UX design specs in `docs/ux/` for user-facing work: UI, CLI commands, API ergonomics, error messages, config formats, and onboarding flows. Designs interaction flows and acceptance criteria before technical planning begins. Never writes implementation code.

---

## Skills

| Skill | Description |
|---|---|
| **dev-team** | Coordinates all five agents for planning and executing development work |
| **dev-init** | Bootstraps `docs/spec/` project specifications for new repositories |

---

## BMO Integration

The agent team uses the [`bmo`](https://github.com/erewok/bmo) CLI for issue tracking. Each agent references `bmo` commands in its system prompt. When you invoke the `dev-team` skill or any individual agent, they manage work through `bmo`.

### Issue Lifecycle

Issues flow through a defined lifecycle enforced by the agent roles:

```
todo → in-progress (senior-engineer claims) → review (senior-engineer when done) → done (orchestrator after staff-engineer sign-off)
```

### Who Does What in BMO

| Agent | claim | move to review | close | comment |
|---|---|---|---|---|
| **senior-engineer** | ✅ only agent that claims | ✅ when done | ❌ | ✅ |
| **qa-engineer** | ❌ | ❌ | ❌ | ✅ only |
| **staff-engineer** | ❌ | ❌ | ❌ | ✅ |
| **project-manager** | ❌ | ❌ | ❌ | ✅ |
| **orchestrator** (dev-team) | ❌ | ❌ | ✅ after review | ❌ |

---

## Usage

### Prerequisites

- Rust toolchain (stable, 1.70+)
- macOS or Linux

### Quick Start

**Basic setup (no customization):**
```bash
cargo run --release
```

**With statusline:**
```bash
cargo run --release -- --with-statusline=true
```

**Using a configuration file (recommended for complex setups):**
```bash
cargo run --release -- --config bmo-config.toml
```

See [CONFIG.md](CONFIG.md) for detailed configuration documentation.

### Build and Run

```bash
# Build
cargo build --release

# Run with defaults
cargo run --release

# Run with custom output directory
cargo run --release -- --output ~/my-claude-env

# Run with configuration file
cargo run --release -- --config bmo-config.toml

# Run with CLI overrides
cargo run --release -- --config bmo-config.toml --with-thinking=false

# Or with justfile
just run
just run ~/my-claude-env
```

### CLI Options

| Flag | Description | Default |
|---|---|---|
| `-o`, `--output <PATH>` | Output directory for the generated environment | `./claude-code-env` |
| `-c`, `--config <FILE>` | Path to TOML configuration file | None |
| `--with-statusline <BOOL>` | Enable/disable statusline (overrides config) | `false` (or from config) |
| `--with-thinking <BOOL>` | Enable/disable always-thinking mode (overrides config) | `true` (or from config) |
| `-h`, `--help` | Print help | |
| `-V`, `--version` | Print version | |

### Configuration Files

Three example configurations are included:

- **`bmo-config.toml`** - Full configuration with all options documented
- **`bmo-config.minimal.toml`** - Minimal config with just the essentials
- **`bmo-config.secure.toml`** - Maximum security for sensitive environments

For complete configuration documentation, see [CONFIG.md](CONFIG.md).

---

## Installing the Output

After the tool runs, copy the generated files into `~/.claude/`:

```bash
# Agent definitions
cp -r ~/my-claude-env/agents ~/.claude/

# Skills
cp -r ~/my-claude-env/skills ~/.claude/

# Settings
cp ~/my-claude-env/claude.settings.json ~/.claude/settings.json

# Statusline script
cp ~/my-claude-env/statusline.sh ~/.claude/statusline.sh
```

Or install everything at once:

```bash
cp -r ~/my-claude-env/{agents,skills} ~/.claude/ && \
  cp ~/my-claude-env/claude.settings.json ~/.claude/settings.json && \
  cp ~/my-claude-env/statusline.sh ~/.claude/statusline.sh
```

### Expected layout after install

```
~/.claude/
├── agents/
│   ├── distributed-systems-engineer.md
│   ├── project-manager.md
│   ├── qa-engineer.md
│   ├── senior-engineer.md
│   ├── staff-engineer.md
│   └── ux-designer.md
├── skills/
│   ├── dev-init/
│   │   └── SKILL.md
│   └── dev-team/
│       └── SKILL.md
├── settings.json
└── statusline.sh
```

---

## Development

```bash
just build       # debug build
just test        # run tests
just lint        # clippy
just fmt         # format
just ci          # full CI pipeline locally
```

---

## Contributing

Contributor: Erik Aker <eraker@gmail.com>
GitHub: https://github.com/erewok/bmo-agent-setup

---

## License

This project is licensed under the [Apache License 2.0](LICENSE).

Additional third-party attribution and adaptation notes are documented in
[NOTICE](NOTICE).
