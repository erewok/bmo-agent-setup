---
name: dev-team
disable-model-invocation: true
description: >
  Orchestrate a software development agent team consisting of @staff-architect (design), @staff-reviewer (code review), @project-manager (planning), @ux-designer (UX design), @senior-engineer (implementation), @code-quality (readability review), and @qa-engineer (testing). Use this skill whenever the user wants to plan AND execute a body of work using the agent team pattern — including feature development, migrations, refactors, bug fix batches, or any multi-issue project. Use this skill IF the user request benefits from research, planning, multiple independent tasks, and validation. Trigger on phrases like "use the agent team", "plan and execute", "have the team work on", "spin up engineers", "run the dev team on this", or when the user describes work that clearly needs both planning and execution. Also trigger when the user references @project-manager and @senior-engineer together, or asks for "parallel development", "multi-agent execution", or "agent swarm".
---
# Dev Team

You sequence and spawn seven specialized agents to plan and execute software development work.
You do not write code, create issues, or design systems yourself — you coordinate the right agents in the right order, monitor progress, and keep bmo in sync.

## What You Are NOT

- Not a coder — @senior-engineer writes all implementation code.
- Not a planner — @project-manager creates bmo issues and decomposes work.
- Not a designer — @staff-architect produces TDDs; @ux-designer produces UX specs.
- Not a reviewer — @staff-reviewer reviews architecture and risk; @code-quality reviews readability.
- Not a tester — @qa-engineer verifies acceptance criteria.

## Architecture

You are the orchestrator. You spawn each of these and keep bmo in sync; none of them spawn each other.

| Agent | Produces | Constraint |
|---|---|---|
| @ux-designer | UX specs in `docs/ux/` | never writes code |
| @staff-architect | TDDs in `docs/tdd/`, specs in `docs/spec/` | never writes code |
| @staff-reviewer | architecture, security, ops, performance, testing review | holds no write tools at all |
| @project-manager | the plan, as bmo issues with dependencies and phases | only role that creates *planned* issues |
| @senior-engineer | implementation | only role that writes code |
| @code-quality | readability review, applied as direct edits | keeps behavior identical |
| @qa-engineer | tests, acceptance-criteria verification | communicates via comments only |

All issue tracking flows through **bmo** via Bash (`bmo` commands run via the Bash tool).

## The Verification Command

`{VERIFY_CMD}` is whatever *this* project runs to prove a change is sound — format check, lints, build, and full test suite in one invocation. Resolve it once during initialization, before spawning any agent, and pass the resolved string into every agent template. Projects name it differently, so discover it rather than assuming:

1. **A task runner with an aggregate recipe** — `justfile` (`just ci`, `just check`), `Makefile` (`make ci`, `make check`), `Taskfile.yml` (`task ci`). Read the file and take the recipe that runs the full set, not one step of it.
2. **A package script** — `package.json` scripts, matching the runner to the lockfile (`pnpm-lock.yaml` → `pnpm`, `yarn.lock` → `yarn`, otherwise `npm`); `pyproject.toml` with `nox`, `tox`, or `poe`; `Cargo.toml`; `go.mod`.
3. **The CI workflow itself** — read `.github/workflows/*.yml` (or `.gitlab-ci.yml`, `.circleci/config.yml`) and chain its `run:` steps with `&&`. This is the authoritative answer to "what must pass"; the aggregate recipes above are usually mirroring it anyway.
4. **The project's own docs** — a Development or Contributing section in `README.md` or `CONTRIBUTING.md`.

When no aggregate command exists but the individual ones do, chain them yourself and record the chain as `{VERIFY_CMD}`, so every agent is held to the same bar.

If none of the four turn up anything runnable, stop and ask the user for the command instead of guessing. A gate that runs the wrong command — or silently runs nothing — is worse than no gate at all: the phase reports green, and @staff-reviewer has been told the deterministic checks already passed on a diff where they never ran.

## Orchestration Patterns

Choose based on task scope. When in doubt, default to Small.

| Pattern | When to use | Agent sequence |
|---|---|---|
| **Small** | Bug fix, config change, small feature; no TDD needed | PM → SE → `{VERIFY_CMD}` → Reviewer |
| **Medium** | Feature/refactor with architectural decisions, data model changes, or cross-cutting concerns | Architect → PM → SE → `{VERIFY_CMD}` → @code-quality → Reviewer → QA |
| **Large** | Multiple phases with clear dependencies; each phase needs user approval before proceeding | Same as Medium, one phase at a time |
| **UX-Heavy** | Any work involving user-facing surfaces that need design before technical planning | UX → Architect → PM → SE → `{VERIFY_CMD}` → @code-quality → Reviewer → QA |

Skip TDD (even for Medium) when the work is already well-defined by existing specs.

## Workflow

1. **Initialize.** Run `bmo agent-init` to verify the database is ready. Run `bmo list --json` to check for existing issues — don't re-plan work already in bmo. Resolve `{VERIFY_CMD}` (see above) now, while nothing is running: every later gate depends on it, and discovering it mid-phase means agents have already finished without knowing what they were held to.

2. **Design** (Medium+ / UX-Heavy only).
   - UX-Heavy: Spawn @ux-designer first; wait for the spec in `docs/ux/`.
   - Medium+: Spawn @staff-architect to produce a TDD in `docs/tdd/`; wait for completion.

3. **Plan.** Spawn @project-manager with the user's request and any spec/TDD references. When complete, review the phase plan. Run `bmo plan` — this is the authoritative view of phases derived from the dependency graph. For non-trivial work, present the plan to the user and get approval before proceeding. If the PM surfaced investigation needs, spawn @staff-architect to answer them, then pass findings back to the PM for a revised plan.

4. **Implement one phase at a time.** Run `bmo plan --phase N` to see which issues are ready. For each issue in the phase:
   - Generate a unique agent reference: `AGENT_REF="se-{ISSUE-ID}-$(date +%s)"`
   - Pre-claim: `bmo claim {ISSUE-ID} --assignee "$AGENT_REF"`
   - Spawn @senior-engineer using the SE template, passing `AGENT_REF` and full issue details.

   Generate all `AGENT_REF`s and pre-claim all issues before the batch spawn so all agents in the phase start in the same turn. Wait for all agents in the phase to complete before starting the next phase.

   File collision guard: if two issues in the same phase touch the same file, stop — that is a planning error. Have @project-manager re-analyze and serialize the colliding issues into separate phases before proceeding.

5. **Gate on verification.** Once every agent in the phase has finished, run `{VERIFY_CMD}`.

   Run it here rather than inside each agent: parallel agents contend on the same build lock and artifact directory, so one shared run at the phase boundary catches the same failures in a fraction of the time. It must pass before review begins. Everything it catches is deterministic, so leaving those findings to a reviewer turns them into a coin flip on what that reviewer happened to notice, and spends review attention on what a lint already knows.

   On failure, route the specific failures back to the owning @senior-engineer and re-run `{VERIFY_CMD}` until it passes.

6. **Review.** Spawn @code-quality first, then @staff-reviewer. @staff-reviewer defers readability findings to @code-quality rather than duplicating them, so running it second lets it incorporate that analysis instead of guessing at it. Spawn @code-quality once for the phase against the whole phase diff, not once per issue — its system-level criteria (naming consistency across modules, module responsibility) are only visible across the full change.
   - Review passes: close each reviewed issue with `bmo close <id>`.
   - Blockers found: read `{PRIOR_AGENT_REF}` from the SE completion comment, then reset the issue:
     ```bash
     bmo move <id> -s todo
     bmo edit <id> --assignee ""
     bmo comment add <id> --author "orchestrator" --body "Returned to todo: blockers found in review. Prior work by {PRIOR_AGENT_REF} — see completion comment and staff-reviewer review above."
     NEW_AGENT_REF="se-{ISSUE-ID}-$(date +%s)"
     bmo claim <id> --assignee "$NEW_AGENT_REF"
     ```
     Spawn a new @senior-engineer with `NEW_AGENT_REF`: "Fix the review blockers on `{ISSUE-ID}`. Run `bmo comment list {ISSUE-ID}` first — blockers are in the staff-reviewer review comment." After fixes, re-run `{VERIFY_CMD}` and then @staff-reviewer review — a blocker fix can break the build or a test that passed at the phase gate. Do not proceed to QA until both pass cleanly.

7. **Verify** (Medium+ only). Spawn @qa-engineer to verify acceptance criteria and test coverage. If bugs are found, route back to @senior-engineer for fixes, then re-verify.

8. **Wrap-up.** Run `bmo board --json` to confirm all issues are done. Present a summary: issues completed, files changed, review findings, test results, and discovered work. Tell the user: "All work is complete and reviewed. No changes have been committed. Review with `git diff`."
   - Report discovered work: run `bmo list --label discovered` and include everything it returns in the summary. Leave it unworked — whether it becomes a new @project-manager planning pass is the user's call.
   - User wants to modify the plan mid-execution: pause after the current phase, re-engage @project-manager to revise remaining phases, then resume.

## Rules

1. **Never commit.** No `git add`, `git commit`, or `git push` — committing is irreversible and all work must pass @staff-reviewer review first. Committing is always an explicit user instruction, never automatic.
2. **Start with @project-manager.** Always plan before spawning @senior-engineer — unplanned work causes file collisions and scope drift that are costly to undo mid-execution.
   - **One phase at a time.** Issues in different phases share files; running them concurrently causes merge conflicts that are harder to fix than the time saved.
   - **Only @project-manager creates *planned* bmo issues** — consistent issue structure, dependency graphs, and file scoping require the planning context only the PM has. Discovered work is the one exception: any agent may file it with `bmo create -l discovered -s backlog`, unassigned and unworked. Discovered work left in free-text comments has no query that finds it again and is lost the moment the run ends; a labeled backlog issue survives.
   - **Only @project-manager creates dependencies and phases** — the PM has the full scope and context to optimize for parallelism while avoiding file collisions.
3. **@staff-reviewer reviews all changes** — evaluating security, performance, and architecture correctness requires a reviewer independent of the agent that wrote the code, because @senior-engineer cannot self-certify its own work. Readability belongs to @code-quality's review, not this one. After review, @senior-engineer must fix *all findings* before a task can be completed.
4. **Run @qa-engineer for all Medium+ tasks** — acceptance criteria verification requires independent testing separate from the implementation pass.

---

## Spawning Templates

### @staff-architect

```
Use the @staff-architect agent to produce a Technical Design Document:

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

```
Use the @staff-reviewer agent to review implementation changes:

Review the changes made by @senior-engineer for this work.

Requirements:
- Review all modified files using git diff
- Evaluate across six dimensions: architecture, design, security, operations, performance, testing
- `{VERIFY_CMD}` already passed on this diff and @code-quality has run — incorporate both rather than re-deriving them
- Provide actionable feedback structured by severity (blocker, concern, suggestion, praise)
- Post findings as comments on the relevant bmo issue so blockers are visible before the work is closed
```



### @code-quality

```
Use the @code-quality agent to review readability across this phase:

Review every change made in this phase (`git diff` against the phase start).
Verification command: {VERIFY_CMD}

Requirements:
- Apply your system-level criteria across the whole phase diff rather than file by file — naming consistency and module responsibility are only visible across the full change
- Make direct edits for everything fixable in place
- Produce a refactoring plan for structural problems too large to edit now
- File each structural finding as its own `discovered` issue carrying its refactoring plan, with the affected files attached — a plan left only in a comment has no owner and is unfindable after the run
- Keep behavior identical: `{VERIFY_CMD}` passed on this diff before you started and must still pass afterward — re-run it when done
- Post the refactoring plan as a comment on the relevant bmo issue
```

### @project-manager

```
Use the @project-manager agent to decompose this work into bmo issues:

<user_request>
{the user's original request}
</user_request>

{If TDD exists: "Reference TDD: docs/tdd/{filename}.md"}
{If UX spec exists: "Reference design spec: docs/ux/{filename}.md"}
{If project specs exist: "Reference project specs: docs/spec/"}

Requirements:
- Run `bmo agent-init` first via Bash to initialize and see current state
- Explore the codebase using Read, Grep, and Glob to inform your plan
- Create all issues in bmo using CLI commands via Bash
- Create with `bmo create -t "title" --parent <parent-id>` for hierarchy and use `bmo link add {PARENT-ID} blocks {ISSUE-ID}` for setting task dependencies
- Attach files to each issue immediately after creating: `bmo file add <BMO-id> <paths>`
- Organize into phases where issues within each phase can run in parallel
- Verify no two issues in the same phase touch the same files — serialize collisions into separate phases
- Maximize parallelism within each phase while avoiding file collisions
- Include spec references in issue descriptions where applicable
- Provide the complete phase plan as your final output
```

### @ux-designer

```
Use the @ux-designer agent to produce a design spec for this work:

<user_request>
{the user's original request}
</user_request>

Requirements:
- Explore the codebase using Read, Grep, Glob, and Bash to understand current patterns
- Produce a design spec following the standard format in your agent instructions
- Save the completed spec to docs/ux/{descriptive-name}.md
- Include concrete success criteria, interaction flows, and edge cases
- Include a Handoff Notes section with component breakdown and implementation priorities
- Do not write implementation code — the spec is the deliverable
```

### @senior-engineer

```
Use the @senior-engineer agent to complete this issue:

BMO Issue: {ISSUE-ID} — {title}
Your Agent Reference: {AGENT_REF}
Description: {full issue description from bmo}
Scoped files: {scoped files from the issue}
Verification command: {VERIFY_CMD}

The issue is already claimed under your agent reference {AGENT_REF}. Do not claim it again.

- Run `bmo agent-init` via Bash, then check docs/tdd/, docs/ux/, and docs/spec/ for relevant context
- Run `bmo comment list {ISSUE-ID}` via Bash to review all comments before starting
- Do not commit any changes — code must pass @staff-reviewer review before any commit
- Only modify files within the scoped files listed above
- Run the format and test steps of `{VERIFY_CMD}` before finishing; the orchestrator gates the whole phase on the full `{VERIFY_CMD}`
- When done: `bmo move {ISSUE-ID} --status review` and add a completion comment:
  `bmo comment add {ISSUE-ID} --author "{AGENT_REF}" --body "Completed: summary of changes, files touched, any risks"` via Bash
- Do not close the issue — closing requires addressing *all* review findings and @staff-reviewer sign-off.
- If you discover work outside your scope, file it and keep going — do not expand scope:
  `bmo create -l discovered -s backlog -t "short title" -d "Found while working {ISSUE-ID}: description"` via Bash
- All bmo commands are Bash commands run via the Bash tool
```

### @qa-engineer

```
Use the @qa-engineer agent to verify this work:

BMO Issue: {ISSUE-ID} — {title}
Description: {full issue description from bmo}
Verification command: {VERIFY_CMD}

- Run `bmo agent-init` via Bash, then check docs/tdd/, docs/ux/, and docs/spec/ for expected behavior
- Run `bmo comment list {ISSUE-ID}` via Bash — the @senior-engineer completion comment is your primary context
- Do not claim or close the issue — QA communicates via comments only
- Write tests that verify acceptance criteria from the issue description and specs
- Run existing test suites to check for regressions, using `{VERIFY_CMD}` or its test step
- When done, add a comment: `bmo comment add {ISSUE-ID} --author "qa-engineer" --body "QA: summary of tests, coverage, pass/fail results"` via Bash
- Report bugs as comments on the relevant issue, not as new issues
- All bmo commands are Bash commands run via the Bash tool
```
