# bmo-agent-setup

A CLI tool to configure a Claude Code environment with BMO.

---

## Attribution

> **All credit for the agents, skills, and core ideas belongs to
> [ALT-F4-LLC/dotfiles.vorpal](https://github.com/ALT-F4-LLC/dotfiles.vorpal).**

The code in this repository was written by [Claude Code](https://docs.anthropic.com/en/docs/claude-code).

The repo owner adapted concepts from `dotfiles.vorpal` to create this simplified setup tool focusing purely on a claude-code environment. For the origin of ideas in here, visit [ALT-F4-LLC/dotfiles.vorpal](https://github.com/ALT-F4-LLC/dotfiles.vorpal).

---

## Overview

`bmo-agent-setup` generates a directory containing everything you need to run a BMO-powered
Claude Code agent team:

- **agents/** — Five agent definition files (Markdown prompts consumed by Claude Code)
- **skills/** — Two orchestration skills (`dev-team` and `dev-init`)
- **settings.json** — Claude Code configuration
- **statusline.sh** — Status bar script for Claude Code

After the tool runs it prints step-by-step instructions showing where to place each generated
file so that Claude Code picks them up automatically.

Future support for GitHub Copilot CLI as an alternative target is planned.

The agent team is configured to use **`bmo`** for issue tracking. `bmo` is an alternative
to Docket and is the canonical issue tracking CLI for this team configuration.

---

## Agent Team

| Agent | Role |
|---|---|
| **Staff Engineer** | Architecture, technical design documents (TDDs), code review |
| **Senior Engineer** | Implementation, code quality, debugging |
| **Project Manager** | Issue planning, task breakdown, dependency management |
| **QA Engineer** | Testing, verification, acceptance criteria |
| **UX Designer** | User experience design specs |

---

## Skills

| Skill | Description |
|---|---|
| **dev-team** | Coordinates all five agents for planning and executing development work |
| **dev-init** | Bootstraps `docs/spec/` project specifications for new repositories |

---

## BMO Integration

The agent team is configured to use the `bmo` CLI for issue tracking. `bmo` provides a
Kanban-style workflow (`todo`, `in-progress`, `done`) via a simple command-line interface
and serves as an alternative to Docket.

Each agent in the team references `bmo` commands in its system prompt. When you invoke
the `dev-team` skill or any individual agent, they manage work through `bmo`.

---

## Usage

### Prerequisites

- Rust toolchain (stable, 1.70+)
- macOS or Linux

### Build and Run

```bash
# Build
cargo build --release

# Run
cargo run --release -- --output ~/my-claude-env

# Or with justfile
just run
just run ~/my-claude-env
```

### CLI Options

| Flag | Description | Default |
|---|---|---|
| `-o`, `--output <PATH>` | Output directory for the generated environment | `./claude-code-env` |
| `-h`, `--help` | Print help | |
| `-V`, `--version` | Print version | |

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

[Apache License 2.0](LICENSE)
