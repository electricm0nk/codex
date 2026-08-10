# SD-29 Release Notes

Populated at closure (Epic 11, Closure Epilogue — was Epic 8 under the retired per-book epic
numbering; renumbered by the 2026-08-10 kind-lane re-cut, `decisions.md §37`). The bundle's
per-cycle receipts in `progress.md` are the per-record evidence; this document summarizes the
release.

## Sections populated at closure

- **Summary** — overall release description, build version, branch.
- **User-visible changes** — **per-lane** per-record rollup (Epic 4 monster+monster_ability
  chassis, Epic 5 race-trait, Epic 6 companion, Epic 7 residual), each broken out by book.
- **Operational changes** — local-file dispatch, no Hermes board, Epic 3's provenance gate.
- **Defects fixed** — any prior-cycle blockers resolved, including Epic 5's `race_trait` classifier
  defect fix (`decisions.md §37` / `../corpus-work-channels.md §9.3`).
- **Operational notes** — tranche promotion PR + post-closure version state.
- **Verification evidence** — per-cycle receipts + reach-gate output + PI-screening sweep output.
- **Known issues** — open items deferred to SD-30 or post-tranche, including the `class_feature`
  (90-unit) Channel D deferral (`decisions.md §37.4`, `successor-forward-scope-register.md C1.3`).
- **Update eligibility** — operator-on-file override indicators.

## Pre-population placeholder

No population yet (closure has not fired). The seven books' per-record counts, rolled up by lane,
will be sourced from `cargo run --locked --bin v06_work_inventory` output, captured at the cycle
that publishes `0.9.<build>` to origin.

## Closure propagation

When Epic 11 fires:

1. The supervisor reads `progress.md`'s per-cycle receipts.
2. The supervisor pulls per-record counts from the post-cycle `docs/work-inventory.json`, grouped
   by lane (kind) and then by book.
3. This file is auto-populated from those receipts.
4. The tranche promotion PR message cites this file as `release-notes`.
