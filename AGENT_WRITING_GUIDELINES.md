# Agent Writing Guidelines

Principles for writing effective Claude Code agent definitions, synthesized from production system prompt analysis, Anthropic's prompting research, and empirical studies on LLM instruction following. Each recommendation cites its source.

Last refreshed 2026-08-21 against Claude 5-era guidance (Sonnet 5, Opus 5) — see #15–17 and the correction to #10 below for what changed since the original March 2026 pass.

---

## Structure

### 1. Open with a two-sentence identity statement

Every production coding agent (Cursor, Copilot, Windsurf, OpenHands) opens with exactly this: one sentence for what the agent *is*, one sentence for what it *does*. Not a paragraph — two sentences. The model needs an identity anchor before processing anything else.

> "You are a powerful agentic AI coding assistant, powered by Claude 3.5 Sonnet. You operate exclusively in Cursor, the world's best IDE."
>
> — Cursor agent prompt (March 2025)

### 2. Put hard constraints at the top or bottom — never in the middle

Information in the middle of a long context is the least reliably used. The "lost in the middle" effect is robust across GPT-3.5, GPT-4, Claude, and LLaMA-2. Hard constraints (NEVER write code, NEVER commit) belong in the first or last paragraph, not buried in a middle section.

> Source: [Lost in the Middle: How Language Models Use Long Contexts — TACL 2024](https://aclanthology.org/2024.tacl-1.9/)

### 3. Put the output format template at the very end

When a model is about to generate output, it uses what it read most recently. A filled-in output format template placed at the end of the prompt is implicitly the most available reference at generation time. Place it last.

> Source: [Lost in the Middle — TACL 2024](https://aclanthology.org/2024.tacl-1.9/); validated by structure in Cursor, Copilot, and OpenHands prompts

### 4. Encode the decision procedure as a numbered workflow, not a list of values

Agents given a goal and a list of values drift back to default LLM behavior between steps. Agents with an explicit numbered workflow (Copilot's 8-step, OpenHands' 5-step) have a fixed frame to re-orient to after each action. The workflow functions like a decision tree the agent always has access to.

> Source: [Deep Dive into GitHub Copilot Agent Mode Prompt Structure — DEV Community](https://dev.to/seiwan-maikuma/a-deep-dive-into-github-copilot-agent-modes-prompt-structure-2i4g); [Instructing Devin Effectively — Devin Docs](https://docs.devin.ai/essential-guidelines/instructing-devin-effectively); [Design Patterns for Securing LLM Agents against Prompt Injections — arXiv 2506.08837](https://arxiv.org/html/2506.08837v2)

### 5. Declare scope boundaries explicitly ("What You Are NOT")

Every mature production agent explicitly states what it is *not* responsible for. Without these declarations, the agent fills ambiguous territory by reverting to default LLM behavior. Scope boundaries are not negative instructions — they are domain definitions.

> Source: Cursor, Copilot, Windsurf, OpenHands prompts; [Writing a Good CLAUDE.md — HumanLayer Blog](https://www.humanlayer.dev/blog/writing-a-good-claude-md)

---

## Rhetoric

### 6. Use a purpose statement, not an expert persona

Do not open with "You are an expert software engineer" or similar role labels. Empirical research shows expert personas reduce accuracy on coding and factual tasks by 3–5%, with degradation increasing for longer, more detailed persona descriptions. The mechanism: persona framing activates an instruction-following mode that prioritizes *sounding* correct over *being* correct.

Purpose statements ("You review code for one quality measure: X") establish identity without triggering the accuracy-degrading persona mode.

> Source: [Expert Personas Improve LLM Alignment but Damage Accuracy (PRISM) — arXiv](https://arxiv.org/html/2603.18507); [Research: "You Are An Expert" Prompts Can Damage Factual Accuracy — Search Engine Journal](https://www.searchenginejournal.com/research-you-are-an-expert-prompts-can-damage-factual-accuracy/570397/)

### 7. Provide the WHY for every rule, not just the WHAT

When the model understands the motivation behind an instruction, it can generalize to edge cases correctly rather than pattern-matching on surface form. Anthropic's documentation is explicit on this: the model is "smart enough to generalize from the explanation."

> Less effective: `NEVER use ellipses`
>
> More effective: `Your response will be read aloud by a text-to-speech engine, so never use ellipses since the text-to-speech engine will not know how to pronounce them.`
>
> — Anthropic Prompting Best Practices

This principle applies equally to agent criteria: a criterion with a stated motivation is followed more consistently than a criterion stated as a bare rule.

> Source: [Prompting Best Practices — Anthropic/Claude API Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices)

### 8. Frame behavior positively; reserve NEVER/ALWAYS for true hard constraints

Negative instructions work through a "delayed effect" — they apply *after* the model has already committed to a direction during initial generation. Positive framing shapes behavior preemptively. Turn "do not do X" into "do Y instead" wherever a positive equivalent exists.

Reserve NEVER, ALWAYS, MUST, CRITICAL for genuinely non-negotiable constraints with no positive equivalent. Using capitalized emphasis throughout dilutes it to noise — Claude 4.x and 5.x models may overtrigger on it. **Confirmed still current (2026-08):** Anthropic's per-model Sonnet 5/Opus 5 prompting pages restate this explicitly — positive, concrete examples now outperform negative instructions as a documented model-specific finding, not just an inference from research literature.

> Source: [Understanding the Impact of Negative Prompts — arXiv 2406.02965](https://arxiv.org/abs/2406.02965); [AI Prompt Engineering in 2025: What Works and What Doesn't — Sander Schulhoff / Lenny's Newsletter](https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff); [Anthropic Prompting Best Practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices); [Prompting Claude Sonnet 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5)

### 9. Embed negative exemplars inside criteria, not as standalone prohibitions

Concrete negative examples ("Is this function doing I/O AND business logic AND formatting? Those are three things.") are most effective when placed *inside* the criterion they illustrate, at the moment the model is applying that criterion. Standalone prohibition lists at the end of a prompt are ignored more often than inline exemplars.

This is distinct from criteria #8 above: you *can* use negative examples, just co-locate them with the relevant positive guidance.

> Source: Few-shot prompting research in [AI Prompt Engineering in 2025 — Schulhoff / Lenny's Newsletter](https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff)

### 10. Place de-escalation guards inside the criterion they guard — but never as a blanket suppression instruction

If you want to prevent overfiring on a *specific* criterion ("do not flag length alone — flag length combined with complexity"), place that guard *inside* the criterion, not in a separate "false positives" section. The model encounters it at the moment it would otherwise fire, which is when it is most effective.

Research on Copilot code review found that excessive false positives cause reviewers to stop trusting the output — the same applies to agent output read by humans or other agents.

> Source: [Rethinking Code Review Workflows with LLM Assistance — arXiv 2505.16339](https://arxiv.org/html/2505.16339v1); [Unlocking the Full Power of Copilot Code Review — GitHub Blog](https://github.blog/ai-and-ml/unlocking-the-full-power-of-copilot-code-review-master-your-instructions-files/)

**Correction (2026-08, Claude 5 finding):** a scoped, per-criterion guard like the one above is still correct. But a *global* suppression instruction — "only report high-severity issues," "be conservative," "focus on the 20% carrying 80% of the risk" — is now empirically risky: Sonnet 5 and Opus 5 follow it literally and under-report as a result, rather than treating it as a soft steer. Don't write review/critic agents that are told to hold back at generation time. Instead, ask for full-coverage output and triage or filter severity as a separate downstream step (a distinct field or section in the output template, not a suppressed finding). If a criterion needs a false-positive guard, scope it narrowly to that one criterion (the original guidance above) rather than to the whole review.

> Source: [Prompting Claude Sonnet 5 / Opus 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5)

### 11. Use a filled-in output template, not a description of the desired output

A filled-in markdown template showing exactly what the output should look like (with placeholder text in each field) is implicit one-shot prompting for format. It is dramatically more effective than describing what the output should contain. Every production review agent (Copilot, staff-reviewer, code-quality) uses this pattern.

> Source: [AI Prompt Engineering in 2025: few-shot prompting section — Schulhoff / Lenny's Newsletter](https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff); validated across all production agent prompts

---

## Length and Density

### 12. Keep instruction sets short — context rot is real and starts early

Performance degrades meaningfully at 500–2,500 tokens of instruction content. At 5,000+ tokens: severe degradation, hallucinations, task refusals. Critically, the model does not prioritize — it degrades *uniformly* across all instructions as length increases.

| Range | Effect |
|---|---|
| 50–250 tokens | Models perform well |
| 500–2,500 tokens | Noticeable degradation on semantic tasks |
| 5,000+ tokens | Severe degradation, frequent hallucinations |

Counterintuitive finding: models sometimes perform better on shuffled instructions than on logically structured ones in long-context settings, likely because coherent narrative flow induces "middle" placement for critical instructions.

> Source: [Context Rot — Chroma Research](https://www.trychroma.com/research/context-rot); [Lost in the Middle — TACL 2024](https://aclanthology.org/2024.tacl-1.9/)

### 13. Include only non-inferable instructions

ETH Zurich research on AGENTS.md/context files found:
- **LLM-generated context files**: reduced task success 3%, raised inference costs 20%+
- **Human-written context files**: improved task success 4%, but also raised costs 19%

The cost increase happened because agents followed the instructions and conducted unnecessary work. The practical rule: include only information the model cannot derive from the code itself or its pretraining — specialized tooling, custom build commands, domain-specific conventions. Do not include architectural overviews, file maps, or patterns inferable from reading the code.

> Source: [New Research Reassesses the Value of AGENTS.md Files — InfoQ](https://www.infoq.com/news/2026/03/agents-context-file-value-review/)

### 14. Frontier models need fewer hedges and caveats

For Claude 4.x and equivalent models, Anthropic explicitly warns: where you might have written "CRITICAL: You MUST use this tool when...", write "Use this tool when..." Aggressive emphasis that improved compliance in older models causes overtriggering in newer ones. **Confirmed still current (2026-08)** — the guidance survives verbatim on the Sonnet 5/Opus 5-specific pages: "Where you might have said 'CRITICAL: You MUST use this tool when...', you can use more normal prompting."

> Source: [Prompting Best Practices — Anthropic/Claude API Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices); [Prompting Claude Sonnet 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5)

---

## Model-Specific (Claude 5, added 2026-08)

### 15. Prompt against the model-specific page, not just the general guide

Anthropic now publishes prompting pages per model (Sonnet 5, Opus 5, Haiku 4.5, Fable 5, Mythos 5), not just one generic "Claude" guide. The general guide (#6–14 above) still holds, but check the model-specific page for the model an agent actually runs as — it can contain guidance the general page doesn't, as #10's correction and #16 below show.

> Source: [Prompting Claude Sonnet 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5); [Prompting Claude Opus 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5)

### 16. Don't tell Opus 5 to verify — it already over-verifies and over-delegates

Opus 5's default tendency is to delegate more and double-check more than earlier models. Anthropic's own guidance: "If your prompt contains explicit verification instructions ('include a final verification step', 'use a subagent to verify'), remove them," and "do not delegate work you can finish yourself in a handful of tool calls." An agent prompt that adds its own "always verify your work" or "spawn a subagent to double-check" instruction compounds a bias the model already has, rather than correcting a gap. Only spell out a verification step where the model genuinely can't infer it should happen — e.g. a domain-specific check with no obvious trigger.

> Source: [Prompting Claude Opus 5 — Anthropic](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5)

### 17. Judge multi-agent topology by what terminates the loop, not by agent count

Anthropic's multi-agent guidance (Jan 2026) recommends 3–5 agents, or the newer "agent teams" primitive (shared task list, direct inter-agent messaging), over many agents split by work type. It warns that splitting by role — one agent writes, one tests, one reviews — adds handoff overhead ("handoffs lose context") and costs 3–10x more tokens than a single agent or a small team.

Three things to get right before applying that warning to a design like this repo's.

**Count agents active per run, not files in a directory.** `agents/` holds eight definitions, but the Medium pattern runs architect → PM → SE → code-quality → reviewer → QA, and @ux-designer, @documentation-writer, and @distributed-systems-expert are conditional — they cost nothing on runs that don't invoke them. That is inside the 3–5 recommendation, not outside it. An earlier version of this entry described the repo as an "8-agent design" in tension with the guidance; that framing was wrong on the facts.

**Separate context isolation from role specialization.** The reason to keep subagents at all is that a large body of work degrades inside one context window, and each subagent starts fresh on a single scoped issue. Anthropic's subagent docs endorse isolation for exactly this. The 3–10x warning targets role *specialization* — a second agent re-reading an artifact a first agent produced — which is a separate property that most designs happen to bundle with isolation. The token multiple comes from cold starts re-deriving context, not from the count of role definitions, so the cheap fix is inlining context into spawn prompts rather than deleting roles.

**The axis that predicts whether a loop pays for itself: does it terminate on a verifier or on a judgment?** The multi-agent systems with the strongest published results put an oracle at the end. Jitskit refines synthesized systems against an evolving executable test suite and beats state-of-the-art on 18/18 specs. Microsoft's MDASH orchestrates 100+ agents through scan → debate → validate → dedupe → exploit and tops CyberGym at 88.45% — its debate stage is affordable precisely because a later stage settles the debate by proving the exploit fires. Both also split by *pipeline stage over one artifact* (every agent sees the same code) rather than by *job title over a project*, which is what makes their handoffs cheap.

A review step with no oracle terminates when a model changes its mind, which is where run-to-run variance and lost findings come from. The practical consequence for agent design: put every mechanically checkable property behind a real command (`just ci`, a test suite, a linter) run as a gate, and reserve model review for what no command can check. Restructuring team topology is a larger decision than any single agent-file audit should make — but wiring in the verifier you already have is not, and it is the higher-value change.

> Source: [Building Multi-Agent Systems: When and How to Use Them — Anthropic](https://claude.com/blog/building-multi-agent-systems-when-and-how-to-use-them); [Subagents — Claude Code Docs](https://code.claude.com/docs/en/sub-agents) (subagents share no context — each starts fresh with its own system prompt, no conversation history — reinforcing #5's scope-boundary rule); [Agent Teams (experimental) — Claude Code Docs](https://code.claude.com/docs/en/agent-teams); [The Time is Here for Just-in-Time Systems — arXiv 2605.24096](https://arxiv.org/abs/2605.24096); [Microsoft MDASH — Microsoft Security Blog](https://www.microsoft.com/en-us/security/blog/2026/05/12/defense-at-ai-speed-microsofts-new-multi-model-agentic-security-system-tops-leading-industry-benchmark/)

---

## What Doesn't Work (Despite Being Common)

| Pattern | Why it fails | Source |
|---|---|---|
| `"You are an expert [role]"` | Reduces factual/coding accuracy 3–5%; worse for longer personas | [PRISM — arXiv](https://arxiv.org/html/2603.18507) |
| `"Think step by step"` in system prompt | 2–3% gain, 20–80% latency cost; reasoning models gain nothing | [Wharton Generative AI Labs](https://gail.wharton.upenn.edu/research-and-insights/tech-report-chain-of-thought/) |
| Dense standalone negative instruction lists | Models ignore or bypass at scale; delayed-effect mechanism means they arrive too late | [arXiv 2406.02965](https://arxiv.org/abs/2406.02965) |
| Architectural overviews in context files | Doesn't reduce file-finding time; wastes token budget | [ETH Zurich via InfoQ](https://www.infoq.com/news/2026/03/agents-context-file-value-review/) |
| `"Ensure quality"` / `"be thorough"` language | Too vague; causes overtriggering on Claude 4.x | [Anthropic Prompting Best Practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices) |
| Emotional/motivational framing | No production deployment uses it; confirmed ineffective in Windsurf R&D experiment | [Simon Willison on leaked Windsurf prompt](https://simonwillison.net/2025/Feb/25/leaked-windsurf-prompt/) |
| CAPITALIZATION used throughout for emphasis | Dilutes the signal; newer models overtrigger | [Anthropic Prompting Best Practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices) |

---

## Checklist for New Agent Definitions

- [ ] Opens with a two-sentence identity statement (not a persona label)
- [ ] Hard constraints appear in the first or last section, not the middle
- [ ] Scope bounds declared: "What You Are NOT"
- [ ] Decision procedure encoded as a numbered workflow
- [ ] Every rule includes its WHY (motivation)
- [ ] Behavioral guidance is positively framed; NEVER/ALWAYS used ≤3 times total
- [ ] Negative exemplars are embedded inside criteria, not in a standalone list
- [ ] De-escalation guards are scoped to one criterion, never a blanket "only report X" / "be conservative" instruction (#10)
- [ ] Output format is a filled-in template, placed last
- [ ] Total instruction length is under ~1,000 words
- [ ] No architectural overviews, file maps, or inferable patterns
- [ ] No explicit "verify your work" / "spawn a subagent to verify" instructions unless the check is genuinely non-obvious (#16)

---

## Sources

| Document | URL |
|---|---|
| Cursor Agent System Prompt (March 2025) | https://gist.github.com/sshh12/25ad2e40529b269a88b80e7cf1c38084 |
| Cursor IDE System Prompt (December 2024) | https://github.com/jujumilk3/leaked-system-prompts/blob/main/cursor-ide-sonnet_20241224.md |
| GitHub Copilot Chat System Prompt (September 2024) | https://github.com/jujumilk3/leaked-system-prompts/blob/main/github-copilot-chat_20240930.md |
| Deep Dive into GitHub Copilot Agent Mode Prompt Structure | https://dev.to/seiwan-maikuma/a-deep-dive-into-github-copilot-agent-modes-prompt-structure-2i4g |
| Unlocking the Full Power of Copilot Code Review | https://github.blog/ai-and-ml/unlocking-the-full-power-of-copilot-code-review-master-your-instructions-files/ |
| Windsurf Cascade System Prompt (December 2024) | https://github.com/jujumilk3/leaked-system-prompts/blob/main/codeium-windsurf-cascade_20241206.md |
| Simon Willison on leaked Windsurf R&D prompt | https://simonwillison.net/2025/Feb/25/leaked-windsurf-prompt/ |
| Claude Code System Prompts Repository | https://github.com/Piebald-AI/claude-code-system-prompts |
| OpenHands CodeAct Agent System Prompt | https://github.com/All-Hands-AI/OpenHands/blob/main/openhands/agenthub/codeact_agent/prompts/system_prompt.j2 |
| Anthropic Prompting Best Practices | https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices |
| Prompting Claude Sonnet 5 — Anthropic (added 2026-08) | https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5 |
| Prompting Claude Opus 5 — Anthropic (added 2026-08) | https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5 |
| Building Multi-Agent Systems: When and How to Use Them — Anthropic (Jan 2026, added 2026-08) | https://claude.com/blog/building-multi-agent-systems-when-and-how-to-use-them |
| Subagents — Claude Code Docs (added 2026-08) | https://code.claude.com/docs/en/sub-agents |
| Agent Teams (experimental) — Claude Code Docs (added 2026-08) | https://code.claude.com/docs/en/agent-teams |
| The Time is Here for Just-in-Time Systems (added 2026-08) | https://arxiv.org/abs/2605.24096 |
| Microsoft MDASH: multi-model agentic security system (added 2026-08) | https://www.microsoft.com/en-us/security/blog/2026/05/12/defense-at-ai-speed-microsofts-new-multi-model-agentic-security-system-tops-leading-industry-benchmark/ |
| Expert Personas Improve LLM Alignment but Damage Accuracy (PRISM) | https://arxiv.org/html/2603.18507 |
| Understanding the Impact of Negative Prompts | https://arxiv.org/abs/2406.02965 |
| Lost in the Middle: How Language Models Use Long Contexts (TACL 2024) | https://aclanthology.org/2024.tacl-1.9/ |
| Context Rot — Chroma Research | https://www.trychroma.com/research/context-rot |
| The Decreasing Value of Chain of Thought in Prompting — Wharton | https://gail.wharton.upenn.edu/research-and-insights/tech-report-chain-of-thought/ |
| New Research Reassesses the Value of AGENTS.md Files — InfoQ | https://www.infoq.com/news/2026/03/agents-context-file-value-review/ |
| Writing a Good CLAUDE.md — HumanLayer Blog | https://www.humanlayer.dev/blog/writing-a-good-claude-md |
| Rethinking Code Review Workflows with LLM Assistance | https://arxiv.org/html/2505.16339v1 |
| Design Patterns for Securing LLM Agents against Prompt Injections | https://arxiv.org/html/2506.08837v2 |
| AI Prompt Engineering in 2025 — Sander Schulhoff / Lenny's Newsletter | https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff |
| Instructing Devin Effectively — Devin Docs | https://docs.devin.ai/essential-guidelines/instructing-devin-effectively |
