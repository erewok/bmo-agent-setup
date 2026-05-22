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
import { fileURLToPath } from "node:url";
import { getAgentDir, parseFrontmatter } from "@earendil-works/pi-coding-agent";

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

  const { frontmatter: fm, body } = parseFrontmatter<Record<string, string>>(content);

  // If no frontmatter, fall back to filename as name
  const name = fm.name ?? path.basename(filePath, ".md");
  if (!name) return null;

  const description = fm.description ?? "";
  const tools = fm.tools ? fm.tools.split(/[,\s]+/).filter(Boolean) : undefined;
  const model = fm.model;

  return { name, description, systemPrompt: body.trim(), tools, model, filePath };
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
  const userDir = path.join(getAgentDir(), "agents");
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
