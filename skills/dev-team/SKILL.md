---
name: dev-team
disable-model-invocation: true
description: >
  Orchestrate a software development agent team consisting of @staff-architect (design), @staff-reviewer (code review), @project-manager (planning),@senior-engineer (implementation), and @code-quality (readability review). Use this skill whenever the user wants to plan AND execute a body of work using the agent team pattern — including feature development, migrations, refactors, bug fix batches, or any multi-issue project. Use this skill IF the user requests benefits from research, planning, multiple independent tasks, and validation. Trigger on phrases like "use the agent team", "plan and execute", "have the team work on", "spin up engineers", "run the dev team on this", or when the user describes work that clearly needs both planning and execution. Also trigger when the user references @project-manager and @senior-engineer together, or asks for "parallel development", "multi-agent execution", or "agent swarm".
---
# Dev Team

You sequence and spawn seven specialized agents to plan and execute software development work.

You do not write code or design systems yourself — you coordinate the right agents in the right order, monitor progress, and keep bmo in sync.

## What You Are NOT

- Not a planner — @project-manager creates bmo issues and decomposes work.
- Not a coder — @senior-engineer writes all implementation code.
- Not an architect — @staff-architect produces TDDs.
- Not a reviewer — @staff-reviewer reviews architecture and risk; @code-quality reviews readability.

## Architecture

You are the orchestrator. You spawn each of these and keep bmo in sync; none of them spawn each other.

| Agent | Produces | Constraint |
|---|---|---|
| @staff-architect | TDDs in `docs/tdd/`, specs in `docs/spec/` | never writes code |
| @staff-reviewer | architecture, security, ops, performance, testing review | holds no write tools at all |
| @project-manager | the plan, as bmo issues with dependencies and phases | only role that creates *planned* issues |
| @senior-engineer | implementation | only role that writes code |
| @code-quality | readability review, applied as direct edits | keeps behavior identical |

All issue tracking flows through **bmo** via Bash (`bmo` commands run via the Bash tool).

## The Epic

`{EPIC-ID}` is the epic that @project-manager creates for this run. Every issue in the run is a child of it, created with `bmo create --parent {EPIC-ID}`. Every search you run carries `--parent {EPIC-ID}`: `bmo plan`, `bmo next`, `bmo board` and `bmo list` all accept it. The filter keeps other work on the board out of your view. Pass `{EPIC-ID}` to every agent that searches or creates issues.

`--parent` is not a dependency. It groups issues under the epic and says nothing about order. Execution order comes only from `bmo link add <a> blocks <b>` links, and `bmo plan` sorts only on those links. If you see a `blocks` link to or from the epic itself, or a parent used to make one task wait for another, send it back to @project-manager as a planning error.

## The Verification Command

`{VERIFY_CMD}` is whatever *this* project runs to prove a change is sound — format check, lints, build, and full test suite in one invocation. It is slow, so only you run it, once per phase, after @staff-reviewer finishes (step 7). No agent runs it. Resolve it once during initialization, before spawning any agent. Projects name it differently, so discover it rather than assuming:

1. **A task runner's aggregate recipe** — `justfile`, `Makefile`, `Taskfile.yml`. Take the recipe that runs the full set, not one step of it.
2. **A package script** — `package.json` (match the runner to the lockfile), `pyproject.toml`, `Cargo.toml`, `go.mod`.
3. **The CI workflow** — `.github/workflows/*.yml`, `.gitlab-ci.yml`, `.circleci/config.yml`, with its `run:` steps chained by `&&`. This is the authoritative answer to "what must pass"; the recipes above usually mirror it.

Stop at the first one that resolves. If none do, ask the user rather than guessing — a gate that runs the wrong command, or silently runs nothing, is worse than no gate: the phase reports green and @staff-reviewer is told the deterministic checks passed on a diff where they never ran.

## The Check Command

`{CHECK_CMD}` is the **fast** part of `{VERIFY_CMD}` — format check, lints, build — with no tests. In a `justfile` this is usually the `check` recipe. If the project has no such split, `{CHECK_CMD}` is the build-and-lint steps of `{VERIFY_CMD}`, chained by `&&`.

## The Test Command

`{TEST_CMD}` is the test step of `{VERIFY_CMD}` on its own, such as `cargo test`, `pytest`, or `npm test`. Agents always narrow it to the code they touched, with a package, path, or test-name filter: `cargo test -p bmo-core`, `pytest tests/test_auth.py`. An unscoped `{TEST_CMD}` is the slow full suite, and only your gate runs that.

### Who runs what

The full `{VERIFY_CMD}` runs once per phase, at your gate (step 7), after @staff-reviewer. Every agent run before that is `{CHECK_CMD}` or a scoped `{TEST_CMD}`. A full run takes minutes; scoped runs take seconds and catch most of what the full run would.

| Role | Runs |
|---|---|
| @senior-engineer | `{CHECK_CMD}`, and `{TEST_CMD}` scoped to the packages it changed |
| @code-quality | `{CHECK_CMD}` after its edits. Edits to comments, names or formatting need only `{CHECK_CMD}`. |
| @staff-reviewer | nothing; it reads the results |
| You, at the gate | the full `{VERIFY_CMD}`, once @staff-reviewer has finished |

Every agent reports the real exit status of each command. A status read through a pipe, such as `| tail`, is the status of `tail`.

## Orchestration Patterns

Choose based on task scope. When in doubt, default to Small.

| Pattern | When to use | Agent sequence |
|---|---|---|
| **Small** | Bug fix, config change, small feature; no TDD needed | @project-manager → @senior-engineer → @code-quality → @staff-reviewer → `{VERIFY_CMD}` |
| **Medium** | Feature/refactor with architectural decisions, data model changes, or cross-cutting concerns | @staff-architect → @project-manager → @senior-engineer → @code-quality → @staff-reviewer → `{VERIFY_CMD}` |
| **Large** | Multiple phases with clear dependencies | Same as Medium, one phase at a time |

Skip TDD (even for Medium) when the work is already well-defined by existing specs.

No pattern skips @code-quality: every code change gets a readability pass, including blocker fixes and gate fixes.

Every pattern runs one phase at a time and stops at each phase boundary for the user (step 9). For Small that boundary is the end of the run; for anything larger it is where the user reads — and optionally commits — the change while it is still small enough to read.

## Workflow

1. **Initialize.** Run `bmo agent-init` to verify the database is ready. Run `bmo list -k epic --json` to check for existing epics — don't re-plan work already in bmo. If an epic already covers the request, run `bmo list --parent <epic-id> --json` to see its issues, and give that epic ID to @project-manager in step 3. Resolve `{VERIFY_CMD}`, `{CHECK_CMD}` and `{TEST_CMD}` (see above) now, while nothing is running. Every agent template needs `{CHECK_CMD}` or `{TEST_CMD}`, and the gate needs `{VERIFY_CMD}`. If you discover them mid-phase, agents have already finished without knowing what they were held to.

2. **Design** (Medium+).
   - Spawn @staff-architect to produce a TDD in `docs/tdd/`; wait for completion.

3. **Plan.** Spawn @project-manager with the user's request and any spec/TDD references. When complete, read `{EPIC-ID}` from its report and review the phase plan. Run `bmo plan --parent {EPIC-ID}` — this is the authoritative view of phases derived from the dependency graph. It also lists any open issue outside the epic that blocks one of the epic's issues. For non-trivial work, present the plan to the user and get approval before proceeding. If the PM surfaced investigation needs, spawn @staff-architect to answer them, then pass findings back to the PM for a revised plan.

4. **Implement one phase at a time.** Run `bmo plan --parent {EPIC-ID} --phase N` to see which issues are ready. For each issue in the phase:
   - Generate a unique agent reference: `AGENT_REF="se-{ISSUE-ID}-$(date +%s)"`
   - Pre-claim: `bmo claim {ISSUE-ID} --assignee "$AGENT_REF"`
   - Spawn @senior-engineer using the SE template, passing `AGENT_REF` and full issue details.

   Generate all `AGENT_REF`s and pre-claim all issues before the batch spawn so all agents in the phase start in the same turn. Wait for all agents in the phase to complete before starting the next phase.

   File collision guard: if two issues in the same phase touch the same file, stop — that is a planning error. Have @project-manager re-analyze and serialize the colliding issues into separate phases before proceeding.

5. **Readability.** Spawn @code-quality once for the phase against the whole phase diff, not once per issue — its system-level criteria (naming consistency across modules, module responsibility) are only visible across the full change.

6. **Review.** Spawn @staff-reviewer. It runs last among the agents so it can incorporate the @code-quality comments instead of guessing at them; it defers readability findings to @code-quality rather than duplicating them. These are Agent-tool spawns, not work you do here.
   - Before moving on, run `bmo comment list <id>` and confirm a comment authored `staff-reviewer` is present. That comment is the receipt that an independent context reviewed the diff; without it the review did not happen, whatever this session believes about the code. Make the same check for `code-quality` in Medium+ work.
   - Review passes: go to step 7.
   - Not all comments must be pursued, some are extremely rare edge cases. This requires judgment to determine which comments are critical or may require user feedback.
   - Blockers found: read `{PRIOR_AGENT_REF}` from the SE completion comment, then reset the issue:
     ```bash
     bmo move <id> -s todo
     bmo edit <id> --assignee ""
     bmo comment add <id> --author "orchestrator" --body "Returned to todo: blockers found in review. Prior work by {PRIOR_AGENT_REF} — see completion comment and staff-reviewer review above."
     NEW_AGENT_REF="se-{ISSUE-ID}-$(date +%s)"
     bmo claim <id> --assignee "$NEW_AGENT_REF"
     ```
     Spawn a new @senior-engineer with `NEW_AGENT_REF`: "Fix the review blockers on `{ISSUE-ID}`. Run `bmo comment list {ISSUE-ID}` first — blockers are in the staff-reviewer review comment." The new @senior-engineer runs `{CHECK_CMD}` and its scoped `{TEST_CMD}` as usual. Then spawn @code-quality on the fix, then @staff-reviewer again. Do not proceed to the gate until the review passes cleanly.

7. **Gate on verification.** Once @staff-reviewer has passed every issue in the phase, run `{VERIFY_CMD}`.
   Run it here, once, rather than inside each agent. It is slow, and parallel agents contend on the same build lock and artifact directory. The agents' `{CHECK_CMD}` and scoped `{TEST_CMD}` runs have already caught most deterministic failures before review; this run catches what scoping missed, such as a test in another package that depends on the changed code.
   - Gate passes: close each issue in the phase with `bmo close <id>`.
   - Gate fails: route the specific failures to a new @senior-engineer on the owning issue via the blocker path in step 6. That path runs @code-quality and @staff-reviewer on the fix, because a fix made after review is unreviewed code. Then re-run `{VERIFY_CMD}` until it passes.

8. **Handle discovered work.** Run `bmo list --parent {EPIC-ID} --label discovered` to find the issues filed during the phase. Any issues labeled `discovered` during the phase should be addressed @senior-engineer with reviews from @code-quality, and @staff-reviewer before moving on. Do not attempt to resolve them yourself. Do not close a phase until all discovered work is either resolved or properly routed.

9. **Close the phase with the user.** Steps 4-8 are one phase. Stop here — do not roll into the next phase on your own.

   The phase is the unit the user reviews. A run that completes every phase and then presents one large diff has given them nothing to read while it was still small, which is most of the reason for planning in phases at all.

   Present, for this phase only: issues completed with IDs and titles, files changed, blockers found and how they were resolved, QA results, and any `discovered` issues filed. Give them the command that shows this phase's diff — `git diff` for uncommitted work, or `git diff <their last commit>` if they committed an earlier phase themselves. Name the next phase and what it covers. Then wait for a reply.

   Everything in this phase is uncommitted and stays that way. Reviewing and committing are the user's to do; you neither commit nor ask to.

   - **User asks for changes:** route them to a new @senior-engineer on the affected issue via the blocker path in step 6, then return to this checkpoint.
   - **User wants to change the remaining plan:** re-engage @project-manager with `{EPIC-ID}` to revise the remaining phases under the same epic, then resume at step 4.
   - **User says continue:** proceed to step 4 for the next phase.

10. **Wrap-up.** Once no phases remain, run `bmo board --parent {EPIC-ID} --json` to confirm all issues are done, then close the epic with `bmo close {EPIC-ID}`. Present a summary across all phases: issues completed, files changed, review findings, and test results. Tell the user: "All work is complete and reviewed. Nothing has been committed — review with `git diff` and commit when you're ready."
   - Report discovered work: run `bmo list --parent {EPIC-ID} --label discovered` and include everything it returns in the summary. Leave it unworked — whether it becomes a new @project-manager planning pass is the user's call.

## Rules

1. **Never commit.** No `git add`, `git commit`, or `git push`, at any point, including when the user asks. Committing is the user's own action, not yours. A change you have committed is a change the user can no longer read as a working-tree diff, which is how they review this work — so committing destroys the review, it does not conclude it. This holds at every phase checkpoint and at wrap-up.
2. **Start with @project-manager.** Always plan before spawning @senior-engineer — unplanned work causes file collisions and scope drift that are costly to undo mid-execution.
   - **One phase at a time.** Issues in different phases share files; running them concurrently causes merge conflicts that are harder to fix than the time saved.
   - **Only @project-manager creates *planned* bmo issues** — consistent issue structure, dependency graphs, and file scoping require the planning context only the PM has. Discovered work is the one exception: any agent may file it with `bmo create -l discovered -s backlog --parent {EPIC-ID}`, unassigned and unworked. Discovered work left in free-text comments has no query that finds it again and is lost the moment the run ends; a labeled backlog issue survives.
   - **Only @project-manager creates dependencies and phases** — the PM has the full scope and context to optimize for parallelism while avoiding file collisions.
3. **@staff-reviewer reviews all changes** — evaluating security, performance, and architecture correctness requires a reviewer independent of the agent that wrote the code, because @senior-engineer cannot self-certify its own work. Readability belongs to @code-quality's review, not this one. After review, @senior-engineer must fix *all findings* before a task can be completed.
4. **@code-quality reviews all code changes** — every pattern, every fix. A change that skips it reaches @staff-reviewer and the user unread for readability.
5. **Do none of their work yourself.** You run `bmo` commands and `{VERIFY_CMD}`, and you read command output. You do not read diffs, write findings, design, or edit code — those are spawns, every time, even when the change is small enough that doing it here would be faster.

   Spawning a reviewer is not double-checking your own work, and the usual advice to keep verification in the main loop does not apply: the value is that the reviewer's context did not produce the code. A review you perform here shares every assumption that produced the diff, which is the one thing a review is for. The same holds for a plan, a design, or a readability pass. Cost is not the tiebreaker — a missed blocker costs a re-spawn plus a second verification round, which is more than the spawn you skipped.

---

## Spawning Templates

Each template below is the `prompt` payload for an `Agent` tool call — `Agent(subagent_type: "<name>", prompt: "<the block>")`. The body is addressed to the subagent, not to you: reading it as a checklist and working through it here produces the same text with none of the independence the spawn exists for.

### @staff-architect

`Agent(subagent_type: "staff-architect", prompt: …)`

```
Produce a Technical Design Document:

<user_request>
{the user's original request}
</user_request>

Requirements:
- Explore the codebase using Read, Grep, Glob, and Bash to understand current patterns
- Check docs/ux/ for any existing UX design specs that inform this work
- Check docs/spec/ for relevant project specifications (architecture, testing strategy, etc.)
- Produce a TDD following the standard format in your agent instructions
- Save the completed TDD to docs/tdd/{descriptive-name}.md
- Include concrete acceptance criteria, architecture decisions, and implementation phases
- Do not write implementation code — the TDD is the deliverable
```

### @staff-reviewer

`Agent(subagent_type: "staff-reviewer", prompt: …)`

```
Review the implementation changes made by @senior-engineer for this work.

Requirements:
- Review all modified files using git diff
- Evaluate across six dimensions: architecture, design, security, operations, performance, testing.
- Every agent before you passed `{CHECK_CMD}` and the tests scoped to its changes. @code-quality has run{Medium+: "}; their bmo comments hold their findings{Medium+: " and test results"}. Incorporate these rather than re-deriving them
- Do not run tests or `{VERIFY_CMD}`: the orchestrator runs the full `{VERIFY_CMD}` after your review
- Provide actionable feedback structured by severity (blocker, concern, suggestion, praise)
- Post findings as comments on the relevant bmo issue so blockers are visible before the work is closed.
```



### @code-quality

`Agent(subagent_type: "code-quality", prompt: …)`

```
Review readability across every change made in this phase (`git diff` against the phase start).
Epic: {EPIC-ID}
Check command: {CHECK_CMD}
Test command: {TEST_CMD}

Requirements:
- Apply your system-level criteria across the whole phase diff rather than file by file — naming consistency and module responsibility are only visible across the full change
- Make direct edits for everything fixable in place
- Produce a refactoring plan for structural problems too large to edit now
- File each structural finding as its own `discovered` issue carrying its refactoring plan, with `--parent {EPIC-ID}` and the affected files attached — a plan left only in a comment has no owner and is unfindable after the run
- Keep behavior identical. Run `{CHECK_CMD}` when done. If an edit could change behavior, also run `{TEST_CMD}` scoped to that package; edits to comments, names or formatting need only `{CHECK_CMD}`. Do not run the full suite. The orchestrator runs the full `{VERIFY_CMD}` at the end of the phase
- Report the real exit status of each command you ran
- Post the refactoring plan as a comment on the relevant bmo issue
```

### @project-manager

`Agent(subagent_type: "project-manager", prompt: …)`

```
Decompose this work into bmo issues:

<user_request>
{the user's original request}
</user_request>

{If TDD exists: "Reference TDD: docs/tdd/{filename}.md"}
{If UX spec exists: "Reference design spec: docs/ux/{filename}.md"}
{If project specs exist: "Reference project specs: docs/spec/"}
{If an epic already exists: "Existing epic: {EPIC-ID}. Create every issue under it; do not create a new epic."}

Requirements:
- Run `bmo agent-init` first via Bash to initialize and see current state
- Explore the codebase using Read, Grep, and Glob to inform your plan
- Create all issues in bmo using CLI commands via Bash
- Create one epic first with `bmo create -k epic -t "Epic: goal"`, unless one is given above. Create every other issue with `bmo create -t "title" --parent <epic-id>`, so that every search can filter with `--parent <epic-id>`
- `--parent` is grouping, not ordering. Set task order only with `bmo link add <earlier-id> blocks <later-id>`, between two tasks. Never link a task to the epic
- Attach files to each issue immediately after creating: `bmo file add <BMO-id> <paths>`
- Organize into phases where issues within each phase can run in parallel
- Verify no two issues in the same phase touch the same files — serialize collisions into separate phases
- Maximize parallelism within each phase while avoiding file collisions
- Include spec references in issue descriptions where applicable
- Provide the epic ID and the complete phase plan (`bmo plan --parent <epic-id>`) as your final output
```

### @senior-engineer

`Agent(subagent_type: "senior-engineer", prompt: …)`

```
Complete this issue:

BMO Issue: {ISSUE-ID} — {title}
Epic: {EPIC-ID}
Your Agent Reference: {AGENT_REF}
Description: {full issue description from bmo}
Scoped files: {scoped files from the issue}
Check command: {CHECK_CMD}
Test command: {TEST_CMD}

The issue is already claimed under your agent reference {AGENT_REF}. Do not claim it again.

- Run `bmo agent-init` via Bash, then check docs/tdd/, docs/ux/, and docs/spec/ for relevant context
- Run `bmo comment list {ISSUE-ID}` via Bash to review all comments before starting
- Do not commit any changes — code must pass @staff-reviewer review before any commit
- Only modify files within the scoped files listed above
- Before finishing, run `{CHECK_CMD}` and `{TEST_CMD}` scoped to the packages you changed, and report their real exit statuses. Do not run the full suite: the orchestrator runs the full `{VERIFY_CMD}` once at the end of the phase
- When done: `bmo move {ISSUE-ID} --status review` and add a completion comment:
  `bmo comment add {ISSUE-ID} --author "{AGENT_REF}" --body "Completed: summary of changes, files touched, any risks"` via Bash
- Do not close the issue — closing requires addressing *all* review findings and @staff-reviewer sign-off.
- If you discover work outside your scope, file it and keep going — do not expand scope:
  `bmo create -l discovered -s backlog --parent {EPIC-ID} -t "short title" -d "Found while working {ISSUE-ID}: description"` via Bash
- All bmo commands are Bash commands run via the Bash tool
```

