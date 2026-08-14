# SD-32 Technical Requirements

## Pre-loop prerequisites

- `tranche/10` checked out, `git pull --ff-only` clean.
- `SD-30-class-feature-archetype-bundle`'s Epic 1 (identifier cleanup) and Epic 2 (pre-launch)
  `COMPLETE`, cited not re-verified per cycle.
- No PI-gate prerequisite (this package does not write corpus content — see `README.md` dependency
  position).

## Normative requirements

- Every race chassis addition passes AT-32-001 (DoD-8 on-screen verification) and AT-32-002 (no
  regression in the 18 already-modeled races).
- The verdict-path classifier is never accepted on movement count alone — AT-32-003's sample-agreement
  bar is mandatory, and the hand-labelled sample (Epic 2-F1) must exist and be committed before any
  classifier code is written.
- Every handoff to `SD-31-corpus-closure-grind` is cited on both sides (AT-32-004).
- `scripts/verify.sh` full passes before any cycle's commit, mirroring SD-30's own standing requirement.

## Out of scope (technical)

- Ingesting `race`/`race_trait` content using the new chassis, or reclassifying `class_feature`'s
  `unknown` bucket using the new verdict paths — that is `SD-31-corpus-closure-grind`'s work, consuming
  this package's output.
- The PI-screening gate, dashboard producer, and consumer-delta probes — SD-30's surfaces.
- Real-time execution engines (RNG, opponent state, turn sequencing) — unchanged repo-wide constraint.
