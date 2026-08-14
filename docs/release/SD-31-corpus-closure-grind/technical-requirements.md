# SD-31 Technical Requirements

## Pre-loop prerequisites

- `tranche/10` checked out, `git pull --ff-only` clean.
- `SD-30-class-feature-archetype-bundle`'s Epic 1 (identifier cleanup) and Epic 2 (pre-launch)
  `COMPLETE`, cited not re-verified per cycle (they gate this package the same way they gated SD-30's
  own former Epic 4/6/10).
- `cargo run --locked --bin v06_work_inventory` regenerates `docs/work-inventory.json` at cycle-0 of
  any card that cites a figure from it — never transcribed stale.

## Normative requirements

- Every ingested record satisfies the reach-gate prime rule (`AT-31-002`).
- Every ingest cycle in Epic 3/4/5 cites its cross-SD PI-gate `COMPLETE` receipt before claiming a book
  (`AT-31-003`).
- Every Epic 4 card records its raw-vs-workable split with command before planning cycles (`AT-31-004`).
- No blended per-class measurement figure (`AT-31-001`).
- `scripts/verify.sh` full passes before any cycle's commit, mirroring SD-30's own standing requirement
  (`SD-30-.../decisions.md §18`, AT-30-002).

## Out of scope (technical)

- The PI-screening gate's own implementation — that is SD-30's Epic 3, consumed not built here.
- The dashboard producer's `doneness_verdict()` table and consumer-delta probes — SD-30's Epic 0.
- The race chassis and verdict-path classifier — `SD-32-engine-capability-builds`.
- Real-time execution engines (RNG, opponent state, turn sequencing) — unchanged repo-wide constraint.
