# Cycle AT-35-E6-003-RULED cycle 1 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `611559cf6d`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `c2f9c8f6b5`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed:
  ```
  live_files=45 live_hits=300 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `scripts/pcgen_residue_gate.py` — the two rulings, implemented. New `cfg_test_ranges` and
    `_live_lines` (B15: region-aware `#[cfg(test)]` skip, from the attribute to the end of the item
    it annotates — brace-matched for a block, `;`-terminated for a braceless `use`). New
    `RUNTIME_IMPORT_PATTERNS = {"pcgen_import": r"\bpcgen_import\b"}`, merged into `PATTERNS` and
    deliberately **not** into `IDENTIFIER_PATTERNS` (B16). Both rulings written into the module doc
    beside B14, each with what forced it and what it does **not** mean.
  - `scripts/tests/test_pcgen_residue_gate.py` — `+177` lines. Two new RED→GREEN classes,
    `TestCfgTestRegionsAreNotLiveCode` (7 assertions across 4 tests) and
    `TestRuntimeConverterImportsAreCounted` (5 tests). One pre-existing test,
    `TestScan::test_every_design_pattern_is_counted`, had its fixture extended and its
    `hits_by_root["apps/desktop"]` assertion moved `13 → 14`: this cycle's own change moved that
    count, which `§8` names self-healable.
  - `docs/release/SD-35-corpus-sheet-completion/decisions.md` — **§18 (B15)** and **§19 (B16)**,
    numbered beside §17 (B14), each carrying what forced the ruling, the enforcing code, the
    pinning test, and what the movement is not.
  - `…/AT-35-E6-003-RULED_cycle1_runtime_import_census.py` / `.json` / `.md` — **new.** Every one
    of the newly counted hits, by file, line, mechanism group and why it is still there. The script
    imports the gate rather than re-implementing it and asserts its own total against
    `scan(ROOT).hits_by_pattern["pcgen_import"]`, so the census and the gate cannot disagree.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 2 `correction` events, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.

  **No `.rs` file, no `data/` file and no corpus record was changed by this cycle** —
  `git status --porcelain` lists no path under `src/`, `apps/`, `tests/` or `data/`. That is not a
  small diff hiding a big one: this cycle's whole subject is the instrument.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff --unified=0 -- scripts/ docs/release/` plus the new untracked census script, filtered
  to `^\+`), `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing. The same
  grep against the cumulative `merge-base ... HEAD` diff returns only earlier cycles' receipt prose
  quoting `tests/sd27_*` / `tests/sd35_*` **filenames**, the disposition every Epic 6 receipt has
  recorded.

- **Wired-integration audit result:** OK_NO_TOKENS. The same added-line set against
  `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` returns nothing.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the dispatch's two rulings, which are this cycle's actual bar: B15 in the gate and in
  `decisions.md`; B16 in the gate, counting run-time `pcgen_import::` call sites from the live
  roots; the honest figure reported; then the call sites cleared.

  **Met for the instrument, not for the clearing.** Both rulings are implemented, both pinned
  RED→GREEN, both written down as numbered decisions. `apps/desktop/` has stopped printing
  `files=0 hits=0`. The 58 hits the corrected gate now sees are **not** cleared, and the census
  names every one with its reason. Per the dispatch's own bar — *"If some call site genuinely
  cannot go … produce a census naming it, its file, its line and why, return `partial`, and STOP"*
  — that is this cycle's disposition.

- **Receipt rows (mechanical):**
  ```
  since=c2f9c8f6b5f01be16b00697764375d0350093acd residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=25
  ```
  `rust_lines_changed=0` and `builds_recorded=0` are correct and are the point: the cycle changed
  the measuring instrument, not the thing measured.

- **PCGen residue:** `live_files=25 live_hits=58 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — below the baseline on both axes, and `pcgen_live_files` fell `45 → 25`, so `§8`'s
  non-self-healable "a cycle that RAISED pcgen_live_files" is not triggered.

  **That single line is two opposite movements, and they are NOT netted.** Reported separately,
  each against its own denominator, and neither reported as progress:

  | movement | what it is | size | denominator |
  |---|---|---|---|
  | B15 `#[cfg(test)]` skip | **INSTRUMENT CORRECTION** — the gate stops counting code that never ships | `−300` hits, `−45` files | every hit the gate counted at `c2f9c8f6b5`; all 300 were class A |
  | B16 `pcgen_import` pattern | **DEFECT THAT WAS ALWAYS THERE** — the gate starts counting code that does ship | `+58` hits, `+25` files, `12` of them under `apps/desktop/` | every shipping line under the five live roots naming `pcgen_import` |

  The `−300` clears no file, closes no unit and changed not one line of shipping code. The `+58` is
  not a regression this cycle caused: those lines were in the shipping binary at `c2f9c8f6b5` and at
  every earlier Epic 6 receipt, invisible. The `45 → 25` on the file axis is arithmetic between two
  different populations and means nothing on its own.

  Per-root, the figure the criterion's Evidence sentence asks for:
  ```
  root src/rules_core files=21 hits=46
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=4 hits=12
  ```
  `apps/desktop` is **not** zero. Before this cycle the gate said it was.

- **Oracle parity:** N/A. Epic 6 touched no live path this cycle — no `.rs` file changed, so no
  rendered text can have moved and there is nothing for the tool-side renderer to disagree with.
  The last parity run stands unchanged at `AT-35-E6-003-FINISH` cycle 3: `compared=919 agree=919
  disagree=0`.

- **Movement, four buckets:**
  - **closure:** **none.** Zero corpus units moved; zero live call sites cleared. Stated plainly
    rather than dressed as the `−300`.
  - **relabel:** none.
  - **reachability:** none — no shipping code changed, so nothing the player reads moved.
  - **instrument-correction:** **two, in opposite directions.** B15 removed 300 non-shipping hits
    across 45 files; B16 added 58 shipping hits across 25 files, 12 under `apps/desktop/` where the
    gate had read zero. Only the first is an instrument correction in the usual sense; the second
    is a **defect the instrument had been hiding**, and it is recorded here so no later reader can
    mistake the net `−242` for work.

- **Refused tokens:** **none** under the token-syntax patterns — B15 moved all 300 out of the
  shipping population and class B was already `0` at `AT-35-E6-003-FINISH` cycle 3. The remainder
  this cycle refuses is the new B16 population, **58 hits across 25 files**, which sums by mechanism
  group:
  ```
  lst_parser_types=22, renderer=8, ingest_record_tokens=7, ir_converter=6,
  trait_and_pool_tokens=5, source_content_payload=5, provenance_prose=5
  ```
  `22+8+7+6+5+5+5 = 58`. Seven groups, under `§8`'s limit of ten distinct refused types. Recorded
  as `deferral 1789239788366-at-35-e6-003-ruled-d22a45`; every line named with file, line and reason
  in `…_cycle1_runtime_import_census.md`.

  **Why none of the seven is trimmable this cycle.** Six need a converted equivalent the package
  does not carry: the live side still **owns** the ingest record structs (`EquipmentRecord`,
  `LstSpellRecord`) as its own data type across 13 files, and `corpus_loader.rs` still **runs**
  `ir_converter::convert_equipment_record` at run time rather than reading its output. Each is a
  type-ownership rewrite with a corpus-wide parity gate, not a line edit. The seventh,
  `provenance_prose`, is 5 string literals in `reach_gate.rs` naming the converter's module path in
  a sentence the row prints — a prose edit that would move the total by 5 without moving one
  dependency, and would charge the desktop crate's ~1,500 s suite for it. The dispatch names that
  exact cycle as the wrong one, so it was not done.

- **Discoveries:** two, both emitted as `correction` retro events.
  - `1789239775227-at-35-e6-003-ruled-5b14cb` — the gate claimed `root apps/desktop files=0 hits=0`
    while 12 shipping lines under the desktop crate call `codex::pcgen_import::` at run time. A root
    reading zero because no pattern was ever aimed at it is the
    `validate-proxies-against-known-truth` shape: a proxy making a confident claim in a region it
    was never tested on.
  - `1789239775354-at-35-e6-003-ruled-563d87` — the gate's `live_hits=300` was **entirely**
    non-shipping. Shipping-code hits under the old patterns were already `0`.

  **The finding under both:** the gate was counting code that does not ship and missing code that
  does, and the two errors were close enough in size (`300` vs `58`) that the total looked
  plausible either way. A green gate that does not measure the thing it claims to is a false green
  — which is why `AT-35-E6-003-FINISH` cycle 1's escalation, asked ten times, was right to refuse
  to close on it.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=25 live_hits=58 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root apps/desktop files=4 hits=12` | the same, restricted to `apps/desktop/**` | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=45 live_hits=300` at the cycle's start | the same five roots under the pre-B15/B16 patterns | `git stash`-free: `git show c2f9c8f6b5:scripts/pcgen_residue_gate.py > /tmp/g.py && python3 /tmp/g.py --check --root .` |
  | `class_A_in_cfg_test_hits=300 files=45`, `class_B_executable_hits=0` | the gate's own hits at `c2f9c8f6b5`, partitioned by brace-matched `#[cfg(test)]` range | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py` |
  | `pcgen_import_hits=58 files=25`; `by_root=apps/desktop=12, src/rules_core=46`; the 7 group sizes | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle1_runtime_import_census.py` |
  | `Ran 78 tests … OK` | `scripts/tests/test_pcgen_residue_gate.py` (27) + `test_cycle_scope_gate.py` (51) | `python3 -m unittest scripts.tests.test_pcgen_residue_gate scripts.tests.test_cycle_scope_gate` |
  | `since=c2f9c8f6b5… closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=25` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since c2f9c8f6b5f01be16b00697764375d0350093acd --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `citation_failures=0 stale_derived_at=False` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 refused=142 token_types=233 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=131 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `pcgen-residue-gate PASS`, `pi-sweep PASS` | the two verify stages this cycle's change can move | `bash scripts/verify.sh --only pcgen-residue-gate`; `bash scripts/verify.sh --only pi-sweep` |

- **Build scope verified:** **the cargo stages are skipped, and named.** This cycle changed **no
  `.rs` file and no `data/` file** (`git status --porcelain` lists only `scripts/*.py`,
  `docs/release/**` and `docs/retro/**`), so `cargo test --no-run` / `--lib` / `--no-fail-fast`,
  `cargo clippy`, `corpus_literal_sweep` and `v06_work_inventory` would each re-examine a tree
  byte-identical to the one `AT-35-E6-003-FINISH` cycle 3 verified at `8fc2c53ca8`
  (`NO_RUN_EXIT=0`; `3341 passed; 0 failed`; **418 targets / 8,870 passed / 0 failed**; clippy `0
  warnings`; desktop crate `569 passed; 0 failed`). Inventing a fresh figure for an unchanged tree
  is the failure `AGENTS.md` rule 9 names. What **was** run, at the tree this receipt commits, is
  every instrument this cycle's change can actually move: the gate's own 27 unit tests, the 51
  `cycle_scope_gate` tests that parse the gate's output, both `verify.sh` stages, and the six
  package gates above. `cargo run --locked --bin sheet_rule_convert -- --check` was run in full
  anyway, cold, because the receipt schema names it: ``records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (116.5 s)`.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD. The two rulings are
  implemented, pinned and recorded; the honest number is reported and decomposed; the remainder is
  named line by line and sums to 58.

- **Notes:**

  **Why the B16 pattern is `\bpcgen_import\b` and not the symbol list.** Cycle 1's census matched a
  hand-written list of converter symbol names (`render_pcgen_desc_tokens`, `lst_parser`,
  `ir_converter`, …). A list decays: the next converter function added is invisible until someone
  remembers to extend it, which is exactly how the `\brender_pcgen_desc\b` blind spot was born. The
  module path is the one token every such call must spell, so the pattern cannot go stale. It costs
  the 5 `provenance_prose` hits, which are counted rather than exempted — the dispatch's "never
  widen an exclusion to make a number fall" cuts both ways.

  **The baseline was not touched and `--rebaseline` was not run.** The ratchet still reads
  `260/12736` from 2026-09-07. Re-baselining after an instrument change would destroy the only
  record of what the old instrument saw.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  One mechanism group per cycle, largest-leverage first, each with a corpus-wide parity gate:

  1. **`renderer`, 8 hits / 2 files.** The two `render_pcgen_desc_with_values` call sites in
     `class_feature_grant_consumer.rs` (lines 968, 1081) plus their `PcgenDisplayValues`,
     `desc_token_arguments` and `leaked_pcgen_syntax` companions, and `feat_catalog.rs:213`'s
     `leaked_pcgen_syntax` guard. The mechanism is proved twice over
     (`AT-35-E6-003-FINISH` cycles 2 and 3); the one undone piece is that
     `resolved_description_for_formula_only_desc_argument` seeds from `%N` **argument text** while
     the converted prose's slots are `Expr`s over variables, so its seeds must move from argument
     text to variable names.
  2. **`lst_parser_types`, 22 hits / 13 files** — the largest group and the real structural work: a
     converted equipment/spell record shape the live side owns, so `EquipmentRecord` and
     `LstSpellRecord` stop being live types.
  3. **`ir_converter` (6) + `ingest_record_tokens` (7) + `source_content_payload` (5)** — they move
     with (2); `corpus_loader.rs` must read a converted artefact instead of running the conversion.
  4. **`trait_and_pool_tokens`, 5** — needs a converted pool-member table.
  5. **`provenance_prose`, 5** — a prose edit in `reach_gate.rs`; fold it into whichever cycle is
     already paying for the desktop crate's suite, never on its own.

  `AT-35-E6-004`'s `--check --closure` bar is unchanged and now measures what ships.
