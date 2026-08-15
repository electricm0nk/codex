---
canonical: true
owner: sd31-orchestrator
purpose: Unattended-mode blocker and open-question log for SD-31. The operator reads THIS file on check-in.
started: 2026-08-15
---

# SD-31 — Open Issues, Blockers, and Operator Rulings Needed

**How this file works.** SD-31 runs unattended. A cycle never stops to ask a question. When a cycle
hits a hard block, needs an operator ruling, or takes a default it wants reviewed, it appends a row
here and keeps working. The operator reads this file on check-in and answers in the `Operator ruling`
column.

**Rules for cycles appending here:**
- Append, never rewrite another entry.
- One row per issue. Give the exact command and exit code, not a narration.
- `Severity`: `BLOCKER` (work stopped on this card), `RULING-NEEDED` (proceeded on a default, operator
  should confirm), `NOTE` (recorded, no action wanted).
- A Structural Exclusion Register proposal (`decisions.md §3`) is logged here as `RULING-NEEDED` with
  a pointer to the proposal — a cycle may propose, only the operator grants.

## Open

| # | Opened | Cycle-id | Severity | Issue | What the cycle did instead | Operator ruling |
|---|--------|----------|----------|-------|-----------------------------|-----------------|
| 1 | 2026-08-15 | SD31-E2-F1-001 | NOTE | `wiring_class::CorpusLines::line()` (`src/rules_core/wiring_class.rs:758`) resolves a unit's corpus row via a **single-level** `dir.join(file)` join. Several books' `.lst` files live in nested subdirectories (`core_essentials/races/<race>/*.lst`, `ultimate_combat/support/*.lst`, `horror_adventures/support/*.lst`, `inner_sea_world_guide/_pfs/*.lst`, `advanced_race_guide/_pfs/*.lst`, `adventurers_guide/support/*.lst`, etc.) — the join silently misses, `CorpusLines::line()` returns `None`, and the unit falls into `ambiguous:no_corpus_line` (D0) even though its row genuinely exists. Re-derived corpus-wide: `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); u=[x for x in d['units'] if x.get('book')!='beginner_box']; ncl=[x for x in u if x.get('wiring_class_reason')=='no_corpus_line']; print(len(ncl))"` → **1,707** units (was documented as 47 in `docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`, now stale). Recursive-glob check confirms **100% (1,707/1,707)** of these rows are findable somewhere under the book directory — none are genuinely synthetic/provenance-free. This is **80.9% of the whole 2,109-unit `ambiguous` population**. Not a blocker to SD31-E2-F1 (the ground-truth sample hand-resolved all 40 affected sampled units from the real row instead), but it is load-bearing for Epic 2-F2 (classifier build) and F3 (`ambiguous` dead-end closure): fixing the path join before building/accepting a classifier will move the vast majority of "ambiguous" units to their real class for free, and F3's "genuinely unreachable, propose to Structural Exclusion Register" review must not run against the current, badly-inflated `no_corpus_line` bucket. | Recorded the bug with full re-derivation and worked examples in `artifacts/SD31-E2-F1-ground-truth-methodology.md` §Finding A; hand-labelled the 40 affected sampled units from their real corpus rows (not the engine's `ambiguous` verdict) so the ground-truth sample itself is not corrupted by the same bug. No production code changed (out of this card's scope). | *(needs operator/Epic-2-F2 owner confirmation before any fix lands)* |
| 2 | 2026-08-15 | SD31-E2-F1-001 | NOTE | `wiring_class::signals()`'s scalar/arithmetic scan (`src/rules_core/wiring_class.rs:398-420`) reads the WHOLE field text, not just the magnitude value. Two consequences, both false positives for `derived`: (a) `BONUS:STAT|<ABILITY>|<literal>` fields always contain the ability's own name as the STAT selector (e.g. `STR`,`DEX`,`INT`), which collides with `SCALARS_WORD` regardless of whether the *value* is scalar-dependent — a flat `BONUS:STAT|DEX|2|TYPE=Racial` (`core_rulebook:race_trait:2_dexterity`) or `BONUS:STAT|INT|2|TYPE=Enhancement` (`ultimate_equipment:equipment:staff_of_mithral_might`) is misclassified `derived` instead of `static`. (b) `has_arith`'s unconditional `value.contains('/')` check fires on PCGen's literal `DR:10/Cold Iron`/`CR:1/3` bypass-type notation, not just division (`bestiary:monster:neothelid` compounds both). Confirmed in 3 of 150 sampled units as a clean single-cause misclassification (others carrying the same false-positive signal, e.g. `core_essentials:companion:pig`, still land on the correct class because a genuinely separate formula also fires). Full worked examples in the methodology note §Finding B. | Hand-labelled the 3 affected units `static` with the false-positive mechanism documented per-unit in the ground-truth JSON's `token_evidence` field. | *(informational — feed into Epic 2-F2's classifier acceptance work, not a blocker)* |

## Resolved

| # | Opened | Closed | Cycle-id | Issue | Resolution |
|---|--------|--------|----------|-------|------------|
| — | — | — | — | *(none yet)* | — |
