# Cycle 002 — Gate 1 shape closure / Family vocabulary reconciliation (`decisions.md §12a`)

- **Card ID:** `family-vocabulary-reconciliation` (kanban `#14`)
- **Commit SHA:** _filled in after commit, see below_
- **Files touched:**
  - `scripts/family_vocabulary_reconcile.py` (new) — canonical vocabulary reader + MT mapping +
    engine-coverage reconciliation
  - `scripts/tests/test_family_vocabulary_reconcile.py` (new) — 8 tests
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/family-vocabulary.json` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/family-vocabulary.md` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json` (regenerated, unchanged population/counts — pure re-run, no content diff expected)
  - `src/rules_core/pilot_compute/bonus_stack_reader.rs` — doc comments only (F10 → F4 relabel)
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` — doc comment only (F10 → F4 relabel)
  - `docs/release/SD-32-compute-library-and-cause-closure/acceptance-and-verification.md` — AT-32-G1-003 cross-check command, AT-32-G2-001, AT-32-G2-003 F1..F9 entry
  - `docs/release/SD-32-compute-library-and-cause-closure/epic-breakdown.md` — Epic 1 framing + F# work table
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — cards 6, 7 (retitled `gate-2-engines-f4-binding`), 8, 14
  - `docs/release/SD-32-compute-library-and-cause-closure/release-notes.md` — Gate 1 family table + engine-coverage note
  - `docs/release/SD-32-compute-library-and-cause-closure/technical-design.md` — Gate 2 engine description
  - `docs/release/SD-32-compute-library-and-cause-closure/technical-requirements.md` — Gate 2 cycle requirement
  - `docs/retro/events/card-14-family-vocabulary.jsonl` (new) — one correction event
  - `docs/retro/events/sd31-transcribe.jsonl` — one appended `shape-coverage-standing-gate` PASS event from this cycle's own re-run (append-only)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  ```
  BASE_BRANCH=$(git merge-base HEAD origin/develop)
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  -> OK_NO_BUNDLE_TAGS
  ```
- **Wired-integration audit result:** `OK_NO_TOKENS`
  ```
  git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
  ```
  Three `placeholder` hits surfaced against the full `BASE_BRANCH...HEAD` diff, all **pre-existing
  doctrine text from earlier cycles** (card 2's row, `AT-32-G3-002`'s own criterion text, Gate 3's
  own module description) describing the anti-gaming "placeholder predicate cannot manufacture
  false coverage" concept — not stub code, and not introduced or touched by this cycle. Re-checked
  scoped to only this cycle's own touched files (`git diff --unified=0 HEAD -- <this cycle's own
  new/edited files>`): zero matches, `OK_TOKENS_MY_FILES`.
- **Acceptance criterion (this card's own, `decisions.md §12a`):** "SD-32 ships exactly one
  shape-family vocabulary. Card 14 picks it, defines it in one committed, re-derivable place, and
  propagates it to the ledger, every engine's module documentation, every AT-32-* criterion that
  names a family, `kanban.md`'s card titles, and `epic-breakdown.md`. Where the two vocabularies
  genuinely disagree about what a shape *is*, the reconciliation states which is correct and why,
  with counts — it does not silently pick one."
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
  — consulted for `preflight-oracle`; the engine-coverage reconciliation reads the already-ingested
  `data/corpus/**/*.json` tree, not the raw oracle directly.
- **Status:** complete

## What this cycle found and fixed

1. **Ruling: `scripts/shape_ledger.py`'s eleven families (F0-F10) are canonical.** It is the only
   one of the two vocabularies that is (a) committed and re-runnable — MT's own walk was "not
   re-committed as a script" (`MEASURE-TWICE.md §7`) — and (b) total over the full 24,914-unit
   not-done population, not only the 4,948-unit formula-bearing subset MT's ten families
   partitioned. **F0 has no MT counterpart** — that is a finding, not an omission: a unit with no
   DEFINE/BONUS token was never assigned to any MT family at all. Nine of the eleven canonical
   families match an MT family by label (F1-F3, F5-F9); counts differ per family (see
   `family-vocabulary.md` §2's mapping table) because MT's own priority order was never recorded
   ("a unit's PRIMARY family is assigned by priority order" — MT never states what that order
   was), while `shape_ledger.py`'s order (F9 > F6 > F10 > F5 > F2 > F7 > F3 > F4 > F1) is stated
   and auditable in `FAMILIES`' own list order. Neither count is "wrong" on its own; going
   forward, `shape_ledger.py`'s is the one every AT-32-* criterion and receipt cites.
2. **The F10/F4 label collision (fixed).** `kanban.md` card 7 and multiple doc comments in
   `bonus_stack_reader.rs`/`formula_interpreter_corpus_wide.rs` attributed the binding-layer
   target family as "F10". `shape_ledger.py`'s own F4 proof-width text already correctly named
   **F4** ("named-counter/pool variable") as "the shape `bonus_stack_reader.rs`'s binding-layer
   pattern targets" — that text did not need to change; every OTHER document naming "F10" as the
   binding-layer family did. F10 is an unrelated 3-unit level-threshold step-count family,
   already evaluated directly by `formula_interpreter.rs` like any other F1..F9 member — it does
   not need a binding layer at all. Retro correction logged:
   `docs/retro/events/card-14-family-vocabulary.jsonl`, id
   `1787447193084-card-14-family-vocabulary-e41771`.
3. **AT-32-G1-003's cross-check command (fixed).** Retargeted from the nonexistent
   "F1..F10 table in `epic-breakdown.md`" (card 5's own correction,
   `docs/retro/events/gate-1-shape.jsonl` id `1787437987996-gate-1-shape-0ae65f`, could not fix —
   out of that card's scope) to `family-vocabulary.md` §1, which this cycle's own script
   generates from the live ledger.
4. **Engine-coverage reconciliation (new, independent re-derivation).** New
   `scripts/family_vocabulary_reconcile.py` walks the full corpus (not restricted to the
   not-done population, matching MT §3.1's own stated scope) and independently re-derives the
   population of distinct F4-shaped bare-identifier formula segments and how many are resolvable
   via `bonus_stack_reader.rs`'s producer-chain mechanism (an identifier is resolvable if it is
   ever the TARGET of a `DEFINE` or `BONUS:VAR` write anywhere in the corpus — the same condition
   `extract_define_base`/`resolve_producer_chain_corpus_wide` test). Result: **390 of 422 (92.4%)**
   — a real, different, and legitimately narrower denominator than MT's identifier-wide
   1,156/893 (77.2%) and card 8's corpus-wide `bonus_stack_reader` run's 4,736/3,519 (a broader
   population — every `BONUS:VAR` write target, not only F4-shaped bare-identifier VALUE
   references). All three are named together wherever the figure is quoted, per `decisions.md
   §12c` ("no bundle document may quote a bare total again without naming which population it
   is").
5. **A vocabulary fix that would move a unit count would be a finding, per this card's own
   constraint — none did.** This cycle changed labels, doc comments, and cross-check targets; it
   did not touch `shape_ledger.py`'s `FAMILIES` predicates or priority order, so no unit's family
   assignment moved. Re-ran the ledger and Gate 3 after the change (below) — population and
   `unclassified_count` are unchanged.

## RED → GREEN evidence

`scripts/tests/test_family_vocabulary_reconcile.py`'s `EngineCoverageReconciliationTest` proved a
real bug in `_producer_targets`'s BONUS:VAR-subtype detection (it checked the token's `key` for a
`":VAR"` suffix that never occurs — PCGen's BONUS subtype lives in the VALUE's first field, not the
key, exactly the shape `shape_ledger.extract_formula_segment`'s own BONUS branch already handles
correctly):

```
$ python3 -m unittest scripts.tests.test_family_vocabulary_reconcile.EngineCoverageReconciliationTest -v
test_only_f4_shaped_segments_counted_and_producer_resolvability_checked ... FAIL
AssertionError: 1 != 2
```

Fixed (`parts[0] == "VAR"` on the value's own split, mirroring `extract_formula_segment`), all 8
tests GREEN:

```
$ python3 -m unittest scripts.tests.test_family_vocabulary_reconcile -v
...
Ran 8 tests in 0.007s
OK
```

Full 28-test `test_shape_ledger.py` suite unaffected (canonical vocabulary itself unchanged):

```
$ python3 -m unittest scripts.tests.test_shape_ledger -v
...
Ran 28 tests in 0.399s
OK
```

**Drift-guard proof (acceptance item 5 — "prove it goes red by mutating one, then revert"):**
`FamilyVocabularyDriftTest.test_canonical_table_reflects_a_shape_ledger_families_mutation`
monkeypatches `shape_ledger.FAMILIES`' F1 label in-memory, proves the canonical table's F1 label
changes with it (proving the table is READ live, not a hand-copy that could silently fork), then
confirms it reverts cleanly once the mock context exits. If `family_vocabulary_reconcile.py` ever
starts hand-copying family data instead of reading `SL.FAMILIES` live, this assertion fails.

## Real run, re-derived after the change

```
$ python3 scripts/family_vocabulary_reconcile.py --inventory docs/work-inventory.json --corpus-root data/corpus \
    --output-json artifacts/gate-1-shape-closure/family-vocabulary.json \
    --output-md artifacts/gate-1-shape-closure/family-vocabulary.md
population (not-done units): 24914  unclassified: 0

canonical family rollup:
  F9        27  Skill-rank-derived (skillinfo/TOTALRANK)
  F6       211  classlevel(...)-derived
  F10        3  Level-threshold step-count (summed >= indicators)
  F5       361  Clamped/capped per-level scaling (min/max/floor/ceil around a level expr)
  F2      1490  Per-level scaling (<Class>LVL bare or arithmetic)
  F7         5  Conditional-step (if/boolean toggle)
  F3       303  Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA)
  F4       570  Named-counter/pool variable (plain identifier reference)
  F1      1790  Flat-constant magnitude (bare literal)
  F0     20113  No formula content (no DEFINE/BONUS token found for this unit)
  F8        41  Other named-variable expression (residual)

engine coverage (F4-scoped, corpus-wide): 390/422 (92.4%) resolvable via bonus_stack_reader.rs producer-chain
```

**Constraint check (must still be 24,914 / unclassified 0):**

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json \
    --output artifacts/gate-1-shape-closure/ledger.json
population (not-done units considered): 24914
unclassified: 0
... (unchanged family rollup, matches above)

$ scripts/verify.sh --only shape-coverage-standing-gate
==> shape-coverage-standing-gate — python3 scripts/shape_coverage_standing_gate.py
    PASS  shape-coverage-standing-gate  (population=24914 unclassified=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
RESULT: PASS
```

Both confirm: population 24,914, `unclassified_count` 0, unchanged from before this cycle. No unit
count moved — this was a labelling/reconciliation fix, not a re-classification.

## Notes

- The canonical family table is read live from `scripts/shape_ledger.py`'s own `FAMILIES` and
  `_family_metadata()` — this module never duplicates a family id, label, or predicate, so it
  cannot itself become a third vocabulary.
- `docs/retro/events/sd31-transcribe.jsonl`'s append is this cycle's own `preflight-oracle`/
  `shape-coverage-standing-gate` verify runs recorded under a stale `RETRO_ACTOR` (shell state does
  not persist between tool calls in this harness; a later bash call lost the `RETRO_ACTOR=
  card-14-family-vocabulary` export and verify.sh's own transcription hook fell back to a prior
  actor name). Append-only, not a stomp on any other cycle's data — left as-is rather than
  hand-edited.
- Historical `progress.md` cycle entries (cards 5, 7, 8, 10) that quote the old "F10" labelling
  are **not** rewritten — they are append-only receipts of what those cycles actually said at the
  time; this cycle's own corrections are additive, per this bundle's own "documents get tests or
  expiry" discipline (a fixed forward, not a retcon).

## Discovery forwards

None — no new `## DISCOVERED` entries. The engine-coverage populations named in item 4 above
(422 distinct F4-shaped identifiers vs. MT's 1,156 vs. card 8's 4,736) are documented as three
legitimately different denominators, not a discovered defect needing its own card.

## Next-cycle plan

Card 15 (`census-scope-closure`) is unblocked — it classifies the 27,847 kind-unenumerable objects
into this cycle's canonical vocabulary.
