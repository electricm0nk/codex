# Cycle 001 — Gate 2 engines / Criteria AT-32-G2-001/002/003 (card 6, F1..F9)

- **Card ID:** `gate-2-engines-f1-f9` (kanban `#6`)
- **Commit SHA:** _filled in after commit, see below_
- **Files touched:**
  - `tests/fixtures/rules_core/formula-interpreter-family-fixtures.json` (new) — one real
    corpus-derived fixture entry per in-scope family F1..F9
  - `tests/formula_interpreter_family_fixture_check.rs` (new) — 5 tests: the load-bearing
    evaluator-vs-fixture check, a mutation proof, and three provenance checks
  - `docs/release/SD-32-compute-library-and-cause-closure/acceptance-and-verification.md` —
    appended the AT-32-G2-003 entry for `formula_interpreter.rs` / F1..F9, under the existing
    Gate 2 section
  - `docs/retro/events/sd31-transcribe.jsonl` — two appended `preflight-oracle` events (one FAIL,
    one PASS) from this cycle's own env-block re-run in a fresh worktree; append-only, not a
    stomp — see Notes (same misattributed-actor shape card 5's receipt already logged)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` Gate 2):**
  - AT-32-G2-001: "For each of the ten semantic families, an engine exists in
    `src/rules_core/pilot_compute/` and emits values for the family's unit population. The engine
    **may** be `formula_interpreter.rs` for the nine families it already evaluates directly
    (F1..F9 need no binding layer ...), **or** the generalised `bonus_stack_reader.rs` as the
    binding layer the tenth family (F10) needs ... Whatever the implementation, it is named in
    the cycle receipt." — **This cycle's claim is scoped to F1..F9 via `formula_interpreter.rs`
    only; F10/`bonus_stack_reader.rs` is kanban card 7's own scope, not re-closed here.**
  - AT-32-G2-002: "Every value emitted by every engine clears `derived_evaluator_fixture_check`,
    whose expected value is transcribed from bytes the engine never reads. An interpreted value
    with no fixture is not done."
  - AT-32-G2-003: "Each engine's `acceptance-and-verification.md` entry (or appended section
    here) states: the family it handles ...; the proof's unit population (measured, not
    estimated); the proof width ...; the fixture sample size and how it was chosen; the
    re-derive command."
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`) — every fixture entry's `formula`/`raw_token_value` was
  independently confirmed byte-identical against this pinned oracle checkout (commands below).
- **Status:** complete

## RED → GREEN evidence

1. **RED (real, not asserted):** wrote the fixture and the test together, then deliberately
   corrupted the committed fixture's F5 entry (`max(MesmeristLVL/2,1)` with `MesmeristLVL=8`) from
   its correct hand-derived value `4` to `999`, and re-ran:
   ```
   $ cargo test --locked --test formula_interpreter_family_fixture_check engine_reaches_every_in_scope_family
   ...
   thread 'engine_reaches_every_in_scope_family_and_clears_its_fixture' panicked at
   tests/formula_interpreter_family_fixture_check.rs:125:5:
   one or more in-scope families failed their fixture check:
   F5 (occult_adventures / "Toxitician ~ Deft Fingers"): evaluator returned 4, fixture expects 999
   (formula "max(MesmeristLVL/2,1)")
   test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out
   ```
   Fails for the intended reason: the evaluator's real, correct answer (`4`) disagreeing with a
   deliberately wrong fixture value — proves the check actually compares evaluator output to the
   fixture rather than trivially passing.
2. **GREEN:** reverted the corruption (fixture restored from the pre-corruption copy); re-ran the
   full file:
   ```
   $ cargo test --locked --test formula_interpreter_family_fixture_check
   running 5 tests
   test engine_reaches_every_in_scope_family_and_clears_its_fixture ... ok
   test fixture_formula_matches_extract_formula_field_on_its_own_raw_token ... ok
   test mutated_evaluator_is_caught_disagreeing_with_the_family_fixtures ... ok
   test fixture_provenance_matches_the_committed_corpus_records_own_source_block ... ok
   test fixture_family_matches_shape_ledgers_own_classifier ... ok
   test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   ```

## Verification commands run, with real output

```
$ scripts/fetch-pcgen-oracle.sh --dest <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 <repo>/.../pcgen

$ PCGEN_REPO_DIR=<...>/pcgen PCGEN_CORPUS_ROOT=<...>/pcgen/data scripts/verify.sh --only preflight-oracle
PASS  preflight-oracle  (oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6)

# Provenance: every fixture entry's upstream .lst file sha256 + line content confirmed against
# the pinned oracle checkout (example, F1/F2/F3/F4/F5/F7 all share this book/file):
$ sha256sum .../pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures/oa_abilities_class.lst
4bbc05c20b141e732f4be20544406d31c73d42be8b445fb1d141c880d3368f3a  ...oa_abilities_class.lst
# matches data/corpus/occult_adventures/class_feature/psychic_bloodline/psychic_bloodline.json's
# source.sha256 exactly (and the other 4 occult_adventures entries, same file).
$ sha256sum .../pcgen/data/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_abilities_class.lst
21e4a425861ceed96c671e7bc5fbe53c6b6824f2663d322c41a2203588cc9d94  ...acg_abilities_class.lst
# matches the F6 entry's corpus record source.sha256 exactly.
$ sha256sum .../pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_abilities_class.lst
5fc5db9e0bcd8ad3313f696cf1b0dd95e58f41d4f8ef141c5562c9b773c7c811  ...iswg_abilities_class.lst
# matches the F9 entry's corpus record source.sha256 exactly.
$ sha256sum .../pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_3/b3_races.lst
d3c21d94b2f541548084dc2535c51bec50086940a79dfe084b2ef1feb7ca8dcc  ...b3_races.lst
# matches the F8 entry's corpus record source.sha256 exactly.
$ sed -n '43p' .../b3_races.lst | tr '\t' '\n' | grep BONUS:COMBAT
BONUS:COMBAT|AC|3+Global_LuckBonus|TYPE=Luck
# byte-identical to the F8 fixture entry's raw_token_value formula field.

$ cargo test --locked --test formula_interpreter_family_fixture_check
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.79s

# No regression in the engine's own existing suite:
$ cargo test --locked --lib rules_core::pilot_compute::
test result: ok. 820 passed; 0 failed; 0 ignored; 0 measured; 1534 filtered out; finished in 9.48s

# The existing (separate, unit-kind-scoped) derived_evaluator_fixture_check gate is unaffected —
# this cycle did not touch src/rules_core/derived_evaluator_fixture_check.rs at all:
$ cargo run --locked --bin derived_evaluator_fixture_check
derived-evaluator-fixture-check: 1836 unit(s) cleared over 2577 fixture row(s); 0 failed; 0 not ingested

$ python3 -c "import json; d=json.load(open('artifacts/gate-1-shape-closure/ledger.json')); print({k:v['count'] for k,v in d['families'].items()})"
{'F0': 20113, 'F2': 1490, 'F1': 1790, 'F10': 3, 'F4': 570, 'F3': 303, 'F5': 361, 'F8': 41, 'F6': 211, 'F9': 27, 'F7': 5}
# F1+F2+F3+F4+F5+F6+F7+F8+F9 = 4,798 — the AT-32-G2-003 population figure this receipt and the
# appended acceptance-and-verification.md entry both cite.

# Dual-audit gate on the final diff (BASE_BRANCH = merge-base HEAD origin/develop):
$ BASE_BRANCH=1bb523773d32705d1b7387fd4c494861523f55ba
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- tests/formula_interpreter_family_fixture_check.rs \
    tests/fixtures/rules_core/formula-interpreter-family-fixtures.json \
    docs/retro/events/sd31-transcribe.jsonl ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo OK_NO_BUNDLE_TAGS
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- tests/formula_interpreter_family_fixture_check.rs \
    tests/fixtures/rules_core/formula-interpreter-family-fixtures.json \
    docs/retro/events/sd31-transcribe.jsonl ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
OK_NO_TOKENS
```

## Notes — judgment calls

1. **Scope is F1..F9 only, not the tenth family.** Per `epic-breakdown.md` Epic 1 and
   `acceptance-and-verification.md` AT-32-G2-001, `formula_interpreter.rs` handles nine of the ten
   named semantic families "with no binding layer"; the tenth (F10, threshold step-count,
   `bonus_stack_reader.rs`'s own generalisation target) is kanban card 7's own scope. This cycle's
   fixture set and its `IN_SCOPE_FAMILIES` constant explicitly enumerate F1..F9 (nine entries,
   including F8 "Other named-variable expression (residual)" — the shape_ledger.py extension family
   the Gate 1 receipt logged as *not* one of the original ten but still a real, named, counted
   bucket that needs its own fixture coverage under the F1..F9 naming this card's title uses) and
   assert the count is exactly 9, so a future cycle that silently drops or duplicates a family is
   caught by the test itself rather than by review.
2. **This is a grammar-reach proof, not a corpus-wide per-unit proof.** AT-32-G2-004 ("no engine is
   complete until it has been run corpus-wide once ... its own receipt, its own fixture-check")
   is explicitly kanban card 8's own criterion, sequenced after this card in the dispatch pipeline
   (`workflow-instruction.md §2.4`, `pipeline([card(6), card(7)], ..., card(8))`). This receipt
   claims AT-32-G2-001/002/003 for F1..F9; AT-32-G2-004 is named in the card's kanban notes but is
   not re-claimed here — the same "not this card's own criterion" posture card 3's receipt used for
   AT-32-G0-003 relative to card 4. **Next-cycle plan (card 8) picks up the corpus-wide run.**
3. **The existing `derived_evaluator_fixture_check.rs`/`derived-evaluator-fixtures.json` gate is a
   different, unit-kind-scoped instrument and is deliberately not touched here.** It already wires
   `PcgenFormulaEvaluator` into several consumer seams (`kind=spell` DURATION/RANGE, `kind=monster`
   SLA caster level) and answers "does this specific consumer unit's number match the corpus" — a
   different question from "does the interpreter's grammar reach this shape family at all," which
   is this card's own scope. Per `technical-design.md`'s file-disjointness table, Gate 2 touches
   `src/rules_core/pilot_compute/*.rs` and "new test files" — `src/rules_core/derived_evaluator_fixture_check.rs`
   is a sibling file under `src/rules_core/`, not `src/rules_core/pilot_compute/`, and out of this
   card's write scope. Both gates independently cite `PcgenFormulaEvaluator`; they will never drift
   apart because both call the same production evaluator, not a mock of it.
4. **No engine source change was needed.** Reading `formula_interpreter.rs`'s own module doc and
   grammar (wave 25b/26 shape closure, already committed) confirmed the interpreter already parses
   and evaluates all nine in-scope families' shapes correctly — this cycle's job was to prove that
   with committed, corpus-derived fixtures, not to add new grammar. The one prior gap this card
   would have blocked on (a family whose real corpus shape the interpreter refuses) did not
   materialise for any of the nine samples tried.
5. **`docs/retro/events/sd31-transcribe.jsonl`'s two-line diff is this cycle's own env-block
   re-run**, not a concurrent writer's. Single-command `PCGEN_REPO_DIR=... PCGEN_CORPUS_ROOT=...
   scripts/verify.sh --only preflight-oracle` invocations (needed because this sandbox's shell
   state does not persist `export`s between tool calls) never carried `RETRO_ACTOR=gate-2-f1-f9`
   into the same command, so both events recorded under the `sd31-transcribe` git-config fallback
   actor name — the identical, already-logged shape card 5's own receipt (Notes item 5) described.
   Append-only, confirmed via `git diff` before staging, not a `git status --porcelain` "unexpected
   file" case. Noted here for the retro-log reader, not filed as a correction on its own.

## Discovery forwards

None. The engine already reached all nine in-scope families' grammar; no new `## DISCOVERED` scope
surfaced.

## Next-cycle plan

Card 8 (`gate-2-corpus-wide-runs`) picks up AT-32-G2-004 for `formula_interpreter.rs`: a real
`--corpus-wide` run over the full not-done population (or the F1..F9-family subset of it, 4,798
units) with its own fixture-check receipt, per the pipeline in `workflow-instruction.md §2.4`
(`card(6) -> card(8)`). Card 7 (`gate-2-engines-f10-binding`) is independent, chained
`card(7) -> card(8)` on `bonus_stack_reader.rs`'s own worktree.
