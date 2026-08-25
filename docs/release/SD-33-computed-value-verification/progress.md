---
canonical: true
owner: god-emporer
bundle_id: SD-33
status: in progress — Epics 1-3 complete, rows 1-12 (AT-33-E1-001..004, AT-33-E2-001..004, AT-33-E3-001..004) all complete
date: 2026-08-24
---

# SD-33 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update `kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

**Launch gates passed 2026-08-25** (`technical-requirements.md §1`, `workflow-instruction.md §1`):

1. SD-32's closure PR merged to `develop` — PR #376 MERGED, `origin/develop` = `f53b8e32da`
2. SD-32's instrument debt closed **inside SD-32** — 29 total / 0 open deferrals, `EXCLUDED_BOOKS = frozenset()`
3. `tranche/13` cut from `develop` and pushed — `origin/tranche/13` = `f652db7ac7`

Epic 1 complete; cycles 1-4 (`AT-33-E1-001` row 1, `AT-33-E1-002` row 2, `AT-33-E1-003` row 3,
`AT-33-E1-004` row 4) all landed. Epic 1 gates every other epic
(`workflow-instruction.md §3`) — Epics 2/3/4 (`parallel: yes`, worktree-isolated) are next.

**Epic 2 complete; rows 5-8 (`AT-33-E2-001..004`) all landed.
RULING: Path A** — the pinned PCGen builds headless on this box, and a
hand-authored `.pcg` round-trips through `BatchExporter` via a
hand-authored template producing real, independently-cross-checked
computed values (13 of 13 hand-derived RAW fields match the real oracle
export exactly). `scripts/oracle_harness/` answers
`(ours, oracle, agree|disagree|unverifiable)` per unit, proven both by
16/16 unit tests (all three verdicts, including a known-disagreeing case)
and by a live end-to-end run whose `disagree` record feeds the real
`scripts/box_ledger.py --check` fail-closed gate to exit 1. **No Path B
fallback was needed; no throughput-reduction escalation is raised** — Epic
5 (gated on Epic 2) can run the live-PCGen path at full mechanism
availability. See `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md`
and `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` for the
full ruling.

**Bundle-level figure (`AT-33-E1-003`'s own evidence bar, not a footnote):** of the corpus's **19**
distinct `kind` values (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`), **8**
carry a probe capable of verifying a computed magnitude and **11** do not
(`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`).
Of the 11: 8 have no engine table at all (`ability`, `template`, `deity`, `power`, `domain`,
`skill`, `language`, `trait`), and 3 have an engine table but only a presence/lookup check, never a
computed-delta observation (`monster`, `monster_ability`, `companion`) — see
`artifacts/epic-1-instruments/probe-surface-census.json` and its cycle receipt for the full
per-kind table and the source citations.

**Cards complete: 12 / 21** (`jq` re-derive: count `complete` rows in `kanban.md`'s table body) —
Epics 1-3 (rows 1-12); Epic 4 (rows 13-15) is a concurrent worktree-isolated lane, status as of this cycle's own commit.

**Denominator gate is now live** (`AT-33-E1-004`): `scripts/verify.sh --only denominator-gate`
runs `scripts/denominator_gate.py --check` against this bundle's own `artifacts/**/*_cycle_receipt.md`
+ `progress.md` (4 files as of this commit, 0 violations) and fails closed on a bare percentage —
proven both ways through the real stage invocation, not just the underlying script:
`DENOMINATOR_GATE_PATHS=<malformed file> bash scripts/verify.sh --only denominator-gate` → exit 1;
corrected form → exit 0. See the cycle receipt for the full transcript.

## Cycle entry schema

Each entry states, at minimum:

- criterion ID and card number
- commit SHA(s)
- **every figure with the command that produces it and its denominator** (`decisions.md §2`)
- **movement in four buckets** — closure / reclassification / reachability / instrument-correction
- receipt path

## Open blockers

None. **This section is not a parking lot.** An entry here is a request for an operator ruling and it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a disposition, never a closure path, and no later cycle may proceed past a blocked card on its own authority.

## Cycles

### Cycle AT-33-E3-001..004 — engine coverage (rows 9-12, Epic 3)

- **Criteria:** `AT-33-E3-001` (root-cause), `AT-33-E3-002` (F1 gap closes), `AT-33-E3-003`
  (F2-F9 close), `AT-33-E3-004` (corpus-wide run reports 100%, 11,652 of 11,652, with its denominator).
- **Files:** `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` (modified),
  `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md` (new),
  `artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json` (new, SD-33's own path —
  SD-32's `artifacts/gate-2-engines/...` never touched), `THE-BOX.md` (append-only note, no
  group/count changed).
- **Root cause (execution-verified, not assumed):** two independent staleness layers, not a code
  defect. (1) SD-32's committed Gate-2 run artifact (`population=4,798`) predates 9 later commits
  (`25dbee17aa..80329736f4`) that grew its own Gate-1 census inside SD-32 itself
  (`ledger.json` F1..F9 grew to 11,338) and was never regenerated — 6,540 of the 6,854-unit gap
  (95.4% of 6,854). (2) that frozen census is itself stale against the CURRENT corpus/inventory —
  314 more units exist today (11,652 of 11,652 fresh `python3 scripts/shape_ledger.py` rollup) —
  4.6% of 6,854. Full trace with concrete sample unit coordinates and commit SHAs:
  `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md`.
- **Fix:** `formula_interpreter_corpus_wide.rs` no longer reads SD-32's frozen
  `docs/release/SD-32-.../artifacts/gate-1-shape-closure/ledger.json`. It regenerates the Gate 1
  census fresh, at scan time, by invoking `scripts/shape_ledger.py` (never re-implemented in Rust —
  `decisions.md §4` single-source-of-truth), caching the result process-wide (`OnceLock`) so
  `cargo test`'s several `#[test]` fns in this module share one ~28s regeneration.
- **Figures:**
  - True F1..F9 population (`README.md §4` row E) = **11,652** —
    `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` (family rollup summed)
  - Prior committed run population = **4,798 of 11,652** —
    `jq .total_population docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines/formula_interpreter.corpus-wide.json`
  - Fresh SD-33 run population = **11,652 of 11,652** —
    `jq .total_population docs/release/SD-33-computed-value-verification/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`
  - Row G comparison (E − new F): `11,652 − 11,652 = 0`
  - F1 (largest family): true = run = **6,308 of 6,308** (up from the prior run's 1,790)
  - Per-family run-population == true-population for all of F1..F9: full table in the receipt
  - Recognition (separate from coverage, per the epic-breakdown NOTE): **10,626 of 11,652**
    recognised, **240 of 11,652** refused (named, e.g. unrecognised `var("CL=...")`), **786 of
    11,652** unjoined (this module's own join is narrower than `shape_ledger.py`'s three-way join —
    named forward scope, not silently folded into "recognised")
- **Movement (four buckets):** closure 6,854 (the full population gap — 6,854 previously-un-walked
  F1..F9 units now walked and either recognised or named-refused) / reclassification 0 /
  reachability 0 / instrument-correction 6,854 (the "41%"/"4,798 of 11,652" figures are corrected
  to their real cause, staleness — both bucket counts describe the same movement from two angles:
  the population figure closes, and the prior figure is corrected).
- **RED→GREEN:** new test
  `f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census`. RED
  (module still reading the frozen census): `assertion left == right failed ... left: 6032 right:
  6308`. GREEN (module regenerates fresh): `cargo test --locked --lib -p codex
  rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population...` → `1 passed`.
  Existing `a_subset_run_trips_the_population_mismatch_check` re-pointed at the new fresh-census
  function and still green.
- **Test scoping:** ran `cargo test --locked --lib -p codex formula_interpreter` (42/42 passed —
  both `formula_interpreter` and `formula_interpreter_corpus_wide` modules, matched by substring)
  and `cargo build --locked --lib -p codex` (clean, pre-existing unrelated warnings only). Did not
  run the full `cargo test --locked --lib -p codex` workspace sweep (2,824+ tests, no other module
  touched) or `apps/desktop/src-tauri` (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-3-engine-coverage/AT-33-E3-001..004_cycle_receipt.md`.

### Cycle AT-33-E2-004 — oracle-path-ruling (row 8, Epic 2)

- **Criterion:** `AT-33-E2-004` — the Path A / Path B ruling is recorded and escalated.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new — carries the ruling), `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md` (new), `progress.md` (this entry).
- **Ruling: Path A.** All three named risks (`decisions.md §5`) resolved in Path A's favor by execution (`AT-33-E2-001`); a real round-trip export produced real values (`AT-33-E2-002`); the comparison harness is built and proven live (`AT-33-E2-003`).
- **Figures:**
  - Named risks resolved without forcing Path B: 3 of 3 named in `decisions.md §5` (`AT-33-E2-001_cycle_receipt.md`)
  - Path B fallback invocations this cycle: 0 of 1 (Epic 2's own spike) (no Java-source-reading fallback file exists under `artifacts/epic-2-oracle-harness/`)
- **Consequence for Epic 5:** none negative — Epic 5 can run the live-PCGen path this cycle proved, at full mechanism availability, rather than the slower per-shape Path B fallback. **No escalation filed** — `decisions.md §5`'s escalation clause is conditioned on Path A *failing*, and it did not.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this is a ruling, not a unit-status change.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md`.

### Cycle AT-33-E2-003 — oracle-comparison-harness (row 7, Epic 2)

- **Criterion:** `AT-33-E2-003` — the comparison harness answers the per-unit question.
- **Commit SHA:** `84a5781c11`
- **Files:** `scripts/oracle_harness/__init__.py`, `scripts/oracle_harness/compare.py`, `scripts/oracle_harness/oracle_export.py`, `scripts/oracle_harness/run.py` (all new), `scripts/tests/test_oracle_harness.py` (new), four fixture JSONs under `artifacts/epic-2-oracle-harness/fixtures/` (new), `artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new).
- **What landed:** `compare_unit(unit_id, ours, oracle)` returns `{"unit_id","ours","oracle","verdict"}`, `verdict ∈ {agree, disagree, unverifiable}`; `unverifiable` is a normal return value on a missing/blank oracle value, never an exception, never folded into `agree`. `run_comparison`/`run.py` produce the exact shape `scripts/box_ledger.py::load_oracle_results` reads.
- **Figures:**
  - Unit test suite (new): 16 passed, 0 failed, of `scripts/tests/test_oracle_harness.py`'s own 16 cases (`python3 -m unittest scripts.tests.test_oracle_harness -v`)
  - Combined with existing box_ledger suite: 41 passed, 0 failed, of both files' combined 41 cases (`python3 -m unittest scripts.tests.test_oracle_harness scripts.tests.test_box_ledger -v`)
  - Live CLI run, agree-only fixture: agree=4, disagree=0, unverifiable=1, of 5 units (`python3 scripts/oracle_harness/run.py --oracle-export .../pf1_fighter_l1.computed.txt --ours .../fixtures/pf1_fighter_l1.ours-sample.json --output .../fixtures/pf1_fighter_l1.oracle-results-demo.json`)
  - Live CLI run, known-disagreeing fixture: agree=3, disagree=1, unverifiable=1, of 5 units (same command with `ours-sample-with-bug.json`) — then fed to `python3 scripts/box_ledger.py --check --oracle-results .../fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json` → exit 1, `oracle_disagreement=1`
- **Fixture discipline:** every `oracle=...` literal in the unit tests was hand-transcribed from the real committed `pf1_fighter_l1.computed.txt` bytes (read by eye, typed as a Python literal); the test file never opens that file, and the one test class that *does* exercise the real parser (`OracleExportParsingTest`) uses only an inline string, never the committed file.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle builds and proves the instrument; the demo fixtures use synthetic unit ids scoped to this cycle's `.pcg`, not real `docs/work-inventory.json` units.
- **RED→GREEN:** `ImportError: cannot import name 'compare' from 'oracle_harness'` before the package existed (intended reason); 16/16 green after.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-003_cycle_receipt.md`.

### Cycle AT-33-E2-002 — oracle-character-roundtrip (row 6, Epic 2)

- **Criterion:** `AT-33-E2-002` — a character round-trips through the oracle.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.pcg` (new), `artifacts/epic-2-oracle-harness/computed-values.txt.ftl` (new), `artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt` (new), `artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log` (new).
- **What landed:** a hand-authored Level 1 Human Fighter `.pcg` (Core Rulebook only) exported through the pinned PCGen's `BatchExporter` via a hand-authored FreeMarker template emitting `pcstring(...)`-token computed variables (HP, AC, BAB, `VAR.CMB`/`VAR.CMD`, all three saves) as machine-readable `KEY=VALUE` lines.
- **Figures:**
  - Export command exit code: 0, of 1 (final, corrected) attempt (`build-transcript-05-batchexport-SUCCESS.log`, last line `BUILD SUCCESSFUL`)
  - `SEVERE`-level log lines: 0, of the full transcript (`grep -c SEVERE artifacts/epic-2-oracle-harness/build-transcript-05-batchexport-SUCCESS.log`)
  - Independently-derived RAW values matching the real oracle output: 13 of 13 fields checked (table in the cycle receipt; re-derive the oracle side with `cat artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — proves a round-trip mechanism, moves no inventory unit.
- **Notes:** first export attempt failed for the intended reason (`data/homebrew`/`data/_universal` outside the checkout's initial sparse scope) — see `AT-33-E2-001`'s entry below.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-002_cycle_receipt.md`.

### Cycle AT-33-E2-001 — oracle-path-a-feasibility (row 5, Epic 2)

- **Criterion:** `AT-33-E2-001` — Path A feasibility is established by execution.
- **Commit SHA:** `84a5781c11`
- **Files:** `artifacts/epic-2-oracle-harness/README.md` (new), `artifacts/epic-2-oracle-harness/.gitignore` (new), `artifacts/epic-2-oracle-harness/build-transcript-{01..04}-*.log` (new).
- **What landed:** fetched the pinned PCGen (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) into a scratch, gitignored, cone-mode sparse checkout inside this cycle's own write scope (never `~/workspace/repos/pcgen`), and ran `./gradlew --version`/`compileJava`/`jar` for real on `OpenJDK 25 Temurin`.
- **All three named risks (`decisions.md §5`) resolved to facts:**
  1. Gradle vs Java 25 — not a conflict (`build.gradle` pins `javaVersion = 25`; Gradle `9.5.1` ran cleanly).
  2. `pcgen.gui2.UIPropertyContext` coupling — real (registered even in batch mode) but non-blocking (its properties are `javafx.scene.paint.Color` value objects, no display-server call; confirmed by a successful headless export in `AT-33-E2-002`).
  3. `.pcg` input authoring — solved by hand-authoring one, using the repo's own `code/testsuite/PCGfiles/*.pcg` samples only to confirm tag vocabulary.
- **Figures:**
  - Named risks resolved to a fact: 3 of 3 named in `decisions.md §5` (manual: read `Main.java`/`UIPropertyContext.java`, then the commands below)
  - `./gradlew compileJava` first attempt: exit 1, of 1 attempt, failed for the intended reason (missing `PCGen-Formula` subproject dir in the initial sparse cone) (`build-transcript-02-compileJava-first-attempt-FAILED.log`)
  - `./gradlew compileJava` corrected attempt: exit 0, of 1 attempt (`build-transcript-03-compileJava-SUCCESS.log`)
  - Plugin jars produced: 11 of 11 `createJarTask` calls in `code/gradle/plugins.gradle` (`ls pcgen-oracle-checkout/plugins/*.jar | wc -l`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — proves a build-feasibility fact, moves no inventory unit.
- **Receipt:** `artifacts/epic-2-oracle-harness/AT-33-E2-001_cycle_receipt.md`.

### Cycle AT-33-E1-004 — denominator-gate (row 4, Epic 1)

- **Criterion:** `AT-33-E1-004` — the denominator gate is a real `scripts/verify.sh` stage.
- **Files:** `scripts/denominator_gate.py` (new), `scripts/tests/test_denominator_gate.py` (new),
  `scripts/verify.sh` (extended — new `denominator-gate` stage in both stage sets + dispatch case).
- **What landed:** a line-level check — a line carrying a bare percentage with no denominator
  marker (`of <N>` / `out of <N>` / `<N>/<M>` fraction / literal `denominator <N>`) anywhere on
  that same line is a violation. Wired into `scripts/verify.sh`'s stage list directly (not a
  standalone script — closes the `SD-31-.../forward-scope-register.md` C1.8 gap named for
  `v06_corpus_trap_report`), in both `ALL_STAGES` and `QUICK_STAGES`. Default scope is
  deliberately this bundle's own generated evidence (`artifacts/**/*_cycle_receipt.md` +
  `progress.md`) — not this bundle's planning prose (outside this criterion's write scope) and not
  every prior bundle's receipts (261 files repo-wide, unaudited, a separate task); overridable via
  `DENOMINATOR_GATE_PATHS`.
- **Figures:**
  - Unit test suite (new): 17 passed, 0 failed (`python3 -m unittest scripts.tests.test_denominator_gate -v`)
  - Regression: `test_box_ledger.py` 25/25, `test_probe_surface_census.py` 11/11 — 36/36
    (`python3 -m unittest scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v`)
  - Live default-scope check: 4 files checked, 0 violations (`python3 scripts/denominator_gate.py --check`)
  - Stage present in both stage sets: `bash scripts/verify.sh --list | grep denominator-gate` →
    `denominator-gate     yes   yes`
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this cycle builds an instrument (a gate with an exit code); it moves
  no inventory unit.
- **RED→GREEN:** `ModuleNotFoundError: No module named 'denominator_gate'` before the module
  existed (intended reason); 17/17 green after. **The criterion's own evidence obligation** — a
  mutation proof through `scripts/verify.sh --only denominator-gate` itself, pointed via
  `DENOMINATOR_GATE_PATHS` at a synthetic fixture whose only figure is a bare, undenominated
  percentage (`decisions.md` §2's own motivating shape) → `FAIL`, exit 1; the identical fixture
  corrected to state 97.9% of 4,798 and 41% of 11,652, denominator in the same construct → `PASS`,
  exit 0; default invocation with no override, against the real committed 4-file scope → `PASS`,
  exit 0. Full transcripts in the receipt.
- **Notes:** scope is deliberately narrow — this bundle's own receipts + `progress.md`, not
  repo-wide and not this bundle's own planning prose (which narrates the same 41%-of-11,652 /
  97.9%-of-4,798 figures `decisions.md` §2 cites as the motivating defect, and is outside this
  criterion's write scope). See the receipt's Notes for the full reasoning and the 261-file
  repo-wide sweep that informed the scoping decision.
- **Test scoping:** ran `scripts/tests/test_denominator_gate.py` (17/17, new) and
  `scripts/tests/test_box_ledger.py` + `test_probe_surface_census.py` (36/36, regression).
  `bash -n scripts/verify.sh` (syntax check) and `bash scripts/verify.sh --only denominator-gate`
  (three invocations — default GREEN, override RED, override GREEN). Did not run `scripts/verify.sh`
  in full (other stages' preconditions unrelated to this cycle's files), the Rust workspace (no
  `.rs` file touched), or `apps/desktop/src-tauri` (separate cargo workspace, untouched).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-004_cycle_receipt.md`.

### Cycle AT-33-E1-003 — probe-surface-census (row 3, Epic 1)

- **Criterion:** `AT-33-E1-003` — the probe surface is enumerated for real.
- **Files:** `scripts/probe_surface_census.py` (new), `scripts/tests/test_probe_surface_census.py`
  (new), `artifacts/epic-1-instruments/probe-surface-census.json` (new, generated).
- **What landed:** every corpus `kind` (19, live) enumerated by reading `src/bin/v06_work_inventory.rs`'s
  exhaustive verdict match arm-by-arm, cross-checked against live evidence strings for every claim
  (not from memory or prior prose, per `decisions.md §7`). 8 kinds carry a probe function that
  changes an input and observes a delta on a rendered computed snapshot (`class`, `class_feature`,
  `feat`, `spell`, `equipment`, `equipment_modifier`, `race`, `race_trait`); 11 do not — 8 with no
  engine table at all, 3 with an engine table but only a presence/lookup check (`monster`,
  `monster_ability`, `companion`).
- **Figures:**
  - Distinct corpus `kind` count: 19 (`jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l`)
  - Kinds with a magnitude probe: **8 of 19**; without: **11 of 19**
    (`python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8 kinds_without_probe=11`)
  - Units covered by a probe-bearing kind: 34,246 of 49,438
    (`jq '[.units[] | select(.kind | IN("class","class_feature","feat","spell","equipment","equipment_modifier","race","race_trait"))] | length' docs/work-inventory.json`)
  - Units in a no-probe kind: 15,192 of 49,438
    (`jq '[.units[] | select(.kind | IN("monster","monster_ability","companion","ability","template","deity","power","domain","skill","language","trait"))] | length' docs/work-inventory.json`)
  - Per-kind probe-fire confirmation (live, execution-derived): `class`=28, `class_feature`=26,
    `feat`=108, `spell`=966, `equipment`+`equipment_modifier`=605, `race`=39, `race_trait`=309 real
    units each — proving each claimed probe genuinely fired, not merely exists in source. All 11
    no-probe kinds confirmed at 0.
  - Full per-kind table with unit counts and re-derive commands: cycle receipt.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 /
  instrument-correction 0 — this cycle builds and runs an enumeration instrument; it moves no
  unit's status.
- **RED→GREEN:** `ModuleNotFoundError: No module named 'probe_surface_census'` before the module
  existed (intended reason); 11/11 green after, including 3 live-corpus acceptance tests run
  against the real `docs/work-inventory.json`. 3 mutation proofs on `--check`'s fail-closed gate:
  an unmapped-kind unit, a `probe_exists:true` kind whose only unit never fires the probe, and a
  `probe_exists:false` kind carrying probe-shaped evidence — all three correctly detected and
  reported by name. Regression: `scripts/tests/test_box_ledger.py` re-run, 25/25 still green
  (untouched this cycle).
- **Notes:** presence-only lookups (`monster`/`monster_ability`/`companion`) are deliberately NOT
  counted as magnitude probes even though the inventory's own vocabulary calls their result
  `grounded` — the criterion's bar is "can verify a computed magnitude", and a `holds_key` table
  lookup answers a different, weaker question. See the receipt's Notes for the full reasoning and
  the recursive-find hazard check (confirmed no probe implementation exists outside
  `v06_work_inventory.rs`'s sibling `v06_content_state_dump.rs`, and confirmed `data/corpus/`'s
  extra `*_generic`/`_parity` directory names are storage-layout artifacts, not a 20th kind).
- **Test scoping:** ran `scripts/tests/test_probe_surface_census.py` (11/11) and
  `scripts/tests/test_box_ledger.py` (25/25, regression). Did not run `scripts/verify.sh` (any
  stage — `AT-33-E1-004` owns wiring this cycle's files into it), the Rust workspace, or
  `apps/desktop/src-tauri` (no `.rs` file touched).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-003_cycle_receipt.md`.

### Cycle AT-33-E1-002 — box-fail-closed (row 2, Epic 1)

- **Criterion:** `AT-33-E1-002` — `box_ledger.py` fails closed on all five conditions.
- **Files:** `scripts/box_ledger.py` (extended), `scripts/tests/test_box_ledger.py` (extended), `THE-BOX.md` (extended — `"unverifiable"` field on every ledger group).
- **What landed:** conditions 3-5 (oracle disagreement / an `unverifiable` unit dispositioned `done` / `derived_at` staleness gate) added to the same `box_ledger.py` `AT-33-E1-001` built; conditions 1-2 (uncovered/overlap) were already implemented and are re-verified here, not re-implemented.
- **Figures:**
  - `box_ledger.py --check` on the committed `THE-BOX.md` → `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False` (`python3 scripts/box_ledger.py --check`)
  - Unit test suite: 25 passed, 0 failed (`python3 -m unittest scripts.tests.test_box_ledger -v`) — 9 carried from the prior cycle, 16 new
  - `unknown` (unverifiable) group population, used in two of the five mutation proofs: 4,224 of 49,438 (`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json`)
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle extends the instrument, moves no unit.
- **RED→GREEN:** 12 `AttributeError`s + 3 `AssertionError`s before implementation (intended reason: the three new mechanisms didn't exist); 25/25 green after. **Five live mutation proofs, one per condition, against the real committed `THE-BOX.md` (or a temp copy with exactly one mutation)** — full transcripts in the receipt: (1) `unknown` group deleted → `uncovered=4224`, exit 1; (2) a colliding group added → `overlap=5099`, exit 1; (3) a real `--oracle-results` fixture with one `verdict: disagree` record → `oracle_disagreement=1`, exit 1, corrected to `agree` → exit 0; (4) the real `unknown` group's disposition changed `unverifiable`→`done` (its own `unverifiable: true` flag left on — reproduces SD-32's exact over-claim) → `unverifiable_done=4224`, exit 1; (5) `derived_at` replaced with a fabricated SHA → `STALE:` + exit 1, committed file's real SHA → exit 0. Every RED case's corresponding GREEN is the untouched committed file, exit 0.
- **Notes:** oracle-disagreement check is wired now (reads `AT-33-E2-003`'s harness output shape), not deferred — it activates automatically once Epic 2 lands `oracle-results.json`, no second cycle needed. `"unverifiable"` is a ledger-group-level flag, not a per-unit field, because `uncovered==0 overlap==0` already makes a unit's disposition equal to its one group's disposition.
- **Test scoping:** ran `scripts/tests/test_box_ledger.py` only (25/25 green) — this criterion's whole file-touch set. Did not run `scripts/verify.sh` (`AT-33-E1-004`'s stage, not yet wired), the Rust workspace, or `apps/desktop/src-tauri` (no files in either changed).
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-002_cycle_receipt.md`.

### Cycle AT-33-E1-001 — box-partition (row 1, Epic 1)

- **Criterion:** `AT-33-E1-001` — `THE-BOX.md` exists as a living partition of the full inventory.
- **Files:** `scripts/box_ledger.py` (new), `scripts/tests/test_box_ledger.py` (new), `THE-BOX.md` (new).
- **Figures:**
  - population = 49,438 (`jq '.units | length' docs/work-inventory.json`; cross-checked against `jq '.totals.units' docs/work-inventory.json`, both agree — no correction needed)
  - `box_ledger.py --check` → `uncovered=0 overlap=0 population=49438` (`python3 scripts/box_ledger.py --check`)
  - 9 groups partition the population by the inventory's `status` field (already exhaustive and non-overlapping — 9 distinct non-null values, 0 duplicate unit ids); group counts: `grounded` 3,234, `literal-verified` 6,589, `fixture-verified` 1,741, `ingested-magnitude` 1,543, `text-complete` 5,099, `deferred-with-reason` 46, `not-ingested` 26,943, `not-started` 19, `unknown` 4,224 — each command in `THE-BOX.md`'s table, sum = 49,438.
- **Movement (four buckets):** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle builds the instrument, moves no unit.
- **Explicit `unverifiable` bucket** (`decisions.md §7`): the `unknown` group, 4,224 units — owned by Epic 4 to move.
- **RED→GREEN:** `python3 -m unittest scripts.tests.test_box_ledger` failed with `ModuleNotFoundError` before `box_ledger.py` existed (intended reason); 9/9 green after, including the live-corpus acceptance case. Mutation proof: `THE-BOX.md` copy with the `unknown` group deleted correctly failed closed (`uncovered=4224`, exit 1); committed file untouched.
- **Receipt:** `artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md`.
