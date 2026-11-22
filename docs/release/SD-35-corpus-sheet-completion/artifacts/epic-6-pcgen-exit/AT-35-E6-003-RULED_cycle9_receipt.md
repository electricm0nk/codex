# Cycle AT-35-E6-003-RULED cycle 9 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `8a36cd54c9` (the code, the generated package, the census script and its JSON, the retro events), cycle start `b732854b91`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `b732854b91`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 8's closing figure — nothing drifted between the two cycles:
  ```
  live_files=16 live_hits=32 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/bin/gen_desktop_fixture_corpus.rs` — **new, 306 lines, converter-side.** The
    build-time producer of the desktop's bundled corpus package. It reads the four `.txt`
    fixtures, parses them with the LST parsers, and writes the converted records into
    `apps/desktop/src-tauri/resources/corpus_fixtures/{spell,equipment}/*.json`. Equipment
    records are written in the same Shape B v1 `data.raw_tokens` / `data.raw_bonus_chains`
    form the real `data/corpus/<book>/equipment/*.json` records carry — including the
    **byte-exact** `raw_pair` value split that `enrich_equipment_raw_tokens.rs` uses, not the
    trimmed one — because that is what `corpus_loader::load_equipment_corpus` reads. Spell
    records are written as **converted fields only** — `key`, `school`, `casting_time`,
    `duration`, … — with **no token array at all**, because `load_spell_corpus` has read
    converted spell fields since cycle 8. `--check` regenerates in memory and fails on byte
    drift.
  - `src/rules_core/corpus_loader.rs` — **−75, +38.** `load_lst_fixture_corpus` and
    `LstFixtureLine` are **deleted**, and with them the module's `parse_lst_spell_row` import
    and its use of `parse_equipment_entries`. In their place, `load_book_corpus`: a live
    two-kind loader that composes `load_equipment_corpus` and `load_spell_corpus` — the two
    that already existed — and parses nothing.
  - `apps/desktop/src-tauri/src/corpus_fixtures.rs` — **−54, +108.** Hands
    `load_book_corpus` a `BookCorpusRoot` pointing at the bundled resource directory and
    asserts the record count it committed to (loud, never partial: a short load is a broken
    build, not a character silently rendered with no equipment). Plus one new test,
    `bundled_records_carry_their_real_converted_values`, which asserts the resolved values on
    the loaded package — Alarm/Abjuration, Blur/Illusion, Longsword's `DAMAGE 1d8`, Chain
    Shirt's first `BONUS` chain qualifier for qualifier — so it fails if the generator ever
    writes a document the live loader cannot read back.
  - `apps/desktop/src-tauri/tauri.conf.json` — **+2.** The two new resource subdirectories.
  - `apps/desktop/src-tauri/resources/corpus_fixtures/{spell,equipment}/*.json` — **new, 4
    files, generated.** The shipped, converted package.
  - `…/AT-35-E6-003-RULED_cycle9_runtime_import_census.py` / `.json` — **new.** Imports cycle
    8's census whole (which imports cycle 7's, … back to cycle 1's) and rewrites two groups'
    reasons with this cycle's measurements. It asserts its own total against the gate's
    (`gate_agreement=OK (29 == 29)`), asserts each of the three closures by row, and asserts
    that the four converted records exist on disk and carry a `data.key` the live loader can
    read — so the claim and the instrument cannot disagree silently.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from this cycle's own instruments and from the shared checkout, not authored
    work:** `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (one field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (one appended event from another session on the
    shared checkout). Committed rather than filtered away, per the standing "clean tree =
    unfiltered `git status` empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty),
  so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start
  tree. **`apps/` WAS touched**, so the desktop crate and the frontend ran here, not at the
  epic wrap-up.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines (the `+`
  lines of the three modified source files plus both new files whole),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing. First run, no
  self-heal.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. The same added
  lines under `grep -nEi '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  return nothing. No `"Would …"` string, no inline mock, no fixture-only data path: the
  generated package is produced from the **real** committed fixture rows by the real parsers,
  and the desktop test asserts the real values on the real loaded package.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  The Evidence sentence's `apps/desktop` clause stays met (`root apps/desktop files=0 hits=0`,
  first met in cycle 4, not regressed here even though this cycle wrote four `apps/` files), and
  the desktop crate and the frontend both ran green at this cycle's final tree. The criterion as
  a whole is **not** met: 29 hits across 16 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=b732854b91cefd1f7ff202b586fa6014529af090 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=581 ratio=n/a builds_recorded=2 pcgen_live_files=16
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=16` is the **file** count, which did not move — all three closures landed in
  one file (`corpus_loader.rs`) that still carries other hits. The hit count did move: 32 → 29.

- **PCGen residue:** `live_files=16 live_hits=29 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 8's `16 / 32` on hits, **and the instrument was not touched
  this cycle** (`git diff --name-only b732854b91..HEAD -- scripts/` is empty), so the `−3` is
  entirely code. Per-root:
  ```
  root src/rules_core         files=16 hits=32  ->  files=16 hits=29
  root src/saved_character    files=0  hits=0
  root src/campaign           files=0  hits=0
  root src/homebrew_authoring files=0  hits=0
  root apps/desktop           files=0  hits=0   (unchanged, though four apps/ files were written)
  ```
  **Three hits, and they are the ones every census since cycle 1 named as clearing on exactly
  this condition.** Not gate-gaming: nothing renamed to duck a regex, no path exempted, no
  rebaseline, and the `use`-collapse still available in `source_content_payload` was **refused
  for the third cycle running**.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical. The parity that
  **was** required for this change is the generated package's own, and it is pinned two ways:
  `gen_desktop_fixture_corpus --check` proves the four documents are byte-reproducible from
  their `.txt` inputs (`files=4 drift=0 verdict=PASS`), and the new desktop test
  `bundled_records_carry_their_real_converted_values` proves the loaded package still carries
  the same real values the raw-`.lst` path produced.

- **Movement, four buckets:**
  - **closure:** 3 live `pcgen_import` hits, named by row —
    `corpus_loader.rs:50` (`use …lst_parser::spell::parse_lst_spell_row`, the last live import
    of a PCGen row parser in the crate), `corpus_loader.rs:515`
    (`ir_converter::convert_spell_record` on the fixture path) and `corpus_loader.rs:528`
    (`ir_converter::convert_equipment_record` on the fixture path). **All three are closures,
    not relabels:** the conversion does not move to another live file, it stops happening at run
    time — cycle 4 booked the same records' move as a *relabel* precisely because the calls
    reappeared elsewhere, and this time they do not reappear. **Zero corpus units**, as Epic 6
    closes none.
  - **relabel:** none. No hit moved from one file or group to another.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff.

- **Refused tokens:** `renderer=5, lst_parser_types=10, ingest_record_tokens=5,
  trait_and_pool_tokens=3, ir_converter=1, source_content_payload=5` — **29 hits / 16 files,
  summing, all under `src/rules_core/`.** Six groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **The condition every census has named since cycle 1 was one cycle of work, not an
    unsized future piece.** Cycles 1–8 recorded the desktop fixture path's converter calls as
    blocked on *"that package [being] produced at build time and read as data"*, with no size
    attached and no owner. It is a 306-line converter-side generator plus a live two-kind
    loader that composes two loaders that already existed, and it removes
    `load_lst_fixture_corpus` from the crate entirely.
    `correction 1789278821965-at-35-e6-003-ruled-738e77`.
  - **A consequence worth stating on its own, because it is wider than the three hits:**
    after this cycle **no live path in the crate parses a raw PCGen row at run time.** The
    census asserts it by row (`live_runtime_row_parse_hits=0`), not by prose. The one
    remaining run-time converter call, `corpus_loader.rs:93`, converts an
    **already-converted** corpus JSON record because the equipment half has no live record
    shape — a different defect, named below.
  - **What is left is the equipment shape, and cycle 8's ruling on it stands unamended.**
    `EquipmentRecord`'s nine live consumers read `record.tokens` and `record.bonus_chains`
    directly; a `CorpusEquipmentRecord` carrying those arrays would move PCGen token
    structures under a live root, which is worse than the hit it clears. It clears when those
    consumers read converted `SheetRule` rows.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=16 live_hits=29`, was `16 / 32` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 29 hits split `renderer=5, lst_parser_types=10, ingest_record_tokens=5, trait_and_pool_tokens=3, ir_converter=1, source_content_payload=5`; `gate_agreement=OK (29 == 29)` | the same 29 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle9_runtime_import_census.py` |
  | 3 hits closed, by row; `live_runtime_row_parse_hits=0` (was 2); `runtime_equipment_conversion_hits=1` (was 2) | the 32 hits at cycle start vs the 29 at HEAD | the census's own row assertions, same command |
  | 4 converted fixture records ship, each carrying a `data.key` | the two `spell/` + two `equipment/` JSON files under the bundled resource root | the census's `shipped_converted_fixture_records` assertion, same command |
  | `files=4 drift=0 verdict=PASS` | the four generated documents vs their four `.txt` inputs | `cargo run --locked --bin gen_desktop_fixture_corpus -- --check` |
  | Longsword `DAMAGE 1d8`, Chain Shirt's first `BONUS` chain, Alarm/Abjuration, Blur/Illusion survive the build-time conversion | the four records of the loaded bundled package | `cd apps/desktop/src-tauri && cargo test --locked bundled_records_carry_their_real_converted_values` |
  | 9 live consumers of `EquipmentRecord` | every live file naming the type | `grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/` |
  | 8 prior receipts, so this is cycle 9 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, desktop
  crate in `/tmp/cargo-sd35-AT-35-E6-003-RULED-desktop`, once each, at the final tree.
  `apps/` was touched, so the desktop crate and the frontend ran **here**, not at the epic
  wrap-up.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3348 passed; 0 failed; 16 ignored
                                         (cycle 8's 3348, unchanged -- this cycle's one new
                                         test is in the DESKTOP crate)
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 targets; 8,877 passed; 0 failed;
                                         69 ignored; zero `test result: FAILED` lines
                                         (cycle 8's 8,877, unchanged, same reason)
  desktop crate (apps/desktop/src-tauri)
                                      -> DESKTOP_EXIT=0; 570 passed; 0 failed; 0 ignored
                                         (cycle 4 recorded 569; the +1 is exactly this
                                         cycle's one new test)
  frontend (apps/desktop, npm test)   -> 101/101 test files passed
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, after one self-heal
                                         (two lints on this cycle's own new bin:
                                         `needless_borrows_for_generic_args`,
                                         `collapsible_if`)
  desktop clippy --locked --tests     -> DCLIPPY_EXIT=0, 1 warning, PRE-EXISTING and named
                                         not swept: `vec_init_then_push`,
                                         `equipment_catalog.rs:889`, untouched file, the
                                         same warning cycles 2 and 4 recorded
  cargo run --bin gen_desktop_fixture_corpus -- --check
                                      -> files=4 drift=0 verdict=PASS
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (115.6s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=16 live_hits=29 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=139 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and
  no `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing `spell_resolver.rs`'s
  `SourceContentPayload` import at `rules_core::source_content`'s own re-export of the same
  enum, for `−1` and zero change in what the module depends on — is **refused for the third
  cycle running**, on the same reasoning cycles 7 and 8 gave. Separately: the generated
  equipment documents carry PCGen token text in `data.raw_tokens`, exactly as every real
  `data/corpus/**` record does. That is **data**, not code, and the residue gate reads only
  `.rs`/`.ts`/`.tsx`/`.js`/`.jsx`/`.mjs`/`.cjs` — the ruling is about what the shipping binary
  reads, and it now reads a JSON field, not a grammar.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 10**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 29-hit remainder. Its largest and only genuinely-unblocked piece is still the
  equipment shape — `lst_parser_types` (10) + `ir_converter` (1) + `source_content_payload`
  (5) + three of `ingest_record_tokens`, **19 hits across 11 files** — which needs
  `equipment_effects`'s nine consumers to read converted `SheetRule` rows rather than
  `EquipmentRecord.tokens`. The `renderer` group (5) stays a converter-parity cycle (cycle 2's
  `disagree=97,332 of 660,320`), and `trait_and_pool_tokens` (3) stays refused on cycle 7's
  corrected number.
