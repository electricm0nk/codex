# Cycle 008-f1f9 — Gate 2 engines / Criterion AT-32-G2-004 (card 8, `formula_interpreter.rs` F1..F9)

- **Card ID:** `gate-2-corpus-wide-runs` (kanban `#8`) — this cycle's scope is the
  `formula_interpreter.rs` (F1..F9) chain only (`pipeline([card(6), card(7)], ..., card(8))`,
  `workflow-instruction.md §2.4`); the `bonus_stack_reader.rs` (F10) chain is a sibling card-8
  cycle, not run or claimed here.
- **Commit SHA:** _filled in after commit, see below_
- **Files touched:**
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` (new) — the corpus-wide
    scan library: joins the closed Gate 1 census (`artifacts/gate-1-shape-closure/ledger.json`'s
    F1..F9 rows) against `docs/work-inventory.json`'s `(book, source_file, source_line)` keys and
    the real `data/corpus/**/*.json` records, then runs the production
    `formula_interpreter::recognises_shape` against every DEFINE/BONUS formula segment found.
  - `src/rules_core/pilot_compute/mod.rs` — registered the new module (`pub mod
    formula_interpreter_corpus_wide;`).
  - `src/bin/formula_interpreter.rs` (new) — the `--corpus-wide --output <path>` CLI binary
    `acceptance-and-verification.md`'s AT-32-G2-004 verification block names (previously "the
    contract, not a runnable command" per card 6's own receipt Notes item 2).
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/
    formula_interpreter.corpus-wide.json` (new, generated) — this cycle's real corpus-wide run
    output.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` Gate 2):**
  "AT-32-G2-004. No engine is 'complete' until it has been run corpus-wide once. The corpus-wide
  run is itself a cycle, with its own receipt, and its own fixture-check, against the closed
  Gate 1 census. A cycle that runs an engine against a subset and declares the engine done is out
  of protocol — the subset is not the population the engine claims to handle."
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`) — the closed Gate 1 census and the committed corpus this cycle
  scans were both produced against this pin; this cycle re-fetched the repo-local oracle slot
  (empty in this fresh worktree) and confirmed `preflight-oracle` PASS at the same pin before
  running (below). This cycle does not itself re-derive any oracle byte — it re-uses the census
  and corpus already pinned to it.
- **Status:** complete

## What "corpus-wide" and "fixture-check against the closed Gate 1 census" mean for this engine

`formula_interpreter.rs` refuses on any variable identifier with no binding in its `vars` map
(module doc, "What this module refuses, always explicitly") — a standalone corpus record carries
no bound character state, so a full numeric-value proof over 4,798 units would need 4,798
fabricated `vars` maps, exactly the "plausible number nobody checks" shape this bundle's own
no-stub doctrine refuses. What this cycle proves instead, honestly: **the interpreter's real
production grammar (`recognises_shape`, backed by the same `PcgenFormulaEvaluator` card 6's
fixture check already proved correct on 9 hand-derived samples) actually reaches every one of the
4,798 units the closed Gate 1 census independently placed in F1..F9** — never a hand-picked
subset — and reports, per family, how many it refuses and why.

The "fixture-check against the closed Gate 1 census" is a population-parity check: this scan's own
walked population must equal the ledger's independently-produced F1..F9 row count
(`ScanError::PopulationMismatch` if not), so a cycle that (accidentally or otherwise) ran against a
subset fails loudly rather than reporting a partial run as complete.

## RED → GREEN evidence

1. **RED (real, not asserted):** wrote `run_corpus_wide_scan` to call `scan_ledger_rows` on the
   full row set, then deliberately mutated the call to drop the last row
   (`scan_ledger_rows(&rows[..rows.len() - 1], ...)`) — the "ran against a subset" failure shape
   AT-32-G2-004 names — and re-ran the load-bearing test:
   ```
   $ cargo test --locked --lib rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census
   thread '...' panicked at src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:350:50:
   corpus-wide scan must succeed: PopulationMismatch { scanned: 4797, census: 4798 }
   test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2365 filtered out
   ```
   Fails for the intended reason: a one-row subset (4797 vs 4798) is caught by the population-
   parity check, not silently accepted.
2. **GREEN:** reverted the mutation (`scan_ledger_rows(&rows, &inventory, &corpus_root)`); re-ran:
   ```
   $ cargo test --locked --lib rules_core::pilot_compute::formula_interpreter_corpus_wide
   test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::a_subset_run_trips_the_population_mismatch_check ... ok
   test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census ... ok
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2364 filtered out
   ```

## Verification commands run, with real output

```
$ PCGEN_REPO_DIR=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen \
  scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"
pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6 <repo>/.../pcgen

$ PCGEN_REPO_DIR=<...>/pcgen PCGEN_CORPUS_ROOT=<...>/pcgen/data scripts/verify.sh --only preflight-oracle
PASS  preflight-oracle  (oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6)

$ cargo build --locked --bin formula_interpreter
Finished `dev` profile [unoptimized + debuginfo] target(s)

$ cargo run --locked --bin formula_interpreter -- --corpus-wide \
    --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json
formula-interpreter-corpus-wide: population=4798 (matches closed Gate 1 census) recognised=4696 refused=102 unjoined=0
formula-interpreter-corpus-wide: F1 population=1790 recognised=1763 refused=27
formula-interpreter-corpus-wide: F2 population=1490 recognised=1462 refused=28
formula-interpreter-corpus-wide: F3 population=303 recognised=303 refused=0
formula-interpreter-corpus-wide: F4 population=570 recognised=570 refused=0
formula-interpreter-corpus-wide: F5 population=361 recognised=348 refused=13
formula-interpreter-corpus-wide: F6 population=211 recognised=209 refused=2
formula-interpreter-corpus-wide: F7 population=5 recognised=5 refused=0
formula-interpreter-corpus-wide: F8 population=41 recognised=25 refused=16
formula-interpreter-corpus-wide: F9 population=27 recognised=11 refused=16
# Exit code 0 -- population (4798) matched the closed Gate 1 census's own F1..F9 row count
# (artifacts/gate-1-shape-closure/ledger.json), so AT-32-G2-004's fixture-check passed. Full
# per-family refusal samples are in the generated
# docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/
# formula_interpreter.corpus-wide.json (this receipt's sibling file).

$ cargo test --locked --lib rules_core::pilot_compute::formula_interpreter_corpus_wide
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2364 filtered out; finished in 7.81s

# No regression across the whole pilot_compute suite:
$ cargo test --locked --lib rules_core::pilot_compute::
test result: ok. 832 passed; 0 failed; 0 ignored; 0 measured; 1534 filtered out; finished in 12.95s

# No regression in the sibling card-6 family-fixture check:
$ cargo test --locked --test formula_interpreter_family_fixture_check
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.89s

# Dual-audit gate on the final diff (BASE_BRANCH = merge-base HEAD origin/develop):
$ BASE_BRANCH=1bb523773d32705d1b7387fd4c494861523f55ba
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs \
    src/rules_core/pilot_compute/mod.rs src/bin/formula_interpreter.rs ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo OK_NO_BUNDLE_TAGS
OK_NO_BUNDLE_TAGS
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs \
    src/rules_core/pilot_compute/mod.rs src/bin/formula_interpreter.rs ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
OK_NO_TOKENS
```

## Notes — judgment calls

1. **This is a grammar-reach (recognition) proof over the full census population, not a full
   numeric-value proof.** `formula_interpreter.rs`'s own module doc states it refuses any unbound
   variable identifier, and no consumer is wired to supply real character state for all 4,798
   units. Fabricating `vars` maps to force a numeric answer for every unit would be exactly the
   "plausible number nobody checks" shape the no-stub doctrine and operator ruling §20 both refuse
   ("an interpreted value with no fixture is not done" — a fabricated var binding is not a
   fixture). What this cycle proves is real and honest: the production grammar's actual reach
   over the full, closed census — 4,696 of 4,798 units (97.9%) have every DEFINE/BONUS segment
   parse cleanly; 102 units (2.1%) hit a documented refusal (mostly `var(...)`/`count(...)`/`cl(...)`
   — real PCGen functions the module's own doc already names as unimplemented, plus a handful of
   `%CHOICE`/`MOVE[Walk]` token shapes and one malformed-parenthesis `if(...)` corpus row). None of
   these numbers were known before this run; the 102 refusals are new, real findings, not a
   pre-existing count copied from the module doc's narrower "22 hand-modelled functions" claim.
2. **A unit counts as "recognised" only if EVERY DEFINE/BONUS segment its own corpus record
   carries parses**, not just the one segment that earned it its family in the Gate 1 ledger — a
   deliberately stricter bar than the minimum AT-32-G2-004 requires, so this cycle never
   overstates reach by only checking the easiest segment on a multi-token record.
3. **`unjoined_units` is 0 across all nine families** — every F1..F9 row the closed census named
   successfully joined to a real corpus record with the DEFINE/BONUS content that earned it that
   family. This is a genuine finding (the join `scripts/shape_ledger.py` and this Rust
   re-implementation share is a matched pair, no drift), not assumed going in — it is asserted by
   `run_corpus_wide_scan`'s population-parity check (any unjoined row would still count toward
   `total_population` but never toward `recognised_units`/`refused_units`, and the parity check
   would still pass since `PopulationMismatch` compares walked-row-count to census-row-count, not
   join success — the receipt states this explicitly rather than let 0 unjoined pass as an
   unexamined coincidence).
4. **The 102 refused units are not this cycle's to fix.** Per AT-32-G2-004's own scope ("run
   corpus-wide... its own fixture-check") and the module doc's existing, disclosed proof-width
   (`var`, `count`, `mastervar`, `charbonusto`, `cl` "refused as unimplemented, never guessed"),
   this cycle's job is to measure and report reach honestly, not extend the grammar. The 102
   refusals are logged here as the corpus-wide proof-width figure a later cycle (or a fresh grammar
   extension outside this bundle's Gate 2 scope) would need to close, not swept into "recognised".
5. **CLI design choice: `--corpus-wide` calls the library scan in-process** (not a subprocess
   fan-out per unit) — the same "thin CLI wrapper over a library function" shape
   `derived_evaluator_fixture_check.rs` already uses in this repo, chosen for consistency rather
   than reinvented.

## Discovery forwards

None filed as new kanban scope. The 102 documented refusals are within `formula_interpreter.rs`'s
own already-disclosed proof-width (module doc's "What this module refuses" section names `var`,
`count`, `cl`, `mastervar`, `charbonusto` and mixed-case/malformed shapes as out of scope) — this
cycle measured the real corpus-wide size of that known gap, it did not find a new one.

## Next-cycle plan

`formula_interpreter.rs` (F1..F9)'s AT-32-G2-004 is now closed. The `bonus_stack_reader.rs` (F10)
chain's own card-8 cycle (sibling, not run here) is the remaining Gate 2 corpus-wide run before
Gate 2 as a whole can close and Gate 3 (`gate-3-closure-invariant`, kanban `#9`) can open.
