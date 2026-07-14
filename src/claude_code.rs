use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Supporting types for nested configuration structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCommand {
    pub command: String,
    #[serde(rename = "type")]
    pub hook_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
    pub hooks: Vec<HookCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allow: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ask: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub deny: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub additional_directories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_bypass_permissions_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_auto_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SandboxNetwork {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowed_domains: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub denied_domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_all_unix_sockets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unix_sockets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_local_binding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_managed_domains_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_mach_lookup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socks_proxy_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialFile {
    pub path: String,
    /// Only "deny" is supported by Claude Code today.
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialEnvVar {
    pub name: String,
    /// Only "deny" is supported by Claude Code today.
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SandboxCredentials {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<CredentialFile>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub env_vars: Vec<CredentialEnvVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sandbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_allow_bash_if_sandboxed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unsandboxed_commands: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub excluded_commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<SandboxNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_weaker_network_isolation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_weaker_nested_sandbox: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_unavailable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_apple_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SandboxCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSuggestion {
    #[serde(rename = "type")]
    pub suggestion_type: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLine {
    #[serde(rename = "type")]
    pub status_type: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "hideVimModeIndicator"
    )]
    pub hide_vim_mode_indicator: Option<bool>,
}

// Main ClaudeCode configuration struct

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaudeCode {
    // Core settings
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_helper: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cleanup_period_days: Option<u32>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    env: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    editor_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort_level: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    available_models: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    fallback_model: Vec<String>,

    // Authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    force_login_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_login_org_uuid: Option<String>,

    // Permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,

    // Sandbox
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<Sandbox>,

    // Attribution
    #[serde(skip_serializing_if = "Option::is_none")]
    attribution: Option<Attribution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_co_authored_by: Option<bool>,

    // MCP Servers
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_all_project_mcp_servers: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    enabled_mcpjson_servers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    disabled_mcpjson_servers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    allowed_mcp_servers: Vec<McpServerRule>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    denied_mcp_servers: Vec<McpServerRule>,

    // Additional Features
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    company_announcements: Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    hooks: BTreeMap<String, Vec<HookConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_line: Option<StatusLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_suggestion: Option<FileSuggestion>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    enabled_plugins: BTreeMap<String, bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_thinking_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_mode: Option<bool>,

    /// Escape hatch for any settings.json key not yet modeled above.
    /// The root settings object allows additional properties, so new
    /// top-level Claude Code settings can be passed straight through here
    /// (e.g. via the `[settings]` table in a TOML config) without requiring
    /// a Rust code change first. Keys here must not duplicate a named field.
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

impl ClaudeCode {
    pub fn new() -> Self {
        Self {
            // Core settings - all None by default
            model: None,
            output_style: None,
            api_key_helper: None,
            cleanup_period_days: None,
            env: BTreeMap::new(),
            agent: None,
            default_shell: None,
            editor_mode: None,
            effort_level: None,
            available_models: Vec::new(),
            fallback_model: Vec::new(),

            // Authentication
            force_login_method: None,
            force_login_org_uuid: None,

            // Permissions
            permissions: None,

            // Sandbox
            sandbox: None,

            // Attribution
            attribution: None,
            include_co_authored_by: None,

            // MCP Servers
            enable_all_project_mcp_servers: None,
            enabled_mcpjson_servers: Vec::new(),
            disabled_mcpjson_servers: Vec::new(),
            allowed_mcp_servers: Vec::new(),
            denied_mcp_servers: Vec::new(),

            // Additional Features
            company_announcements: Vec::new(),
            hooks: BTreeMap::new(),
            status_line: None,
            file_suggestion: None,
            enabled_plugins: BTreeMap::new(),
            always_thinking_enabled: None,
            fast_mode: None,
            extra: BTreeMap::new(),
        }
    }

    // Core settings builder methods

    #[allow(dead_code)]
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_output_style(mut self, style: &str) -> Self {
        self.output_style = Some(style.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_api_key_helper(mut self, helper: &str) -> Self {
        self.api_key_helper = Some(helper.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_cleanup_period_days(mut self, days: u32) -> Self {
        self.cleanup_period_days = Some(days);
        self
    }

    #[allow(dead_code)]
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_env_vars(mut self, vars: BTreeMap<String, String>) -> Self {
        self.env = vars;
        self
    }

    #[allow(dead_code)]
    pub fn with_agent(mut self, agent: &str) -> Self {
        self.agent = Some(agent.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_default_shell(mut self, shell: &str) -> Self {
        self.default_shell = Some(shell.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_editor_mode(mut self, mode: &str) -> Self {
        self.editor_mode = Some(mode.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_effort_level(mut self, level: &str) -> Self {
        self.effort_level = Some(level.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_available_models(mut self, models: Vec<String>) -> Self {
        self.available_models = models;
        self
    }

    #[allow(dead_code)]
    pub fn with_fallback_model(mut self, models: Vec<String>) -> Self {
        self.fallback_model = models;
        self
    }

    /// Set an arbitrary top-level settings.json key not otherwise modeled by
    /// this struct. Panics-free: silently overwrites if called twice for the
    /// same key. Do not use this for keys that already have a dedicated
    /// field/method above (e.g. "model", "sandbox") -- that produces a
    /// duplicate-key serialization error.
    #[allow(dead_code)]
    pub fn with_extra_setting(mut self, key: &str, value: serde_json::Value) -> Self {
        self.extra.insert(key.to_string(), value);
        self
    }

    // Authentication builder methods

    #[allow(dead_code)]
    pub fn with_force_login_method(mut self, method: &str) -> Self {
        self.force_login_method = Some(method.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_force_login_org_uuid(mut self, uuid: &str) -> Self {
        self.force_login_org_uuid = Some(uuid.to_string());
        self
    }

    // Permissions builder methods

    #[allow(dead_code)]
    pub fn with_permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_allow(mut self, rule: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.allow.push(rule.to_string());
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_ask(mut self, rule: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.ask.push(rule.to_string());
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_deny(mut self, rule: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.deny.push(rule.to_string());
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_additional_directories(mut self, dirs: Vec<String>) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.additional_directories = dirs;
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_default_mode(mut self, mode: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.default_mode = Some(mode.to_string());
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_disable_bypass_permissions_mode(mut self, value: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.disable_bypass_permissions_mode = Some(value.to_string());
        self.permissions = Some(perms);
        self
    }

    #[allow(dead_code)]
    pub fn with_permission_disable_auto_mode(mut self, value: &str) -> Self {
        let mut perms = self.permissions.unwrap_or_default();
        perms.disable_auto_mode = Some(value.to_string());
        self.permissions = Some(perms);
        self
    }

    // Sandbox builder methods

    #[allow(dead_code)]
    pub fn with_sandbox(mut self, sandbox: Sandbox) -> Self {
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_enabled(mut self, enabled: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.enabled = Some(enabled);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_auto_allow_bash(mut self, auto_allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.auto_allow_bash_if_sandboxed = Some(auto_allow);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_allow_unsandboxed_commands(mut self, allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.allow_unsandboxed_commands = Some(allow);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_excluded_commands(mut self, commands: Vec<String>) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.excluded_commands = commands;
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allowed_domains(mut self, domains: Vec<String>) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allowed_domains = domains;
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allow_unix_sockets(mut self, sockets: Vec<String>) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allow_unix_sockets = Some(sockets);
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allow_local_binding(mut self, allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allow_local_binding = Some(allow);
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_denied_domains(mut self, domains: Vec<String>) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.denied_domains = domains;
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allow_all_unix_sockets(mut self, allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allow_all_unix_sockets = Some(allow);
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allow_managed_domains_only(mut self, allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allow_managed_domains_only = Some(allow);
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_allow_mach_lookup(mut self, allow: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.allow_mach_lookup = Some(allow);
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_network_proxy_ports(
        mut self,
        http_port: Option<u16>,
        socks_port: Option<u16>,
    ) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut network = sandbox.network.unwrap_or_default();
        network.http_proxy_port = http_port;
        network.socks_proxy_port = socks_port;
        sandbox.network = Some(network);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_enable_weaker_network_isolation(mut self, enabled: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.enable_weaker_network_isolation = Some(enabled);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_enable_weaker_nested_sandbox(mut self, enabled: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.enable_weaker_nested_sandbox = Some(enabled);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_fail_if_unavailable(mut self, enabled: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.fail_if_unavailable = Some(enabled);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_allow_apple_events(mut self, enabled: bool) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        sandbox.allow_apple_events = Some(enabled);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_credential_file(mut self, path: &str) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut creds = sandbox.credentials.unwrap_or_default();
        creds.files.push(CredentialFile {
            path: path.to_string(),
            mode: "deny".to_string(),
        });
        sandbox.credentials = Some(creds);
        self.sandbox = Some(sandbox);
        self
    }

    #[allow(dead_code)]
    pub fn with_sandbox_credential_env_var(mut self, name: &str) -> Self {
        let mut sandbox = self.sandbox.unwrap_or_default();
        let mut creds = sandbox.credentials.unwrap_or_default();
        creds.env_vars.push(CredentialEnvVar {
            name: name.to_string(),
            mode: "deny".to_string(),
        });
        sandbox.credentials = Some(creds);
        self.sandbox = Some(sandbox);
        self
    }

    // Attribution builder methods

    #[allow(dead_code)]
    pub fn with_attribution(mut self, attribution: Attribution) -> Self {
        self.attribution = Some(attribution);
        self
    }

    #[allow(dead_code)]
    pub fn with_attribution_commit(mut self, commit: &str) -> Self {
        let mut attr = self.attribution.unwrap_or_default();
        attr.commit = Some(commit.to_string());
        self.attribution = Some(attr);
        self
    }

    #[allow(dead_code)]
    pub fn with_attribution_pr(mut self, pr: &str) -> Self {
        let mut attr = self.attribution.unwrap_or_default();
        attr.pr = Some(pr.to_string());
        self.attribution = Some(attr);
        self
    }

    #[allow(dead_code)]
    pub fn with_include_co_authored_by(mut self, enabled: bool) -> Self {
        self.include_co_authored_by = Some(enabled);
        self
    }

    // MCP Servers builder methods

    #[allow(dead_code)]
    pub fn with_enable_all_project_mcp_servers(mut self, enabled: bool) -> Self {
        self.enable_all_project_mcp_servers = Some(enabled);
        self
    }

    #[allow(dead_code)]
    pub fn with_enabled_mcpjson_server(mut self, server: &str) -> Self {
        self.enabled_mcpjson_servers.push(server.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_disabled_mcpjson_server(mut self, server: &str) -> Self {
        self.disabled_mcpjson_servers.push(server.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_allowed_mcp_server(mut self, rule: McpServerRule) -> Self {
        self.allowed_mcp_servers.push(rule);
        self
    }

    #[allow(dead_code)]
    pub fn with_denied_mcp_server(mut self, rule: McpServerRule) -> Self {
        self.denied_mcp_servers.push(rule);
        self
    }

    // Additional Features builder methods

    #[allow(dead_code)]
    pub fn with_company_announcements(mut self, announcements: Vec<String>) -> Self {
        self.company_announcements = announcements;
        self
    }

    #[allow(dead_code)]
    pub fn with_company_announcement(mut self, announcement: &str) -> Self {
        self.company_announcements.push(announcement.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_hook(
        mut self,
        hook_name: &str,
        matcher: Option<&str>,
        command: &str,
        hook_type: &str,
    ) -> Self {
        let hook_command = HookCommand {
            command: command.to_string(),
            hook_type: hook_type.to_string(),
        };
        let hook_config = HookConfig {
            matcher: matcher.map(|m| m.to_string()),
            hooks: vec![hook_command],
        };
        self.hooks
            .entry(hook_name.to_string())
            .or_default()
            .push(hook_config);
        self
    }

    #[allow(dead_code)]
    pub fn with_status_line(mut self, command: &str) -> Self {
        self.status_line = Some(StatusLine {
            status_type: "command".to_string(),
            command: command.to_string(),
            padding: None,
            refresh_interval: None,
            hide_vim_mode_indicator: None,
        });
        self
    }

    #[allow(dead_code)]
    pub fn with_status_line_padding(mut self, padding: u32) -> Self {
        if let Some(ref mut sl) = self.status_line {
            sl.padding = Some(padding);
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_status_line_refresh_interval(mut self, seconds: u32) -> Self {
        if let Some(ref mut sl) = self.status_line {
            sl.refresh_interval = Some(seconds);
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_status_line_hide_vim_mode_indicator(mut self, hide: bool) -> Self {
        if let Some(ref mut sl) = self.status_line {
            sl.hide_vim_mode_indicator = Some(hide);
        }
        self
    }

    #[allow(dead_code)]
    pub fn without_status_line(mut self) -> Self {
        self.status_line = None;
        self
    }

    #[allow(dead_code)]
    pub fn with_file_suggestion(mut self, command: &str) -> Self {
        self.file_suggestion = Some(FileSuggestion {
            suggestion_type: "command".to_string(),
            command: command.to_string(),
        });
        self
    }

    #[allow(dead_code)]
    pub fn with_enabled_plugin(mut self, plugin: &str, enabled: bool) -> Self {
        self.enabled_plugins.insert(plugin.to_string(), enabled);
        self
    }

    #[allow(dead_code)]
    pub fn with_always_thinking_enabled(mut self, enabled: bool) -> Self {
        self.always_thinking_enabled = Some(enabled);
        self
    }

    #[allow(dead_code)]
    pub fn with_fast_mode(mut self, enabled: bool) -> Self {
        self.fast_mode = Some(enabled);
        self
    }

    pub fn build(self) -> Result<ClaudeCode> {
        Ok(self)
    }
}

impl Default for ClaudeCode {
    fn default() -> Self {
        Self::new()
    }
}
