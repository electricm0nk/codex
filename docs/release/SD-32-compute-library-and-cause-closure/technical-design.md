---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
---

# SD-32 Technical Design

## Architectural surface

This bundle's work touches five distinct surfaces, organised by the four gates that govern them:

### Gate 0 — Census closure

- **`scripts/census_*.py` (new)** — the independent walker that proves the existing walker isn't
  blind. Structured as `reader` / `analyser` / `reporter` so the seam is present and reusable.
  The LST reader exists in substance already (every PCGen-derived walker in `scripts/`); the new
  walker reuses it and competes with it on output equality.
- **Audit of `data/corpus/`** — the 158 directories in the pinned oracle against the 37 books in
  the active inventory. Every excluded directory gets named and justified as scope, not
  oversight. Object-definition rules for `.MOD` continuations, `.COPY=` derivations, and
  template rows are written down before any count is trusted.

### Gate 1 — Shape closure

- **`scripts/coverage_ledger.py` (extended)** or `scripts/shape_ledger.py` (new) — the same
  fail-closed-on-empty posture that `coverage_ledger.py` already uses for the 46-group partition,
  raised to shapes. Vocabulary lives in two places: the **procedure** (extract / normalise /
  cluster / count / report) is portable per `decisions.md §4`; the **PF1e binding** (the ten
  semantic families SD-31 wave 31 identified) is system-specific and not portable.

### Gate 2 — Engines

- **`src/rules_core/pilot_compute/formula_interpreter.rs`** — the existing 9-of-10 engine, the
  ground truth for everything else. Authorised by operator ruling §20 on the explicit condition
  that every interpreted value clears `derived_evaluator_fixture_check`; that condition is
  restated as Gate 2's normative requirement.
- **`src/rules_core/pilot_compute/bonus_stack_reader.rs`** (generalised) — wave 26's 329-line
  binding-layer precedent. The pattern *"read the producers of a named variable and sum them"*
  targets the **canonical F4 family** ("named-counter/pool variable", `scripts/shape_ledger.py`)
  — not the unrelated F10 (a 3-unit level-threshold step-count heuristic); this was a labelling
  defect card `family-vocabulary-reconciliation` fixed (`decisions.md §12a`,
  `artifacts/gate-1-shape-closure/family-vocabulary.md`). SD-31 wave 31 measured this pattern
  reaching **77.2% (893/1,156)** of the corpus's distinct custom identifiers by an identifier-wide
  walk; `family-vocabulary.md`'s own independent, F4-predicate-scoped re-derivation (a narrower,
  differently-defined population — see that document §3) found 92.4% (390/422) of F4-shaped
  bare-identifier strings specifically resolvable the same way. One lane framed the binding layer
  at 46.8% using a narrower mechanism; the broader already-proven one reaches the figures above.
  The correction ran in both directions, which is why both were re-derived.

### Gate 3 — Closure invariant

- **`scripts/shape_coverage_standing_gate.py` (new)** — mirrors `scripts/coverage_ledger.py`'s
  shape: a verifier that fails closed on an empty predicate, so a placeholder shape cannot
  manufacture false 100% coverage. The gate is wired into `scripts/verify.sh` as a real stage,
  not a courtesy check.

### Epic 5 — Automation, decided on evidence

- **Rust generator self-erasure sweep** — the protective check across all 29 Rust generators
  (`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`, verified 2026-08-22; HANDOFF's "~30" was an
  estimate — Epic 5's first cycle re-runs the command and states the live count). One
  already live-reproduced wiping 93 spell and 15 equipment records
  (`gen_advanced_race_guide`); the fix that closed it is the template, not an exception.
- **`scripts/coverage_ledger.py`** (read, not modified) — proves inventory completeness
  mechanically. Already earns its place; no changes.

## What this bundle does not touch

- **`scripts/observer/pf1e_dashboard_producer.py` and its `doneness_verdict()` table** —
  SD-30's Epic 0 surface, read-only from this bundle. Generator/producer same-commit discipline
  (`SD-30-.../state-goals-and-lessons.md §1.3` hazard 4) binds any future touch; this bundle does
  not own the producer.
- **`pi_screening.rs` and the declared-PI reader wiring into the ingest path** — SD-30's Epic 3
  surface, consumed (via the cross-SD gate) not modified here. SD-32 is not an ingest bundle and
  has no per-book PI-screening call site.
- **Reach-gate (`apps/desktop/src-tauri/src/reach_gate.rs`)** — read as the definition of done
  for any record this bundle eventually surfaces into the desktop app; never rewritten or
  generalised. SD-32 does not produce records that ship into the player-facing app in its own
  right; the engines it builds are surfaced via the same `reach_gate.rs` contract SD-30
  established.
- **Identifier cleanup** — not a SD-32 epic. Source-identifier discipline is enforced by the
  dual-audit gate in `workflow-instruction.md §6` (the grep for `sd[0-9]+_` / `SD[0-9]+_` / `Sd[0-9]+`
  patterns), not by an epic. SD-32 inherits the discipline unchanged.

## File-disjointness

By construction, each gate's work touches a different surface:

| Phase | Surface |
|---|---|
| Gate 0 | `scripts/census_*.py` (new); read-only on corpus |
| Gate 1 | `scripts/{coverage,shape}_ledger.py` (read or new); read-only on corpus |
| Gate 2 | `src/rules_core/pilot_compute/*.rs`; new test files |
| Gate 3 | `scripts/shape_coverage_standing_gate.py` (new); `scripts/verify.sh` |
| Epic 5 | `src/bin/{gen_,ingest_,enrich_}*.rs` (read + assertion); the protective sweep is read-mostly, the fix where needed is per-generator |

A cycle that finds itself editing a file outside this list should stop and check whether it has
drifted out of scope. Parallel phases (Pre-G0, Gate 2's two engine chains, Epics 1-3) must use
`isolation: 'worktree'` (`workflow-instruction.md §3`); Gate 0 (card 4 behind card 3), Gate 1
(single card) and Gate 3 are serial by construction.
