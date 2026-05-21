/**
 * bmo-dev-team pi extension
 *
 * Provides the bmo-aware dev-team agent suite for pi-code:
 *   - `subagent` tool for spawning staff-engineer, project-manager,
 *     senior-engineer, qa-engineer, ux-designer, code-quality, and
 *     documentation-writer agents in isolated pi subprocesses
 *   - Skills: dev-team orchestration workflow, documentation-driver
 *
 * The bmo_* tools (bmo_create, bmo_show, bmo_comment, etc.) are provided
 * by the pi-bmo extension, which this package bundles as a dependency.
 * You do not need to install pi-bmo separately.
 *
 * Install:
 *   pi install git:github.com/erewok/bmo-agent-setup
 *
 * Or for local dev, add both to ~/.pi/agent/settings.json:
 *   {
 *     "extensions": [
 *       "/path/to/bmo/pi-extension",
 *       "/path/to/bmo-agent-setup/pi-extension"
 *     ]
 *   }
 */

import * as path from "node:path";
import { fileURLToPath } from "node:url";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { registerSubagentTool } from "./subagent.ts";

function extensionDir(): string {
  try {
    return path.dirname(fileURLToPath(import.meta.url));
  } catch {
    return __dirname;
  }
}

export default function (pi: ExtensionAPI): void {
  const extDir = extensionDir();

  // ── Subagent tool ──────────────────────────────────────────────────────
  // Spawns isolated pi subprocesses for each dev-team agent invocation.
  // Agents are discovered from this extension's agents/ directory, with
  // project-local (.pi/agents/) and user-level (~/.pi/agent/agents/)
  // overrides taking higher priority.
  registerSubagentTool(pi);

  // ── Skills auto-discovery ──────────────────────────────────────────────
  // Contributes this extension's skills/ directory to pi so that
  // dev-team and documentation-driver are available in every project
  // where this extension is loaded.
  pi.on("resources_discover", async () => ({
    skillPaths: [path.join(extDir, "skills")],
  }));
}
