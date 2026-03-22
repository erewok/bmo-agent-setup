# Configuration Guide

## Quick Start

### Without Config File (Simple)

```bash
# Basic setup with thinking enabled
cargo run

# With statusline
cargo run -- --with-statusline=true

# Without thinking mode
cargo run -- --with-thinking=false
```

### With Config File (Recommended)

```bash
# Use a config file
cargo run -- --config bmo-config.toml

# Override config settings via CLI
cargo run -- --config bmo-config.toml --with-thinking=false
```

## Configuration Files

Three example configurations are provided:

### `bmo-config.toml` - Full Configuration

Complete configuration with all available options documented. Good starting point for customization.

### `bmo-config.minimal.toml` - Minimal

Just the essentials:

- Thinking enabled
- Statusline enabled
- Ask permission mode

### `bmo-config.secure.toml` - Secure/Restricted

Maximum safety for sensitive environments:

- Very restrictive permissions
- Sandbox enabled
- Limited network access
- Explicit denials for dangerous operations

## Configuration Reference

### Core Settings

```toml
[core]
model = "claude-sonnet-4"
output-style = "concise"
thinking-enabled = true
fast-mode = false
api-key-helper = "op read op://Private/anthropic/api-key"
cleanup-period-days = 7

[core.env]
CUSTOM_VAR = "value"
```

### Statusline

```toml
[statusline]
enabled = true
command = "/custom/path/to/statusline.sh"  # Optional
padding = 2
```

### Permissions

```toml
[permissions]
default-mode = "ask"  # "allow", "ask", or "deny"

allow = [
    "Read",
]

ask = [
    "Edit",
    "Write",
    "Bash",
]

deny = [
    "Write(//**)",          # block all absolute filesystem writes
    "Write(~/.ssh/**)",     # credential files not covered by // prefix
    "Write(~/.aws/**)",
    "Write(~/.config/**)",
    "Bash(sudo *)",
]

additional-directories = ["/path/to/extra/dir"]
```

### Sandbox

```toml
[sandbox]
enabled = true
auto-allow-bash = true
allow-unsandboxed-commands = false
excluded-commands = ["git", "cargo"]

[sandbox.network]
allowed-domains = [
    "github.com",
    "api.anthropic.com",
]
allow-unix-sockets = ["/var/run/docker.sock"]
allow-local-binding = true
```

### Attribution

```toml
[attribution]
commit = "Co-authored-by: Claude <claude@anthropic.com>"
pr = "Assisted by Claude Code"
include-co-authored-by = true
```

### MCP Servers

```toml
[mcp]
enable-all-project-servers = false
enabled-servers = ["filesystem", "github"]
disabled-servers = ["some-server"]
```

### Hooks

```toml
[[hooks.onFileCreate]]
matcher = "*.rs"
command = "rustfmt"
type = "sync"

[[hooks.onFileCreate]]
matcher = "*.py"
command = "black"
type = "sync"
```

## CLI Flags

All CLI flags **override** config file settings:

- `-o, --output <PATH>` - Output directory (default: `./claude-code-env`)
- `-c, --config <FILE>` - Path to TOML configuration file
- `--with-statusline <true|false>` - Enable/disable statusline (overrides config)
- `--with-thinking <true|false>` - Enable/disable thinking mode (overrides config)

## Precedence

Settings are applied in this order (later overrides earlier):

1. **Defaults** - Built-in defaults
2. **Config File** - Settings from TOML file (if provided)
3. **CLI Flags** - Command-line overrides

## Examples

### Development Setup

```bash
cargo run -- --config bmo-config.toml
```

### Production/Secure Setup

```bash
cargo run -- --config bmo-config.secure.toml --output ~/prod-claude-env
```

### Quick Test (No Permissions)

```bash
cargo run -- --config bmo-config.minimal.toml --with-thinking=false
```

### Override Config Temporarily

```bash
# Disable thinking from config file
cargo run -- --config bmo-config.toml --with-thinking=false

# Enable statusline even if config disables it
cargo run -- --config my-config.toml --with-statusline=true
```

## Creating Your Own Config

1. Copy one of the example files:

```bash
cp bmo-config.toml my-config.toml
```

2. Edit to your preferences:

```bash
$EDITOR my-config.toml
```

3. Use it:

```bash
cargo run -- --config my-config.toml
```

## Common Patterns

### Maximize Safety

Use `bmo-config.secure.toml` as a base and customize the `allow` rules for your specific project needs.

### Maximize Convenience

Use `bmo-config.toml` and set broader `allow` rules, disable sandbox.

### Balanced Approach

Use `bmo-config.minimal.toml` and add specific `ask` or `allow` rules as needed.
