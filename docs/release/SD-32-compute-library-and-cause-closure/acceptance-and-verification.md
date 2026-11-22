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
(`scripts/pcgen-oracle-pin.env`), read from the **repo-local slot**
`artifacts/corpus/operator-supplied/pcgen/data` (`$PCGEN_CORPUS_ROOT` as exported by
`workflow-instruction.md §2.1`; `artifacts/corpus/README.md`) — never from a path outside the repo.

When the new independent walker (`scripts/census_independent.py`, Gate 0 deliverable) runs against
it.

Then the per-book diff against the inventory's 37-book roster reaches zero-unexplained: every
excluded directory is named and justified as scope (not oversight) in
`artifacts/gate-0-census-closure/excluded-directories.md`.

> **Derivation command (added `t9-onboarding`, 2026-08-23, closing the `## DISCOVERED` finding that
> this figure carried no reproducible command):** `python3 scripts/census_independent.py
> --pcgen-root "$PCGEN_CORPUS_ROOT" --inventory docs/work-inventory.json --output <path>` →
> `discovered_book_dirs` in the JSON output. Re-run against the pinned oracle 2026-08-23:
> `discovered=186`. The `158` above is Gate 0's frozen launch-time figure and is **not** re-stated
> as current — 186 is the live, reproducible count; both numbers coexist deliberately (`decisions.md
> §12c`: name the population, never quote a bare total).

**AT-32-G0-002.** Given the inventory's current `docs/work-inventory.json` denominator of `38,372`
units (re-derived at Gate 0's first cycle, never transcribed from a prior wave).

> **Derivation command (added `t9-onboarding`, 2026-08-23, same finding as above):**
> `jq '.totals.units' docs/work-inventory.json`. Re-run 2026-08-23: **49,490** — the inventory has
> grown substantially since Gate 0's launch (card 15's `no_record`-closure campaign, `decisions.md
> §20`). `38,372` is frozen launch-time history, not current; do not quote either number without
> re-running the command above and naming the date, per `decisions.md §12c`.

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
# Oracle is the repo-local slot (workflow-instruction.md §2.1)
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/verify.sh --only preflight-oracle

# Independent walker diff against inventory (scripts/census_independent.py is the Gate 0 deliverable — card 3 creates it)
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
not silently subsumed under another family. **`unclassified_count: 0` alone does not satisfy this
criterion** (`decisions.md` §14a/§14b, finding of record from the 2026-08-22 reclosure): F0 ("no
formula content") collapses three structurally different populations — `matched` (a real corpus
record with a formula), `no_formula_tokens` (a real corpus record with none), and `no_record` (the
join found no corpus record at all, i.e. absence of evidence, not evidence of no formula). A
statement of this criterion being met must quote the `join_status` split alongside
`unclassified_count`, not `unclassified_count` alone.

**AT-32-G1-002.** The shape ledger fails closed on empty predicates, mirroring
`scripts/coverage_ledger.py`'s posture. A placeholder family with zero units behind it cannot
manufacture false 100% coverage. The verification command's exit code is the gate; "the script
ran and reported 100%" without a non-zero-on-empty proof is out of protocol.

**AT-32-G1-004** (added `decisions.md` §14b). Every quoted statement of Gate 1's coverage — in the
ledger's own printed output, `ledger.json`, the Gate 3 standing gate's output,
`family-vocabulary.md`'s canonical table, and this file's own criteria — carries the `join_status`
split (`matched` / `no_formula_tokens` / `no_record`) alongside any `unclassified_count` or family
total it names, per `decisions.md` §12c (no bare totals). F0 itself is not deleted, renamed, or
subsumed to satisfy this — the split is surfaced *in addition to* F0, not instead of it.

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

# AT-32-G1-004: the join-status split, not unclassified_count alone
# (decisions.md §14b) -- expect matched=4802 no_formula_tokens=9705
# no_record=21521 over population 36028 at the pinned corpus SHA (moved
# from 4802/9720/13968 over 28490 when a chain of card 15 landings added
# Kind::Ability (4824 new-kind units, all no_record) plus 112 no_record
# feat units surfaced by the same classifier, then narrowed
# is_internal_category for Kind::ClassFeature (2617 more no_record rows),
# then wired 15 class_feature units to text-complete -- see
# no_record_budget_provenance.jsonl repin 4; re-derive with the command
# below, which always reflects the live population rather than a frozen
# figure).
jq -r '.join_status_counts' artifacts/gate-1-shape-closure/ledger.json

# Closed-on-empty proof
python3 scripts/shape_ledger.py --inventory /dev/null 2>&1 | grep -q "no coverage" \
  && echo "GATE_G1_FAILS_CLOSED_ON_EMPTY_OK"

# Per-family unit counts match the canonical vocabulary artifact (hand cross-check, not
# JSON diff -- family-vocabulary.md is prose/markdown, not a machine-readable file).
# `epic-breakdown.md` Epic 1's F1/F2/F3 rows are three *work items*, not a semantic-family
# count table -- do not diff against them (card `family-vocabulary-reconciliation` correction,
# `decisions.md §12a`, retro id `1787437987996-gate-1-shape-0ae65f`).
python3 -c "
import json
l=json.load(open('artifacts/gate-1-shape-closure/ledger.json'))
print(l['families'])
"
# then diff the printed per-family counts against the canonical family table in
# artifacts/gate-1-shape-closure/family-vocabulary.md §1 (regenerate via
# `python3 scripts/family_vocabulary_reconcile.py --output-json artifacts/gate-1-shape-closure/family-vocabulary.json
#  --output-md artifacts/gate-1-shape-closure/family-vocabulary.md`) by eye; a cycle that finds a
# mismatch stops and reports it, it does not silently update whichever side is more convenient
```

## Gate 2 — Engines

**AT-32-G2-001.** For each of the eleven canonical families (`scripts/shape_ledger.py` F0-F10;
`artifacts/gate-1-shape-closure/family-vocabulary.md`), an engine exists in
`src/rules_core/pilot_compute/` and emits values for the family's unit population. The engine
**may** be `formula_interpreter.rs`, whose grammar directly evaluates all nine formula-bearing
families F1..F9 (F0 carries no formula to evaluate), **or** the generalised
`bonus_stack_reader.rs` as the binding layer that resolves the producer-bound subset of **F4**'s
bare-identifier values (a value formula_interpreter's grammar can evaluate once bound, but cannot
resolve on its own), or a new engine. **F10 is not the binding-layer family** — it is a 3-unit
level-threshold step-count heuristic formula_interpreter's grammar already evaluates directly like
any other F1..F9 member; the "tenth family needs a binding layer" framing was the labelling defect
card `family-vocabulary-reconciliation` fixed (`decisions.md §12a`). Whatever the implementation,
it is named in the cycle receipt.

**AT-32-G2-002.** Every value emitted by every engine clears `derived_evaluator_fixture_check`,
whose expected value is transcribed from bytes the engine never reads. **An interpreted value
with no fixture is not done** (`decisions.md §3`, operator ruling §20).

**AT-32-G2-003.** Each engine's `acceptance-and-verification.md` entry (or appended section here)
states:

* The family it handles (F1..F10, named).
* The proof's unit population (measured, not estimated).
* The proof width — which shapes the corpus's value space the engine does **not** cover.
* The fixture sample size and how it was chosen.
* The re-derive command (`cargo run --locked --bin <engine> -- --emit-fixtures` or equivalent — the
  per-engine binary is a Gate 2 deliverable; today `formula_interpreter` and `bonus_stack_reader`
  are library modules under `src/rules_core/pilot_compute/`, not `src/bin/` targets).

**AT-32-G2-004.** No engine is "complete" until it has been run corpus-wide once. The corpus-wide
run is itself a cycle, with its own receipt, and its own fixture-check, against the closed Gate 1
census. A cycle that runs an engine against a subset and declares the engine done is out of
protocol — the subset is not the population the engine claims to handle.

**Verification commands:**

```bash
# The fixture-check gate is the existing Rust CLI src/bin/derived_evaluator_fixture_check.rs
# (library: src/rules_core/derived_evaluator_fixture_check.rs; standing fixture:
# tests/fixtures/rules_core/derived-evaluator-fixtures.json). There is no scripts/*.py for it.
cargo run --locked --bin derived_evaluator_fixture_check -- --help   # flags are the CLI's own; cite the ones used

# Per-engine fixture emission (the `--bin <engine>` targets are Gate 2 deliverables — cards 6/7
# create them; until then this block is the contract, not a runnable command)
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
for engine in formula_interpreter bonus_stack_reader; do
  cargo run --locked --bin "$engine" -- --emit-fixtures \
    > "artifacts/gate-2-engines/${engine}.fixtures.json"
done

# Corpus-wide run, one engine at a time (card 8, one cycle per engine), fixture-checked against
# an expected-value file that lives in THIS bundle's artifacts and is transcribed from oracle bytes
# the engine does NOT read (never regenerated from engine output)
cargo run --locked --bin formula_interpreter -- --corpus-wide \
  --output artifacts/gate-2-engines/formula_interpreter.corpus-wide.json
cargo run --locked --bin derived_evaluator_fixture_check -- \
  --input artifacts/gate-2-engines/formula_interpreter.corpus-wide.json \
  --expected-from artifacts/gate-2-engines/formula_interpreter.expected.json
# Receipt cites: grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env
```

### AT-32-G2-003 entry — `formula_interpreter.rs` / families F1..F9 (kanban `#6`)

Per AT-32-G2-003's own text ("each engine's `acceptance-and-verification.md` entry ... states"),
appended here rather than in a separate file, since `formula_interpreter.rs` is a library module,
not a `src/bin/` target (`technical-design.md` Gate 2).

* **Family it handles:** F1..F9 — `scripts/shape_ledger.py`'s nine formula-bearing shape families,
  which the interpreter's grammar evaluates directly once a formula's identifiers are bound.
  **F4**'s producer-bound bare-identifier subset additionally needs `bonus_stack_reader.rs`'s
  binding layer (kanban card 7's own scope) to resolve *what value* the identifier holds before
  this engine's grammar can evaluate it; F4's grammar reach itself is still this card's own claim.
  F10 is not the binding-layer family (a labelling defect fixed by card
  `family-vocabulary-reconciliation`, `decisions.md §12a`) — it is a 3-unit level-threshold
  step-count family this engine already evaluates directly like any other F1..F9 member.
* **Proof's unit population (measured):** per Gate 1's `artifacts/gate-1-shape-closure/ledger.json`
  family rollup (re-derivable: `python3 -c "import json; d=json.load(open('artifacts/gate-1-shape-closure/ledger.json')); print({k:v['count'] for k,v in d['families'].items()})"`
  from `docs/release/SD-32-compute-library-and-cause-closure/`, or re-run the ledger fresh via
  `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json`
  from the repo root and read `/tmp/ledger.json` the same way), F1=1,790, F2=1,490, F3=303,
  F4=570, F5=361, F6=211, F7=5, F8=41, F9=27 — **4,798 not-done units** across the nine in-scope
  families (24,914-unit not-done population).
* **Proof width — what this card's fixture-check does NOT cover:** one real corpus formula sample
  per family (9 total), not every one of the 4,798 units' own formula text. This is a **grammar
  reach** proof ("the interpreter's parser+evaluator accepts and correctly computes this family's
  shape, on a representative real sample"), not a claim that all 4,798 units individually clear
  the fixture-check — that corpus-wide, per-unit claim is AT-32-G2-004's own criterion, carried by
  kanban card 8 (`gate-2-corpus-wide-runs`), not this card. Known narrower gaps inherited from the
  engine's own module doc (`formula_interpreter.rs` lines 99-212, unchanged by this cycle):
  `classlevel(...)` does not verify its class-name argument (single `__LEVEL__` binding only, no
  per-class environment) — the F6 fixture entry below is written to be correct under that known
  restriction (single-class formula), not a claim the restriction is fixed; `skillinfo(...)`
  implements only the `"TOTALRANK"` first argument; `if(...)`'s condition still refuses a bare
  numeric value; five real PCGen functions (`var`, `count`, `mastervar`, `charbonusto`, `cl`) are
  refused as unimplemented, never guessed.
* **Fixture sample size and how it was chosen:** 9 entries, one per in-scope family, each a real
  `BONUS`/`DEFINE` formula segment pulled from `data/corpus/**/*.json`'s `raw_tokens` (via
  `extract_formula_field`, the same positional heuristic Gate 1's ledger and
  `tests::corpus_shape_coverage` both use) and independently confirmed byte-identical against the
  pinned PCGen oracle checkout at authoring time (`sha256sum`/`sed` against the named upstream
  `.lst` file and line — see the fixture's own `"derivation"` field and the cycle receipt).
  Selection was **first real match found** walking `data/corpus` in filesystem order per family
  (a deterministic, reproducible procedure, not a hand-picked "nice" example) — the script that
  produced the walk is
  `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/001_cycle_receipt.md`'s
  own commands.
* **Re-derive command:**
  ```bash
  cargo test --locked --test formula_interpreter_family_fixture_check
  ```
  Fixture: `tests/fixtures/rules_core/formula-interpreter-family-fixtures.json`. Test:
  `tests/formula_interpreter_family_fixture_check.rs`. There is no `--bin formula_interpreter`
  target yet (technical-design.md's own note that the per-engine binary is a still-open Gate 2
  deliverable) — this `cargo test` invocation is today's real re-derive command for this card's
  own AT-32-G2-001/002/003 claim; AT-32-G2-004's corpus-wide run and its own binary/CLI target are
  kanban card 8's scope.

## Gate 3 — Closure invariant

**AT-32-G3-001.** A standing test exists (`scripts/shape_coverage_standing_gate.py` or wired into
`scripts/verify.sh` as a real stage, named `shape-coverage-standing-gate` or equivalent) that goes
red when any object appears that no shape covers. The gate runs on every `scripts/verify.sh`
invocation — not on demand, not as a courtesy check. **The red-proof must reach this state through
the real classification path (real `classify_unit`/`build_corpus_index`/`build_ledger`), never by
patching `shape_ledger.build_ledger` or any other code under test to fabricate it**
(`decisions.md` §1a/§14a, finding of record: the prior version of this criterion was accepted on a
`mock.patch`-based proof that could not occur for any real object — 80 fabricated units returned
`exit 0, PASS`). The real, organically-reachable invariant is `join_status == "no_record"`: a unit
whose join finds no corpus record is precisely "an object no shape covers." Because 10,530 of the
current 25,055-unit population are already `no_record` and cannot be closed to zero within this
cycle, the gate enforces a **committed, evidence-gated budget** on `no_record`'s share of the
population (`NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` in
`scripts/shape_coverage_standing_gate.py`) rather than demanding zero outright.

The budget is not a pure shrink-only ratchet: card 15 (`decisions.md` §12b) must enumerate ~9,000
real objects into 8 brand-new kinds the corpus ingest pipeline does not reach yet, and every one of
them is organically `no_record` on landing through no defect of its own — a budget that could only
shrink would go redder with every unit of that mandated work, "a gate pointed at the wrong thing"
rather than a working one. Instead the two constants move only together with a matching, append-only
entry in `no_record_budget_provenance.jsonl`, each naming the real git commit that landed the
population growth and the reason it is legitimate enumeration rather than drift.
`test_shape_coverage_standing_gate.py`'s `BudgetProvenanceTest` mechanically enforces: the constants
match the log's latest entry; population strictly increases entry-to-entry (no repin without real
growth); a repin's `no_record` delta never exceeds its population delta (the budget cannot widen
faster than content was added); and every `evidence_commit` is a real, reachable commit. A run with
no matching provenance entry — including the orchestrator's 80-fabricated-object reproduction, which
lands no commit and adds no log entry — is still measured against the last **committed** baseline
and fails it.

**AT-32-G3-002.** The gate fails closed on an empty predicate. A placeholder shape with zero units
behind it cannot manufacture false coverage; a placeholder predicate with zero matches cannot
manufacture false 100%. The verifier itself is part of the proof.

**AT-32-G3-003.** The gate's first live run is the closure cycle's own
`scripts/verify.sh --only shape-coverage-standing-gate`, producing a receipt that:

* Names the per-family unit count at closure.
* Names the unclassified count (must be zero for Gate 3 to be met).
* Names the `join_status` split (`matched`/`no_formula_tokens`/`no_record`) and states whether the
  `no_record` share is within its committed budget (`decisions.md` §14b) — not just
  `unclassified_count`.
* Names the corpus SHA (`scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SHA`) against which the
  count was re-derived, read from the repo-local slot `artifacts/corpus/operator-supplied/pcgen`.

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

# AT-32-G3-001 red-proof, through the REAL path -- no mock.patch anywhere.
# 80 real units across 8 kinds, all pointing at a nonexistent corpus file;
# corpus_root itself is unreachable, so the join organically produces
# no_record rows. Must now be nonzero exit / no_record_budget_exceeded:
# true (before this fix: exit 0, PASS -- decisions.md §14a).
python3 -c "
import sys; sys.path.insert(0,'scripts')
import shape_coverage_standing_gate as G
u=[{'id':f'b:{k}:{i}','kind':k,'book':'b','status':'not-started','wiring_class':'static','source_file':'totally_fake_file.lst','source_line':i} for k in ('ability','skill','template','deity','power','domain','language','kit') for i in range(1,11)]
print(G.run_gate({'units':u}, corpus_root='/nonexistent'))"
```

## Epic-level acceptance criteria (in addition to the four gates)

**AT-32-E1-001 — Compute library delivers 3,201 ceiling.** (Epic 1 F1/F2/F3 deliver.) The
ceiling figure is the union of unit populations across the ten families, minus the 1,747-unit
flat-constant family (which gets zero benefit from any shared function — see
`epic-breakdown.md Epic 1`). The ceiling is not a target; it is the measured upper bound.

**AT-32-E2-001 — Cause closure closes by class, not by instance.** (Epic 2 T2a/T2b deliver.) The
eight measured blocker shapes (T2a, T2b, T9, T4, T12, T5, T1, T3 — see `epic-breakdown.md Epic 2`;
T5 is credited via Epic 4's card 4 and T3 via Epic 5's card 1, and Epic 2's receipt cites both rather
than re-closing them) are each closed corpus-wide rather than instance-by-instance. T8/T7 (16 units
together) close opportunistically; T10 has no unit count and is a census-process item. A cycle that closes T2a for a single
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
across all 29 Rust generators (`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`) runs **before Gate 0**. A cycle that touches an engine before this
sweep is out of protocol — the failure mode is documented in `artifacts/HANDOFF.md`
(`scripts/derive_derived_evaluator_fixtures.py` was destroying 2,110 fixture entries per run
before the fix).

## Bundle closure criterion (in addition to the four gates and five epics)

**AT-32-CLOSE-001 — The bundle closure epilogue actually ran, not just the PR.** The closure trigger
is the **Definition of Done — all four gates' AT-32-* criteria met AND every Epic 1-5 kanban card at
`complete`** — never a wave count, date, or budget (operator rulings 2026-08-22, `decisions.md §2`
and **`decisions.md §10`**). Per `workflow-instruction.md §13`, closure then requires, in order,
before the PR opens:

1. Every gate G0-G3 met and **every Epic 1-5 card at `complete`**. A card at `returned-to-backlog`,
   `in-progress`, or `DISCOVERED-forked` blocks closure, as does a card marked `complete` with a
   half of its criterion explicitly deferred. Filing under `## Open blockers` is a request for an
   operator ruling, **not** a closure path (`decisions.md §10`, superseding this item's earlier
   "complete or filed under Open blockers" wording).
2. `docs/retro/sd32-compute-library-and-cause-closure-retrospective.md` written (grounded in
   `scripts/retro.py summary`, not recollection) **and cited from `references/README.md`** — a
   retrospective that exists but isn't linked from this package is not a closed criterion.
3. A full worktree/branch sweep for the whole bundle, with a real count (not "none found" without
   having actually run `git worktree list`).

Only after all three does the `tranche/12 → develop` PR open, architecture docs refresh, and
release notes populate. A PR opened before step 2 or 3 completed is out of protocol — the
verification command is simply running `workflow-instruction.md §13`'s steps in the stated order
and checking each produced a real artifact (the retrospective file, the worktree-sweep count in
`progress.md`) before the PR-open step.

## What does NOT close SD-32

A figure in a release note. A green `scripts/verify.sh` exit code. A `gate-X` Slack post. None of
these are proofs; they are *reports* of proofs. The proof is the receipt; the receipt names the
corpus SHA, the command, and the fixture whose expected value was transcribed from bytes the
engine never reads. **A cycle without all three is not a closed cycle.**
