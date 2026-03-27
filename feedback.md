# Issues

## Blocks

```sh
$ bmo issue show BMO-52
  ...
  Relations:
    ← blocked by BMO-53
    → blocks BMO-53
```

There's a circular blockage? That's definitely a problem: how did we manage to calculate a circular blockage? We should make bmo disallow this type of relationship (should be DAG only, yes?).

## With Assignee

1. "unknown: HANDOFF` ->

```sh
$ bmo issue show BMO-52
  ...
  Comments (1):
    [2026-03-27] unknown: HANDOFF: CSS classes added. Modifier classes: .app-detail-section--live/.accepted/.closed on .app-detail-section (application-detail.css). Chip classes: .offer-state-chip--live/.accepted on .applications-status-badge base (application-detail.css). Tokens --offer-accent-live (#e6893d) and --offer-accent-accepted (#16a34a) added to src/app/app.css :root block. All additions are purely additive — no existing rules modified. Build clean (pnpm build ✓, tsc -b ✓). BMO-53 is unblocked.
```

This is `unknown`, so our identifiers are not working.

2. assignee is not getting cleared off: I see it on "done" status tickets:
