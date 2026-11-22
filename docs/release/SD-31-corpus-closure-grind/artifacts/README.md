# SD-31 Artifacts

Per-cycle receipts and finding logs land here as cycles run.

## Shared instrument tooling

This package does not duplicate the corpus-wide instrument scripts (`derive-movable-mass.py`,
`static-sweep-coverage.py`, `why-in-progress-equipment-stalls.py`, `ground-spell-units/`) — they remain
canonical under `../SD-30-class-feature-archetype-bundle/artifacts/`, since SD-30's Epic 0 owns their
maintenance. Cycles in this package invoke them by that path; see
`SD-30-class-feature-archetype-bundle/artifacts/README.md` for each script's current status (note:
`derive-movable-mass.py` currently raises on the `literal-verified`/`fixture-verified` rungs per
`SD-30-.../acceptance-and-verification.md AT-30-015`'s own note — use the hand-derivation command that
file documents until the script itself is fixed).
