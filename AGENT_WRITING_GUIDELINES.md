# Agent Writing Guidelines

Principles for writing effective Claude Code agent definitions, synthesized from production system prompt analysis, Anthropic's prompting research, and empirical studies on LLM instruction following. Each recommendation cites its source.

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

Reserve NEVER, ALWAYS, MUST, CRITICAL for genuinely non-negotiable constraints with no positive equivalent. Using capitalized emphasis throughout dilutes it to noise — newer Claude models (4.x) may even overtrigger on it.

> Source: [Understanding the Impact of Negative Prompts — arXiv 2406.02965](https://arxiv.org/abs/2406.02965); [AI Prompt Engineering in 2025: What Works and What Doesn't — Sander Schulhoff / Lenny's Newsletter](https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff); [Anthropic Prompting Best Practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices)

### 9. Embed negative exemplars inside criteria, not as standalone prohibitions

Concrete negative examples ("Is this function doing I/O AND business logic AND formatting? Those are three things.") are most effective when placed *inside* the criterion they illustrate, at the moment the model is applying that criterion. Standalone prohibition lists at the end of a prompt are ignored more often than inline exemplars.

This is distinct from criteria #8 above: you *can* use negative examples, just co-locate them with the relevant positive guidance.

> Source: Few-shot prompting research in [AI Prompt Engineering in 2025 — Schulhoff / Lenny's Newsletter](https://www.lennysnewsletter.com/p/ai-prompt-engineering-in-2025-sander-schulhoff)

### 10. Place de-escalation guards inside the criterion they guard

If you want to prevent overfiring on a criterion ("do not flag length alone — flag length combined with complexity"), place that guard *inside* the length criterion, not in a separate "false positives" section. The model encounters it at the moment it would otherwise fire, which is when it is most effective.

Research on Copilot code review found that excessive false positives cause reviewers to stop trusting the output — the same applies to agent output read by humans or other agents.

> Source: [Rethinking Code Review Workflows with LLM Assistance — arXiv 2505.16339](https://arxiv.org/html/2505.16339v1); [Unlocking the Full Power of Copilot Code Review — GitHub Blog](https://github.blog/ai-and-ml/unlocking-the-full-power-of-copilot-code-review-master-your-instructions-files/)

### 11. Use a filled-in output template, not a description of the desired output

A filled-in markdown template showing exactly what the output should look like (with placeholder text in each field) is implicit one-shot prompting for format. It is dramatically more effective than describing what the output should contain. Every production review agent (Copilot, staff-engineer, code-quality) uses this pattern.

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

For Claude 4.x and equivalent models, Anthropic explicitly warns: where you might have written "CRITICAL: You MUST use this tool when...", write "Use this tool when..." Aggressive emphasis that improved compliance in older models causes overtriggering in newer ones.

> Source: [Prompting Best Practices — Anthropic/Claude API Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices)

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
- [ ] De-escalation guards sit inside the criterion they guard
- [ ] Output format is a filled-in template, placed last
- [ ] Total instruction length is under ~1,000 words
- [ ] No architectural overviews, file maps, or inferable patterns

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
