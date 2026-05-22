---
name: pi-bmo-code-quality
description: >
  Code quality review focused on naming, complexity, dead code, test quality, and
  project conventions. Complements staff-engineer review. Use when the user wants a
  dedicated quality pass on implementation changes — phrases like "code quality review",
  "check code quality", "review naming and complexity", "quality pass", "check
  conventions", "dead code check", or "review test quality". Independent of the dev-team
  workflow; can be invoked directly on any bmo issue.
---

# Code Quality Review

You spawn a `code-quality` subagent to perform a targeted quality review on a bmo issue.
The agent reviews naming, complexity, dead code, test coverage mechanics, and adherence to
project conventions, then posts findings as a bmo comment.

> **CRITICAL: Never commit any changes.**

## When to Use

Use for any bmo issue where implementation changes need a quality-focused review pass:
- Naming clarity (variables, functions, types)
- Function/method complexity or deep nesting
- Dead code (unused imports, variables, branches)
- Test quality (behavior vs. implementation testing)
- Adherence to existing codebase conventions
- Documentation of complex sections

## Workflow

1. Identify the relevant bmo issue ID — ask the user if unclear.
2. Spawn the agent via the `subagent` tool:

```
subagent(
  agent="code-quality",
  agentScope="all",
  task="""
Perform a code quality review for BMO-{ID}: {title}

- Call bmo_show(id={ID}) to understand the scope of changes
- Read bmo_comment(action="list", id={ID}) for existing context
- Read the modified files
- Evaluate: naming, complexity, dead code, test quality, conventions, documentation
- Post findings: bmo_comment(action="add", id={ID}, author="code-quality", body="Quality review: [findings]")
- Do NOT write implementation code. Do NOT commit.
"""
)
```

3. Report the agent's findings back to the user.

## Multi-Issue Quality Pass

To run quality reviews on multiple issues in parallel:

```
subagent(
  agentScope="all",
  tasks=[
    { agent: "code-quality", task: "Quality review for BMO-{ID1}: ..." },
    { agent: "code-quality", task: "Quality review for BMO-{ID2}: ..." },
  ]
)
```
