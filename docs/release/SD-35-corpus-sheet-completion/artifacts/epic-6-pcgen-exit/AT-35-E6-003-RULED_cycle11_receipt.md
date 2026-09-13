# Cycle AT-35-E6-003-RULED cycle 11 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `2daf94f6b3` (the code, the census script and its JSON, the retro events),
  cycle start `47e9e4ceee`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `47e9e4ceee`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 10's closing figure — nothing drifted between the two cycles:
  ```
  live_files=13 live_hits=26 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/pcgen_import/ir_converter.rs` — **+~300, converter side.** Nine reads moved here
    verbatim from the three live modules below, as
    `arms_armor_stat_effect_of`, `armor_class_chain_bonus_of`, `tempbonus_combat_ac_of`,
    `eqmarmor_chain_value_of`, `skill_check_bonus_of`, `tempbonus_skill_of`,
    `swim_speed_racial_bonus_of`, `var_bonuses_of`, `var_reference_of`,
    `weapon_enhancement_of`, `spell_resistance_bonus_of` and `eqmod_references_of`, plus the
    two shared token accessors. `equipment_record_to_corpus` calls them; it is the same **one**
    call `convert_equipment_record` already made, doing more — **not a second call**, which is
    why the `ir_converter` hit count did not rise.
  - `src/rules_core/equipment_record.rs` — **seven new settled fields** on
    `CorpusEquipmentRecord`: `stat_effect`, `armor_class_chain_bonus`, `skill_check_bonus`,
    `var_bonuses`, `weapon_enhancement`, `spell_resistance_bonus`, `eqmod_references`. Still
    **no token array and no bonus-chain array** — the shape cycles 8 and 9 refused, and that
    refusal still stands. Carries this cycle's parity test.
  - `src/rules_core/equipment_effects/arms_armor.rs` — **`pcgen_import` import GONE** (both of
    them: `lst_parser::equipment::EquipmentRecord` and `equipment_bonus_reader`). The
    `ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:` token reads, the `BONUS:COMBAT|AC` chain scan with its
    circumstance-type exclusion, the `TEMPBONUS:` fallback and the `BONUS:EQMARMOR|<field>`
    family scan are all deleted from the live side. `compute_arms_armor_effect` reports
    `record.stat_effect`; `apply_eqmod_armor_class_bonus` sums each modifier's
    `armor_class_chain_bonus`.
  - `src/rules_core/equipment_effects/general.rs` — **`pcgen_import` import GONE.** The
    `BONUS:SKILL` and `BONUS:VAR` chain scans, the `TEMPBONUS:` single-skill fallback with its
    three wildcard exclusions, and the `MOVE:` swim-speed read are deleted from the live side.
    `compute_general_effect` reports `record.skill_check_bonus`; `compute_var_effect` reports
    `record.var_bonuses`.
  - `src/rules_core/equipment_effects/equipmods.rs` — **`pcgen_import` import GONE** (both).
    The `BONUS:WEAPON`/`WEAPONPROF=` roll-chain walk, the sibling-`VAR` magnitude substitution
    and the `SR:` token read are deleted from the live side. `compute_equipmods_effect` reports
    `record.weapon_enhancement`; `resolve_spell_resistance_bonus` reports
    `record.spell_resistance_bonus`.
  - `src/rules_core/equipment_effects.rs` — new **live** `eqmod_referenced_converted_records`,
    the converted-record sibling of `eqmod_referenced_records`. It reads
    `CorpusEquipmentRecord::eqmod_references` — **a list of item identities, not a token** — and
    resolves each through `equipment_converted_resolve`, with the same resolve-or-skip
    discipline the live side applied to the token's own segments. `resolve_category_effect` and
    `resolve_weapon_to_hit_bonus` take converted records. This file keeps its own hit; it is one
    of the four consumers that have not moved.
  - `src/rules_core/equipment_resolver.rs` — new **live** `equipment_pair_resolve`, exposing the
    private three-way resolution `equipment_id_resolve` and `equipment_converted_resolve`
    already shared. `compute_equipment_effects` resolves **once** instead of twice, so the
    parser row and the converted record it reads can never come from different records.
  - `…/AT-35-E6-003-RULED_cycle11_runtime_import_census.py` / `.json` — **new.** Imports cycle
    10's census whole (which imports cycle 9's, … back to cycle 1's) and rewrites two groups'
    reasons with this cycle's measurements.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from the shared checkout, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`) and
    `docs/retro/events/sd31-transcribe.jsonl` (one appended event from another session).
    Committed rather than filtered away, per the standing "clean tree = unfiltered
    `git status` empty" rule.

  **No `data/` file and no corpus record was changed** (`git status --porcelain data/` empty),
  so `data/sheet_rules/` and `docs/work-inventory.json` are byte-identical to the cycle's start
  tree. **`apps/` was NOT touched**, so the desktop crate and the frontend run at the epic
  wrap-up, not here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff -- src/ | grep '^+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns nothing. First run, no
  self-heal. (A naive run over the whole `BASE_BRANCH...HEAD` diff of the epic's file-touch set
  reports four hits; all four are **doc citations of pre-existing test file names** written by
  earlier cycles' receipts and comments, not identifiers in this cycle's added code.)

- **Wired-integration audit result:** OK_NO_TOKENS. First run, no self-heal:
  `grep -niE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over the same
  added lines returns nothing. No `"Would …"` string, no inline mock, no fixture-only data path:
  the converter reads the same real corpus records the live side read, and the parity test
  asserts the real values over the whole live corpus.

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
  first met in cycle 4, not regressed here). The criterion as a whole is **not** met: 21 hits
  across 10 files remain under `src/rules_core/`, and `render_pcgen_desc_with_values` is still
  called there.

- **Receipt rows (mechanical):**
  ```
  since=47e9e4ceeefbbcca88cb09622271f83111bb28fb target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=1535 ratio=n/a builds_recorded=3 pcgen_live_files=10
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.
  `pcgen_live_files=10` is the **file** count, down 3 from cycle 10's 13 — the second
  consecutive cycle to move it. The hit count moved with it: 26 → 21.
  `builds_recorded=3` counts **compile sessions in `$CARGO_TARGET_DIR`**, not verification
  passes: the authoring compile that proved the three moved modules type-check, the final
  `cargo test` build, and `cargo clippy`. There was **one** verification pass, after the last
  figure-moving change (`decisions.md` §3).

- **PCGen residue:** `live_files=10 live_hits=21 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 10's `13 / 26` on **both** counts, **and the instrument was
  not touched this cycle** (`git diff --name-only 47e9e4ceee..HEAD -- scripts/` is empty), so
  the `−3 files / −5 hits` is entirely code. Per-root:
  ```
  root src/rules_core         files=13 hits=26  ->  files=10 hits=21
  root src/saved_character    files=0  hits=0
  root src/campaign           files=0  hits=0
  root src/homebrew_authoring files=0  hits=0
  root apps/desktop           files=0  hits=0   (unchanged; no apps/ file was written)
  ```
  **Five hits and three whole files, and all three files are closures, not relabels.** The read
  does not move to another live file — it stops being a token read. Not gate-gaming: nothing
  renamed to duck a regex, no path exempted, no rebaseline, and the `use`-collapse still
  available in `source_content_payload` was **refused for the fifth cycle running**.

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical
  (`sheet_rule_convert -- --check` `verdict=PASS`). The parity that **was** required for this
  change is the moved reads' own, and it is as wide as cycle 10's:
  `every_live_corpus_equipment_record_carries_the_same_armour_skill_and_weapon_values_the_token_reads_produced`
  loads **every** book under `data/corpus/` — **7,803 equipment records** — re-derives all nine
  moved reads the way the live modules derived them, straight off the parser row still paired
  with the converted record in the canonical envelope, and compares **field for field**:
  the four `EquipmentStatEffect` fields, the referenced-modifier AC contribution, the skill
  bonus with its swim rule, the named-variable rows, the full `WeaponEnhancementBonus` struct
  including both scopes, the Spell Resistance grant, and the attachment identity list.
  **0 disagreements.** The test was **mutation-proved**: adding `+ 1` to the converter's Spell
  Resistance read turns it red on **16 of 7,803** records (`Mantle of Spell Resistance` and
  `Scarab of Protection` among them), and the mutation was reverted and re-verified green.

- **Movement, four buckets:**
  - **closure:** 5 live `pcgen_import` hits and 3 whole files, named by row —
    `equipment_effects/arms_armor.rs:21` (`equipment_bonus_reader`) and `:22`
    (`lst_parser::equipment::EquipmentRecord`), `equipment_effects/general.rs:19`
    (`EquipmentRecord`), `equipment_effects/equipmods.rs:86` (`equipment_bonus_reader`) and
    `:87` (`EquipmentRecord`). **All three files are closures:** each file's token reading is
    deleted from the live side, not moved to another live file — cycle 4 booked a similar move
    as a *relabel* precisely because the calls reappeared elsewhere, and these do not. The
    census asserts it by file (`cleared_by_cycle11=3`) and re-asserts cycle 10's three
    (`cleared_by_cycle10=3 (still clear)`). **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** none. No hit moved from one file or group to another. The one place a relabel
    could have hidden — the `EQMOD:` attachment grammar the three moved consumers reached
    through `eqmod_referenced_records` — did not: `eqmod_references` is a list of item
    identities, the grammar stayed on the converter, and `eqmod_referenced_converted_records`
    names no `pcgen_import` symbol.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff.

- **Refused tokens:** `renderer=5, lst_parser_types=4, ingest_record_tokens=5,
  trait_and_pool_tokens=3, ir_converter=1, source_content_payload=3` — **21 hits / 10 files,
  summing, all under `src/rules_core/`.** Six groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **Cycle 10's own next-cycle figure was wrong about what a consumer move clears**
    (`correction 1789290932971-at-35-e6-003-ruled-f1b63e`). Its receipt and census said the
    unblocked equipment piece was *"18 hits across 11 files"*. The three consumers this cycle
    took carried **5 hits across 3 files**, and clearing them moved the whole-record total from
    `26 / 13` to `21 / 10`. The `18 / 11` figure counted files that carry a hit from a
    **different** group — `equipment_resolver`'s `SourceContentPayload` import,
    `corpus_loader`'s `ingest_record` reads — so it is a count of files the equipment work
    *touches*, not of hits a consumer move *clears*. Re-derive:
    `python3 scripts/pcgen_residue_gate.py --check` at each SHA, cross-checked against the
    census's per-group rows.
  - **The three moved modules' rules did not change, and that is the point.** Every rule each
    function encodes — the circumstance exclusion on an AC chain, the base-token-beats-modifier
    -chain ordering, the three wildcard exclusions on a `TEMPBONUS` skill grant, the swim-speed
    `+8`, the per-roll summation across two chains, the `WEAPONPROF=` scope, the sibling-`VAR`
    magnitude substitution, the refusal to read a chooser placeholder as a number — moved to
    the converter **verbatim**, with its doc comment and its real-corpus witness. The parity
    test is what proves it, over 7,803 records rather than the ~30 the three modules' own
    fixtures cover.
  - **`eqmod_references` is the shape that makes the rest of the sequence cheap.** The three
    moved consumers all needed the attached-modifier records, and the attachment lived in a
    token. Settling it as a **list of identities** — not a token, not a parsed structure —
    let all three move at once and leaves `damage_total`'s and `resolve_eqm_weightdiv_effect`'s
    own uses of the same list a straight substitution.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=10 live_hits=21`, was `13 / 26` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 21 hits split `renderer=5, lst_parser_types=4, ingest_record_tokens=5, trait_and_pool_tokens=3, ir_converter=1, source_content_payload=3`; `gate_agreement=OK (21 == 21)` | the same 21 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle11_runtime_import_census.py` |
  | 3 files cleared, by name (`cleared_by_cycle11=3`); cycle 10's three still clear; `settled_fields_added_by_cycle11=7`; `token_arrays_on_it=0`; `eqmod_referenced_converted_records=present`; `equipment_pair_resolve=present` | the 26 hits at cycle start vs the 21 at HEAD | the census's own row and file assertions, same command |
  | 7,803 equipment records compared, 0 disagreements | every `data/corpus/<book>/equipment/**/*.json` the live loader reads | `cargo test --locked --lib every_live_corpus_equipment_record_carries_the_same_armour_skill_and_weapon_values_the_token_reads_produced` |
  | that test can fail — 16 of 7,803 disagree | the same population, with `+ 1` on the converter's Spell Resistance read | the same command, after the mutation — recorded red, then reverted and re-verified green |
  | 4 live `EquipmentRecord` imports remain (was 7), in 4 files | every live file naming the type; the single-line grep prints **3** because `corpus_loader.rs:39` spells the import across two lines, so the census's own row list is the authority | `python3 …_cycle11_runtime_import_census.py` (group `lst_parser_types`); single-line form: `grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/` |
  | 0 live `equipment_bonus_reader` imports remain (was 2) | every live file naming it; the one remaining hit the grep prints, `equipment_record.rs:330`, is inside a `#[cfg(test)]` module and is not live code (operator ruling B15, `decisions.md` §18) — the gate agrees, it is not among the 21 | `grep -rn 'pcgen_import::equipment_bonus_reader' --include=*.rs src/rules_core/` |
  | 10 prior receipts, so this is cycle 11 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at the
  final tree. `apps/` was **not** touched, so the desktop crate and the frontend run at the epic
  wrap-up, not here.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3350 passed; 0 failed; 16 ignored
                                         (cycle 10's 3349 + this cycle's one new parity test)
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 Running targets + 1 Doc-tests;
                                         8,879 passed; 0 failed; 69 ignored; zero
                                         `test result: FAILED` lines (cycle 10 recorded 8,878;
                                         the +1 is exactly this cycle's one new test)
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, after one self-heal
                                         (`unused_mut` on a closure in this cycle's own parity
                                         test, caught at its first compile and fixed there)
  cargo run --bin sheet_rule_convert -- --check
                                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS (115.2s)
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=10 live_hits=21 verdict=PASS
  python3 -m unittest scripts.tests.test_pcgen_residue_gate -> Ran 27 tests, OK
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
                                                     missing_clearing_mechanisms=0
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=140 violations=0
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
  enum, for `−1` and zero change in what the module depends on — is **refused for the fifth
  cycle running**, on the same reasoning cycles 7 through 10 gave. Separately, on the pair: the
  `Equipment` payload still carries both the parser row and the converted record, and that is
  still written down as a transition shape in the variant's own doc comment. It collapses to the
  converted half alone when `damage_total`, `equipment_resolver`, `equipment_effects` and
  `corpus_loader` have moved.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 12**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 21-hit remainder. The unblocked piece is the rest of the equipment sequence, in this
  order: `damage_total` (base damage dice, crit range and multiplier, wield category, plus the
  `EQMWEAPON|DAMAGESIZE` step that reuses `eqmod_references`), `equipment_effects`'s own weapon
  typing and `resolve_eqm_weightdiv_effect`, `equipment_resolver`'s key token, then
  `corpus_loader`'s rebuild last — at which point the payload collapses to the converted half
  alone and `source_content_payload` and `ir_converter` clear with it. The `renderer` group (5)
  stays a converter-parity cycle (cycle 2's `disagree=97,332 of 660,320`), and
  `trait_and_pool_tokens` (3) stays refused on cycle 7's corrected number.
