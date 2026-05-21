/**
 * Agent discovery for the bmo dev-team extension.
 *
 * Adapted from the pi subagent example. Discovers agents from:
 *   1. This extension's bundled agents/  directory (always loaded)
 *   2. Project-local .pi/agents/          (when agentScope includes "project")
 *   3. User-level ~/.pi/agent/agents/     (when agentScope includes "user")
 *
 * Bundled agents take lowest priority — project and user agents override them.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import { fileURLToPath } from "node:url";

export type AgentScope = "bundled" | "user" | "project" | "all";

export interface AgentConfig {
  name: string;
  description: string;
  systemPrompt: string;
  tools?: string[];
  model?: string;
  source: "bundled" | "user" | "project";
  filePath: string;
}

function parseAgentFile(filePath: string): Omit<AgentConfig, "source"> | null {
  let content: string;
  try {
    content = fs.readFileSync(filePath, "utf-8");
  } catch {
    return null;
  }

  const fmMatch = content.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!fmMatch) {
    // No frontmatter — use filename as name, whole file as prompt
    const name = path.basename(filePath, ".md");
    return { name, description: "", systemPrompt: content.trim(), filePath };
  }

  const frontmatterStr = fmMatch[1];
  const body = fmMatch[2].trim();

  // Simple YAML key:value parser (handles quoted and unquoted values, multi-line with >)
  const fm: Record<string, string> = {};
  const lines = frontmatterStr.split(/\r?\n/);
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    const kv = line.match(/^(\w[\w-]*):\s*(.*)/);
    if (kv) {
      const key = kv[1];
      let val = kv[2].trim();
      // Block scalar >
      if (val === ">") {
        const parts: string[] = [];
        i++;
        while (i < lines.length && (lines[i].startsWith("  ") || lines[i] === "")) {
          parts.push(lines[i].replace(/^  /, ""));
          i++;
        }
        val = parts.join(" ").trim();
        continue;
      }
      // Quoted
      if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) {
        val = val.slice(1, -1);
      }
      fm[key] = val;
    }
    i++;
  }

  const name = fm.name ?? path.basename(filePath, ".md");
  const description = fm.description ?? "";
  const tools = fm.tools ? fm.tools.split(/[,\s]+/).filter(Boolean) : undefined;
  const model = fm.model;

  return { name, description, systemPrompt: body, tools, model, filePath };
}

function loadAgentsFromDir(dir: string, source: AgentConfig["source"]): AgentConfig[] {
  if (!fs.existsSync(dir)) return [];
  const agents: AgentConfig[] = [];
  try {
    for (const file of fs.readdirSync(dir)) {
      if (!file.endsWith(".md")) continue;
      const filePath = path.join(dir, file);
      const parsed = parseAgentFile(filePath);
      if (parsed && parsed.name) {
        agents.push({ ...parsed, source });
      }
    }
  } catch {
    // ignore read errors
  }
  return agents;
}

/** Directory containing the bundled agent definitions */
export function getBundledAgentsDir(): string {
  // Support both ESM (__dirname via import.meta) and CJS
  try {
    const dir = path.dirname(fileURLToPath(import.meta.url));
    return path.join(dir, "agents");
  } catch {
    return path.join(__dirname, "agents");
  }
}

export interface AgentDiscovery {
  agents: AgentConfig[];
  bundledAgentsDir: string;
  projectAgentsDir: string | null;
  userAgentsDir: string | null;
}

/**
 * Discover agents based on scope.
 * Later entries in the returned list have higher priority (override earlier).
 * Deduplication keeps the last agent found with a given name.
 */
export function discoverAgents(cwd: string, scope: AgentScope = "all"): AgentDiscovery {
  const bundledDir = getBundledAgentsDir();
  const userDir = path.join(os.homedir(), ".pi", "agent", "agents");
  const projectDir = path.join(cwd, ".pi", "agents");

  const all: AgentConfig[] = [];

  // Bundled always loaded — lowest priority
  if (scope === "bundled" || scope === "all") {
    all.push(...loadAgentsFromDir(bundledDir, "bundled"));
  }

  // User agents
  if (scope === "user" || scope === "all") {
    all.push(...loadAgentsFromDir(userDir, "user"));
  }

  // Project agents
  if (scope === "project" || scope === "all") {
    all.push(...loadAgentsFromDir(projectDir, "project"));
  }

  // Deduplicate: last definition wins (project > user > bundled)
  const seen = new Map<string, AgentConfig>();
  for (const agent of all) {
    seen.set(agent.name, agent);
  }

  return {
    agents: Array.from(seen.values()),
    bundledAgentsDir: bundledDir,
    projectAgentsDir: fs.existsSync(projectDir) ? projectDir : null,
    userAgentsDir: fs.existsSync(userDir) ? userDir : null,
  };
}
