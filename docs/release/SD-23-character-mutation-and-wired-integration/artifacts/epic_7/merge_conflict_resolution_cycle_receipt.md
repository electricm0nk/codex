# Criterion 28 — Merge conflict resolution cycle, pre-flight mode (cycle 14)

`resolve_merge_conflicts.py --mode pre-flight` run against `origin/develop` (integration target), branch `tranche/5-1`, bundle SD-23.

## Safety check before running

`git pull --rebase origin develop` is a real, mutating operation on the branch (not a simulation) — before running it for real, re-confirmed `git fetch origin develop` still showed `origin/develop` HEAD (`f36c211`) equal to `merge-base(tranche/5-1, origin/develop)`, meaning develop hasn't moved since `tranche/5-1` was cut (unchanged across cycles 2, 11, and this cycle). This guarantees the rebase is a no-op — no actual conflict risk, no force-push needed regardless of outcome.

## Result

```
[merge-conflict] running pre-flight rebase: git pull --rebase origin develop
[merge-conflict] rebase exit=0, conflicts=0, outcome=clean
```

- **Branch tip before/after:** `cf897f69` (unchanged — confirms the no-op prediction).
- **Local/remote sync:** `git rev-parse HEAD` == `git rev-parse origin/tranche/5-1` after the rebase — no divergence introduced.
- **Receipt:** appended to `receipts.md`, `row_or_kind: merge_conflict:pre_flight_rebase`, `outcome: clean`, `conflict_files: []`.

Commit SHA (receipt): `15a61b7`. `tranche/5-1` is confirmed clean and rebased on current `develop` HEAD, ready for the promotion PR (Criterion 29).
