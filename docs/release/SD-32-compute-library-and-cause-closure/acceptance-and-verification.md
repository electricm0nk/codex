---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
---

# SD-32 Acceptance and Verification

The four gates (G0/G1/G2/G3) are the closure gates for this bundle. AT-32-* criteria below are the
*means* by which a gate is met; the gates themselves are the *ends*. General discipline
(identifier discipline, wired-integration discipline, `verify.sh` stage posture, dual-audit gate
inline enforcement) is inherited unchanged from `SD-31-corpus-closure-grind/acceptance-and-verification.md`
and is not re-derived here.

## Gate 0 — Census closure

**AT-32-G0-001.** Given the 158-book PCGen oracle directory tree at the pinned SHA
(`scripts/pcgen-oracle-pin.env`, `$PCGEN_CORPUS_ROOT`).

When the new independent walker (`scripts/census_independent.py`, Gate 0 deliverable) runs against
it.

Then the per-book diff against the inventory's 37-book roster reaches zero-unexplained: every
excluded directory is named and justified as scope (not oversight) in
`artifacts/gate-0-census-closure/excluded-directories.md`.

**AT-32-G0-002.** Given the inventory's current `docs/work-inventory.json` denominator of `38,372`
units (re-derived at Gate 0's first cycle, never transcribed from a prior wave).

When an honest object-definition rule is written for each kind (`feat`, `class`, `spell`,
`monster`, `monster_ability`, `equipment`, `equipment_modifier`, `companion`, `race`,
`race_trait`), covering `.MOD` continuations, `.COPY=` derivations, and template rows.

Then a "kind-unenumerable" category, if any exists, is named and counted — not pretended to be
zero. The verification command re-runs at every cycle that touches a kind's denominator
(`cargo run --locked --bin v06_work_inventory` + the new walker, both, not one without the other).

**AT-32-G0-003.** The four unbuilt books (Epic 4 scope) land their compiled rule sets before Gate 0
is declared closed. The decision to demote book onboarding from epic to precondition
(`scope-draft.md`, "The gap that makes Gate 0 necessary") is binding: running Gate 1 against an
open hole in the census guarantees the rerun the operator does not want.

**Verification commands:**

```bash
# Independent walker diff against inventory
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json --output artifacts/gate-0-census-closure/diff.json
test "$(jq -r '.unexplained' artifacts/gate-0-census-closure/diff.json)" = "0"

# Re-derive denominator
cargo run --locked --bin v06_work_inventory  # writes docs/work-inventory.json
jq '.total_units' docs/work-inventory.json   # the live denominator, not a frozen figure

# Per-book onboarding
scripts/verify.sh --only root-full  # gates that book onboarding's wiring lives behind
```

## Gate 1 — Shape closure

**AT-32-G1-001.** Given the closed Gate 0 census.

When the shape ledger (`scripts/shape_ledger.py`, Gate 1 deliverable) runs against it.

Then every unit in the closed census maps to one of the ten semantic families from SD-31 wave 31,
**or** the vocabulary is honestly extended — the family is added with measured units behind it,
not silently subsumed under another family.

**AT-32-G1-002.** The shape ledger fails closed on empty predicates, mirroring
`scripts/coverage_ledger.py`'s posture. A placeholder family with zero units behind it cannot
manufacture false 100% coverage. The verification command's exit code is the gate; "the script
ran and reported 100%" without a non-zero-on-empty proof is out of protocol.

**AT-32-G1-003.** Each family carries a stated proof width: which corpus shapes the proof does
and does not cover. The ten families are documented with their measured unit populations in
`epic-breakdown.md Epic 1 F1`. A cycle that adds a new family without stating the proof width is
out of protocol — the SD-31 wave 21 lesson (proof width gap → 73.4% fabrication) is the reason.

**Verification commands:**

```bash
# Shape ledger: every unit covered
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json \
  --output artifacts/gate-1-shape-closure/ledger.json
test "$(jq -r '.unclassified_count' artifacts/gate-1-shape-closure/ledger.json)" = "0"

# Closed-on-empty proof
python3 scripts/shape_ledger.py --inventory /dev/null 2>&1 | grep -q "no coverage" \
  && echo "GATE_G1_FAILS_CLOSED_ON_EMPTY_OK"

# Per-family unit counts match epic-breakdown.md
python3 -c "
import json
l=json.load(open('artifacts/gate-1-shape-closure/ledger.json'))
e=json.load(open('epic-breakdown.md'))  # parsed family table
assert l['families'] == e['families'], 'family counts drifted'
"
```

## Gate 2 — Engines

**AT-32-G2-001.** For each of the ten semantic families, an engine exists in
`src/rules_core/pilot_compute/` and emits values for the family's unit population. The engine
**may** be `formula_interpreter.rs` for the nine families it already handles (the binding layer
for F1..F9 is in place), **or** the generalised `bonus_stack_reader.rs` for the tenth family, or
a new engine. Whatever the implementation, it is named in the cycle receipt.

**AT-32-G2-002.** Every value emitted by every engine clears `derived_evaluator_fixture_check`,
whose expected value is transcribed from bytes the engine never reads. **An interpreted value
with no fixture is not done** (`decisions.md §3`, operator ruling §20).

**AT-32-G2-003.** Each engine's `acceptance-and-verification.md` entry (or appended section here)
states:

* The family it handles (F1..F10, named).
* The proof's unit population (measured, not estimated).
* The proof width — which shapes the corpus's value space the engine does **not** cover.
* The fixture sample size and how it was chosen.
* The re-derive command (`cargo run --locked --bin <engine> --emit-fixtures` or equivalent).

**AT-32-G2-004.** No engine is "complete" until it has been run corpus-wide once. The corpus-wide
run is itself a cycle, with its own receipt, and its own fixture-check, against the closed Gate 1
census. A cycle that runs an engine against a subset and declares the engine done is out of
protocol — the subset is not the population the engine claims to handle.

**Verification commands:**

```bash
# Per-engine fixture check
for engine in formula_interpreter bonus_stack_reader; do
  cargo run --locked --bin "$engine" --emit-fixtures \
    | tee "artifacts/gate-2-engines/${engine}.fixtures.json" \
    | python3 scripts/derived_evaluator_fixture_check.py --input /dev/stdin
done

# Corpus-wide run, one engine at a time, fixture-checked
cargo run --locked --bin formula_interpreter --corpus-wide \
  --output artifacts/gate-2-engines/formula_interpreter.corpus-wide.json
python3 scripts/derived_evaluator_fixture_check.py \
  --input artifacts/gate-2-engines/formula_interpreter.corpus-wide.json \
  --expected-from "$(PCGEN_CORPUS_ROOT)/expected.json"  # bytes the engine does NOT read
```

## Gate 3 — Closure invariant

**AT-32-G3-001.** A standing test exists (`scripts/shape_coverage_standing_gate.py` or wired into
`scripts/verify.sh` as a real stage, named `shape-coverage-standing-gate` or equivalent) that goes
red when any object appears that no shape covers. The gate runs on every `scripts/verify.sh`
invocation — not on demand, not as a courtesy check.

**AT-32-G3-002.** The gate fails closed on an empty predicate. A placeholder shape with zero units
behind it cannot manufacture false coverage; a placeholder predicate with zero matches cannot
manufacture false 100%. The verifier itself is part of the proof.

**AT-32-G3-003.** The gate's first live run is the closure cycle's own
`scripts/verify.sh --only shape-coverage-standing-gate`, producing a receipt that:

* Names the per-family unit count at closure.
* Names the unclassified count (must be zero for Gate 3 to be met).
* Names the corpus SHA (`scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SHA`) against which the
  count was re-derived.

**Verification commands:**

```bash
# The gate itself, wired into verify.sh
scripts/verify.sh --list | grep shape-coverage-standing-gate

# Per-cycle invocation
scripts/verify.sh --only shape-coverage-standing-gate 2>&1 | tee \
  artifacts/gate-3-closure-invariant/$(date +%Y%m%d-%H%M%S).run.json

# Closed-on-empty proof
echo '{}' | python3 scripts/shape_coverage_standing_gate.py 2>&1 | grep -q "no coverage" \
  && echo "GATE_G3_FAILS_CLOSED_ON_EMPTY_OK"
```

## Epic-level acceptance criteria (in addition to the four gates)

**AT-32-E1-001 — Compute library delivers 3,201 ceiling.** (Epic 1 F1/F2/F3 deliver.) The
ceiling figure is the union of unit populations across the ten families, minus the 1,747-unit
flat-constant family (which gets zero benefit from any shared function — see
`epic-breakdown.md Epic 1`). The ceiling is not a target; it is the measured upper bound.

**AT-32-E2-001 — Cause closure closes by class, not by instance.** (Epic 2 T2a/T2b deliver.) The
eight blocker shapes (T2a, T2b, T9, T4, T12, T5, T1, T3 — see `epic-breakdown.md Epic 2`) are
each closed corpus-wide rather than instance-by-instance. A cycle that closes T2a for a single
class and stops is out of protocol; the rule is *class-closure*, not *instance-closure*.

**AT-32-E3-001 — Class reachability.** (Epic 3.) The 77 prestige classes have entry-requirement
gating that exists nowhere in the codebase today; the cycle that builds it cites the
`compute_class_chassis` call site and proves the gating runs (fixture-checked, of course). The
18 real base classes without tables and the 28 books-without-ruleset both feed this epic from
Epic 4.

**AT-32-E4-001 — Book onboarding.** (Epic 4.) Four books have no compiled rule set; ~422 units
behind a missing `RuleSetId` enum variant. The `adventurers_guide` precedent shows cost is
~1.5–2h per book, dominated by ~7 count-pinning files. **Sequenced behind Gate 0 by construction**
(`scope-draft.md`, `decisions.md §2`).

**AT-32-E5-001 — Automation, decided on evidence.** (Epic 5.) The protective self-erasure sweep
across all Rust generators runs **before Gate 0**. A cycle that touches an engine before this
sweep is out of protocol — the failure mode is documented in `artifacts/HANDOFF.md`
(`scripts/derive_derived_evaluator_fixtures.py` was destroying 2,110 fixture entries per run
before the fix).

## What does NOT close SD-32

A figure in a release note. A green `scripts/verify.sh` exit code. A `gate-X` Slack post. None of
these are proofs; they are *reports* of proofs. The proof is the receipt; the receipt names the
corpus SHA, the command, and the fixture whose expected value was transcribed from bytes the
engine never reads. **A cycle without all three is not a closed cycle.**
