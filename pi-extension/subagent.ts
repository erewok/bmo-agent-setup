/**
 * Subagent tool for the bmo dev-team extension.
 *
 * Spawns isolated `pi` subprocesses for each agent invocation.
 * Adapted from the pi subagent example with the following changes:
 *   - Default agent scope is "all" (includes bundled bmo agents)
 *   - Agents are auto-discovered from this extension's agents/ directory
 *   - Simplified rendering focused on dev-team use cases
 *
 * Supports three modes:
 *   - single:   { agent, task }
 *   - parallel: { tasks: [{agent, task}, ...] }
 *   - chain:    { chain: [{agent, task}, ...] }  (with {previous} placeholder)
 */

import { spawn } from "node:child_process";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { withFileMutationQueue } from "@earendil-works/pi-coding-agent";
import { StringEnum } from "@earendil-works/pi-ai";
import { Text } from "@earendil-works/pi-tui";
import { Type } from "typebox";
import { type AgentConfig, type AgentScope, discoverAgents } from "./agents.ts";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface UsageStats {
  input: number;
  output: number;
  cacheRead: number;
  cacheWrite: number;
  cost: number;
  contextTokens: number;
  turns: number;
}

interface SingleResult {
  agent: string;
  agentSource: "bundled" | "user" | "project" | "unknown";
  task: string;
  exitCode: number;
  finalOutput: string;
  stderr: string;
  usage: UsageStats;
  model?: string;
  stopReason?: string;
  errorMessage?: string;
  step?: number;
}

interface SubagentDetails {
  mode: "single" | "parallel" | "chain";
  results: SingleResult[];
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const MAX_PARALLEL = 8;
const MAX_CONCURRENCY = 4;
const PER_TASK_CAP = 50 * 1024;

function emptyUsage(): UsageStats {
  return { input: 0, output: 0, cacheRead: 0, cacheWrite: 0, cost: 0, contextTokens: 0, turns: 0 };
}

function isFailure(r: SingleResult): boolean {
  return r.exitCode !== 0 || r.stopReason === "error" || r.stopReason === "aborted";
}

function truncate(text: string): string {
  const bytes = Buffer.byteLength(text, "utf8");
  if (bytes <= PER_TASK_CAP) return text;
  let t = text.slice(0, PER_TASK_CAP);
  while (Buffer.byteLength(t, "utf8") > PER_TASK_CAP) t = t.slice(0, -1);
  return `${t}\n[…output truncated]`;
}

async function writeTempPrompt(agentName: string, prompt: string): Promise<{ dir: string; file: string }> {
  const tmpDir = await fs.promises.mkdtemp(path.join(os.tmpdir(), "pi-bmo-"));
  const safe = agentName.replace(/[^\w.-]+/g, "_");
  const file = path.join(tmpDir, `prompt-${safe}.md`);
  await withFileMutationQueue(file, () => fs.promises.writeFile(file, prompt, { encoding: "utf-8", mode: 0o600 }));
  return { dir: tmpDir, file };
}

function getPiInvocation(args: string[]): { cmd: string; args: string[] } {
  const script = process.argv[1];
  const isBun = script?.startsWith("/$bunfs/root/");
  if (script && !isBun && fs.existsSync(script)) return { cmd: process.execPath, args: [script, ...args] };
  const exec = path.basename(process.execPath).toLowerCase();
  if (/^(node|bun)(\.exe)?$/.test(exec)) return { cmd: "pi", args };
  return { cmd: process.execPath, args };
}

// ---------------------------------------------------------------------------
// Core runner
// ---------------------------------------------------------------------------

async function runAgent(
  cwd: string,
  agents: AgentConfig[],
  agentName: string,
  task: string,
  agentCwd: string | undefined,
  step: number | undefined,
  signal: AbortSignal | undefined,
  onUpdate: ((r: SingleResult) => void) | undefined,
): Promise<SingleResult> {
  const agent = agents.find((a) => a.name === agentName);
  if (!agent) {
    const available = agents.map((a) => a.name).join(", ") || "none";
    return {
      agent: agentName,
      agentSource: "unknown",
      task,
      exitCode: 1,
      finalOutput: "",
      stderr: `Unknown agent "${agentName}". Available: ${available}`,
      usage: emptyUsage(),
      step,
    };
  }

  const result: SingleResult = {
    agent: agentName,
    agentSource: agent.source,
    task,
    exitCode: 0,
    finalOutput: "",
    stderr: "",
    usage: emptyUsage(),
    model: agent.model,
    step,
  };

  const args: string[] = ["--mode", "json", "-p", "--no-session"];
  if (agent.model) args.push("--model", agent.model);
  if (agent.tools?.length) args.push("--tools", agent.tools.join(","));

  let tmpDir: string | null = null;
  let tmpFile: string | null = null;

  try {
    if (agent.systemPrompt.trim()) {
      ({ dir: tmpDir, file: tmpFile } = await writeTempPrompt(agent.name, agent.systemPrompt));
      args.push("--append-system-prompt", tmpFile);
    }
    args.push(`Task: ${task}`);

    let wasAborted = false;
    const exitCode = await new Promise<number>((resolve) => {
      const { cmd, args: piArgs } = getPiInvocation(args);
      const proc = spawn(cmd, piArgs, {
        cwd: agentCwd ?? cwd,
        shell: false,
        stdio: ["ignore", "pipe", "pipe"],
      });

      let buffer = "";
      const processLine = (line: string) => {
        if (!line.trim()) return;
        let event: any;
        try { event = JSON.parse(line); } catch { return; }

        if (event.type === "message_end" && event.message) {
          const msg = event.message;
          if (msg.role === "assistant") {
            result.usage.turns++;
            const u = msg.usage;
            if (u) {
              result.usage.input += u.input || 0;
              result.usage.output += u.output || 0;
              result.usage.cacheRead += u.cacheRead || 0;
              result.usage.cacheWrite += u.cacheWrite || 0;
              result.usage.cost += u.cost?.total || 0;
              result.usage.contextTokens = u.totalTokens || 0;
            }
            if (!result.model && msg.model) result.model = msg.model;
            if (msg.stopReason) result.stopReason = msg.stopReason;
            if (msg.errorMessage) result.errorMessage = msg.errorMessage;
            // Extract final text
            for (const part of msg.content ?? []) {
              if (part.type === "text") result.finalOutput = part.text;
            }
          }
          onUpdate?.(result);
        }
      };

      proc.stdout.on("data", (data: Buffer) => {
        buffer += data.toString();
        const lines = buffer.split("\n");
        buffer = lines.pop() ?? "";
        for (const line of lines) processLine(line);
      });
      proc.stderr.on("data", (data: Buffer) => { result.stderr += data.toString(); });
      proc.on("close", (code) => {
        if (buffer.trim()) processLine(buffer);
        resolve(code ?? 0);
      });
      proc.on("error", () => resolve(1));

      if (signal) {
        const kill = () => { wasAborted = true; proc.kill("SIGTERM"); setTimeout(() => { if (!proc.killed) proc.kill("SIGKILL"); }, 5000); };
        if (signal.aborted) kill();
        else signal.addEventListener("abort", kill, { once: true });
      }
    });

    result.exitCode = exitCode;
    if (wasAborted) throw new Error("Aborted");
    return result;
  } finally {
    if (tmpFile) try { fs.unlinkSync(tmpFile); } catch { /* ignore */ }
    if (tmpDir) try { fs.rmdirSync(tmpDir); } catch { /* ignore */ }
  }
}

async function mapConcurrent<T, U>(items: T[], limit: number, fn: (item: T, i: number) => Promise<U>): Promise<U[]> {
  const results: U[] = new Array(items.length);
  let next = 0;
  await Promise.all(new Array(Math.min(limit, items.length)).fill(null).map(async () => {
    while (true) {
      const i = next++;
      if (i >= items.length) return;
      results[i] = await fn(items[i], i);
    }
  }));
  return results;
}

// ---------------------------------------------------------------------------
// Tool registration
// ---------------------------------------------------------------------------

const TaskItem = Type.Object({
  agent: Type.String({ description: "Agent name" }),
  task: Type.String({ description: "Task description" }),
  cwd: Type.Optional(Type.String({ description: "Working directory override" })),
});

export function registerSubagentTool(pi: ExtensionAPI): void {
  pi.registerTool({
    name: "subagent",
    label: "Subagent",
    description: [
      "Delegate tasks to specialized bmo dev-team agents in isolated contexts.",
      "Modes: single {agent, task}, parallel {tasks:[{agent,task}...]}, chain {chain:[{agent,task}...]} with {previous} placeholder.",
      "Bundled agents: staff-engineer, project-manager, senior-engineer, qa-engineer, ux-designer, code-quality, documentation-writer.",
      'Set agentScope to "all" (default) to also pick up user/project overrides.',
    ].join(" "),
    promptSnippet: "Spawn a dev-team agent (single, parallel, or chained)",
    promptGuidelines: [
      "Use subagent to spawn staff-engineer, project-manager, senior-engineer, qa-engineer, ux-designer, code-quality, or documentation-writer agents.",
      "For the dev-team workflow, follow the orchestration pattern in the dev-team skill: PM first, then parallel SE agents, then review, then QA.",
    ],
    parameters: Type.Object({
      agent: Type.Optional(Type.String({ description: "Agent name (single mode)" })),
      task: Type.Optional(Type.String({ description: "Task text (single mode)" })),
      tasks: Type.Optional(Type.Array(TaskItem, { description: "Parallel tasks" })),
      chain: Type.Optional(Type.Array(TaskItem, { description: "Sequential chain (use {previous} for prior output)" })),
      agentScope: Type.Optional(
        StringEnum(["bundled", "user", "project", "all"] as const, {
          description: 'Agent discovery scope. Default: "all" (bundled + user + project, with project taking priority).',
          default: "all",
        }),
      ),
      cwd: Type.Optional(Type.String({ description: "Working directory for the subprocess (single mode)" })),
    }),

    async execute(_toolCallId, params, signal, onUpdate, ctx) {
      const scope: AgentScope = (params.agentScope as AgentScope) ?? "all";
      const { agents } = discoverAgents(ctx.cwd, scope);

      const isSingle = Boolean(params.agent && params.task);
      const isParallel = (params.tasks?.length ?? 0) > 0;
      const isChain = (params.chain?.length ?? 0) > 0;
      const modeCount = Number(isSingle) + Number(isParallel) + Number(isChain);

      const makeResult = (mode: SubagentDetails["mode"], results: SingleResult[]) => ({
        content: [{ type: "text" as const, text: results.map((r) => `[${r.agent}] ${r.finalOutput || r.stderr || "(no output)"}`).join("\n\n") }],
        details: { mode, results } as SubagentDetails,
      });

      if (modeCount !== 1) {
        return { content: [{ type: "text", text: "Provide exactly one mode: {agent,task}, {tasks:[...]}, or {chain:[...]}." }], details: { mode: "single", results: [] } as SubagentDetails };
      }

      // ---- CHAIN ----
      if (params.chain && params.chain.length > 0) {
        const results: SingleResult[] = [];
        let prev = "";
        for (let i = 0; i < params.chain.length; i++) {
          const step = params.chain[i];
          const task = step.task.replace(/\{previous\}/g, prev);
          const result = await runAgent(ctx.cwd, agents, step.agent, task, step.cwd, i + 1, signal, (r) => {
            onUpdate?.({ content: [{ type: "text", text: `Chain step ${i + 1}/${params.chain!.length} (${step.agent}): ${r.finalOutput || "running..."}` }], details: { mode: "chain", results: [...results, r] } });
          });
          results.push(result);
          if (isFailure(result)) {
            return { content: [{ type: "text", text: `Chain failed at step ${i + 1} (${step.agent}): ${result.errorMessage || result.stderr || result.finalOutput}` }], details: { mode: "chain", results }, isError: true };
          }
          prev = result.finalOutput;
        }
        return makeResult("chain", results);
      }

      // ---- PARALLEL ----
      if (params.tasks && params.tasks.length > 0) {
        if (params.tasks.length > MAX_PARALLEL) {
          return { content: [{ type: "text", text: `Too many parallel tasks (max ${MAX_PARALLEL})` }], details: { mode: "parallel", results: [] } as SubagentDetails, isError: true };
        }
        const placeholders: SingleResult[] = params.tasks.map((t) => ({
          agent: t.agent, agentSource: "unknown" as const, task: t.task, exitCode: -1,
          finalOutput: "", stderr: "", usage: emptyUsage(),
        }));
        const emitParallel = () => {
          const done = placeholders.filter((r) => r.exitCode !== -1).length;
          const running = placeholders.length - done;
          onUpdate?.({ content: [{ type: "text", text: `Parallel: ${done}/${placeholders.length} done, ${running} running` }], details: { mode: "parallel", results: [...placeholders] } });
        };
        const results = await mapConcurrent(params.tasks, MAX_CONCURRENCY, async (t, i) => {
          const result = await runAgent(ctx.cwd, agents, t.agent, t.task, t.cwd, undefined, signal, (r) => {
            placeholders[i] = r;
            emitParallel();
          });
          placeholders[i] = result;
          emitParallel();
          return result;
        });
        const ok = results.filter((r) => !isFailure(r)).length;
        const summaries = results.map((r) => `### ${r.agent} (${isFailure(r) ? "failed" : "ok"})\n\n${truncate(r.finalOutput || r.stderr || "(no output)")}`);
        return { content: [{ type: "text", text: `Parallel: ${ok}/${results.length} succeeded\n\n${summaries.join("\n\n---\n\n")}` }], details: { mode: "parallel", results } };
      }

      // ---- SINGLE ----
      const result = await runAgent(ctx.cwd, agents, params.agent!, params.task!, params.cwd, undefined, signal, (r) => {
        onUpdate?.({ content: [{ type: "text", text: r.finalOutput || "running..." }], details: { mode: "single", results: [r] } });
      });
      if (isFailure(result)) {
        return { content: [{ type: "text", text: `${result.agent} failed: ${result.errorMessage || result.stderr || result.finalOutput}` }], details: { mode: "single", results: [result] }, isError: true };
      }
      return { content: [{ type: "text", text: result.finalOutput || "(no output)" }], details: { mode: "single", results: [result] } };
    },

    renderCall(args, theme) {
      if (args.chain?.length) {
        const names = (args.chain as {agent: string}[]).map((s) => s.agent).join(" → ");
        return new Text(`${theme.fg("toolTitle", theme.bold("subagent "))}${theme.fg("accent", `chain: ${names}`)}`, 0, 0);
      }
      if (args.tasks?.length) {
        const names = (args.tasks as {agent: string}[]).map((t) => t.agent).join(", ");
        return new Text(`${theme.fg("toolTitle", theme.bold("subagent "))}${theme.fg("accent", `parallel: ${names}`)}`, 0, 0);
      }
      const preview = (args.task as string ?? "").slice(0, 60);
      return new Text(`${theme.fg("toolTitle", theme.bold("subagent "))}${theme.fg("accent", args.agent ?? "?")}${theme.fg("dim", ` ${preview}`)}`, 0, 0);
    },

    renderResult(result, _opts, theme) {
      const details = result.details as SubagentDetails | undefined;
      if (!details?.results.length) return new Text(result.content[0]?.type === "text" ? (result.content[0] as {text: string}).text : "(no output)", 0, 0);
      const lines = details.results.map((r) => {
        const icon = r.exitCode === -1 ? theme.fg("warning", "⏳") : isFailure(r) ? theme.fg("error", "✗") : theme.fg("success", "✓");
        const preview = (r.finalOutput || r.stderr || "").split("\n")[0]?.slice(0, 60) ?? "";
        return `${icon} ${theme.fg("accent", r.agent)}${theme.fg("dim", ` ${preview}`)}`;
      });
      return new Text(lines.join("\n"), 0, 0);
    },
  });
}
