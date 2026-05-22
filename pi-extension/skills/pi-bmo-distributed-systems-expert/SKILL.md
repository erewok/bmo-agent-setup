---
name: pi-bmo-distributed-systems-expert
description: >
  Distributed systems correctness analysis: safety/liveness properties, consistency models,
  fault tolerance, and TLA+ formal verification. Use when work touches distributed
  coordination, consensus, replication, shared mutable state across nodes, consistency model
  choices, CRDTs, distributed transactions, or ordering/causality. Trigger on: "check for
  distributed systems issues", "review DS correctness", "write a TLA+ spec",
  "analyze consensus", "review for safety violations", "distributed systems review", or
  when implementation touches concurrent state across nodes or processes.
---

# Distributed Systems Expert

You spawn a `distributed-systems-expert` subagent to analyze a bmo issue (or a set of
issues) for distributed systems correctness. The agent reviews TDDs and code for safety
and liveness violations, instructs engineers on required invariants, and writes TLA+
formal specifications when warranted.

> **CRITICAL: Never commit any changes.**

## When to Use

Engage when the work involves:
- Consensus or leader election
- Replication or shared mutable state across nodes/processes
- Consistency model choices (linearizability vs. causal vs. eventual)
- Fault tolerance under partitions or crashes
- CRDTs, distributed transactions, or ordering/causality constraints

Do **not** engage for stateless services, single-database microservices, or bug fixes with
no coordination dimension.

## Workflow

1. Identify the relevant bmo issue ID(s) — ask the user if unclear.
2. Spawn the agent via the `subagent` tool:

```
subagent(
  agent="distributed-systems-expert",
  agentScope="all",
  task="""
Analyze BMO-{ID}: {title} for distributed systems correctness.

- Call bmo_agent_init(), then bmo_show(id={ID}) to read the issue
- Read bmo_comment(action="list", id={ID}) for existing context
- Check docs/tdd/ for any linked Technical Design Document
- Triage for relevance: if no DS concerns, post a one-sentence "Standing down" comment and stop
- If relevant: analyze safety/liveness properties, review code if available, write TLA+ spec if warranted
- Post all findings as bmo comments — do not move issue status or create issues
- Do NOT write application code. Do NOT commit.
"""
)
```

3. Report the agent's findings back to the user.

## Multi-Issue Analysis

To analyze multiple issues in parallel:

```
subagent(
  agentScope="all",
  tasks=[
    { agent: "distributed-systems-expert", task: "Analyze BMO-{ID1}: ..." },
    { agent: "distributed-systems-expert", task: "Analyze BMO-{ID2}: ..." },
  ]
)
```
