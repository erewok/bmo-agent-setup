use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::claude_code::{ClaudeCode, Permissions};

/// TOML configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<CoreConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statusline: Option<StatusLineConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<SandboxConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<AttributionConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<McpConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<BTreeMap<String, Vec<HookEntry>>>,

    /// Escape hatch for any settings.json key this file doesn't model with a
    /// dedicated section. Keys are the literal camelCase settings.json key
    /// names (not kebab-case) and values pass straight through to the
    /// generated settings.json. See CONFIG.md for details. Do not repeat a
    /// key that already has a dedicated section above (core, statusline,
    /// permissions, sandbox, attribution, mcp) -- that produces a duplicate
    /// key when the settings.json is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CoreConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_mode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_helper: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup_period_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,

    /// Named subagent to run the main thread as (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// "bash" or "powershell" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shell: Option<String>,

    /// "normal" or "vim" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_mode: Option<String>,

    /// "low", "medium", "high", or "xhigh" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort_level: Option<String>,

    /// Restrict the selectable model list (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_models: Option<Vec<String>>,

    /// Fallback model chain, max 3 entries (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_model: Option<Vec<String>>,

    /// Path to a script that generates @ file-picker suggestions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_suggestion_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StatusLineConfig {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<u32>,

    /// Re-run the status line command every N seconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<u32>,

    /// Suppress the built-in vim mode indicator (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_vim_mode_indicator: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PermissionsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_directories: Option<Vec<String>>,

    /// Set to "disable" to prevent bypassPermissions mode from being used (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_bypass_permissions_mode: Option<String>,

    /// Set to "disable" to prevent auto mode from being activated (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_auto_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SandboxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_allow_bash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unsandboxed_commands: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_commands: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkConfig>,

    /// macOS only; required for some Go-based CLIs behind a MITM proxy (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_weaker_network_isolation: Option<bool>,

    /// For unprivileged docker environments where --proc mounting fails (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_weaker_nested_sandbox: Option<bool>,

    /// Hard-fail startup if sandbox dependencies are missing (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_unavailable: Option<bool>,

    /// macOS only; allow sandboxed commands to send Apple Events (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_apple_events: Option<bool>,

    /// Hide credential files/env vars from sandboxed commands (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<CredentialsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetworkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    /// Blocks specific domains even when allowed-domains would permit them (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unix_sockets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_local_binding: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_all_unix_sockets: Option<bool>,

    /// Managed settings only (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_managed_domains_only: Option<bool>,

    /// macOS only (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_mach_lookup: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy_port: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub socks_proxy_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CredentialsConfig {
    /// Credential file paths to hide from sandboxed commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    /// Secret environment variable names to unset for sandboxed commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AttributionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_co_authored_by: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct McpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_all_project_servers: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_servers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_servers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,

    pub command: String,

    #[serde(rename = "type")]
    pub hook_type: String,
}

impl ConfigFile {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::from_toml_str(&contents)
    }

    /// Parse configuration from a TOML string (e.g. the built-in default,
    /// embedded via `include_str!`)
    pub fn from_toml_str(contents: &str) -> Result<Self> {
        let config: ConfigFile = toml::from_str(contents)?;
        Ok(config)
    }

    /// Apply this configuration to a ClaudeCode builder
    pub fn apply_to_builder(&self, mut builder: ClaudeCode) -> ClaudeCode {
        // Core settings
        if let Some(ref core) = self.core {
            if let Some(ref model) = core.model {
                builder = builder.with_model(model);
            }
            if let Some(ref style) = core.output_style {
                builder = builder.with_output_style(style);
            }
            if let Some(thinking) = core.thinking_enabled {
                builder = builder.with_always_thinking_enabled(thinking);
            }
            if let Some(fast) = core.fast_mode {
                builder = builder.with_fast_mode(fast);
            }
            if let Some(ref helper) = core.api_key_helper {
                builder = builder.with_api_key_helper(helper);
            }
            if let Some(days) = core.cleanup_period_days {
                builder = builder.with_cleanup_period_days(days);
            }
            if let Some(ref env_vars) = core.env {
                builder = builder.with_env_vars(env_vars.clone());
            }
            if let Some(ref agent) = core.agent {
                builder = builder.with_agent(agent);
            }
            if let Some(ref shell) = core.default_shell {
                builder = builder.with_default_shell(shell);
            }
            if let Some(ref mode) = core.editor_mode {
                builder = builder.with_editor_mode(mode);
            }
            if let Some(ref level) = core.effort_level {
                builder = builder.with_effort_level(level);
            }
            if let Some(ref models) = core.available_models {
                builder = builder.with_available_models(models.clone());
            }
            if let Some(ref models) = core.fallback_model {
                builder = builder.with_fallback_model(models.clone());
            }
            if let Some(ref command) = core.file_suggestion_command {
                builder = builder.with_file_suggestion(command);
            }
        }

        // Statusline
        if let Some(ref statusline) = self.statusline {
            if statusline.enabled {
                let cmd = statusline
                    .command
                    .as_deref()
                    .unwrap_or("$HOME/.claude/statusline.sh");
                builder = builder.with_status_line(cmd);

                if let Some(padding) = statusline.padding {
                    builder = builder.with_status_line_padding(padding);
                }
                if let Some(interval) = statusline.refresh_interval {
                    builder = builder.with_status_line_refresh_interval(interval);
                }
                if let Some(hide) = statusline.hide_vim_mode_indicator {
                    builder = builder.with_status_line_hide_vim_mode_indicator(hide);
                }
            }
        }

        // Permissions
        if let Some(ref perms) = self.permissions {
            let mut permissions = Permissions::default();

            if let Some(ref mode) = perms.default_mode {
                permissions.default_mode = Some(mode.clone());
            }
            if let Some(ref allow) = perms.allow {
                permissions.allow = allow.clone();
            }
            if let Some(ref ask) = perms.ask {
                permissions.ask = ask.clone();
            }
            if let Some(ref deny) = perms.deny {
                permissions.deny = deny.clone();
            }
            if let Some(ref dirs) = perms.additional_directories {
                permissions.additional_directories = dirs.clone();
            }
            if let Some(ref value) = perms.disable_bypass_permissions_mode {
                permissions.disable_bypass_permissions_mode = Some(value.clone());
            }
            if let Some(ref value) = perms.disable_auto_mode {
                permissions.disable_auto_mode = Some(value.clone());
            }

            builder = builder.with_permissions(permissions);
        }

        // Sandbox
        if let Some(ref sandbox) = self.sandbox {
            if let Some(enabled) = sandbox.enabled {
                builder = builder.with_sandbox_enabled(enabled);
            }
            if let Some(auto_bash) = sandbox.auto_allow_bash {
                builder = builder.with_sandbox_auto_allow_bash(auto_bash);
            }
            if let Some(allow_unsandboxed) = sandbox.allow_unsandboxed_commands {
                builder = builder.with_sandbox_allow_unsandboxed_commands(allow_unsandboxed);
            }
            if let Some(ref excluded) = sandbox.excluded_commands {
                builder = builder.with_sandbox_excluded_commands(excluded.clone());
            }
            if let Some(ref network) = sandbox.network {
                if let Some(ref domains) = network.allowed_domains {
                    builder = builder.with_sandbox_network_allowed_domains(domains.clone());
                }
                if let Some(ref domains) = network.denied_domains {
                    builder = builder.with_sandbox_network_denied_domains(domains.clone());
                }
                if let Some(ref sockets) = network.allow_unix_sockets {
                    builder = builder.with_sandbox_network_allow_unix_sockets(sockets.clone());
                }
                if let Some(local_binding) = network.allow_local_binding {
                    builder = builder.with_sandbox_network_allow_local_binding(local_binding);
                }
                if let Some(allow_all) = network.allow_all_unix_sockets {
                    builder = builder.with_sandbox_network_allow_all_unix_sockets(allow_all);
                }
                if let Some(managed_only) = network.allow_managed_domains_only {
                    builder = builder.with_sandbox_network_allow_managed_domains_only(managed_only);
                }
                if let Some(mach_lookup) = network.allow_mach_lookup {
                    builder = builder.with_sandbox_network_allow_mach_lookup(mach_lookup);
                }
                if let (Some(http), Some(socks)) =
                    (network.http_proxy_port, network.socks_proxy_port)
                {
                    builder = builder.with_sandbox_network_proxy_ports(Some(http), Some(socks));
                } else if let Some(http) = network.http_proxy_port {
                    builder = builder.with_sandbox_network_proxy_ports(Some(http), None);
                } else if let Some(socks) = network.socks_proxy_port {
                    builder = builder.with_sandbox_network_proxy_ports(None, Some(socks));
                }
            }
            if let Some(enabled) = sandbox.enable_weaker_network_isolation {
                builder = builder.with_sandbox_enable_weaker_network_isolation(enabled);
            }
            if let Some(enabled) = sandbox.enable_weaker_nested_sandbox {
                builder = builder.with_sandbox_enable_weaker_nested_sandbox(enabled);
            }
            if let Some(enabled) = sandbox.fail_if_unavailable {
                builder = builder.with_sandbox_fail_if_unavailable(enabled);
            }
            if let Some(enabled) = sandbox.allow_apple_events {
                builder = builder.with_sandbox_allow_apple_events(enabled);
            }
            if let Some(ref credentials) = sandbox.credentials {
                if let Some(ref files) = credentials.files {
                    for path in files {
                        builder = builder.with_sandbox_credential_file(path);
                    }
                }
                if let Some(ref env_vars) = credentials.env_vars {
                    for name in env_vars {
                        builder = builder.with_sandbox_credential_env_var(name);
                    }
                }
            }
        }

        // Attribution
        if let Some(ref attr) = self.attribution {
            if let Some(ref commit) = attr.commit {
                builder = builder.with_attribution_commit(commit);
            }
            if let Some(ref pr) = attr.pr {
                builder = builder.with_attribution_pr(pr);
            }
            if let Some(co_authored) = attr.include_co_authored_by {
                builder = builder.with_include_co_authored_by(co_authored);
            }
        }

        // MCP
        if let Some(ref mcp) = self.mcp {
            if let Some(enable_all) = mcp.enable_all_project_servers {
                builder = builder.with_enable_all_project_mcp_servers(enable_all);
            }
            if let Some(ref enabled) = mcp.enabled_servers {
                for server in enabled {
                    builder = builder.with_enabled_mcpjson_server(server);
                }
            }
            if let Some(ref disabled) = mcp.disabled_servers {
                for server in disabled {
                    builder = builder.with_disabled_mcpjson_server(server);
                }
            }
        }

        // Hooks
        if let Some(ref hooks) = self.hooks {
            for (hook_name, entries) in hooks {
                for entry in entries {
                    builder = builder.with_hook(
                        hook_name,
                        entry.matcher.as_deref(),
                        &entry.command,
                        &entry.hook_type,
                    );
                }
            }
        }

        // Escape hatch: any settings.json key not modeled above
        if let Some(ref settings) = self.settings {
            for (key, value) in settings {
                let json_value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
                builder = builder.with_extra_setting(key, json_value);
            }
        }

        builder
    }
}
