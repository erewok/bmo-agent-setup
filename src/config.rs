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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StatusLineConfig {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<u32>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetworkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unix_sockets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_local_binding: Option<bool>,
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
        let config: ConfigFile = toml::from_str(&contents)?;
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
                if let Some(ref sockets) = network.allow_unix_sockets {
                    builder = builder.with_sandbox_network_allow_unix_sockets(sockets.clone());
                }
                if let Some(local_binding) = network.allow_local_binding {
                    builder = builder.with_sandbox_network_allow_local_binding(local_binding);
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

        builder
    }
}
