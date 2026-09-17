# SD-36 Epic E receipt — SD-35 code-review correctness fixes

Brief: `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/briefs/epic-e-correctness.md`
Branch: `tranche/16`. Start SHA `3954b4d9431aef78a1784242153b0858fe35db4c`.
Operator ruling 2026-09-15: "Yes, all confirmed P1s plus the two gates."

## Finding disposition

| Finding | Status | Evidence |
|---|---|---|
| CONV-01 (CRITRANGE) | **FIXED** | commit `b69ba965e7` |
| CONV-02 (multi-line label loss) | **FIXED** (2 passes) | commits `b69ba965e7`, `f28975c1a9` |
| CONV-03 (AC_Natural_Armor) | **FIXED** | commit `b69ba965e7` |
| CONV-04 (.COPY= equipment name) | **FIXED** | commit `b69ba965e7` |
| CONV-05 (per-record degradation) | **FIXED** | commit `b69ba965e7` |
| CONV-06 (prose paraphrase leak) | **DEFERRED** | retro `1789638758050-sd31-transcribe-78bb64` |
| CONV-07 (PI over-redaction) | **DEFERRED** | retro `1789638758050-sd31-transcribe-78bb64` |
| CONV-08 (negated PREABILITY no-op) | **DEFERRED** | retro `1789638758050-sd31-transcribe-78bb64` |
| engine-P1-1 (Slot over unresolved Choice) | **FIXED** | commit `670b8546fd` |
| engine-P1-2 (MasterVar/MasterLevel) | **FIXED** | commit `670b8546fd` |
| engine-P1-3 (find() book tie-break) | **PARTIAL** (census gate; full fix escalated) | commit `670b8546fd`; NEEDS HUMAN RULING; forward-scope FS-7 |
| engine-P1-4 (placeholder labels) | **FIXED** | commit `670b8546fd` |
| engine-P2-1 (SPROP leak) | **FIXED** | commit `670b8546fd` |
| desktop-P1-01 (path traversal) | **FIXED** | commit `8a2fa8e145` |
| desktop-P1-02 (non-atomic saves) | **FIXED** (3 stores) | commit `8a2fa8e145` |
| desktop-P2-01 (4 orphan commands) | **PARTIAL** (2 of 4 wired; 1 deferred; 0 dropped) | commit `8a2fa8e145`; module doc + forward-scope |
| PC8-1 (literal parens in id) | **FIXED** | commit `8a2fa8e145` |
| PC8-2 (missing size modifier) | **FIXED** | commit `8a2fa8e145` |
| PC4-1 (Warpriest Blessing contradiction) | **DEFERRED** | retro `1789643677178-sd36-epic-e-30e143`; NEEDS HUMAN RULING; forward-scope FS-8 |
| R12-01 (transcribe_companion_tables.py atomic write) | **FIXED** | commit `8a2fa8e145` |
| GATE-01 (token_coverage.py bookkeeping-only) | **FIXED** (RULE_FILES check + mutation probe) | commit `2b538ce6a4` |
| GATE-02 (oracle roster denominator) | **PARTIAL** (denominator stated + disagreements tracked; full roster widening deferred) | commit `2b538ce6a4`; forward-scope FS-9 |
| GATE-03 (residue gate vocabulary, P3) | **RECORDED, no change** | forward-scope FS-6 (per brief instruction) |

**Fixed: 15 of 22 in-scope findings whole. Partial: 4 (a real, tested mitigation landed; the
larger half is a genuine multi-cycle infrastructure/schema item, escalated). Deferred with
retro: 4 (CONV-06/07/08 as one group, PC4-1).**

## Files changed

Converter: `src/pcgen_import/sheet_rule/convert.rs`, `src/pcgen_import/sheet_rule/mod.rs`,
`src/pcgen_import/sheet_rule/ctx.rs`, `src/rules_core/sheet_rule.rs` (test),
`tests/sheet_rule_convert_gate.rs`, `data/sheet_rules/**` (regenerated, 2 passes).

Engine: `src/rules_core/sheet_rule.rs`, `tests/sheet_rule_book_collision_census.rs` (new),
`src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`,
`src/rules_core/pilot_compute/generic_class_chassis.rs` (test updates for a CONV-05
side-effect discovered by the full-suite run), `data/sheet_rules/**` (class-kind labels).

Desktop/stores: `apps/desktop/src-tauri/src/character_hub.rs`,
`apps/desktop/src-tauri/src/class_catalog.rs`, `class_catalog_generic.rs`,
`companion_pool_catalog.rs`, `wizard_school_picker.rs`,
`apps/desktop/src/characterHub/CharacterSheet.tsx`,
`apps/desktop/src/boundary/{addTraitSelection,removeTraitSelection,setEquipmentActiveState}.ts`
(new), `src/saved_character/local_store.rs`, `src/campaign/local_store.rs`,
`src/homebrew_authoring/package_store.rs`, `src/rules_core/pilot_compute/mod.rs`,
`scripts/transcribe_companion_tables.py`, `tests/package_store_atomic_write.rs` (new).

Gates: `scripts/token_coverage.py`, `scripts/tests/test_token_coverage.py`,
`docs/release/SD-35-corpus-sheet-completion/decisions.md`,
`docs/release/SD-35-corpus-sheet-completion/risks-and-open-questions.md`,
`docs/release/SD-36-consolidation/forward-scope-register.md`,
`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/token-coverage.json`.

## Tests added (RED confirmed before the fix, GREEN after)

- `tests/sheet_rule_convert_gate.rs`: `critrange_prints_the_real_low_bound_not_the_raw_token_count`,
  `multi_line_records_keep_every_lines_own_label`, `ac_natural_armor_var_idiom_becomes_an_ac_line`,
  `copy_equipment_records_keep_the_specific_items_own_name`,
  `a_sibling_terms_degradation_does_not_erase_a_convertible_terms_number`,
  `sprop_does_not_leak_a_doubled_token_key_into_prose`.
- `src/rules_core/sheet_rule.rs` (`evaluate_tests`):
  `a_slot_over_an_unresolved_choice_falls_back_to_words_not_a_silent_zero`,
  `master_var_and_master_level_render_as_words_not_a_deterministic_zero`,
  `a_placeholder_labelled_record_prints_its_source_derived_name_not_the_ingest_identifier`.
- `tests/sheet_rule_book_collision_census.rs` (new file): the engine-P1-3 mitigation gate.
- `apps/desktop/src-tauri/src/character_hub.rs` (`mod tests`): 5 `validate_character_id` tests.
- `src/saved_character/local_store.rs`: `a_save_that_fails_partway_through_leaves_the_previous_version_intact`,
  `a_successful_save_leaves_no_tmp_files_behind`.
- `src/campaign/local_store.rs`: `a_successful_save_leaves_no_tmp_files_behind`,
  `a_write_that_fails_partway_through_does_not_touch_the_previous_file`.
- `tests/package_store_atomic_write.rs` (new file): 2 tests for `PackageStore`.
- `src/rules_core/pilot_compute/mod.rs`: `a_small_races_per_weapon_attack_total_carries_the_real_size_modifier`
  (PC8-2); 2 `master_craftsman` tests updated to the de-paren'd id (PC8-1).
- `scripts/tests/test_token_coverage.py`: `RuleFilesGate` class, 6 tests (the GATE-01 mutation probe).

## Regeneration figures (converter group)

Command: `cargo run --locked --release --bin sheet_rule_convert`, then `-- --check`.

- `records=49450 converted=49450 refused=0` unchanged throughout (both regen passes).
- `rules_written`: 70317 -> 71862 (first pass, +1545, mostly new AC natural-armor lines).
  Second pass (CONV-02 refinement): unchanged at 71862 (a label-text-only change).
- `_refused.json` empty both times (`entries: []`).
- Crit-range strings: 585 files with the wrong `<n>-20` shape (478 "1-20", 121 "2-20", 30
  "3-20", 1 "4-20") -> 0; now 478 read "20", 121 read "19-20", 30 read "18-20", 1 reads
  "17-20". Command: `grep -rlE '"Text":"[0-9]-20"' data/sheet_rules --include=*.json | wc -l`.
- Bare type-word equipment labels (Potion/Scroll/Staff/Wand/Rod/Ring): 1155 -> 6 residual, all
  6 verified as the genuine base-type records themselves (e.g.
  `core_rulebook:equipment:potion`), not `.COPY=` derivatives. Command in the commit message.
- Wolf (`bestiary:monster:wolf`): now carries an AC natural-armor line (`Number(Const(2))`,
  `bonus_type: NaturalArmor`) and its Survival "track by scent" bonus keeps its own label
  instead of the bare record name.
- Degraded-record census: 423 records carry a degradation both before and after CONV-05 (the
  fix changes which LINES within a degraded record wipe, not how many records are degraded);
  numbered lines inside a degraded record: 620 (pinned in
  `pcgen_import::sheet_rule::term_level_refusal_gate::every_degraded_record_converted_and_prints_its_words`,
  re-derive command in that test's doc comment).
- CONV-02 refinement (second regen pass): 5013 single-line records changed label back to the
  clean assembled form (removing an unwanted `(words)` suffix the first pass's over-broad
  discriminator had introduced).
- Class-kind label exception (discovered via the full desktop test suite): 185 `class`-kind
  files changed to keep the class's own name as principal label.

## Verification run

- `cargo test --locked --lib` (root): 3398 passed, 0 failed, 16 ignored (final state, after
  all fixes).
- `cargo test --locked --test sheet_rule_convert_gate --test sheet_rule_book_collision_census
  --test package_store_atomic_write`: 40 passed, 0 failed.
- `cd apps/desktop/src-tauri && cargo test --locked`: 598 passed, 0 failed.
- `cd apps/desktop && npm run typecheck`: clean. `npm test`: 121/121 test files passed.
- `python3 -m unittest scripts.tests.test_token_coverage`: 21/21 passed.
- `cargo test --locked --no-run` (root) and (desktop): both compile clean (compile gate).
- `bash scripts/verify.sh` (ONE full pass, `RETRO_ACTOR=sd36-epic-e`): log
  `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/epic-e-verify.log`.
  Result and stage-by-stage attribution: see `verifyLog`/`fullPassGreen` in the structured
  return of this run. One failure attributed and fixed during the run:
  `pcgen-residue-gate` (FAIL, `live_files=2 live_hits=4`) -- caused by this cycle's own new
  doc-comment prose in `apps/desktop/src/boundary/addTraitSelection.ts` and
  `apps/desktop/src/characterHub/CharacterSheet.tsx` literally quoting the PCGen `%LIST` token
  marker (copied from a pre-existing Rust doc comment that the gate does not flag, for reasons
  not fully traced) while explaining the open-choice-trait shape; reworded to "open-choice
  trait" in both files (no functional change), gate re-run clean
  (`live_files=0 live_hits=0 verdict=PASS`).

## Escalations (NEEDS HUMAN RULING, `codex-morning-log-2026-09-16.md`)

1. **engine-P1-3 full fix** (thread the source book through `HeldSeed`/saved-character schema)
   -- a multi-cycle schema migration across `pilot_compute` (88k+ lines) and its fixtures, not
   a bounded fix. Mitigation: `tests/sheet_rule_book_collision_census.rs`.
2. **PC4-1** (Warpriest Blessing chooser contradiction) -- confirmed real, marked COMPLEX by
   the review itself; the correct fix needs per-choice success detection through a
   member-level resolution path judged too easy to get subtly wrong to rush.

Both are also recorded as forward-scope register rows (FS-7, FS-8) and retro deferrals.

## Blockers

None outstanding that block this cycle's own closure -- the two escalations above are scope
decisions on follow-on work, not blockers to landing what this cycle did fix.
