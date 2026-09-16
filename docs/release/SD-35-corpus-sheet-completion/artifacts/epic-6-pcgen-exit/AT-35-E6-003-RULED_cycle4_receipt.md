# Cycle AT-35-E6-003-RULED cycle 4 — Epic 6 PCGen exit / AT-35-E6-003-RULED

- **Commit SHA:** `1ebbe4b9bf` (the code), `bbc9db6ddb` (this receipt, the census and the retro
  events), cycle start `2bf2b4fa2b`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `2bf2b4fa2b`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed:
  ```
  live_files=22 live_hits=44 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/pilot_compute/resolved_prose.rs` — **+120 lines.** New public
    `leaked_markup(text) -> Option<&'static str>`: the predicate that answers *did any unrendered
    markup survive onto this sheet line?* It reads **output prose** — it never opens a corpus
    record, never names a token key, never needs the ingest grammar. It is the sheet rule
    (`decisions.md` §1) stated as a check, so it belongs to the side that prints the line. The
    body is `pcgen_desc::leaked_pcgen_syntax`'s, moved intact: same four checks, same percentile-
    dice exception, same six returned strings. Pinned RED→GREEN by the new
    `leaked_markup_tests`, whose cases are the ones `pcgen_desc`'s own tests already pinned, so a
    drift in either direction fails.
  - `src/pcgen_import/pcgen_desc.rs` — **−104 lines.** `leaked_pcgen_syntax` is now a two-line
    **delegation** to the live-side predicate, not a second copy. The converter is allowed to
    depend on the live side's definition of a clean sheet line; the live side is not allowed to
    depend on the converter. The alias stays because the converter and the `src/bin` ingest
    generators check their own output against the same bar, and `decisions.md` §11 KEEPS the
    converter. `PCGEN_ENTITIES` stays here for `decode`, which is the converter's own job.
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — the two live leak checks
    (`:1019`, `:1135`) call `super::resolved_prose::leaked_markup`. **2 hits gone.**
  - `src/rules_core/race_resolver.rs` — **+30 lines.** Three methods on `RaceTraitRecord`:
    `exclusion_guard_flags`, `negated_fact_gates`, `declares_negated_ability_guard`. This module
    already owns the type and already imports the token reader; the desktop picker held the record
    and had to import the converter to ask three questions about it.
  - `src/rules_core/corpus_loader.rs` — **+70 lines.** New `LstFixtureLine` and
    `load_lst_fixture_corpus`: the third caller of the parse-and-convert machinery this module
    already runs for `load_equipment_corpus` and `load_spell_corpus` — same two parsers, same two
    converters, same `Box::leak` for the `'static` borrow the package's lifetime needs. Failure is
    loud and never partial (`Err` naming the fixture label and the line), exactly the contract the
    desktop's own `expect`/`panic!` messages carried.
  - `apps/desktop/src-tauri/src/corpus_fixtures.rs` — **5 hits gone.** No longer imports
    `pcgen_import::lst_parser` or `pcgen_import::ir_converter`; it resolves the bundled resource
    path and reads the files, which is its concern, and hands the record lines to
    `corpus_loader::load_lst_fixture_corpus`.
  - `apps/desktop/src-tauri/src/feat_catalog.rs` — **1 hit gone.** `row_description`'s `clean`
    closure calls `rules_core::pilot_compute::resolved_prose::leaked_markup`.
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — **1 hit gone.** The `use
    codex::pcgen_import::race_trait_tokens;` line is deleted and the three call sites ask the
    record. Two doc comments repointed at the live-side methods.
  - `…/AT-35-E6-003-RULED_cycle4_runtime_import_census.py` / `.json` — **new.** Imports cycle 3's
    census whole (which imports cycle 2's, which imports cycle 1's) and rewrites two groups'
    reasons with this cycle's measurements. Adds one printed line, `evidence_sentence_met=`, so
    the criterion's own bar is a machine-checked figure rather than a claim.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 2 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not this cycle's work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one field,
    `derived_at`, restamped by this cycle's own `completion_atlas.py --check`),
    `docs/retro/events/root.jsonl` and `docs/retro/events/sd31-transcribe.jsonl` (appended lines
    from another session on the shared checkout, plus this cycle's own misfiled first event —
    see Discoveries). Committed rather than filtered away, per the standing "clean tree =
    unfiltered `git status` empty" rule.

  **No `data/` file and no corpus record was changed**, so `data/sheet_rules/` and
  `docs/work-inventory.json` are byte-identical to the cycle's start tree (`diff -q` against
  `git show 2bf2b4fa2b:docs/work-inventory.json` → no difference). **`apps/` WAS touched**, so
  the desktop crate runs here rather than at the epic wrap-up (`§6` step 3).

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff --unified=0 <cycle-start>..HEAD -- src/ apps/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing at all — this
  cycle added no test file and quoted no wave identifier.

- **Wired-integration audit result:** OK_NO_TOKENS, **after one self-heal**. The first run of
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the added
  lines returned exactly one: the word *placeholder* inside a comment moved verbatim from
  `pcgen_desc.rs`, describing a corpus defect ("read `text-complete` with the placeholder still
  visible"), not a stub marker. `§8` names a single-token audit violation self-healable; the
  sentence now reads "with that hole still visible on the sheet" and the audit is clean. No
  `"Would …"` string, no inline mock, no fixture-only data path.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  **The Evidence sentence's first clause is MET for the first time in this criterion's family.**
  `pcgen_residue_gate.py --check` reads `root apps/desktop files=0 hits=0` — measured under
  ruling B16, which is what makes the reading mean anything: cycle 1 found the same sentence
  "satisfied" by a blind spot, with 12 shipping lines calling `codex::pcgen_import::` at run time.
  The criterion as a whole is **not** met: 37 hits across 19 files remain under
  `src/rules_core/`, and `render_pcgen_desc_with_values` is still called there (5 of them).

- **Receipt rows (mechanical):**
  ```
  since=2bf2b4fa2bd1d31cc6b1058c1b55b354fb729dad residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=367 ratio=n/a builds_recorded=1 pcgen_live_files=19
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.

- **PCGen residue:** `live_files=19 live_hits=37 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — down from cycle 3's `22 / 44` on both axes, and **the instrument was not touched this cycle**
  (`scripts/pcgen_residue_gate.py` and `scripts/pcgen-residue-baseline.env` are both absent from
  this cycle's diff: `git diff --name-only 2bf2b4fa2b..HEAD -- scripts/` is empty), so the
  `−7 / −3` is entirely code. Per-root, the figure the criterion's Evidence sentence asks for:
  ```
  root src/rules_core        files=19 hits=37  ->  files=19 hits=37   (unchanged)
  root src/saved_character   files=0  hits=0
  root src/campaign          files=0  hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop          files=3  hits=7   ->  files=0  hits=0    ZERO
  ```
  **The whole drop is the desktop crate, and `src/rules_core` is flat — that is stated, not
  netted.** Two of the three desktop moves removed a live read outright (the leak predicate, the
  race-trait questions). The third did not: `corpus_fixtures.rs`'s two
  `ir_converter::convert_*_record` calls left the shipping desktop binary and reappear once in
  `corpus_loader::load_lst_fixture_corpus`, beside the two identical calls that module already
  made. `src/rules_core` therefore reads `19 / 37` before and after: `−2` from
  `class_feature_grant_consumer.rs`, `+2` in `corpus_loader.rs`. **The conversion did not stop
  happening at run time; it stopped happening in two places.** It clears when the converted
  equipment/spell shape is produced at build time and read as data — this criterion's own
  remainder, not something this move quietly closed.

  **Not gate-gaming, by the test cycle 3 used to refuse its own third move: does the shipping
  binary change?** It does. `apps/desktop/src-tauri` no longer links the LST parsers, the IR
  converter or the PCGen prose module on any path. Nothing was renamed to duck a regex, no path
  was exempted, no `use` was collapsed to turn four counted lines into one.

- **Oracle parity:** N/A. No `Number` mapping was added and no rendered value changed. The leak
  predicate moved byte-for-byte in behaviour (its own new tests pin every case against the strings
  `pcgen_desc`'s tests already pinned); the three race-trait methods are one-line forwards to the
  same reader with the same argument; the fixture loader runs the same two parsers into the same
  two converters in the same order. The standing renderer-group parity measurement from cycle 2
  (`compared=660320 agree=562988 disagree=97332`) is unchanged and still refuses that group; it is
  re-derivable by the command in the figures table.

- **Movement, four buckets:**
  - **closure:** **none in corpus units** (Epic 6 closes zero by design). On the B16 population:
    `apps/desktop` `7 → 0` and `3 → 0` files — the criterion's Evidence sentence, met. By group,
    `renderer` `8 → 5`, `lst_parser_types` `16 → 12`, `trait_and_pool_tokens` `5 → 4`; 7 hits and
    3 whole files cleared.
  - **relabel:** `ir_converter` stays at 4 while 1 desktop hit left and 2 new live-side lines
    appeared — booked here, as movement between files, not as closure.
  - **reachability:** none — no rendered sheet line moved.
  - **instrument-correction:** **none.** The gate script, its baseline file and its patterns are
    untouched. Every hit that left did so because the code that made it left the live roots.

- **Refused tokens:** **37 hits across 19 files, six groups**, the same six as cycle 3:
  ```
  renderer=5, lst_parser_types=12, ingest_record_tokens=7, trait_and_pool_tokens=4,
  ir_converter=4, source_content_payload=5
  ```
  `5+12+7+4+4+5 = 37`. Six groups, under `§8`'s limit of ten. Recorded as
  `deferral 1789257475736-at-35-e6-003-ruled-eaa1d8`; every line named with file, line and reason
  in `…_cycle4_runtime_import_census.json` and re-derivable by its script.

  **Why each group did not go.** `renderer` (5) is refused by cycle 2's measurement, not by
  difficulty: the converted candidate exists and runs, and it disagrees with the live path on
  97,332 of 660,320 renderings across 2,443 record keys, every shape of it converter-side. What
  is left in the group is now the renderer itself — `PcgenDisplayValues`,
  `render_pcgen_desc_with_values`, `desc_token_arguments` — the two `leaked_pcgen_syntax` calls
  having proved to be a different mechanism entirely. `lst_parser_types` (12) +
  `ingest_record_tokens` (7) + `ir_converter` (4) + `source_content_payload` (5) = **28 hits are
  one piece of work**: the live side still **owns** `EquipmentRecord` / `LstSpellRecord` /
  `SourceContentPayload` as its own data types across 13 files, and `corpus_loader.rs` still
  **runs** the conversion at load time rather than reading its output. That is a multi-file
  type-ownership change plus a build step, not a call-site repoint.
  `trait_and_pool_tokens` (4) wants the converted rule's `applies` gates and a converted
  pool-member table.

- **Discoveries:** two, both emitted as `correction` retro events.
  - `1789257461317-at-35-e6-003-ruled-915037` — **cycle 3's census was wrong about all three
    desktop files, and in the same direction each time: it read a converter *import* as evidence
    of a converter *dependency*.** The recorded reason was that the desktop's 3 files / 7 hits
    "all need a converted equivalent the package does not carry". None of them did.
    `feat_catalog.rs` wanted an output-prose hygiene predicate that never opens a record;
    `race_trait_picker.rs` wanted three answers `race_resolver` already had the type and the
    reader to give; `corpus_fixtures.rs` wanted parse-and-convert work `corpus_loader` already
    performs twice. The lesson is `shipped-prose-is-not-a-source-of-truth` in census form: a
    group's stated reason is an assertion about a dependency, and an assertion about a dependency
    has to be checked against what the caller actually needs, not against the module it names.
    Three cycles priced this criterion's desktop half as blocked on converter work; it cost one
    pass over three files.
  - `1789257461475-at-35-e6-003-ruled-ddedc9` — **this cycle's own first retro event was
    misfiled.** `RETRO_ACTOR` does not persist between this harness's `Bash` calls, and the
    export in the setup call did not reach the `retro.py` call, so
    `1789257447105-sd31-transcribe-3a43e3` landed in `docs/retro/events/sd31-transcribe.jsonl`
    under the wrong actor. Same content, wrong file; re-emitted correctly and the misfile named
    rather than left to look like another session's event. The mechanical control is one line:
    `export RETRO_ACTOR=… && python3 scripts/retro.py …` in the same call, every time.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=19 live_hits=37 baseline_files=260 baseline_hits=12736 verdict=PASS` | every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`) under the five live roots, comment lines excluded (B14) and `#[cfg(test)]` regions excluded (B15) | `python3 scripts/pcgen_residue_gate.py --check` |
  | `root src/rules_core files=19 hits=37`; `root apps/desktop files=0 hits=0` | the same, restricted to that root | `python3 scripts/pcgen_residue_gate.py --check` |
  | `pcgen_import_hits=37 files=19`; `by_root=src/rules_core=37`; `apps_desktop_hits=0 evidence_sentence_met=YES`; the six group sizes | the shipping lines under the five live roots naming `pcgen_import` | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle4_runtime_import_census.py` |
  | the gate script and its baseline are absent from this cycle's diff | `scripts/` | `git diff --name-only 2bf2b4fa2bd1d31cc6b1058c1b55b354fb729dad..HEAD -- scripts/` |
  | `records=16508 compared=660320 agree=562988 both_none=242160 disagree=97332` (the renderer group's standing refusal, measured by cycle 2, unchanged here) | every `class_feature` record `class_feature_record_tokens()` carries, × levels 1..=20 × 2 ability probes | `AT35_E6_PROSE_PARITY=docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle2_prose_parity_census.json cargo test --locked --lib -j 6 -- --ignored --nocapture class_feature_grant_consumer::tests::class_feature_prose_parity_census` |
  | `records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS` (124.3 s) | the whole converted package | `cargo run --locked --bin sheet_rule_convert -- --check` |
  | `0` files under `data/sheet_rules/` carrying ingest syntax | the whole converted package | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` |
  | `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` | the completion atlas | `python3 scripts/completion_atlas.py --check` |
  | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS` | token coverage | `python3 scripts/token_coverage.py --check` |
  | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` | shape/engine boundary | `python3 scripts/shape_engine_boundary.py --check` |
  | `population=0 kinds=0 citation_failures=0` | missing engine tables | `python3 scripts/missing_engine_tables.py --check` |
  | `files_checked=134 violations=0` | the bundle package's markdown | `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` |
  | `files_checked=250 figures_examined=581 violations=0` | the figure-provenance sections the `verify.sh` stage reads | `python3 scripts/denominator_gate.py --check-provenance` |
  | `Ran 27 tests ... OK` | the residue gate's own unit tests, which pin B15 and B16 | `python3 -m unittest scripts.tests.test_pcgen_residue_gate` |
  | `RESULT: PASS` (`pi-sweep`) | the Product-Identity sweep stage | `bash scripts/verify.sh --only pi-sweep` |
  | `site/dashboard/PF1e-dashboard.json input pin matches docs/work-inventory.json` | the dashboard feed's one pinned input | `./scripts/publish-site-dashboard.sh --check-pin` |
  | `NO_RUN_EXIT=0`; full workspace `FULL_EXIT=0`, 418 targets, 8,872 passed, 0 failed, 69 ignored, **zero `test result: FAILED` lines**; desktop crate `569 passed; 0 failed; 0 ignored`, `DESKTOP_EXIT=0` | the whole root workspace, and the separate `apps/desktop/src-tauri` workspace | `cargo test --locked --no-run -j 6`; `cargo test --locked --no-fail-fast -j 6`; `cd apps/desktop/src-tauri && cargo test --locked -j 6` |
  | root-workspace clippy **0 warnings** | the root workspace with tests | `cargo clippy --locked --tests -j 6` |
  | `closed=0 relabeled=0 rust_lines_changed=367 ratio=n/a builds_recorded=1 pcgen_live_files=19` | `docs/work-inventory.json` before vs after | `python3 scripts/cycle_scope_gate.py --receipt --since 2bf2b4fa2bd1d31cc6b1058c1b55b354fb729dad --before /tmp/wi-before-AT-35-E6-003-RULED.json --after docs/work-inventory.json` |

- **Build scope verified:** **the whole root workspace AND the desktop crate**, at the final tree.
  `apps/` is in this cycle's diff, which is the condition `§6` step 3 states for running the
  desktop crate here rather than at the epic wrap-up.

  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --no-fail-fast -j 6` → **`FULL_EXIT=0`** — 418 targets, **8,872 passed,
    0 failed, 69 ignored**, and `grep -cE '^test result: FAILED'` over the log returns **0**. No
    failure to attribute this cycle: cycle 3's single red line
    (`sd17_b5_equipment::parse_runs_in_linear_time_on_a_synthetic_large_file`, a 2-second
    wall-clock budget measured under six parallel cargo jobs, `incident
    1789256480840-at-35-e6-003-ruled-bd3974`) did not recur here, which is consistent with its
    recorded diagnosis as a threshold rather than a regression detector.
  - The lib target inside that run → `3343 passed; 0 failed; 16 ignored` (44.4 s). Cycle 3's was
    `3341`; the `+2` is exactly this cycle's two new `resolved_prose::leaked_markup_tests` cases,
    and nothing else in the lib moved.
  - `cd apps/desktop/src-tauri && cargo test --locked -j 6` → `569 passed; 0 failed; 0 ignored`
    (1506.9 s), `DESKTOP_EXIT=0`. **`569` is the standing figure**, recorded identically by eight
    earlier Epic 6 receipts, so this cycle removed no desktop test and added none — it changed
    three call sites and one builder body. The 19 on-screen tests the criterion's Evidence row
    names are inside that 569 and are green.
  - `cargo clippy --locked --tests -j 6` → **0 warnings** (the command emitted no `warning:` and
    no `error:` line at all). The desktop crate's `cargo check --locked --all-targets` emits only
    pre-existing `dead_code` warnings in `character_hub.rs`, `class_catalog_generic.rs` and
    `corpus_full.rs` — none of the three is in this cycle's diff.

- **Sweep population:** N/A — no corpus record changed, so `corpus_literal_sweep` would re-examine
  a byte-identical `data/`.

- **Oracle pin:** N/A. No figure in this receipt came from the pinned PCGen checkout;
  `scripts/pcgen-oracle-pin.env` is unchanged.

- **Status:** **partial.** The criterion's population is not zero at HEAD: 37 hits across 19 files
  remain under `src/rules_core/`. The Evidence sentence's `apps/desktop` clause is met; the
  criterion is not.

- **Notes:**

  One rule, applied three times: **ask what the caller actually needs, not what the module it
  names contains.** A leak check over rendered output needed no ingest grammar. A picker holding a
  record needed three answers, not a token reader. A fixture bundle needed a parsed corpus, not a
  parser. Each had been priced for three cycles as blocked on converter work it did not need. The
  honest counterweight is in the residue row: the one move that did *not* remove a dependency —
  the fixture loader — is reported as flat, not as progress.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design)`.
  The remainder is 37 hits / 19 files, all under `src/rules_core/`, in two pieces:

  1. **The `renderer` group (5) is a CONVERTER cycle.** Three named populations, enumerated by key
     in `AT-35-E6-003-RULED_cycle2_prose_parity_census.json`: 1,351 keys whose converted rule
     carries extra `Desc` segments; 718 keys the converter renders and the live path does not; 374
     the converted rule cannot render. The live swap in `class_feature_grant_consumer.rs` is one
     commit once that census reads `disagree=0`.
  2. **`lst_parser_types` (12) + `ingest_record_tokens` (7) + `ir_converter` (4) +
     `source_content_payload` (5) = 28 hits are ONE piece of work**, and it is the largest single
     item left in Epic 6: the live side must own a converted equipment/spell/source-content record
     shape that `sheet_rule_convert` emits, and `corpus_loader` must read that artefact instead of
     running `ir_converter` at load time. 13 files import `EquipmentRecord` or `LstSpellRecord` as
     their own data type. `trait_and_pool_tokens` (4) rides on the same change for pool members.
