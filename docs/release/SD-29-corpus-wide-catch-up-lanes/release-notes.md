# SD-29 Release Notes

Populated at closure (Epic 11, Closure Epilogue — was Epic 8 under the retired per-book epic
numbering; renumbered by the 2026-08-10 kind-lane re-cut, `decisions.md §37`). The bundle's
per-cycle receipts in `progress.md` are the per-record evidence; this document summarizes the
release.

**RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** Epic numbers below shifted: Epic 4 is
now the corpus-wide Proven-Path Content Lanes tier; monster+monster_ability chassis moved to Epic
5, race-trait to Epic 6, companion to Epic 7. Scope widened from seven books to all 37 in-scope
books (`../corpus-work-channels.md §10.2`).

## Sections populated at closure

- **Summary** — overall release description, build version, branch.
- **User-visible changes** — **per-lane** per-record rollup (Epic 4 proven-path kinds, Epic 5
  monster+monster_ability chassis, Epic 6 race-trait, Epic 7 companion), each broken out by book,
  corpus-wide.
- **Operational changes** — local-file dispatch, no Hermes board, Epic 3's provenance gate.
- **Defects fixed** — any prior-cycle blockers resolved, including Epic 6's `race_trait` classifier
  defect fix (`decisions.md §38.3` / `../corpus-work-channels.md §9.3`).
- **Operational notes** — tranche promotion PR + post-closure version state.
- **Verification evidence** — per-cycle receipts + reach-gate output + PI-screening sweep output.
- **Known issues** — open items deferred, including the `class_feature` (15,472-unit corpus-wide)
  Tier-3 deferral (`decisions.md §38.4`, `successor-forward-scope-register.md C1.3`) and the SD-30
  collision (`decisions.md §38.5`, `risks-and-open-questions.md` OQ-29-004), if still unresolved at
  closure.
- **Update eligibility** — operator-on-file override indicators.

## Pre-population placeholder

No population yet (closure has not fired). The 37 in-scope books' per-record counts, rolled up by
lane, will be sourced from `cargo run --locked --bin v06_work_inventory` output, captured at the
cycle that publishes `0.9.<build>` to origin.

## Closure propagation

When Epic 11 fires:

1. The supervisor reads `progress.md`'s per-cycle receipts.
2. The supervisor pulls per-record counts from the post-cycle `docs/work-inventory.json`, grouped
   by lane (kind) and then by book.
3. This file is auto-populated from those receipts.
4. The tranche promotion PR message cites this file as `release-notes`.
