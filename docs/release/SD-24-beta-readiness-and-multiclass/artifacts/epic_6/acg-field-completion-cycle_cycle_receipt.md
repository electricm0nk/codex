# Cycle acg-field-completion-cycle — Epic 6 / Criteria 6.2, 6.3, 6.4, 6.5 (ACG)

- **Card ID:** `t_a3c9e1f4` (`codex-tranche-5`, `done`) — placeholder pending step 8/9 backfill if the real hermes card ID differs.
- **Commit SHA:** `f657005` (feat, includes the rebase-conflict resolution merging this cycle's ACG additions with the concurrently-landed `apg-field-completion-cycle`'s own edits to the shared `tests/sd24_equipment_coverage_audit.rs`)
- **Files touched:**
  - `src/rules_core/rules_tables/acg/equipment_tables.rs` (rewritten: full corpus coverage, `weight_lbs`/`description` fields, `Equipmods` category, aggregated `equipment_tables()`)
  - `src/rules_core/rules_tables/acg/equipment_data/{mod,general,arms_armor,magic_items,equipmods}.rs` (new — mirrors CRB's per-category module shape)
  - `src/rules_core/rules_tables/acg/spell_list.rs` (rewritten: full 144-record corpus coverage, full 9-school `Pf1SchoolId` enum, full spell text)
  - `src/rules_core/rules_tables/acg/mod.rs` (added `pub mod equipment_data;`)
  - `tests/sd24_acg_equipment_field_completion.rs` (new)
  - `tests/sd24_equipment_coverage_audit.rs` (updated: ACG-specific assertions corrected to reflect full coverage; APG assertions untouched, split into their own tests)
  - `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_6/acg-field-completion-cycle_cycle_receipt.md` (this file)
  - `docs/release/SD-24-beta-readiness-and-multiclass/progress.md` (updated in place)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (this cycle's own diff; raw `BASE_BRANCH...HEAD` diff also shows the pre-existing, already-documented `crb-field-completion-cycle` "Plant Growth"/"hack" line — inherited branch history, not introduced by this cycle)
- **Wired-integration audit result:** OK_NO_TOKENS (this cycle's own diff, staged); 0 new forbidden-token hits from this cycle's ACG content (independently grepped the generated equipment/spell text for `STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack` before shipping — 0 hits)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  - 6.2 "Equipment content completion: cost field" — cycle artifact: this receipt + the widened `EquipmentFieldCoverage`/`SpellFieldCoverage` reports.
  - 6.3 "Equipment content completion: weight field" — cycle artifact: `weight_lbs` populated per record.
  - 6.4 "Equipment content completion: full description" — cycle artifact: `description` populated per record.
  - 6.5 "Spell content completion: full text per SRD/PRD" — cycle artifact: full spell text populated per record.
  - Epic purpose (`decisions.md §5`, operator-pinned): "strict 100% field coverage" for cost/weight (where applicable) and full description, beta-tier corpus (CRB + APG + ACG + Bestiary 1). This cycle's granted file-touch set is ACG-only (`equipment_tables.rs`, `equipment_data/`, `spell_list.rs`); APG and Bestiary 1 remediation remain separate, disjoint-file-touch follow-on work.
- **Status:** 6.2 complete (ACG) · 6.3 complete (ACG, honest "where applicable" ceiling) · 6.4 complete (ACG, honest ceiling — see Notes on the `SPROP:`-sourcing decision) · 6.5 complete (ACG, clean 100%)
- **Notes:**
  - **6.2 (cost/record coverage):** ACG equipment went from the SD-22 bootstrap (3 records) to full corpus coverage: 221 real, active (non-`.MOD`) records from `acg_equip.lst` (`TYPE:`-disambiguated: 60 General + 20 Arms/Armor + 141 Magic Items) plus 48 real, `KEY:`-bearing modifier records from `acg_equipmods.lst` (excluding that file's own trailing "Old KEYs" `.COPY=`-only block) = **269 total**, added as a new `EquipmentCategory::Equipmods` variant mirroring CRB's own four-category scope. Criterion 6.1's original audit scoped ACG equipment to 221 (not counting `acg_equipmods.lst` at all); this cycle widens that scope — recorded as a `## DISCOVERED` correction, not silently substituted. `has_cost` is 231/269 (86%): every `None` is a genuine corpus absence (Equipmods rows priced via `PLUS:` — an enhancement-bonus slot, not a flat `COST:` — and a handful of General/Arms-Armor rows with no independent price), never fabricated.
  - **6.3 (weight):** added `weight_lbs: Option<f64>` to ACG's `EquipmentTableEntry`, populated from the real corpus `WT:` token. Coverage: General 30/60, Arms/Armor 19/20, Magic Items 86/141, Equipmods 0/48 (equipment *modifiers* have zero `WT:` tokens anywhere in `acg_equipmods.lst` — the same finding CRB's own Equipmods category already established) — **135/269 (50.2%)**. Every `None` is a genuine corpus absence.
  - **6.4 (description):** added `description: Option<&'static str>` to ACG's `EquipmentTableEntry`. **Sourcing decision (recorded here, not silently assumed):** `acg_equip.lst`/`acg_equipmods.lst` carry **zero** `DESC:` tokens anywhere (confirmed by direct grep of both files) — unlike CRB's equipment corpus. The closest real per-item prose ACG's LST corpus provides is the `SPROP:` ("Special Property") token, so `description` is sourced from `SPROP:` instead, joined with `"; "` when a record has more than one `SPROP:` entry. A trailing `|<conditional-tag>` qualifier (e.g. `|PRECLASS:1,Slayer=1`, `|PREABILITY:1,CATEGORY=Special Ability,Bloodline ~ Aberrant`) is stripped before storage — verified by inspecting every one of the 77 `|`-suffixed `SPROP:` occurrences in `acg_equip.lst`: all follow the `<prose>|<directive>` shape, never real item text after the pipe. Coverage: General 57/60, Arms/Armor 20/20, Magic Items 139/141, Equipmods 48/48 — **264/269 (98.1%)**, a materially higher ceiling than CRB's 61.2% because `SPROP:` is a near-universal convention in this corpus (unlike CRB's `DESC:`, which many rows never carry). This closes the CRB cycle's `## Open blockers` question for ACG specifically: the `SPROP:`-sourcing decision reaches near-100% without fabrication, so no open blocker is filed for ACG's `description` field.
  - **6.5 (full spell text):** ACG's spell list went from the SD-22 bootstrap (4 records) to full corpus coverage: 144 real, level-and-school-bearing spell records (134 `CLASSES:`-bearing + 9 `Naturalist Summon Nature's Ally I`-`IX` variants, whose level comes from the roman-numeral name suffix since they carry a `KEY:` token but no `CLASSES:` token + 1 `DOMAINS:`-only variant, `Summon Monster V (fire elementals only)`). Criterion 6.1's original "145" figure double-counted the file's own `SOURCELONG:` header line as a spell — the same measurement-error shape the CRB cycle already found and corrected for its own "675" figure; recorded as a `## DISCOVERED` audit correction. **No schema change needed and, unlike CRB, no `.MOD`-record cross-reference needed for full text**: `acg_spells.lst`'s base (non-`.MOD`) record already carries the *full* multi-sentence text (tagged `|PRERULE:1,DisplayFullSpell`), while the `.MOD` record carries the *short* summary (tagged `|!PRERULE:1,DisplayFullSpell`) — the reverse of CRB's convention. `full_text_verified` is **144/144 (100%)** — every present ACG spell carries the fullest corpus text, a clean honest 100% with no residual gap.
  - Data generation used a one-time Python parsing script (kept in scratchpad, not shipped — matching CRB's own `equipment_data/*.rs` generation-method precedent) that parses the real corpus and emits each `.rs` array; every existing bootstrap `key`/`category`/`name`/`cost_gp` value for the pre-cycle sample (`Marlinspike`, `Headsman's Blade`, `Ring of Eloquence`, `Blade Lash`, `Air Geyser`, `Beastspeak`, `Anti-Incorporeal Shell`) is preserved verbatim — confirmed by the pre-existing `tests/sd22_acg_equipment_resolves.rs`/`tests/sd22_acg_spell_list_resolves.rs` standing tests, both still GREEN, unmodified.
- **Discovery forwards:**
  - `epic-6-audit-correction` (ACG equipment record-coverage scope widened from 221 to 269 by adding `acg_equipmods.lst` as a 4th category, mirroring CRB's own four-category treatment) — suggested: fold into the same documentation-only follow-up already queued for Epic 6's other path/doc corrections; correct `equipment-coverage-matrix.md`'s "221" citation if referenced elsewhere.
  - `epic-6-audit-correction` (ACG spell record-coverage: criterion 6.1's "145" figure double-counted the file's own `SOURCELONG:` header line as a spell; the real count is 144) — suggested: same documentation-only follow-up.
  - `epic-6-sourcing-decision` (ACG's `description` field is sourced from `SPROP:` rather than `DESC:`, since ACG's LST corpus has no `DESC:` token on equipment rows at all; this reaches a 98.1% ceiling, closing the analogous CRB Open Blocker for ACG specifically) — suggested: APG's follow-on cycle should check whether `apg_equip_*.lst` has a `DESC:` token convention or needs the same `SPROP:`-sourcing decision before its own criterion 6.4 work.
- **Next-cycle plan:** APG equipment/spell record-coverage (338 of 341 equipment + 294 of 298 spell records remaining, per criterion 6.1's own findings) and Bestiary 1's from-scratch equipment module (7 records) remain unstarted, disjoint-file-touch follow-on work. Epic 6 criteria 6.2-6.5 are now closed for CRB and ACG; APG and Bestiary 1 are the remaining scope before Epic 6 can be marked fully closed (plus the operator decision on CRB's `description` Open Blocker, unaffected by this cycle).

## RED → GREEN evidence

**RED (compile-time, genuine):** before this cycle, `acg::equipment_tables` had no `equipment_data` submodule, `EquipmentTableEntry` had no `weight_lbs`/`description` fields, and `EquipmentCategory` had no `Equipmods` variant, and `acg::spell_list::Pf1SchoolId` had only 4 of 9 schools. `tests/sd24_acg_equipment_field_completion.rs` (written against the intended post-cycle API) failed to compile against the pre-cycle code with 5 real errors, reproduced by stashing this cycle's production changes and recompiling just the new test:
```
error[E0425]: cannot find function `equipment_tables` in module `equipment_tables`
error[E0599]: no variant, associated function, or constant named `Equipmods` found for enum `EquipmentCategory`
error[E0609]: no field `weight_lbs` on type `&EquipmentTableEntry`
error[E0609]: no field `description` on type `&EquipmentTableEntry`
error[E0599]: no variant, associated function, or constant named `Conjuration` found for enum `Pf1SchoolId`
```
Separately, `tests/sd24_equipment_coverage_audit.rs`'s pre-existing ACG-bootstrap assertions (`total_records == 3`, `has_weight == 0`, `full_text_verified == 0`) failed against the post-cycle production code once it was implemented — a genuine, expected regression of the old "still bootstrap-only" assumption, not a defect (see GREEN below for the corrected assertions).

**GREEN:**
```
cargo test --locked --test sd24_acg_equipment_field_completion
running 9 tests
test acg_equipment_record_coverage_is_now_full ... ok
test acg_equipment_weight_and_description_are_populated_to_the_corpus_honest_ceiling ... ok
test acg_spells_carry_full_untruncated_corpus_text ... ok
test blade_lash_spell_carries_full_corpus_text_not_the_old_bootstrap_summary ... ok
test amorphous_armor_special_ability_resolves_from_the_new_equipmods_category ... ok
test fake_rapier_corpus_quirk_is_faithfully_ingested_not_fabricated_or_dropped ... ok
test marlinspike_has_real_weight_and_sprop_sourced_description ... ok
test ring_of_ancestral_blood_magic_resolves_as_a_newly_ingested_magic_item ... ok
test naturalist_summon_natures_ally_variant_resolves_with_level_from_its_name_suffix ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
`tests/sd24_equipment_coverage_audit.rs` (updated with corrected ACG assertions, APG assertions untouched) also re-ran GREEN: 9 passed, 0 failed. The pre-existing SD-22 bootstrap standing tests (`tests/sd22_acg_equipment_resolves.rs`, `tests/sd22_acg_spell_list_resolves.rs`) remain GREEN unmodified: 6 passed/1 ignored and 7 passed/1 ignored respectively — proving this cycle's full-corpus ingest preserved every pre-existing bootstrap sample value exactly.

**Full-suite verification:** `cargo check --lib` (whole crate) — clean, 0 errors. A full `cargo test --locked --tests` run could not be completed in this cycle's environment: the shared disk (`/`, 96G total) was repeatedly driven to 0 bytes free by concurrent sibling loop-cycles building in their own worktrees at the same time (observed directly: `wf_0204a5d8-425-15`'s own `cargo test --locked --tests` running concurrently, plus this repo's own `target/` and the other worktrees' `target/` directories together consuming effectively all 96G) — `ld` crashed with `Bus error` (`signal 7`) from an out-of-space `mmap` failure, twice, unrelated to this cycle's code. `cargo clean` in this worktree (freeing ~5G each time) restored enough headroom for the scoped test runs above to complete cleanly; a full from-scratch link of all ~565 test binaries needs more concurrent disk than was available while a sibling cycle was mid-build. This is recorded here as an environment/infra constraint, not a defect in this cycle's diff — `cargo check --lib` (whole-crate compile) and the four scoped test files covering every file this cycle touched are the verification evidence for this receipt.

## Dual-audit gate (final)

`BASE_BRANCH=09e43c3` (SD-23 closure commit, `merge-base HEAD origin/develop`).

- **Identifier audit (raw `BASE_BRANCH...HEAD` diff):** `OK_NO_BUNDLE_TAGS`.
- **Wired-integration audit (raw `BASE_BRANCH...HEAD` diff):** 1 literal hit — the pre-existing, already-documented `crb-field-completion-cycle`/`Plant Growth`/"hack" finding (inherited branch history from an earlier commit on `tranche/5-2`, not introduced by this cycle; see that cycle's own receipt for the self-heal record). This cycle's own diff (`git diff --staged`, this cycle's changes only) is independently clean: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`, no self-heal needed.
