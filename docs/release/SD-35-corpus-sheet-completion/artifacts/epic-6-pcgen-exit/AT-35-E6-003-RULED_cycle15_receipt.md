# Cycle AT-35-E6-003-RULED cycle 15 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `4b77b31ee3` (the code, the 48 settled bundles, the census script and its JSON,
  three retro events, and the folded shared-checkout artifacts), cycle start `66b36389b3`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `66b36389b3`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 14's closing figure — nothing drifted between the two cycles:
  ```
  live_files=7 live_hits=12 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**

  **The live side — it reads settled data now, and names no converter:**
  - `src/rules_core/settled_corpus.rs` — **new, +~250, live side.** The live reader for a book's
    settled-record bundle: `SettledBundle<T>` (a `kind` tag plus a `BTreeMap` from *record path
    relative to its kind directory* → settled record), `SettledEquipmentEntry` (the settled
    record **plus** the provenance anchor the canonical envelope carries), `bundle_key`,
    `settled_bundle_path`, `corpus_book_dirs`, and a `SettledBundleError` enum whose four
    variants are each reported by name. **Pure serde.** The census asserts by text that this
    file names `pcgen_import` nowhere in shipping code — a live reader that named the converter
    would just be the old boundary under a new name.
  - `src/rules_core/corpus_loader.rs` — **two hits → ZERO; the file is cleared.**
    `load_equipment_corpus` reads `<book>/_settled/equipment.json` once per root and wraps each
    settled record with the new live `SourceContentRecord::equipment` constructor; the two race
    boundary functions cycle 14 parked here are **deleted outright** along with
    `use crate::pcgen_import::corpus_race_json`. The file walk, the per-file diagnostics and the
    push order are unchanged, so a resolver that decides a key collision by position sees the
    same package it saw before.
  - `src/rules_core/source_content.rs` — `SourceContentRecord::equipment(source_ref, record)`,
    the equipment sibling of cycle 8's `::spell` constructor, plus `SourceRef` gaining
    `Serialize`/`Deserialize` so the stored provenance anchor round-trips. Its own
    `source_content_payload` hit is **untouched and still refused** (see **Notes**).
  - `src/rules_core/race_resolver.rs` — `load_chassis_dir` / `load_trait_dir` read
    `<book>/_settled/race.json` and `…/race_trait.json`. They still walk the corpus files, still
    read each record's `source`, `license` and `pi_field` off disk, and still report a duplicate
    chassis and a redacted description exactly as before; the payload deserialization narrowed
    from `serde_json::Value` to `serde::de::IgnoredAny`, because nothing on the live side reads
    that object any more. The file stays at **zero** hits, held from cycle 14.
  - Eighteen settled types gained `Serialize`/`Deserialize`, and nothing else:
    `CorpusEquipmentRecord`, `CorpusRaceRecord`, `CorpusRaceTraitRecord`, `SourceRef`,
    `AbilityScoreBonus`, `IntelligentItemContribution`, `ItemAlignment`, `EquipmentStatEffect`,
    `SkillCheckBonus`, `VarBonus`, `WeaponEnhancementBonus`, `DiceExpression`, `WieldCategory`,
    `SizeCategory`, `DeclaredBonuses`, `AbilityAdjustment`, `VarContribution`, `TargetBonus`.
    Every one is already a settled value type — a weight in pounds, a resolved `+2`, a `1d8` —
    so serializing one carries no ingest grammar across the boundary. No field was added,
    removed or renamed.

  **The converter side — it does the same reading, once, at authoring time:**
  - `src/pcgen_import/corpus_settled_bundle.rs` — **new, +~330, converter side.** Builds each
    bundle by making **the exact call the live loader made at run time before this cycle**:
    `corpus_equipment_json::corpus_equipment_source_record`,
    `corpus_race_json::corpus_race_source_record` and `::corpus_race_trait_source_record`. It
    restates the two live loaders' traversal rule (recurse, skip `_parity/`, skip
    `LICENSE.json`, sorted) and **asserts that restatement against the real corpus** rather than
    asserting it in prose. Plus three whole-corpus proofs — see **Oracle parity**.
  - `src/bin/gen_settled_corpus.rs` — **new, +~120.** The producer, with `--check`. Its roots are
    every book directory under `data/corpus/` **plus** the desktop's bundled
    `corpus_fixtures/` root, which `corpus_loader` reads with the same function.
  - `src/pcgen_import/mod.rs`, `src/rules_core/mod.rs` — the two new modules registered.
  - `src/bin/gen_desktop_fixture_corpus.rs` — doc comment only: it now says that a fixture
    record change must be followed by `gen_settled_corpus`, and that `--check` fails loudly if
    it is not.

  **The data — 48 new files, and not one corpus record rewritten:**
  - `data/corpus/<book>/_settled/{equipment,race,race_trait}.json` — **47 new bundles across 39
    books, 8,761 settled records, 11 MB**, plus
    `apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json` (2 records) = 48
    bundles / 8,763 records. `_settled/` sits **beside** the kind directories, never inside one
    (the census asserts `bundles_inside_a_kind_dir=0`), so no walk that starts *at* a kind
    directory — which is every live loader — can mistake a bundle for a record. Four
    instruments that enumerate a book's subdirectories instead **did** need to learn it; see
    below, and see **Discoveries** for the correction that recorded the wrong assumption.
    **`git status --porcelain` lists no modified file under `data/corpus/`**: every ingested
    record, its license block and its `pi_*` redaction stamps are byte-identical. That was the
    deciding reason for a bundle beside the records rather than a settled block inside each of
    8,762 of them — post-hoc mutation of generated corpus artifacts is a recorded way this repo
    has destroyed those stamps before.
  - `apps/desktop/src-tauri/tauri.conf.json` — one line: `resources/corpus_fixtures/_settled/`.
    Tauri's `resources` list is per-directory and non-recursive (`spell/` and `equipment/` are
    already listed individually), so without this line a **packaged** installer would ship the
    fixture records without their bundle and the desktop would load zero fixture equipment.
    This is the "closed under the change it mandates" check, not scope creep.

  **Four corpus-shape gates learned the new directory — named as a category, not exempted:**
  Seven tests across four binaries refused `_settled/` on the first verification pass, and every
  one of them was right to: they enumerate a book's subdirectories and require each to be either
  a named content kind or a named non-record directory. **All four already carried exactly this
  disposition for `_parity/`**, so `_settled` was added beside it with its own written rationale
  and no gate was weakened:
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `NON_CONTENT_CORPUS_DIRS` gains `_settled`
    ("a generated per-book index of already-counted equipment/race/race_trait records … every
    entry is keyed by the path of a record this gate already counts under its own kind
    directory"). Counting it as a kind would have **double-counted all 8,761 records** and
    invented three unreachable families per book.
  - `tests/sd27_book_license_record_counts.rs` — `NON_RECORD_DIRS` becomes
    `["_parity", "_settled"]`, so the OGL compliance count is not inflated by a kind that ships
    no licensed text of its own.
  - `tests/sd26_cache_core_rulebook.rs`, `tests/sd27_license_stripping_shape_v1.rs` — their
    recursive walks skip the directory, the same way one of them already skipped `_parity/`.

  **Tests:**
  - `src/rules_core/race_resolver.rs`'s two synthetic-corpus tests call
    `pcgen_import::corpus_settled_bundle::write_bundles_for_book` on their temp dir — in
    `#[cfg(test)]` code, which is where `decisions.md` §11 allows the converter to be named, and
    which ruling B15 excludes from the gate. Both tests' assertions are unchanged.

  - `…/AT-35-E6-003-RULED_cycle15_runtime_import_census.py` / `.json` — **new.** Imports cycle
    14's census whole (which imports cycle 13's, … back to cycle 1's), **widens no regex**, and
    adds five assertions of its own.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `incident`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (`derived_at` restamped by this cycle's `completion_atlas.py --check`), plus
    `docs/retro/events/root.jsonl` and `docs/retro/events/sd31-transcribe.jsonl` (appended events
    from another session). Committed rather than filtered away, per the standing "clean tree =
    unfiltered `git status` empty" rule.

  **`docs/work-inventory.json` is byte-identical**, which is correct: Epic 6 closes zero corpus
  units and no corpus record changed. See **Figures** for the run that proves the refusal to
  rewrite it is pre-existing rather than caused here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 66b36389b3 -- src/ tests/ apps/desktop/src-tauri/src/ | grep '^+' | grep -v '^+++'`,
  plus the three new Rust files and the new census script in full),
  `grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` prints **0**. First run, no
  self-heal. The prescribed tranche-wide form of the same grep
  (`git diff --unified=0 fe5ae6cd4a...HEAD -- <scoped paths>`) prints **155**, every one of them
  a prior cycle's `tests/sd27_*` / `tests/sd35_*` file name in a doc citation and none of them
  added here — the cycle-local figure above is the one this cycle is accountable for, the same
  reading cycles 11 through 14 recorded.

- **Wired-integration audit result:** OK_NO_TOKENS. First run, no self-heal.
  `grep -ciE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the same
  added lines prints **0** (tranche-wide: **158**, all pre-existing, dominated by the repo's
  established *"PCGen's `%LIST` placeholder"* idiom). No `"Would …"` string, no inline mock, no
  fixture-only data path, no stub: the producer runs the same converter calls over the real
  corpus, the live side reads the real files those calls wrote, and three whole-corpus proofs
  compare the two.

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
  first met in cycle 4, not regressed here — this cycle wrote no line of Rust under `apps/`).
  The criterion as a whole is **not** met: 10 hits across 6 files remain under
  `src/rules_core/`, and `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=66b36389b3 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1034 ratio=n/a builds_recorded=2 pcgen_live_files=6
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no corpus
  record changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=6` is the **file** count, down one from cycle 14's 7 — the second
  consecutive cycle to clear a whole file. The **hit** count moved too: 12 → 10.
  `rust_lines_changed=1034` is the committed Rust diff, the three new modules included; the 11 MB
  of new `data/` is generated, not written by hand, and is not counted as source.
  **There were TWO verification passes and this receipt says so.** The first, after the last
  figure-moving change, was the one that found the four corpus-shape gates (5 workspace tests
  across 3 binaries, 2 desktop tests); the second, after the named fix, is the one reported
  below. That is `decisions.md` §3's rule working as written — one pass, a real red, one fix,
  one re-run — not one item per build.

- **PCGen residue:** `live_files=6 live_hits=10 baseline_files=260 baseline_hits=12736
  verdict=PASS` — files down 1 from cycle 14's 7, hits down 2 from 12, **and the instrument was
  not touched this cycle** (`git diff --name-only 66b36389b3..HEAD -- scripts/` is empty).
  Per-root:
  ```
  root src/rules_core         files=7 hits=12  ->  files=6 hits=10
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; no Rust line under apps/ was written)
  ```
  Not gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the
  boundary group's regex is **byte-identical to cycle 14's** — it simply matches nothing under a
  live root any more. The `use`-collapse still available in `source_content_payload` was
  **refused for the ninth cycle running**. The gate's own self-test is green
  (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` `verdict=PASS`). The parity this change required is **three
  new whole-corpus proofs**, in `pcgen_import::corpus_settled_bundle`:
  `every_on_disk_equipment_bundle_matches_the_run_time_call_it_replaced`,
  `every_on_disk_race_bundle_matches_the_run_time_call_it_replaced` and
  `no_on_disk_bundle_has_drifted_from_the_corpus`. The first two rebuild every bundle in memory
  **by re-running the exact call the live loader made at run time**, on the exact same corpus
  files, and compare it field for field — provenance anchor included — against what is on disk,
  over **every** book under `data/corpus/`. The third is `--check`'s own claim as a test. A
  reading widened, narrowed or reordered by the move fails there, on the real corpus, not on a
  fixture. All three green, **0 disagreements**, and a fourth
  (`the_generator_walks_exactly_the_files_the_live_loader_walks`) pins the traversal rule the
  producer restates. Kept alongside them and unchanged: the racial-trait prose oracle
  (`every_racial_trait_renders_the_same_sentence_from_the_converted_package`, `compared == 919`,
  0 disagreements), which still reads the ingest arrays off disk itself because that is what
  makes it an oracle.

- **Movement, four buckets:**
  - **closure:** 2 live `pcgen_import` hits and **1 whole file**, named by row and asserted by
    file **and symbol** in the census (`closed_by_cycle15=2 cleared_files_by_cycle15=1`) —
    `corpus_loader.rs`'s `corpus_equipment_json::corpus_equipment_source_record` call and its
    `corpus_race_json` import, and with them the live side's **last converter call of any kind**.
    The census asserts the file names `pcgen_import`, `corpus_equipment_json`,
    `corpus_race_json`, `raw_tokens` and `raw_bonus_chains` **nowhere** in shipping code
    (`corpus_loader_shipping_pcgen_import_hits=0 (was 2)`) rather than inferring it from the
    gate's file list. **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** **none**, and booked explicitly as none (`relabelled_by_cycle15=0`). Cycle 14
    had to book one because its boundary call moved from one live file to another. This one did
    not move to another live file: it left the live side for a build-time producer, so all of
    `12 → 10` is closure.
  - **reachability:** none.
  - **instrument-correction:** none. `scripts/pcgen_residue_gate.py` and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff. (The SD-34 atlas
    artifact restamped by `completion_atlas.py --check` is *that* instrument re-deriving its own
    output on an unchanged claim; it closes nothing and is folded, not claimed.)

- **Refused tokens:** `renderer=5, source_content_payload=3, ingest_record_tokens=1,
  trait_and_pool_tokens=1` — **10 hits / 6 files, summing, all under `src/rules_core/`.** Four
  groups, under this cycle's flag-cap of 10, and one group fewer than cycle 14 had.

- **Discoveries:**
  - **The boundary was a placement problem, not a conversion problem — again, and this time the
    fix was data.** Cycles 13 and 14 both moved *where the reading is spelled* and both were
    left holding the *call*. The call could not move anywhere on the live side; what it needed
    was to stop happening at run time at all. Producing the answer at authoring time cost one
    generator, one serde reader and eighteen `derive` lines, and it is the same shape
    `data/sheet_rules/` has had since `AT-35-E2-001`. **A boundary call is a build step wearing
    a function call's clothes.**
  - **The provenance anchor had to be settled too, and nothing had noticed.** The equipment
    envelope's `source_ref` was not read off the corpus JSON — it came from the *rebuilt ingest
    row* inside `convert_equipment_record`. Deriving it live would have been the live side
    re-deriving a converter value under a different name, so `SettledEquipmentEntry` stores it.
    The race kinds needed no equivalent: their resolver already read `source` off the corpus
    record itself.
  - **A per-book bundle beat a per-record field on a measured risk, not a preference.** Writing
    a settled block into each of 8,762 ingested records is a read-modify-write of the files that
    carry this project's license and `pi_*` redaction stamps — the exact operation whose
    post-hoc form has destroyed those fields here before. 48 files beside the records mutate
    none of them, and keep the loader's existing per-file diagnostics and push order intact.
  - **A new generated directory under `data/corpus/` is not inert, and the assumption that it
    was is this cycle's one recorded correction.** The live loaders' walks start *at* a kind
    directory, so a bundle beside them is invisible to those — but **four** other instruments
    enumerate a book's subdirectories and demand each be a named kind or a named non-record
    directory. Seven tests said so on the first verification pass, before any prose did. Every
    one of them already had the `_parity/` disposition to copy; the fix was to make the same
    category statement, not to relax a gate. Logged as a retro `correction` with the failing and
    passing commands as its `--verified-by`.
  - **`corpus_literal_sweep` reads the bundles and is unbothered**: `51523` files read and
    `48706` records **examined**, 0 findings, CLEAN — the bundles are read and then skipped,
    because a settled record is not a corpus record and carries no source citation to compare.
    (The read count includes this cycle's 48 new files; the examined count is the figure that
    must not move, and it did not.) The 48 new files carry no `BONUS:`,
    `DEFINE:`, `PRE…:`, `%CHOICE` or `CL=` string — verified by grep, 0 of 48 — because a
    settled record has no grammar in it to carry.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=6 live_hits=10`, was `7 / 12` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 10 hits split `renderer=5, source_content_payload=3, ingest_record_tokens=1, trait_and_pool_tokens=1`; `gate_agreement=OK (10 == 10)` | the same 10 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle15_runtime_import_census.py` |
  | `closed_by_cycle15=2 relabelled_by_cycle15=0 cleared_files_by_cycle15=1`; `corpus_loader_shipping_pcgen_import_hits=0 (was 2)`; `settled_corpus=live_serde_only converter_names_in_reader=0`; `corpus_settled_bundle=present moved_boundary_calls_on_converter_side=3 producer_bin=present`; cycles 10/11/12/14's cleared files still clear | the 12 hits at cycle start vs the 10 at HEAD | the census's own row, file and symbol assertions, same command |
  | `books=39 settled_bundles=47 settled_records=8761 missing_bundles=0 bundles_inside_a_kind_dir=0` | every book directory under `data/corpus/` that states an `equipment/`, `race/` or `race_trait/` directory | the same census command |
  | `roots=40 bundles=48 records=8763 skipped=0 drifted=0 verdict=PASS` | the 39 books plus the desktop fixture root | `cargo run --locked --bin gen_settled_corpus -- --check` |
  | the three whole-corpus bundle proofs green, 0 disagreements each; `books >= 20`, `compared >= 5000` equipment, `chassis >= 20`, `traits >= 500` asserted as floors | every `data/corpus/<book>/{equipment,race,race_trait}/**/*.json` the live loaders read | `cargo test --locked --lib -j 6` (`pcgen_import::corpus_settled_bundle::tests::*`) |
  | 0 of 48 settled bundles carry a PCGen literal | the 48 bundle files | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' $(find data/corpus apps/desktop -path '*_settled*' -name '*.json') \| wc -l` |
  | the racial-trait prose oracle green, `compared == 919`, 0 disagreements | every racial-trait record every book under `data/corpus/` loads | `cargo test --locked --test sd35_race_trait_prose_comes_from_the_converted_package` |
  | 3,365 lib tests pass (cycle 14 recorded 3,355; the delta is this cycle's ten new tests — six in `settled_corpus`, four in `corpus_settled_bundle`) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,894 tests pass across 419 targets + 1 Doc-tests, 0 failed, 69 ignored (cycle 14 recorded 8,884 across 418; +1 target is the new `corpus_settled_bundle` test surface reached through the lib, +10 tests are this cycle's own) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | **the work-inventory refusal is pre-existing, not caused here**: the identical message, the same `230` stamps, the same 10 first offenders and the same `sheet-complete … stamped 23085 unit(s) (dice=911, number=4768, words=17406)` rung are produced at cycle 14's own HEAD | the 32,617 verification stamps the committed inventory carries | `git worktree add <tmp> 66b36389b3 && cd <tmp> && CORPUS_LITERAL_SWEEP_REPORT=… DERIVED_FIXTURE_CHECK_REPORT=… cargo run --locked --release --bin v06_work_inventory` |
  | 14 prior receipts, so this is cycle 15 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree. The desktop crate ran in its own `/tmp/cargo-sd35-AT-35-E6-003-RULED-desktop`
  because this cycle changed how `corpus_loader` — which the desktop calls directly — obtains
  every equipment record, and added a bundled resource it must find.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3365 passed; 0 failed; 16 ignored
  cargo test --locked --no-fail-fast  -> FULL2_EXIT=0; 419 Running targets + 1 Doc-tests;
                                         8,894 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, no self-heal needed
  cargo run --locked --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (122.9s)
  cargo run --locked --bin gen_settled_corpus -- --check
                                      -> roots=40 bundles=48 records=8763 skipped=0
                                         drifted=0 verdict=PASS
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  the same grep over the 48 new `_settled/` bundles                            -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=6 live_hits=10 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=145 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS (11 hits, 11 baseline rows)
  cargo run --locked --bin declared_pi_shipping_audit
                                      -> declared-pi-audit: CLEAN (run because this cycle adds
                                         files under `data/corpus/`, and this is the stage that
                                         cross-checks what shipped against the corpus's own
                                         NAMEISPI:/DESCISPI: declarations)
  cargo run --locked --bin corpus_literal_sweep   -> 48706 records examined of 51523 read,
                                                     413314 tokens compared, 0 findings, CLEAN
  desktop crate (apps/desktop/src-tauri)          -> `DESKTOP_EXIT=0`; `test result: ok.
                                                     570 passed; 0 failed; 0 ignored`
                                                     (1,450.98s). 570 is the standing figure
                                                     cycle 14 recorded; this cycle added and
                                                     removed none. `cargo clippy --locked
                                                     --tests -j 4` `DESKTOP_CLIPPY_EXIT=0`,
                                                     1 warning, `equipment_catalog.rs:889` --
                                                     a `#[cfg(test)]` pinned list in a file this
                                                     cycle did not touch, pre-existing, the same
                                                     warning cycle 14 reported, not denied
  frontend                                        -> not run; no `.ts`/`.tsx` file was written
                                                     this cycle (`git status --porcelain
                                                     apps/desktop/src/` empty); the frontend
                                                     runs at the epic wrap-up

  FIRST PASS, before the fix, reported in full rather than overwritten:
  cargo test --locked --no-fail-fast  -> FULL_EXIT=101; 3 binaries FAILED, 5 tests:
                                         sd26_cache_core_rulebook (2),
                                         sd27_book_license_record_counts (2),
                                         sd27_license_stripping_shape_v1 (1)
  desktop crate                       -> DESKTOP_EXIT=101; 568 passed; 2 failed:
                                         reach_gate::tests::the_inventory_is_populated_from_all_
                                         three_live_sources and ::dispatch_gap_race_and_monster_
                                         families_all_have_book_level_reach_arms
  Every one of the seven was the same finding -- a book subdirectory the instrument could not
  name -- and every one was fixed by the named-category disposition each instrument already
  carried for `_parity/`.
  ```

- **Sweep population:** `corpus_literal_sweep` **48,706 records examined of 51,523 read**, 0
  findings, CLEAN. It ran because files were added under `data/`; the record population it
  examines is unmoved, the read population rises by exactly the 48 new bundle files.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing an import at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and zero change in what
  the module depends on — is **refused for the ninth cycle running**, on the same reasoning
  cycles 7 through 14 gave. Two doc-comment-only edits
  (`gen_desktop_fixture_corpus.rs`'s header, and the one-line `tauri.conf.json` resource entry)
  landed after the workspace run started; neither moves a figure, and the final tree was
  re-proved to build with `cargo test --locked --no-run -j 6` at the end.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 16**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`, on
  the 10-hit remainder. The largest piece is **`renderer` (5)** and it is unchanged in shape
  from cycle 2's measurement: a **converter-parity** cycle, closing the gap that made the
  converted candidate disagree with `render_pcgen_desc_with_values` on 97,332 of 660,320
  comparisons across 2,443 record keys. Clearing it takes the gate to `5 / 5` and removes the
  last live *call* into the renderer. `source_content_payload` (3) stays refused on the
  use-collapse reasoning and clears when the four remaining parser-borrowing variants of
  `SourceContentPayload` are converted, `trait_and_pool_tokens` (1,
  `class_feature_pool_catalog`) stays refused on cycle 7's corrected number, and
  `ingest_record_tokens` (1, `derived_evaluator_fixture_check`) stays measured non-relocatable
  (cycle 3).
