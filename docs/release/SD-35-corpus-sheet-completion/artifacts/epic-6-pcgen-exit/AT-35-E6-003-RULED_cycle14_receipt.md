# Cycle AT-35-E6-003-RULED cycle 14 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `64224a0c8e` (the code, the census script and its JSON, the retro event, and
  two folded shared-checkout artifacts), cycle start `8555cee7b4`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `8555cee7b4`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 13's closing figure — nothing drifted between the two cycles:
  ```
  live_files=8 live_hits=14 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/race_record.rs` — **new, +156, live side.** `CorpusRaceRecord` and
    `CorpusRaceTraitRecord`: the settled canonical shape of one `data/corpus/<book>/race/…` and
    one `…/race_trait/…` record. Stored corpus content first, then **twelve settled readings**,
    each field's doc comment naming the converter-side function that fills it. **Neither struct
    carries a token array or a bonus-chain array**, deliberately — a settled record that still
    carried `raw_tokens` would let a future live module re-open the ingest format behind the
    gate's back, which is the precise defect ruling B16 exists to make visible. The census
    asserts that by text (`race_record=settled_only ingest_arrays_on_settled_records=0`).
  - `src/rules_core/declared_bonuses.rs` — **new, +107, live side.** `AbilityAdjustment`,
    `VarContribution` and `DeclaredBonuses` **moved here from
    `pcgen_import::bonus_chain_reader`**, plus one new `TargetBonus`. All four were already
    settled *answers* rather than ingest *grammar* — no qualifier position, no chain keyword, no
    ingest field name appears in any of them — but a live module holding one had to write
    `pcgen_import` to name its type. **The reader did not move and did not change**:
    `bonus_chain_reader::declared_bonuses` still does every bit of the chain walking, and still
    re-exports these four names so every converter-side call site keeps its path.
  - `src/pcgen_import/corpus_race_json.rs` — **new, +280, converter side.** The race-side
    sibling of cycle 13's `corpus_equipment_json`. `corpus_race_source_record` /
    `corpus_race_trait_source_record` deserialize the cache payload and run **the same twelve
    reading calls `race_resolver` used to make at run time**, in the same order, on the same
    input. Plus two `#[cfg(test)]` whole-corpus parity proofs — see **Oracle parity**.
  - `src/rules_core/race_resolver.rs` — **three hits → ZERO; the file is cleared.** The three
    `use crate::pcgen_import::…` lines are gone and every accessor reads a settled field:
    `exclusion_guard_flags`, `negated_fact_gates`, `declares_negated_ability_guard`,
    `automatic_trait_grants`, `skinwalker_change_shape_kin`, `positive_prefact_flag`,
    `declared_walk_speed_ft`, `declared_size`, `declared_vision`, `declared_bonuses` and
    `adopted_race_pool_suffix`. `load_chassis_dir` / `load_trait_dir` deserialize
    `CorpusRecordV1<serde_json::Value>` and ask the boundary; a `data` object that is not a race
    payload lands as a **named diagnostic** rather than a silent skip. `classify` takes the
    settled record. The three import comments were rewritten to say what the file reads and what
    it does not.
  - `src/rules_core/corpus_loader.rs` — **one hit → two.** `corpus_race_record` /
    `corpus_race_trait_record`, the same boundary question cycle 13 taught this file to ask for
    equipment, asked for a race and a racial trait. **This is the cycle's one relabel and the
    receipt does not dress it up** — see **Movement**.
  - `src/pcgen_import/bonus_chain_reader.rs` — three struct declarations deleted, one
    `pub use` of their new live home added, one new reader `target_bonuses` (12 lines) that
    transcribes a `BONUS:<keyword>|<target>|<magnitude>` chain and yields `None` for a magnitude
    that is not a plain integer (`decisions.md §24`).
  - `src/pcgen_import/mod.rs`, `src/rules_core/mod.rs` — the three new modules registered.
  - `src/bin/v06_work_inventory.rs` — its `declared_template_bonus_languages(&record.data
    .raw_tokens)` reads the settled `template_bonus_languages` field. The reading itself is
    unchanged and still lives in `race_trait_tokens`; it runs once, at the boundary. The now-dead
    `use` was removed. **A bin, not a live root** — this is a compile consequence of the settled
    record, not a residue closure, and is not counted as one.
  - `tests/sd27_alternate_racial_trait_reachability.rs` — two sites that walked
    `record.data.raw_bonus_chains` for `SAVE`/`SKILL` magnitudes read
    `declared_bonuses.target_bonuses`. Same predicate, same refusal of a non-integer magnitude.
  - `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs` — the `!PREFACT` presence
    scan reads `negated_fact_gates`, which holds **one entry per `!PREFACT` token on the row**,
    so the presence test is byte-for-byte the one it replaces; the flag comparison is now a flag
    comparison rather than a substring of token text. Its pinned `from_row == 366` is unmoved.
  - `tests/sd35_race_trait_prose_comes_from_the_converted_package.rs` — **the oracle is KEPT.**
    It still reads the ingest arrays, because it *is* the ingest-format reading this file
    compares the converted path against; it reads them off disk itself now, in `tests/`, which is
    exactly where `decisions.md §11` says an oracle belongs. Same files, same records, same
    population (`compared == 919`, unmoved) — only the route from a record to its arrays changed,
    and a record with no payload on disk **panics** rather than being skipped.
  - `…/AT-35-E6-003-RULED_cycle14_runtime_import_census.py` / `.json` — **new.** Imports cycle
    13's census whole (which imports cycle 12's, … back to cycle 1's), widens one group's regex
    and rewrites three groups' reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (`derived_at`
    restamped by this cycle's `completion_atlas.py --check`) and `…/shape-engine-boundary.md`
    (one line number, `16297 → 16303`, restamped by `shape_engine_boundary.py --check` because
    this cycle's edit moved `v06_work_inventory.rs`'s `fn classify` down six lines — the
    instrument finds that anchor by content on every run, which is why it moves and stays green),
    plus `docs/retro/events/sd31-transcribe.jsonl` (appended events from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered `git status`
    empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty), so
  `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start tree.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 8555cee7b4 -- src/ tests/ apps/ | grep '^+' | grep -v '^+++'`, plus the three new
  files and the new census script in full),
  `grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` prints **1**, and that one hit is
  the census script's docstring citing the **pre-existing test file name**
  `tests/sd35_race_trait_prose_comes_from_the_converted_package.rs` — a doc citation, not an
  identifier in code. First run, no self-heal. Cycles 11, 12 and 13 recorded the same shape of
  finding.

- **Wired-integration audit result:** OK_NO_TOKENS. First run, no self-heal.
  `grep -ciE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the same
  added lines prints **3**, and all three are the word *placeholder* in a doc comment describing
  **upstream's own `%LIST` construct** ("PCGen's *whatever the player chose* placeholder"), the
  established repo idiom — `src/pcgen_import/race_trait_tokens.rs:449` carries the identical
  sentence and predates this cycle. No `"Would …"` string, no inline mock, no fixture-only data
  path, no stub: the converter-side reader parses the same real corpus JSON the live resolver
  parsed, and the two parity tests assert the real values over the whole live corpus.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is rulings B15 and B16 applied **and the call
  sites the corrected gate now sees cleared**.

  The Evidence sentence's `apps/desktop` clause stays met (`root apps/desktop files=0 hits=0`,
  first met in cycle 4, not regressed here — this cycle wrote no line under `apps/` at all). The
  criterion as a whole is **not** met: 12 hits across 7 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=8555cee7b40471147a2052feb8084869dc1894dc target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=919 ratio=n/a builds_recorded=5 pcgen_live_files=7
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=7` is the **file** count, down one from cycle 13's 8 — the first file this
  criterion has cleared since cycle 12. The **hit** count moved too: 14 → 12.
  `builds_recorded=5` counts **compile sessions in `$CARGO_TARGET_DIR`**, not verification
  passes: the warm-up build, three authoring compiles that type-checked the moved readings and
  the two settled structs, and the final `cargo test` + `clippy` + `sheet_rule_convert` build.
  There was **one** verification pass, after the last figure-moving change (`decisions.md` §3);
  the clippy fix that followed it is an iterator-order change with identical output, and its
  target's 636 tests were re-run.

- **PCGen residue:** `live_files=7 live_hits=12 baseline_files=260 baseline_hits=12736
  verdict=PASS` — files down 1 from cycle 13's 8, hits down 2 from 14, **and the instrument was
  not touched this cycle** (`git diff --name-only 8555cee7b4..HEAD -- scripts/` is empty).
  Per-root:
  ```
  root src/rules_core         files=8 hits=14  ->  files=7 hits=12
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; no line under apps/ was written)
  ```
  Not gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the
  `use`-collapse still available in `source_content_payload` was **refused for the eighth cycle
  running**. The gate's own self-test is green (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no converter
  mapping row changed, and `data/sheet_rules/` is byte-identical (`sheet_rule_convert -- --check`
  `verdict=PASS`). The parity this change required is **two new whole-corpus proofs**, in
  `pcgen_import::corpus_race_json`:
  `every_live_corpus_race_record_carries_the_same_values_the_payload_read_produced` and
  `every_live_corpus_race_trait_record_carries_the_same_values_the_token_reads_produced`. Each
  assertion re-runs **the exact call `race_resolver` made at run time**, on the exact same
  deserialized payload, and compares it against the field the settled record now carries, over
  **every** `data/corpus/<book>/race/**/*.json` and `…/race_trait/**/*.json` file the live
  resolver walks — by that resolver's own traversal rule (recurse, skip `_parity/` and
  `LICENSE.json`). A reading widened, narrowed or reordered by the move fails there, on the real
  corpus, not on a fixture. Both are green in the verification run below, **0 disagreements**.
  Kept alongside them and unchanged: the racial-trait prose oracle
  (`every_racial_trait_renders_the_same_sentence_from_the_converted_package`, `compared == 919`,
  0 disagreements) and `no_race_contributes_two_alternate_trait_bonuses_to_one_save`.

- **Movement, four buckets:**
  - **closure:** 3 live `pcgen_import` hits and **1 whole file**, named by row and asserted by
    file **and symbol** in the census (`closed_by_cycle14=3 cleared_files_by_cycle14=1`) —
    `race_resolver.rs`'s `ingest_payload::{RaceCacheData, RaceTraitCacheData}`,
    `race_trait_tokens` and `bonus_chain_reader::{self, DeclaredBonuses}` imports, and with them
    the twelve run-time readings behind them. The census asserts the file names the converter
    **nowhere** in shipping code (`race_resolver_shipping_pcgen_import_hits=0 (was 3)`) rather
    than inferring it from the gate's file list. **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** 1, named so it can never be counted as a closure
    (`relabelled_by_cycle14=1`). The boundary call did not vanish — it moved from
    `race_resolver` into `corpus_loader`, which already owned the live side's one ingest
    boundary, and **that file's own count rises from 1 to 2**. Three hits out, one hit back,
    net −2.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and `scripts/pcgen-residue-baseline.env`
    are absent from this cycle's diff. (The two SD-34 atlas artifacts restamped by
    `completion_atlas.py --check` / `shape_engine_boundary.py --check` are *those* instruments
    re-deriving their own output on an unchanged claim; they close nothing and are folded, not
    claimed.)

- **Refused tokens:** `corpus_json_boundary=2, renderer=5, ingest_record_tokens=1,
  trait_and_pool_tokens=1, source_content_payload=3` — **12 hits / 7 files, summing, all under
  `src/rules_core/`.** Five groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **`race_resolver`'s three imports were one problem, and it was the same one `corpus_loader`
    had.** Cycle 13 predicted this remainder would need "the race-trait rule shape this epic's
    remaining piece produces" — a converter cycle. It did not: the eleven readings are all
    *transcription* (`decisions.md §24`), so they can be settled **at the ingest boundary**
    without waiting for the converted package to carry anything new. What blocked them was not
    the converter's coverage but *where the answer was declared*. Moving the declaration, not the
    derivation, cleared the file.
  - **Two of the four bonus-chain types never had to be on the converter side at all.**
    `DeclaredBonuses`, `AbilityAdjustment` and `VarContribution` were written into
    `src/pcgen_import/` in `AT-35-E6-002` cycle 5 by the module that *produces* them, and the
    module's own doc comment already said what they are: "narrowed, already-classified values …
    not the chains under another name". Under ruling B16 that placement made every live holder a
    converter reader. The whole of the `trait_and_pool_tokens` reduction is that one observation.
  - **The settled record forced four call sites outside the epic's file-touch set**, none of
    them predicted: two `tests/sd27_*` suites walking `raw_bonus_chains` and `raw_tokens`
    directly, `src/bin/v06_work_inventory.rs`, and the racial-trait prose oracle. Three got a
    new settled field filled by the converter-side reader (`target_bonuses`,
    `template_bonus_languages`) — the doctrine `race_record.rs`'s own doc comment states — and
    the fourth, the **oracle**, got the opposite treatment on purpose: it reads the payload off
    disk itself, because an oracle that stops reading the ingest format stops being an oracle.
  - **`corpus_loader` is now the live side's single ingest boundary, and that is the shape the
    data step clears.** Both its hits are the same question asked twice. Neither clears by moving
    more code; both clear together when `data/corpus/` JSON carries the settled fields itself.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=7 live_hits=12`, was `8 / 14` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 12 hits split `corpus_json_boundary=2, renderer=5, ingest_record_tokens=1, trait_and_pool_tokens=1, source_content_payload=3`; `gate_agreement=OK (12 == 12)` | the same 12 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle14_runtime_import_census.py` |
  | `closed_by_cycle14=3 relabelled_by_cycle14=1 cleared_files_by_cycle14=1`; `race_resolver_shipping_pcgen_import_hits=0 (was 3)`; `corpus_race_json=present token_readings_on_converter_side=12`; `race_record=settled_only ingest_arrays_on_settled_records=0`; cycles 10/11/12's cleared files still clear | the 14 hits at cycle start vs the 12 at HEAD | the census's own row, file and symbol assertions, same command |
  | the two whole-corpus race parity proofs green, 0 disagreements each; `compared >= 20` chassis and `>= 100` trait records asserted as floors | every `data/corpus/<book>/race/**/*.json` and `…/race_trait/**/*.json` the live resolver reads | `cargo test --locked --lib -j 6` (`pcgen_import::corpus_race_json::tests::every_live_corpus_race_*`) |
  | the racial-trait prose oracle green, `compared == 919`, 0 disagreements | every racial-trait record every book under `data/corpus/` loads | `cargo test --locked --test sd35_race_trait_prose_comes_from_the_converted_package` |
  | 0 live files name `race_trait_tokens`, `bonus_chain_reader` or `ingest_payload` outside `derived_evaluator_fixture_check` | every live file, comments and `#[cfg(test)]` excluded as the gate excludes them | `python3 …_cycle14_runtime_import_census.py` (groups `ingest_record_tokens`, `trait_and_pool_tokens`) |
  | 3,355 lib tests pass (cycle 13 recorded 3,352; +3 are this cycle's three `corpus_race_json` tests) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,884 tests pass across 418 targets + 1 Doc-tests (cycle 13 recorded 8,881; the +3 are the same three new tests) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | 13 prior receipts, so this is cycle 14 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree. The desktop crate ran in its own `/tmp/cargo-sd35-AT-35-E6-003-RULED-desktop`
  **even though no line under `apps/` was written**, because this cycle changed the **type** of
  two `pub` fields the desktop crate reads (`RaceChassisRecord::data`, `RaceTraitRecord::data`) —
  a compile risk the "only when `apps/` is touched" rule does not cover, and which is exactly the
  "verify at the widest build scope" rule in `AGENTS.md`.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3355 passed; 0 failed; 16 ignored
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 Running targets + 1 Doc-tests;
                                         8,884 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings. ONE self-heal, named:
                                         the first run raised `unnecessarily eager cloning`
                                         at `v06_work_inventory.rs:6943` in this cycle's own
                                         new lines; `.filter()` now precedes `.cloned()`,
                                         identical output, and that target's 636 tests were
                                         re-run green
  cargo test --locked --bin v06_work_inventory -> WI_EXIT=0; 636 passed; 0 failed
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (113.9s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=7 live_hits=12 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=143 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  desktop crate (apps/desktop/src-tauri)          -> `DESKTOP_EXIT=0`; `test result: ok.
                                                     570 passed; 0 failed; 0 ignored`
                                                     (1,380.8s). `cargo clippy --locked
                                                     --tests -j 4` `DESKTOP_CLIPPY_EXIT=0`,
                                                     1 warning, in `equipment_catalog.rs:889`
                                                     -- a `#[cfg(test)]` pinned list in a file
                                                     this cycle did not touch, pre-existing
                                                     and not denied. 570 is the standing
                                                     figure cycle 13 recorded; this cycle
                                                     added and removed none
  frontend                                        -> not run; no `.ts`/`.tsx` file was written
                                                     this cycle (`git status --porcelain
                                                     apps/desktop/src/` empty); the frontend
                                                     runs at the epic wrap-up
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and no
  `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing an import at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and zero change in what
  the module depends on — is **refused for the eighth cycle running**, on the same reasoning
  cycles 7 through 13 gave. The same refusal applies to the two `corpus_json_boundary` hits:
  collapsing them into one `use crate::pcgen_import::{corpus_equipment_json, corpus_race_json};`
  line would take the gate down by one and change nothing about what the module depends on, so it
  was not done — the second call is booked as a relabel and reported at its real cost.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 15**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`, on
  the 12-hit remainder. The unblocked piece is **`corpus_json_boundary` (2)**, and it is a
  **data** step rather than a code step, unchanged from cycle 13's reading and now worth twice
  what it was: an ingest-side generator that writes the settled equipment **and race** fields into
  `data/corpus/<book>/<kind>/**/*.json` (or a sidecar beside it), the same producer shape
  `data/sheet_rules/` already has, after which `corpus_loader` deserializes settled values with
  serde and names nothing. That clears **both** its hits and its whole file, taking the gate to
  `6 / 10`. The `renderer` group (5) stays a converter-parity cycle (cycle 2's
  `disagree=97,332 of 660,320`), `trait_and_pool_tokens` (1, `class_feature_pool_catalog`) stays
  refused on cycle 7's corrected number, `source_content_payload` (3) stays refused on the
  use-collapse reasoning, and `ingest_record_tokens` (1, `derived_evaluator_fixture_check`) stays
  measured non-relocatable (cycle 3).
