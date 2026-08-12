---
title: SD-13 Loop Model — Excerpt
status: reference (operator-authored source: /home/ubuntu/workspace/sd13-class-uplift-loop-prompt.md)
date: 2026-07-12
purpose: Bundle reference for SD-18's loop inheritance. NOT a replacement for the source file; the source file is canonical.
---

# SD-13 Loop Model — Excerpt

This file extracts the patterns from `~/workspace/sd13-class-uplift-loop-prompt.md` that SD-18 inherits unchanged. SD-18's loop instruction document should reference these patterns by section, not re-state them.

## File-touch partition (the hard rule)

Two concurrent slices that touch the same file are guaranteed to collide. SD-13's surface is:

| File | Lanes that may touch it |
|---|---|
| `src/rules_core/pilot_compute.rs` | One lane at a time, full stop |
| `src/rules_core/support_state_matrix.rs` | One lane at a time |
| `tests/sd18_<class>_<burden>.rs` | One lane per file |
| `tests/fixtures/rules_core/pf1_human_<class>_<level>_sd18_*.txt` | One lane per fixture |
| `programs/codex/.../artifacts/sd18-*-execution-handoff-*.md` | One lane per handoff doc |

In SD-18, the equivalent file is `pilot_compute.rs` and the matrix carrier is `core-roster-and-support-state-matrix.md`. The same 1-lane-at-a-time rule applies.

## Per-cycle spawn budget

Default: **1 loop iteration in flight at a time.** Reason: the file-touch partition collapses any parallel attempt to a serial one for `pilot_compute.rs`. Two iterations in parallel means two iterations serializing on rebase and racing on matrix edits.

Documentation-only cycles (e.g. updating the progress doc, refreshing the matrix markdown) are the only exception.

## In-flight detection

```bash
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep
```

If any `claude` process is running with a prompt that names a row or criterion, do not pick that row. The in-flight process owns it. The loop's progress doc tracks `## Lane claims` with `claimed_at` and `expires_at` timestamps; if the timestamp is in the future, another invocation has the row.

## Per-cycle procedure (the steps, in order)

1. Read state (scope doc + progress doc + live `git` state + in-flight detection).
2. Pick an open acceptance criterion.
3. Claim it in the progress doc with `claimed_at` and `expires_at`.
4. Create a feature branch off `tranche/3` (in a worktree, if using one).
5. Write the failing test first (TDD red).
6. Implement the smallest change that makes the test pass (TDD green).
7. Run `cargo test --locked`, `cargo clippy --locked --tests -- -D warnings`.
8. Commit with explicit user identity.
9. Push to origin.
10. Auto-merge to `tranche/3` (per matured SD-13 doctrine — NOT explicit in the as-written prompt).
11. Self-heal conflicts inline if possible; otherwise write to `## Open blockers` and exit `FAIL`.
12. Delete the feature branch from local and origin.
13. Clean up `target/` directories (per matured SD-13 §5).
14. Mint a kanban card on `codex-tranche-3` with the §4.3 schema fields.
15. Update the progress doc (move the criterion from open to done, remove from `## Lane claims`, append to `## Cycle log`).
16. Print a final 5-line report.
17. Exit.

## Self-healable conditions (preserved from matured SD-13 posture)

- Branch diverged from `tranche/3` mid-iteration: rebase, re-test, re-merge.
- `target/` disk pressure: strip worktree `target/` directories.
- Coordination file drift: refresh the snapshot, retry.
- Cargo build cache corruption: `cargo clean`, rebuild.

## Non-self-healable conditions (preserved)

- Conflict requires a domain decision.
- Slice branch needs manual rebase.
- Two live claude processes would touch `pilot_compute.rs`.
- A chosen burden needs a new subsystem.
- Disk at 100% with no `target/`-strip remedy.

## What SD-18 changes from the as-written model

The as-written SD-13 prompt's "operator reviews every PR" was tightened post-tranche to "operator reviews PRs to develop; tranche-branch PRs auto-merge." SD-18 inherits the tightened posture. This is the key delta; everything else is preserved.

The as-written prompt has no "delete feature branch after merge" step. SD-18 adds this. Reason: ephemeral branches keep the loop's per-iteration footprint small.

The as-written prompt uses `integration/sd13-e5-class-uplift` as the integration branch. SD-18 uses `tranche/3` directly as the integration branch (since SD-18's breadth covers multiple epics, not just one epic's uplift; the integration branch is the tranche itself).

## Source-of-truth reminder

This file is a reference excerpt, not a replacement. The full source is at `/home/ubuntu/workspace/sd13-class-uplift-loop-prompt.md`. SD-18's loop instruction document inherits from this excerpt but should also point to the source for the un-shortened form.
