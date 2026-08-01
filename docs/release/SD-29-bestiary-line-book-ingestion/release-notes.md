# SD-29 Release Notes

Populated at closure (Epic 8, Closure Epilogue). The bundle's per-cycle
receipts in `progress.md` are the per-record evidence; this document
summarizes the release.

## Sections populated at closure

- **Summary** — overall release description, build version, branch.
- **User-visible changes** — per-bestiary per-record rollup.
- **Operational changes** — local-file dispatch, no Hermes board.
- **Defects fixed** — any prior-cycle blockers resolved.
- **Operational notes** — tranche promotion PR + post-closure version state.
- **Verification evidence** — per-cycle receipts + reach-gate output.
- **Known issues** — open items deferred to SD-30 or post-tranche.
- **Update eligibility** — operator-on-file override indicators.

## Pre-population placeholder

No population yet (closure has not fired). The four bestiaries' per-record
counts will be sourced from `cargo run --locked --bin v06_work_inventory`
output, captured at the cycle that publishes `0.9.<build>` to origin.

## Closure propagation

When Epic 8 fires:

1. The supervisor reads `progress.md`'s per-cycle receipts.
2. The supervisor pulls per-record counts from the post-cycle `docs/work-inventory.json`.
3. This file is auto-populated from those receipts.
4. The tranche promotion PR message cites this file as `release-notes`.
