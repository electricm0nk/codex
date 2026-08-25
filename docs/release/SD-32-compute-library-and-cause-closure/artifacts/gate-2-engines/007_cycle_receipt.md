# Cycle 007 — Gate 2 — Engines / Criterion AT-32-G2-001 (F10 binding layer)

- **Card ID:** `gate-2-engines-f10-binding` (kanban.md #7)
- **Commit SHA:** (recorded after commit — see push step below; this receipt is written before the
  commit per §6 step 5, and the actual SHA is confirmed in `progress.md`'s append)
- **Files touched:**
  - `src/rules_core/pilot_compute/bonus_stack_reader.rs` — generalised the wave-26 single-record
    accumulator into a data-driven, corpus-wide producer-chain resolver.
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/007_cycle_receipt.md`
    (this file).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  ```
  git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba -- src/rules_core/pilot_compute/bonus_stack_reader.rs ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
  ```
- **Wired-integration audit result:** `OK_NO_TOKENS`
  ```
  git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba -- src/rules_core/pilot_compute/bonus_stack_reader.rs ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
  ```
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` AT-32-G2-001):**
  > For each of the ten semantic families, an engine exists in `src/rules_core/pilot_compute/` and
  > emits values for the family's unit population. The engine **may** be `formula_interpreter.rs`
  > for the nine families it already evaluates directly (F1..F9 need no binding layer —
  > `epic-breakdown.md` Epic 1), **or** the generalised `bonus_stack_reader.rs` as the binding
  > layer the tenth family (F10) needs, or a new engine. Whatever the implementation, it is named
  > in the cycle receipt.
  >
  > Engine named: **`src/rules_core/pilot_compute/bonus_stack_reader.rs`**, generalised this
  > cycle.

## What this cycle built

The wave-26 `bonus_stack_reader.rs` proved the "read the producers of a named variable and sum
them" pattern for exactly one narrow case: `extract_addends`/`evaluate_stack` took ONE
caller-preselected record's tokens and resolved ONE target variable's `BONUS:VAR` addends found on
THAT record alone, ignoring any `DEFINE:` base entirely. `epic-breakdown.md` Epic 1 (F2) and
`MEASURE-TWICE.md` §3.1 both name the gap this leaves: **77.2% (893 of 1,156) of the corpus's
distinct custom identifiers are resolvable via this pattern generalised to be *data-driven* —
scanning across every record, not one preselected record.** The worked example
(`MEASURE-TWICE.md` §3.1) is `AlchemistBombLVL`: `DEFINE:AlchemistBombLVL|0` plus
`BONUS:VAR|AlchemistBombLVL|AlchemistLVL` both live on
`data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json`, but a THIRD producer,
`BONUS:VAR|AlchemistBombLVL|MasterChymistLVL`, lives on an entirely different record
(`.../master_chymist/bomb_thrower.json`). No single record holds every producer — the wave-26
reader could never see the master_chymist contribution, because nothing handed it that second
record's tokens.

This cycle adds, without modifying any existing public function's signature or behaviour:

1. **`extract_define_base`** — scans a token set for a `DEFINE:<target_var>|<formula>` entry
   (the base PCGen's own `getVariableValue` starts from, per `PlayerCharacter.java`'s
   `getVariable`/`BonusManager.getTotalBonusTo` citation already in this module's doc). Refuses
   (never guesses) on two *disagreeing* `DEFINE`s for the same variable; tolerates identical
   duplicates (e.g. a `.MOD` continuation restating its parent).
2. **`ProducerChain`** — `{ base: Option<String>, addends: Vec<ConditionalAddend> }`, the resolved
   shape.
3. **`resolve_producer_chain_corpus_wide`** — the data-driven generalisation itself: takes ANY
   iterable of per-record token iterables (not one preselected record) and merges every record's
   `DEFINE` base and `BONUS:VAR` addends for `target_var`, in scan order. Reuses
   `extract_define_base`/`extract_addends` unmodified per record — no new per-token parsing, only
   the multi-record aggregation the wave-26 module never had. An addend gate this reader has not
   verified still refuses the WHOLE call (propagated from `extract_addends`'s own refusal),
   whichever record it came from.
4. **`evaluate_producer_chain`** — evaluates the full chain: `DEFINE` base (0 if none) plus
   `evaluate_stack`'s gated sum of every addend. This is PCGen's own `getVariableValue` shape
   (base + active bonuses), not a new arithmetic rule.

## RED → GREEN evidence

**RED (symbols did not exist at the pinned base):**
```
git show HEAD:src/rules_core/pilot_compute/bonus_stack_reader.rs \
  | grep -c "resolve_producer_chain_corpus_wide\|ProducerChain\|extract_define_base"
# -> 0
```
None of `extract_define_base`, `ProducerChain`, `resolve_producer_chain_corpus_wide`,
`evaluate_producer_chain` existed in the module before this cycle; the 11 new tests exercising them
(below) did not exist either, and could not compile against the pre-cycle module.

**GREEN:**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-f10-binding \
  cargo test --locked --lib rules_core::pilot_compute::bonus_stack_reader
# test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 2346 filtered out
```
All 18 tests in the module pass (7 pre-existing wave-26 tests, unchanged and still green — proving
no regression — plus 11 new tests for this cycle's generalisation). Widest relevant scope also run
clean:
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-gate-2-f10-binding \
  cargo test --locked --lib rules_core::pilot_compute::
# test result: ok. 830 passed; 0 failed; 0 ignored; 0 measured; 1534 filtered out
```

**The load-bearing proof, against real (not synthetic) corpus bytes**
(`resolves_the_alchemist_bomb_lvl_producer_chain_across_two_real_corpus_records`): reads
`data/corpus/advanced_players_guide/class_feature/alchemist/bomb.json` AND
`.../master_chymist/bomb_thrower.json` directly off disk, resolves `AlchemistBombLVL`'s chain
across both, and asserts `base = Some("0")` and both addends (`AlchemistLVL`, `MasterChymistLVL`)
are found. At `AlchemistLVL=6, MasterChymistLVL=2`: total = 8 (0 + 6 + 2).

**Mutation proof** (`single_record_scope_undercounts_the_alchemist_bomb_lvl_chain`, the Decision
1(a) shape): resolving the SAME target variable from only the `alchemist/bomb.json` record (the
wave-26 reader's own scope) yields 6, not 8 — silently dropping the `MasterChymistLVL`
contribution. This proves the multi-record scan is load-bearing, not a renamed no-op: a reviewer
regressing `resolve_producer_chain_corpus_wide` back to "only look at the first record" makes this
test fail.

**No-regression proof** (`resolve_producer_chain_corpus_wide_over_one_record_matches_the_narrow_extract_addends`,
`evaluate_producer_chain_with_no_define_defaults_base_to_zero_and_matches_evaluate_stack`): the
generalised entry point, run over exactly the token set the wave-26 reader already handled
(`WitchWardBonus` / `ward.json`), reproduces the exact same addends and evaluated totals at every
gate boundary the pre-existing tests already prove — the generalisation adds reach, it does not
change behaviour for the case already proven.

**Refusal-propagation proofs** (`resolve_producer_chain_corpus_wide_refuses_when_an_addend_gate_is_unrecognised`,
`resolve_producer_chain_corpus_wide_refuses_on_cross_record_disagreeing_define`): an unrecognised
PRE-tag on ANY scanned record's addend, or two records disagreeing on the same variable's `DEFINE`
base, refuses the whole call — never a partial, silently-wrong aggregate.

## Corpus SHA

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — the
77.2%/893-of-1,156 figure cited above is `MEASURE-TWICE.md` §3.1's own figure, re-cited (not
re-derived by this cycle); this cycle's own re-derivable figure is the two real corpus files it
reads directly (`alchemist/bomb.json`, `master_chymist/bomb_thrower.json`), both under
`data/corpus/` at the repo's own committed state (not the PCGen oracle — these are already-ingested
records, not a live oracle read).

## Status

**complete**

## Notes

- **Scope discipline:** this cycle is the ENGINE (AT-32-G2-001/002/003), not the corpus-wide run.
  AT-32-G2-004 ("no engine is complete until run corpus-wide once... its own cycle, its own
  receipt") is kanban card 8's job, gated on this card per `workflow-instruction.md` §2.4's
  pipeline (`card(6)/card(7)` → `card(8)`, "engineFrom"). This receipt does not claim AT-32-G2-004.
- **AT-32-G2-002** (every emitted value clears `derived_evaluator_fixture_check`) is not yet
  exercised by this cycle in the CLI sense — no `--bin bonus_stack_reader` target exists yet (the
  acceptance doc itself notes: "today `formula_interpreter` and `bonus_stack_reader` are library
  modules under `src/rules_core/pilot_compute/`, not `src/bin/` targets" — a Gate 2 deliverable
  still open). This cycle's own correctness proof is the RED→GREEN test suite above (18 tests,
  including a mutation proof and a real-corpus-bytes proof), not yet a `--emit-fixtures` CLI run.
  Flagged here rather than silently claimed as satisfied.
- **Proof width, stated explicitly (AGENTS.md rule 7):** this generalisation still inherits every
  narrowness the wave-26 module already disclosed in its own doc comment — exactly one PRE-tag kind
  recognised (`PREVARGTEQ`), exactly one `BONUS` `TAG` recognised (`VAR`), a token with more than
  one trailing PRE-tag field refused rather than guessed at. It does NOT yet resolve a producer
  chain recursively (an addend formula that is itself another custom identifier with its own
  producer chain, rather than a class-LVL term) — that shape was not measured as part of the
  893-of-1,156 figure and is out of this cycle's scope. It does NOT decide corpus traversal order
  or scope (which records to feed it) — that remains the caller's responsibility, unchanged from
  the wave-26 module's own disclosed boundary.
- **Not self-healed / not blocked:** no audit violations, no build breakage, no divergence from
  the pinned base.

## Discovery forwards

None filed this cycle.

## Next-cycle plan

Card 8 (`gate-2-corpus-wide-runs`) picks this engine up for its own corpus-wide run against the
closed Gate 1 census, per `workflow-instruction.md` §2.4's `card(8), { engineFrom: c }` pipeline
step and AT-32-G2-004. That cycle is also where a `--bin bonus_stack_reader`-shaped CLI entry point
(or equivalent) and the `derived_evaluator_fixture_check` clearance for this engine's emitted
values (AT-32-G2-002) should land, since AT-32-G2-004 requires the corpus-wide run to itself be
fixture-checked.
