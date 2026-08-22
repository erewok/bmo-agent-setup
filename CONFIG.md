# Configuration Guide

## Quick Start

### No `--config` flag (uses the built-in default)

```bash
# Uses bmo-config.default.toml, compiled into the binary
cargo run

# Override individual settings from it via CLI
cargo run -- --with-thinking=false
cargo run -- --with-statusline=true
```

If you don't pass `--config`, this tool does **not** fall back to bare
hardcoded defaults — it loads `bmo-config.default.toml`, embedded into the
binary at compile time. See [Configuration Files](#configuration-files)
below for what that posture is and how to pick a different one.

### With an explicit config file

```bash
# Use one of the other built-in presets
cargo run -- --config bmo-config.yolo-mode.toml
cargo run -- --config bmo-config.hardened.toml

# Or your own
cargo run -- --config my-config.toml

# CLI flags always override whatever the config file set
cargo run -- --config bmo-config.hardened.toml --with-thinking=false
```

## Configuration Files

Three presets are provided. If you're not sure which one you want: **default**
is what most people want day to day — it's what `cargo run` uses with no
flag — reach for **hardened** in an unfamiliar or higher-stakes codebase, and
**yolo-mode** in a throwaway environment when you don't want to be asked
about anything.

### `bmo-config.default.toml` - Default (used automatically)

Auto-mode-first, and what `cargo run` uses when no `--config` is given.
`defaultMode = "auto"` hands routine judgment calls to Claude Code's own
classifier instead of a static ask-list, so editing the project you opened
Claude Code in doesn't stop to ask; deny still blocks a short list of
obviously dangerous paths outright, and a couple of read-only commands are
allowed outright too. The sandbox is off. Also doubles as the
fully-documented reference for every option this tool supports — see
[Configuration Reference](#configuration-reference) below.

### `bmo-config.yolo-mode.toml` - YOLO

Maximum convenience, minimum friction — `defaultMode = "bypassPermissions"`,
so Claude Code does not stop to ask about anything except a small,
non-overridable deny list (`rm -rf /`, writing to `~/.ssh`, etc). Sandbox
is off. **Only use this in a disposable environment** (container, throwaway
VM, worktree you don't mind losing) — this is the "just let it cook" preset.

### `bmo-config.hardened.toml` - Hardened

Reads are unrestricted; writes and edits are allowed only inside the
current working directory; everything else (writes elsewhere, shell
commands) asks first; a fixed list of dangerous paths is denied outright
regardless. Sandbox is enabled, with network restricted to
`api.anthropic.com` and credential files/env vars hidden from sandboxed
commands. Good default posture for an unfamiliar codebase or a
higher-stakes environment.

## Configuration Reference

This tool generates a `~/.claude/settings.json` for Claude Code. The full,
current settings.json schema is documented at
https://code.claude.com/docs/en/settings and published as JSON Schema at
https://json.schemastore.org/claude-code-settings.json. The sections below
cover what this tool models with a dedicated, typed TOML section; anything
else can be set via the [`[settings]` escape hatch](#escape-hatch-for-new-or-unmodeled-settings).

### Permission rule syntax

Permission rules (`allow`/`ask`/`deny`) use `ToolName` or
`ToolName(pattern)` — **not** the old `ToolName:pattern` colon syntax.
A bare tool name (e.g. `"Bash"`) matches every call to that tool.

```
"Bash"                  # every Bash call
"Bash(git commit *)"    # Bash calls matching this pattern
"Read(src/**)"          # Read calls scoped to a path glob
"Edit(/etc/**)"         # file edits scoped to a path glob
```

Path-scoped file rules must use `Edit(...)`, never `Write(...)`. A single `Edit`
rule covers every file-editing tool (Write, Edit, NotebookEdit). A `Write(path)`
rule is ignored by file permission checks — Claude Code warns about it at startup,
and in a `deny` list it silently protects nothing.

### Core Settings

```toml
[core]
model = "claude-sonnet-5"
output-style = "concise"
thinking-enabled = true
fast-mode = false                  # Opus-only; requires extra usage enabled
api-key-helper = "op read op://Private/anthropic/api-key"
cleanup-period-days = 7
agent = "my-custom-agent"          # Named subagent to run the main thread as
default-shell = "bash"             # "bash" or "powershell"
editor-mode = "normal"             # "normal" or "vim"
effort-level = "high"              # "low", "medium", "high", or "xhigh"
available-models = ["claude-opus-4-8", "claude-sonnet-5"]
fallback-model = ["claude-sonnet-5"]   # max 3 entries
file-suggestion-command = "/custom/path/to/file-suggestion.sh"

[core.env]
CUSTOM_VAR = "value"
```

### Statusline

```toml
[statusline]
enabled = true
command = "/custom/path/to/statusline.sh"  # Optional
padding = 2
refresh-interval = 5               # Re-run every N seconds (optional)
hide-vim-mode-indicator = false    # Optional
```

### Permissions

```toml
[permissions]
# "acceptEdits", "auto", "bypassPermissions", "default",
# "delegate" (experimental agent teams), "dontAsk", "plan"
default-mode = "default"

allow = [
    "Read(*)",
    "Edit(./**)",    # edits inside the current directory are not asked about
]

ask = [
    "Edit",          # any edit outside the current directory still asks
    "Bash",
]

deny = [
    "Edit(/etc/**)",
    "Bash(sudo *)",
]

additional-directories = ["/path/to/extra/dir"]
disable-bypass-permissions-mode = "disable"  # Optional, set to "disable"
disable-auto-mode = "disable"                # Optional, set to "disable"
```

`deny` always wins. Below that, a more specific pattern for the same tool
wins over a broader one regardless of which list it's in — that's why
`"Edit(./**)"` in `allow` above takes effect for in-directory edits even
though the broader `"Edit"` in `ask` would otherwise catch them. This is
the general technique for "allow X in this scope, ask about X everywhere
else": put the narrow pattern in `allow`, the bare tool name in `ask`.

### Auto Mode

```toml
[auto-mode]
environment = [
    "Source control: github.com/your-org/your-repo",
    "Protected branches: main",
    "Trusted internal domains: *.your-company.com",
]
allow = []            # Override the classifier's default allow rules
soft-deny = []        # Override the classifier's default soft-block rules
hard-deny = []        # Override the classifier's default hard-block rules
classify-all-shell = false   # v2.1.193+
```

`default-mode = "auto"` (set under `[permissions]` above) turns on a
classifier that reviews routine actions instead of prompting for each one —
it's Anthropic's own documented default for Pro/Max/Team plans. It runs
*underneath* `allow`/`ask`/`deny`, not instead of them: `deny` still blocks
unconditionally, `ask` still forces a prompt even in auto mode, `allow`
still short-circuits to yes. `[auto-mode]` only tunes the classifier's own
judgment for whatever isn't already covered by those three lists.

`environment` is prose describing this specific project's trust boundaries
(source control, sensitive domains, protected branches) — it's what the
classifier reads to judge context-dependent actions. It's inherently
project- and org-specific, so this tool won't template it for you; write it
per project, the way `bmo-config.default.toml` leaves it commented out.

### Sandbox

```toml
[sandbox]
enabled = true
auto-allow-bash = true
allow-unsandboxed-commands = false
excluded-commands = ["git", "cargo"]
enable-weaker-network-isolation = false  # macOS only, for MITM-proxy CLIs
enable-weaker-nested-sandbox = false     # For unprivileged docker
fail-if-unavailable = false              # Hard-fail vs. warn-and-skip
allow-apple-events = false               # macOS only

[sandbox.network]
allowed-domains = [
    "github.com",
    "api.anthropic.com",
]
denied-domains = ["*.internal.example.com"]  # Takes precedence over allow
allow-unix-sockets = ["/var/run/docker.sock"]
allow-local-binding = true
http-proxy-port = 8080     # Optional, use a custom proxy instead of auto-start
socks-proxy-port = 1080    # Optional

[sandbox.credentials]
# Hide these from sandboxed commands (v2.1.187+)
files = ["~/.aws/credentials", "~/.ssh/id_ed25519"]
env-vars = ["ANTHROPIC_API_KEY", "AWS_SECRET_ACCESS_KEY"]
```

### Attribution

```toml
[attribution]
commit = "Co-authored-by: Claude <claude@anthropic.com>"
pr = "Assisted by Claude Code"
# include-co-authored-by is DEPRECATED in favor of commit/pr above
```

### MCP Servers

```toml
[mcp]
enable-all-project-servers = false
enabled-servers = ["filesystem", "github"]
disabled-servers = ["some-server"]
```

### Hooks

Hook table names must be real Claude Code lifecycle events — see
https://code.claude.com/docs/en/hooks for the full list (`PreToolUse`,
`PostToolUse`, `Stop`, `SessionStart`, `UserPromptSubmit`, etc.) and for
matcher syntax. The matcher scopes by **tool name**, not by file glob; to
scope by file type, have the command inspect the JSON piped to its stdin.
`type` must be `"command"`.

```toml
[[hooks.PostToolUse]]
matcher = "Edit|Write"
command = "rustfmt"
type = "command"
```

### Escape hatch for new or unmodeled settings

Claude Code's settings.json accepts arbitrary top-level keys (the schema's
root object allows additional properties), and new ones are added
regularly. Rather than waiting for this tool's Rust structs to catch up,
drop any top-level setting into a `[settings]` table. Keys here are the
**literal camelCase settings.json key names** (not kebab-case, unlike every
other section above), and values pass straight through into the generated
file:

```toml
[settings]
respectGitignore = true
autoCompactEnabled = true
theme = "dark"
```

Don't repeat a key that already has a dedicated section above (`core`,
`statusline`, `permissions`, `sandbox`, `attribution`, `mcp`) — that
produces a duplicate JSON key when settings.json is generated. Nested
objects like `sandbox` or `permissions` are closed schemas
(`additionalProperties: false`), so new fields *inside* those objects still
require a Rust code change in `src/claude_code.rs`; this escape hatch only
covers new top-level keys, which is where Claude Code has historically
added new settings.

## CLI Flags

All CLI flags **override** config file settings:

- `-o, --output <PATH>` - Output directory (default: `./claude-code-env`)
- `-c, --config <FILE>` - Path to TOML configuration file
- `--with-statusline <true|false>` - Enable/disable statusline (overrides config)
- `--with-thinking <true|false>` - Enable/disable thinking mode (overrides config)

## Precedence

Settings are applied in this order (later overrides earlier):

1. **Built-in default** - `bmo-config.default.toml`, embedded at compile time
2. **Config File** - Settings from an explicit `--config` TOML file (replaces
   the built-in default entirely, rather than layering on top of it)
3. **CLI Flags** - `--with-thinking`/`--with-statusline` override whichever
   of the above was used

## Examples

### Just try it

```bash
cargo run
```

### Everyday hardened setup

```bash
cargo run -- --config bmo-config.hardened.toml
```

### Throwaway/YOLO environment

```bash
cargo run -- --config bmo-config.yolo-mode.toml --output ~/scratch-claude-env
```

### Override Config Temporarily

```bash
# Disable thinking from a config file
cargo run -- --config bmo-config.hardened.toml --with-thinking=false

# Force statusline on/off regardless of what the config says
cargo run -- --config my-config.toml --with-statusline=true
cargo run -- --config my-config.toml --with-statusline=false
```

## Creating Your Own Config

1. Copy one of the presets as a starting point:

```bash
cp bmo-config.hardened.toml my-config.toml
```

2. Edit to your preferences:

```bash
$EDITOR my-config.toml
```

3. Use it:

```bash
cargo run -- --config my-config.toml
```

`my-config.toml` is gitignored, so it's a safe place for a personal config
that doesn't affect what `cargo run` does for everyone else.

## Common Patterns

### Maximize Safety

Use `bmo-config.hardened.toml` as a base and loosen the `allow` rules only
for the specific paths/commands your project needs.

### Maximize Convenience

Use `bmo-config.yolo-mode.toml` — but only in a disposable environment.

### Balanced / Not Sure

Use `bmo-config.default.toml` (or just don't pass `--config` at all) and
tighten `ask`/`deny` rules as you learn what you actually want to allow.
