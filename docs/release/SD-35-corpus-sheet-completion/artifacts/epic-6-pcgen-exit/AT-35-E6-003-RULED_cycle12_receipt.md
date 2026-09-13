# Cycle AT-35-E6-003-RULED cycle 12 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `da8eed6c45` (the code, the census script and its JSON, the retro events),
  cycle start `8d454fa1b9`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `8d454fa1b9`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 11's closing figure — nothing drifted between the two cycles:
  ```
  live_files=10 live_hits=21 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/pcgen_import/ir_converter.rs` — **+~120, converter side.** Ten reads moved here
    verbatim from the two live modules below, as `base_damage_dice_of`,
    `states_base_damage_of`, `base_item_of`, `wield_category_of`, `critical_threat_range_of`,
    `critical_multiplier_of`, `damage_size_steps_of`, `weight_divisor_of`,
    `is_natural_attack_of` and `is_shield_of`. `equipment_record_to_corpus` calls them; it is
    the same **one** call `convert_equipment_record` already made, doing more — **not a second
    call**, which is why the `ir_converter` hit count did not rise.
  - `src/rules_core/equipment_record.rs` — **ten new settled fields** on
    `CorpusEquipmentRecord`: `base_damage_dice`, `states_base_damage`, `base_item`,
    `wield_category`, `critical_threat_range`, `critical_multiplier`, `damage_size_steps`,
    `weight_divisor`, `is_natural_attack`, `is_shield`. Still **no token array and no
    bonus-chain array** — the shape cycles 8 and 9 refused, and that refusal still stands.
    Carries this cycle's parity test.
  - `src/rules_core/damage_total.rs` — **`pcgen_import` import GONE.** The `DAMAGE:`,
    `BASEITEM:`, `WIELD:`, `CRITRANGE:` and `CRITMULT:` token reads and the
    `BONUS:EQMWEAPON|DAMAGESIZE` chain scan are deleted from the live side, together with the
    five private `*_token` functions that held them. All six work-units resolve through
    `equipment_converted_resolve_with_cell` and report `record.identity` as the weapon record
    key — the same KEY-or-name rule `equipment_key_token` applied, settled since cycle 10. The
    one thing that stayed live is the `BASEITEM:` **chase**: `base_item_damage_dice` reads the
    settled identity and resolves it through the same resolver, one hop, because a corpus
    resolution is not something the converter can do.
  - `src/rules_core/equipment_effects.rs` — **`pcgen_import` import GONE.** The `TYPE:` reads
    behind `is_natural_attack_weapon` and `is_weapon_record`, and the `BONUS:EQM|WEIGHTDIV`
    chain scan inside `resolve_eqm_weightdiv_effect`, are deleted from the live side. The
    parser-row `eqmod_referenced_records` is **deleted whole**, its last two callers having
    moved to `eqmod_referenced_converted_records`. `resolve_weapon_to_hit_bonus` and
    `compute_equipment_effects` take converted records only.
  - `src/rules_core/equipment_resolver.rs` — cycle 11's `equipment_pair_resolve` is replaced by
    `equipment_converted_resolve_with_cell`, which answers with the settled record and the
    table cell and never names the parser row. Its sole caller,
    `compute_equipment_effects`, stopped needing the parser row this cycle. This file keeps its
    own two hits; see **Refused tokens**.
  - `…/AT-35-E6-003-RULED_cycle12_runtime_import_census.py` / `.json` — **new.** Imports cycle
    11's census whole (which imports cycle 10's, … back to cycle 1's) and rewrites two groups'
    reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (appended events from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty),
  so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start
  tree. **`apps/` was NOT touched**, so the desktop crate and the frontend run at the epic
  wrap-up, not here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff -- src/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing, and so does the
  same grep over the new census script. First run, no self-heal. (A naive run over the whole
  `BASE_BRANCH...HEAD` diff of the epic's file-touch set reports hits; every one is a **doc
  citation of a pre-existing test file name** written by an earlier cycle's receipt, not an
  identifier in this cycle's added code — the same finding cycle 11 recorded.)

- **Wired-integration audit result:** OK_NO_TOKENS. First run, no self-heal:
  `grep -niE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the same
  added lines returns nothing. No `"Would …"` string, no inline mock, no fixture-only data
  path: the converter reads the same real corpus records the live side read, and the parity
  test asserts the real values over the whole live corpus.

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
  first met in cycle 4, not regressed here). The criterion as a whole is **not** met: 19 hits
  across 8 files remain under `src/rules_core/`, and `render_pcgen_desc_with_values` is still
  called there.

- **Receipt rows (mechanical):**
  ```
  since=8d454fa1b99c5ed0a6276a79d7f03fd139ce0e8b target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=793 ratio=n/a builds_recorded=2 pcgen_live_files=8
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=8` is the **file** count, down 2 from cycle 11's 10 — the third consecutive
  cycle to move it. The hit count moved with it: 21 → 19.
  `builds_recorded=2` counts **compile sessions in `$CARGO_TARGET_DIR`**, not verification
  passes: the authoring compile that proved the moved modules type-check and ran the parity
  test with its mutation, and the final `cargo test` + `clippy` build. There was **one**
  verification pass, after the last figure-moving change (`decisions.md` §3).

- **PCGen residue:** `live_files=8 live_hits=19 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 11's `10 / 21` on **both** counts, **and the instrument was
  not touched this cycle** (`git diff --name-only 8d454fa1b9..HEAD -- scripts/` is empty), so
  the `−2 files / −2 hits` is entirely code. Per-root:
  ```
  root src/rules_core         files=10 hits=21  ->  files=8 hits=19
  root src/saved_character    files=0  hits=0
  root src/campaign           files=0  hits=0
  root src/homebrew_authoring files=0  hits=0
  root apps/desktop           files=0  hits=0   (unchanged; no apps/ file was written)
  ```
  **Two hits and two whole files, and both files are closures, not relabels.** The read does
  not move to another live file — it stops being a token read. Not gate-gaming: nothing renamed
  to duck a regex, no path exempted, no rebaseline, and the `use`-collapse still available in
  `source_content_payload` was **refused for the sixth cycle running**.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` `verdict=PASS`). The parity that **was** required for this
  change is the moved reads' own, and it is as wide as cycles 10's and 11's:
  `every_live_corpus_equipment_record_carries_the_same_weapon_values_the_token_reads_produced`
  loads **every** book under `data/corpus/` — **7,803 equipment records** — re-derives all ten
  moved reads the way the live modules derived them, straight off the parser row still paired
  with the converted record in the canonical envelope, and compares **field for field**: the
  base damage die, the damage-presence flag `is_weapon_record` actually tested, the stand-in
  item identity, the wield category, the threat range converted from a stated width, the
  multiplier with its `x` prefix, the summed damage-size steps, the weight divisor, and the two
  weapon type predicates. It **also** compares the weight divisor's dividend as `f32`, because
  the live side parsed `WT:` directly to `f32` and now narrows the settled `f64` — not
  textually the same operation, so it is measured rather than assumed. **0 disagreements.**
  The test was **mutation-proved**: adding `+ 1` to the converter's critical-multiplier read
  turns it red on **522 of 7,803** records (`Bardiche`, `Bec de Corbin`, `Swashbuckler's
  Rapier` among them), and the mutation was reverted and re-verified green.

- **Movement, four buckets:**
  - **closure:** 2 live `pcgen_import` hits and 2 whole files, named by row —
    `damage_total.rs:106` and `equipment_effects.rs:51`, both
    `lst_parser::equipment::EquipmentRecord`. **Both files are closures:** each file's token
    reading is deleted from the live side, not moved to another live file — cycle 4 booked a
    similar move as a *relabel* precisely because the calls reappeared elsewhere, and these do
    not. The census asserts it by file (`cleared_by_cycle12=2`) and re-asserts cycles 10's and
    11's three each (`cleared_by_cycle10=3 (still clear) cleared_by_cycle11=3 (still clear)`).
    **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** none. No hit moved from one file or group to another. The one place a relabel
    could have hidden — `equipment_pair_resolve`, which existed only to hand the parser row to
    `compute_equipment_effects` — did not: it was **deleted**, not repointed, and its
    replacement `equipment_converted_resolve_with_cell` names no `pcgen_import` symbol.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff.

- **Refused tokens:** `renderer=5, lst_parser_types=2, ingest_record_tokens=5,
  trait_and_pool_tokens=3, ir_converter=1, source_content_payload=3` — **19 hits / 8 files,
  summing, all under `src/rules_core/`.** Six groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **Cycle 11's next-cycle scope was wrong about how many consumer moves were left**
    (`correction 1789295230498-at-35-e6-003-ruled-d69b55`). It named four, in order:
    `damage_total`, `equipment_effects`, `equipment_resolver`'s KEY token, then
    `corpus_loader`'s rebuild. Only **two** of the four are consumer moves. `equipment_resolver`
    reads **no rules value at all**: its `equipment_key_token` is the same KEY-or-name rule
    already settled as `CorpusEquipmentRecord::identity` back in cycle 10, so there was no token
    left to move, and its import survives purely because `equipment_id_resolve`'s **signature**
    hands the parser row out for `corpus_loader`'s own envelope. `equipment_resolver` and
    `corpus_loader` are therefore **one** move, not two, and taking either alone would book a
    relabel — the type would simply be named in the other file instead. Re-derive:
    `python3 …_cycle12_runtime_import_census.py` (group `lst_parser_types`: 2 rows, both
    envelope signatures, zero token reads).
  - **`is_weapon_record` tested presence, not parseability, and the distinction is real.**
    The live predicate asked "does this record carry a `DAMAGE:` token", while
    `resolve_base_damage_dice` asked "does it parse as `<count>d<size>`". Settling only the
    parsed die would have silently reclassified every item stating damage in a shape
    `DiceExpression::parse` refuses. `states_base_damage` carries the presence half separately
    and the parity test compares both, so the two questions stay two questions.
  - **The `BASEITEM:` chase is the shape that does not move, and saying so is the point.**
    Every other read in this cycle was a token read with a settled answer. This one is a
    *corpus resolution* — it needs the whole loaded package, which the converter does not have
    — so the converter settles the **identity** and the live side keeps the one-hop chase.
    The same split already worked for `eqmod_references` in cycle 11.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=8 live_hits=19`, was `10 / 21` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 19 hits split `renderer=5, lst_parser_types=2, ingest_record_tokens=5, trait_and_pool_tokens=3, ir_converter=1, source_content_payload=3`; `gate_agreement=OK (19 == 19)` | the same 19 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle12_runtime_import_census.py` |
  | 2 files cleared, by name (`cleared_by_cycle12=2`); cycles 10's and 11's three each still clear; `settled_fields_added_by_cycle12=10`; `token_arrays_on_it=0`; `eqmod_referenced_records=deleted`; `equipment_pair_resolve=deleted` | the 21 hits at cycle start vs the 19 at HEAD | the census's own row and file assertions, same command |
  | 7,803 equipment records compared, 0 disagreements | every `data/corpus/<book>/equipment/**/*.json` the live loader reads | `cargo test --locked --lib every_live_corpus_equipment_record_carries_the_same_weapon_values_the_token_reads_produced` |
  | that test can fail — 522 of 7,803 disagree | the same population, with `+ 1` on the converter's critical-multiplier read | the same command, after the mutation — recorded red, then reverted and re-verified green |
  | 2 live `EquipmentRecord` imports remain (was 4), in 2 files, both `corpus_loader.rs` and `equipment_resolver.rs` | every live file naming the type; the single-line grep prints **1** because `corpus_loader.rs:39` spells the import across two lines, so the census's own row list is the authority | `python3 …_cycle12_runtime_import_census.py` (group `lst_parser_types`); single-line form: `grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/` |
  | 11 prior receipts, so this is cycle 12 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree. `apps/` was **not** touched, so the desktop crate and the frontend run at the epic
  wrap-up, not here.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3351 passed; 0 failed; 16 ignored
                                         (cycle 11's 3350 + this cycle's one new parity test)
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 Running targets + 1 Doc-tests;
                                         8,880 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines (cycle 11 recorded 8,879;
                                         the +1 is exactly this cycle's one new test)
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, after one self-heal
                                         (`expected outer doc comment` on this cycle's own
                                         module-doc note, caught at its first compile and
                                         fixed there)
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (118.9s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=8 live_hits=19 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
                                                     done_evidence_violations=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=141 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  desktop crate / frontend                        -> not run; `apps/` untouched this cycle
                                                     (workflow-instruction.md §6 step 3)
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and
  no `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** The `source_content_payload` trim — repointing `spell_resolver.rs`'s
  `SourceContentPayload` import at `rules_core::source_content`'s own re-export of the same
  enum, for `−1` and zero change in what the module depends on — is **refused for the sixth
  cycle running**, on the same reasoning cycles 7 through 11 gave. The `Equipment` payload
  still carries both the parser row and the converted record, and that is still written down as
  a transition shape in the variant's own doc comment; after this cycle **no live consumer
  reads the parser half** — only `corpus_loader` (which builds it) and `equipment_resolver`
  (whose signature hands it out) still name it.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 13**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 19-hit remainder. The unblocked piece is **one move, not a sequence**: collapse
  `SourceContentPayload::Equipment` to the converted half alone. `corpus_loader` stops
  rebuilding an `EquipmentRecord` out of the corpus JSON and builds the `CorpusEquipmentRecord`
  directly, `equipment_resolver`'s `equipment_id_resolve` and `equipment_key_token` go with the
  row they hand out, and the `#[cfg(test)]` loader tests that assert on the row assert on the
  settled record instead. That one move clears `lst_parser_types` (2), `ir_converter` (1),
  `ingest_record_tokens`' three `corpus_loader` rebuild calls and the `equipment_resolver` half
  of `source_content_payload` — up to 7 of the 19. The `renderer` group (5) stays a
  converter-parity cycle (cycle 2's `disagree=97,332 of 660,320`), `trait_and_pool_tokens` (3)
  stays refused on cycle 7's corrected number, and `derived_evaluator_fixture_check` (1) stays
  measured non-relocatable (cycle 3).
