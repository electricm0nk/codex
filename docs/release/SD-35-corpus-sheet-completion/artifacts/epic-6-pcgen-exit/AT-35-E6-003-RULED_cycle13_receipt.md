# Cycle AT-35-E6-003-RULED cycle 13 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `1bbeb8ce2a` (the code, the census script and its JSON, the retro
  events), cycle start `14dd17c319`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `14dd17c319`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 12's closing figure — nothing drifted between the two cycles:
  ```
  live_files=8 live_hits=19 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/pcgen_import/corpus_equipment_json.rs` — **new, +~250, converter side.** The whole of
    `corpus_loader::equipment_record_from_json` moved here verbatim, comments and synthesis
    rules included, plus `corpus_equipment_source_record`, the one question the live loader
    now asks: *"what canonical record does this corpus JSON object stand for?"*. It also holds
    `every_live_corpus_equipment_pair`, a `#[cfg(test)]` walker that yields the
    (ingest row, settled record) pairing the cycle-10/11/12 parity proofs compare against —
    on the converter side, because the envelope stopped carrying a row this cycle.
  - `src/rules_core/corpus_loader.rs` — **five hits → one.** The three ingest-array reads
    (`ingest_record::token_pairs` / `bonus_chain_qualifiers` / `rebuild_bonus_token`), the
    `lst_parser::equipment` import and the `ir_converter::convert_equipment_record` call are
    all gone from the live side; one call to the converter-side reader replaces them. The
    module doc comment was rewritten to say what the loader reads and what it does not.
  - `src/rules_core/equipment_resolver.rs` — **`lst_parser` import GONE.**
    `equipment_key_token` is **deleted**: it read a parser row's `KEY:` token to answer the
    KEY-or-name identity rule, settled as `CorpusEquipmentRecord::identity` since cycle 10.
    `equipment_id_resolve` answers with the settled record now, and cycle 12's
    `equipment_converted_resolve_with_cell` — which became its **exact duplicate** the moment
    the row left the envelope — is **deleted** rather than kept as a pass-through alias. Its
    six call sites in `damage_total` and `equipment_effects` point at `equipment_id_resolve`.
    This file keeps its own one hit; see **Refused tokens**.
  - `src/pcgen_import/source_content_payload.rs` — **the `Equipment` variant collapsed** to
    `Equipment(&'a CorpusEquipmentRecord)`. The `lst_parser::equipment::EquipmentRecord`
    import went with it. `kind_token()` answers `"EQUIP"`/`"EQUIPMOD"` off a settled field
    instead of the parser row's enum.
  - `src/rules_core/equipment_record.rs` — **one new settled field**, `is_modifier`, the
    two-valued answer `kind_token()` used to read off the row. Still no token array and no
    bonus-chain array. Its three whole-corpus parity tests take their pairing from
    `every_live_corpus_equipment_pair` now; **the population, the records and every assertion
    are unchanged.**
  - `src/pcgen_import/ir_converter.rs` — `equipment_record_to_corpus` settles `is_modifier`;
    `convert_equipment_record` builds the single-arm payload and returns `'static`.
  - `src/rules_core/pilot_compute_corpus.rs` — the two live `equipment_key_token` calls read
    `record.identity`. Same string, settled.
  - `src/rules_core/damage_total.rs`, `src/rules_core/equipment_effects.rs` — the deleted
    alias's call sites repointed. No behaviour change.
  - `apps/desktop/src-tauri/src/corpus_fixtures.rs`, `…/corpus_full.rs` — **`#[cfg(test)]`
    regions only.** Both asserted on parser-row tokens through the payload; they assert the
    same three facts on settled fields (`base_damage_dice`, `stat_effect.armor_class_bonus`,
    `weight_lbs`). No shipping desktop line changed, and `root apps/desktop files=0 hits=0`
    is unmoved.
  - `tests/sd17_c_ir_convert.rs`, `tests/sd17_d_record_aggregate.rs`,
    `tests/sd17_e_source_ir_shape.rs`, `tests/sd19_equipment_{arms_armor,equipmods,general,magic_items}.rs`
    — the payload-arity and `equipment_key_token` call sites. The four `sd19_*` files got a
    local `row_identity` helper that asks the converter for a parser row's identity, which is
    what `equipment_key_token` was.
  - `…/AT-35-E6-003-RULED_cycle13_runtime_import_census.py` / `.json` — **new.** Imports cycle
    12's census whole (which imports cycle 11's, … back to cycle 1's), adds one group and
    rewrites four groups' reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `deferral`, 1 `correction`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (appended events from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty),
  so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start
  tree.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff 14dd17c319 -- src/ tests/ apps/ | grep '^+' | grep -v '^+++'`),
  `grep -cE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` prints **0**, and so does the
  same grep over the new census script. First run, no self-heal. (A naive run over the whole
  `BASE_BRANCH...HEAD` diff of the epic's file-touch set reports hits; every one is a **doc
  citation of a pre-existing test file name** written by an earlier cycle's receipt, or a
  `+++ b/tests/sd19_*.rs` diff header — not an identifier in this cycle's added code. Cycles 11
  and 12 recorded the same finding.)

- **Wired-integration audit result:** OK_NO_TOKENS. First run, no self-heal:
  `grep -ciE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the same
  added lines prints **0**. No `"Would …"` string, no inline mock, no fixture-only data path:
  the converter-side reader parses the same real corpus JSON the live loader parsed, and the
  three parity tests assert the real values over the whole live corpus.

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
  first met in cycle 4, not regressed here — this cycle wrote only `#[cfg(test)]` lines under
  `apps/`). The criterion as a whole is **not** met: 14 hits across 8 files remain under
  `src/rules_core/`, and `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=14dd17c319ac6f6b6940b536f70ef9b5f245f667 target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=681 ratio=n/a builds_recorded=3 pcgen_live_files=8
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=8` is the **file** count, unmoved from cycle 12 — and this receipt does not
  dress that up. The **hit** count moved: 19 → 14. `builds_recorded=3` counts **compile
  sessions in `$CARGO_TARGET_DIR`**, not verification passes: the authoring compiles that
  type-checked the moved module and the collapsed payload, the final `cargo test` + `clippy`
  build, and the `sheet_rule_convert --check` build. There was **one** verification pass, after
  the last figure-moving change (`decisions.md` §3).

- **PCGen residue:** `live_files=8 live_hits=14 baseline_files=260 baseline_hits=12736
  verdict=PASS` — hits down 5 from cycle 12's 19, files unchanged at 8, **and the instrument
  was not touched this cycle** (`git diff --name-only 14dd17c319..HEAD -- scripts/` is empty).
  Per-root:
  ```
  root src/rules_core         files=8 hits=19  ->  files=8 hits=14
  root src/saved_character    files=0 hits=0
  root src/campaign           files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop           files=0 hits=0   (unchanged; only #[cfg(test)] lines written)
  ```
  Not gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the
  `use`-collapse still available in `source_content_payload` was **refused for the seventh
  cycle running**. The gate's own self-test is green (`Ran 27 tests, OK`).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` `verdict=PASS`). The parity this change required is the
  three **whole-corpus** proofs cycles 10, 11 and 12 built, which are **kept and unchanged**:
  `every_live_corpus_equipment_record_carries_the_same_values_the_token_reads_produced`,
  `…_the_same_armour_skill_and_weapon_values_…` and `…_the_same_weapon_values_…` still
  re-derive every moved read the way the live modules derived it, straight off the ingest row,
  and compare field for field over **every** book under `data/corpus/`. What changed is only
  where the pairing comes from: `corpus_equipment_json::every_live_corpus_equipment_pair`
  instead of the canonical envelope, because the envelope stopped carrying a row this cycle.
  All three are green in the verification run below (`3352 passed; 0 failed`).

- **Movement, four buckets:**
  - **closure:** 5 live `pcgen_import` hits, named by row and asserted by file **and symbol**
    in the census (`closed_by_cycle13=5`) — `corpus_loader.rs`'s three
    `ingest_record::token_pairs` / `bonus_chain_qualifiers` / `rebuild_bonus_token` reads and
    its `lst_parser::equipment` import, plus `equipment_resolver.rs`'s
    `lst_parser::equipment::EquipmentRecord` import. Two whole census groups read zero as a
    result: `lst_parser_types` and `ir_converter`. **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** 1, named so it can never be counted as a closure
    (`relabelled_by_cycle13=1`). `corpus_loader.rs`'s `ir_converter::convert_equipment_record`
    call did not vanish — it became one call to
    `corpus_equipment_json::corpus_equipment_source_record`, in the same file, booked under
    the census's new `corpus_json_boundary` group. Six hits out, one hit back, net −5.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and `scripts/pcgen-residue-baseline.env`
    are absent from this cycle's diff.

- **Refused tokens:** `corpus_json_boundary=1, renderer=5, ingest_record_tokens=2,
  trait_and_pool_tokens=3, source_content_payload=3` — **14 hits / 8 files, summing, all under
  `src/rules_core/`.** Five groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **The two "last two consumers" were not consumers at all, and collapsing the payload was
    one move rather than a sequence.** Cycle 12 said so and was right: neither
    `equipment_resolver` nor `corpus_loader` read a rules value. What cycle 12 did **not**
    predict is that clearing them makes cycle 12's own `equipment_converted_resolve_with_cell`
    an **exact duplicate** of `equipment_id_resolve` — same body, same signature — because the
    only thing that distinguished them was which half of the pair they threw away. Keeping
    both would have been duplicated shipping code, so the alias is deleted and its six call
    sites point at the original name. Re-derive: the census asserts
    `equipment_converted_resolve_with_cell=deleted`.
  - **The payload's arity was load-bearing in seven test files, none of them named in the
    epic's file-touch set.** Two integration tests asserted `std::ptr::eq(payload, entry)` —
    the zero-copy projection the enum's own doc comment promises — and four `sd19_*` suites
    called `equipment_key_token` on a parser row. The zero-copy claim is now false for two of
    the seven variants (spell since cycle 8, equipment since this cycle) and the test that
    asserted it says so in its own doc comment instead of being deleted.
  - **`corpus_loader`'s residue was never five separate problems.** It was one: the loader
    rebuilding an ingest row out of corpus JSON. Moving that one function moved four hits and
    left one, and the one that is left is the ingest boundary itself — which does not clear by
    moving code at all. It clears when `data/corpus/` equipment JSON carries the settled
    fields, the shape `data/sheet_rules/` already has. That is the first time this criterion's
    remainder has named a **data** step rather than a code step.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=8 live_hits=14`, was `8 / 19` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 14 hits split `corpus_json_boundary=1, renderer=5, ingest_record_tokens=2, trait_and_pool_tokens=3, source_content_payload=3`; `gate_agreement=OK (14 == 14)` | the same 14 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle13_runtime_import_census.py` |
  | `closed_by_cycle13=5 relabelled_by_cycle13=1 cleared_files_by_cycle13=0`; `runtime_equipment_conversion_hits=0` (was 1); `equipment_key_token=deleted`; `equipment_converted_resolve_with_cell=deleted`; `parser_borrowing_variants=5`; cycles 10/11/12's cleared files still clear | the 19 hits at cycle start vs the 14 at HEAD | the census's own row, file and symbol assertions, same command |
  | the three whole-corpus parity proofs green, 0 disagreements each | every `data/corpus/<book>/equipment/**/*.json` the live loader reads | `cargo test --locked --lib -j 6` (all three are `rules_core::equipment_record::tests::every_live_corpus_equipment_record_*`) |
  | 0 live files name `lst_parser::` (was 2) | every live file, comments and `#[cfg(test)]` excluded as the gate excludes them | `python3 …_cycle13_runtime_import_census.py` (group `lst_parser_types`: hits=0 CLEARED) |
  | 3,352 lib tests pass (cycle 12 recorded 3,351; +1 is this cycle's `a_data_object_with_no_key_is_not_a_record`; the three `equipment_record_from_json_*` tests moved rather than were added) | the crate's own unit tests | `cargo test --locked --lib -j 6` |
  | 8,881 tests pass across 418 targets + 1 Doc-tests (cycle 12 recorded 8,880; the +1 is the same new test) | every target in the root workspace | `cargo test --locked --no-fail-fast -j 6` |
  | 12 prior receipts, so this is cycle 13 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree; the desktop crate in its own `/tmp/cargo-sd35-AT-35-E6-003-RULED-desktop`, because
  `apps/` was written this cycle (`#[cfg(test)]` lines only, but they must compile and pass).
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3352 passed; 0 failed; 16 ignored
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 Running targets + 1 Doc-tests;
                                         8,881 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, no self-heal
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (116.6s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=8 live_hits=14 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=142 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  desktop crate (apps/desktop/src-tauri)          -> `DESKTOP_EXIT=0`; `test result: ok. 570 passed; 0 failed; 0 ignored`
                                             (1,398s). `cargo clippy --locked --tests -j 4`
                                             `DESKTOP_CLIPPY_EXIT=0`, 1 warning, in
                                             `equipment_catalog.rs:889` — a `#[cfg(test)]`
                                             pinned list in a file this cycle did not touch,
                                             pre-existing and not denied.
                                             570 is exactly the crate's `#[test]` count at
                                             BOTH this cycle's start tree and HEAD, so this
                                             cycle added and removed none; earlier receipts'
                                             575 is a figure from an older tree
  frontend                                        -> not run; no `.ts`/`.tsx` file was
                                                     written this cycle
                                                     (`git status --porcelain apps/desktop/src/`
                                                     empty); the frontend runs at the epic
                                                     wrap-up
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and
  no `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing an import at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and zero change in
  what the module depends on — is **refused for the seventh cycle running**, on the same
  reasoning cycles 7 through 12 gave. This cycle moved the real thing instead: the `Equipment`
  variant no longer borrows a parser entry type, so equipment joins cycle 8's spell as a kind
  whose canonical payload names no `pcgen_import` type at all. Five of the enum's seven
  variants still do, which is why the enum itself cannot move yet.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 14**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 14-hit remainder. Three of the five groups are refused on measured numbers and one is
  measured non-relocatable, so the unblocked piece is **`corpus_json_boundary` (1) plus
  `ingest_record_tokens`' `race_resolver` half**, and the first of those is a **data** step,
  not a code step: an ingest-side generator that writes the settled equipment fields into
  `data/corpus/<book>/equipment/**/*.json` (or a sidecar beside it), the same producer shape
  `data/sheet_rules/` already has, after which `corpus_loader` deserializes settled values with
  serde and names nothing. That clears the loader's last hit **and its whole file**. The
  `renderer` group (5) stays a converter-parity cycle (cycle 2's `disagree=97,332 of 660,320`),
  `trait_and_pool_tokens` (3) stays refused on cycle 7's corrected number,
  `source_content_payload` (3) stays refused on the use-collapse reasoning, and
  `derived_evaluator_fixture_check` (1) stays measured non-relocatable (cycle 3).
