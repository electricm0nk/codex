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
| CONV-06 (prose paraphrase leak) | **DEFERRED** | retro `1789638758050-sd36-epic-e-78bb64` |
| CONV-07 (PI over-redaction) | **DEFERRED** | retro `1789638758050-sd36-epic-e-78bb64` |
| CONV-08 (negated PREABILITY no-op) | **DEFERRED** | retro `1789638758050-sd36-epic-e-78bb64` |
| engine-P1-1 (Slot over unresolved Choice) | **FIXED** | commit `670b8546fd` |
| engine-P1-2 (MasterVar/MasterLevel) | **FIXED** | commit `670b8546fd` |
| engine-P1-3 (find() book tie-break) | **PARTIAL** (census gate; full fix escalated) | commit `670b8546fd`; NEEDS HUMAN RULING; forward-scope FS-7 |
| engine-P1-4 (placeholder labels) | **FIXED** (4 call sites; see fix-cycle addendum) | commits `670b8546fd`, fix-cycle commits below |
| engine-P2-1 (SPROP leak) | **FIXED** | commit `670b8546fd` |
| desktop-P1-01 (path traversal) | **FIXED** (2 bypasses; see fix-cycle addendum) | commit `8a2fa8e145`, fix-cycle commits below |
| desktop-P1-02 (non-atomic saves) | **FIXED** (3 stores) | commit `8a2fa8e145` |
| desktop-P2-01 (4 orphan commands) | **FIXED** (3 of 4 wired; 1 dropped, this fix cycle) | commit `8a2fa8e145`, fix-cycle commits below |
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

**Fix-cycle update (independent verifier, this cycle): 16 FIXED whole, 3 PARTIAL,
4 DEFERRED with retro. `desktop-P2-01` moved PARTIAL -> FIXED (the 4th orphan command,
`list_wizard_school_options`, is now DROPPED per the brief's own fallback instruction rather
than left registered with no caller). `engine-P1-4` and `desktop-P1-01` stay FIXED but now
cover additional call sites/bypasses the independent verifier found reaching past the first
pass's fix — see the addendum below for the full disposition of every fix-cycle finding.**

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

**Fix cycle, additionally:** `src/rules_core/sheet_rule.rs` (`display_label` `pub`, doc
comment), `src/rules_core/level_up_option_filter.rs` (`filter_option_pool`, `label_of`, 2 new
tests), `src/rules_core/pilot_compute/class_chassis_sheet_rules.rs` (`display_name`, 1 new
test), `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs` (new
`granted_feat_display`, 1 new test -- NOT in the original brief's desktop write scope; see
fix-cycle finding 9), `apps/desktop/src-tauri/src/character_hub.rs`
(`validate_character_id` `pub(crate)`), `apps/desktop/src-tauri/src/characterHub/
recomputeCharacter.rs` (duplicate resolver deleted, 2 new tests),
`apps/desktop/src-tauri/src/main.rs` (`wizard_school_picker` mod/use/registration removed),
`apps/desktop/src-tauri/src/wizard_school_picker.rs` (deleted, 5 tests removed),
`apps/desktop/src-tauri/src/class_catalog_generic.rs` (1 test assertion corrected, 81 -> 78),
`src/saved_character/local_store.rs`, `src/campaign/local_store.rs`,
`src/homebrew_authoring/package_store.rs` (doc comments only, no behavior change),
`docs/retro/events/root.jsonl` (folded), `docs/retro/events/sd31-transcribe.jsonl` (misfiled
entry removed), `docs/retro/events/sd36-epic-e.jsonl` (corrected entry appended),
`scripts/verify-baselines.env` (updated once, after the fix cycle's own full verify pass --
see that section), this receipt.

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

**Fix cycle, additionally** (finding 2/3/4 in the fix-cycle addendum; each RED-confirmed --
directly for the new tests, or by temporarily reverting the fix for the ones written and
fixed in the same edit -- before GREEN):
- `src/rules_core/level_up_option_filter.rs`:
  `a_placeholder_label_is_resolved_to_the_source_derived_name_not_printed_raw`,
  `label_of_resolves_a_placeholder_label_to_the_source_derived_name`.
- `src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`:
  `a_placeholder_principal_label_resolves_to_the_source_derived_name`.
- `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs`:
  `granted_feat_display_resolves_a_placeholder_label_to_the_source_derived_name`.
- `apps/desktop/src-tauri/src/characterHub/recomputeCharacter.rs`:
  `this_module_defines_no_local_unvalidated_character_root_resolver` (a permanent
  source-grep regression control, not just a one-time fix verification),
  `the_shared_validator_this_command_now_delegates_to_rejects_traversal`.

## Regeneration figures (converter group)

Command: `cargo run --locked --release --bin sheet_rule_convert`, then `-- --check`.

- `records=49450 converted=49450 refused=0` unchanged throughout (both regen passes).
- `rules_written`: 70317 -> 71862 (first pass, +1545, mostly new AC natural-armor lines).
  Second pass (CONV-02 refinement): unchanged at 71862 (a label-text-only change).
- `_refused.json` empty both times (`entries: []`).
- Crit-range strings: 585 files with the wrong `<n>-20` shape at the start SHA
  (`3954b4d9`; command: `git archive 3954b4d9431aef78a1784242153b0858fe35db4c -- data/sheet_rules
  | tar -x -C <dir> && grep -rlE '"Text":"[0-9]-20"' <dir>/data/sheet_rules --include=*.json |
  wc -l`) -> 0. Fix-cycle correction: the receipt originally reported 478/121 for the "1-20"/
  "2-20" per-pattern breakdown; independently re-derived at the start SHA
  (`grep -rlF '"Text":"1-20"' … | wc -l` / same for `"2-20"`) as **437** and **119**, not
  478/121 -- the per-pattern counts (437+119+30+1=587) sum to 2 more than the combined,
  deduplicated 585 because 2 files carry more than one of these strings on different lines
  and are counted once in the combined regex but twice across the per-pattern greps; the 30
  ("3-20") and 1 ("4-20") figures do reproduce unchanged. Post-fix: `grep -rlF
  '"Text":"19-20"' data/sheet_rules --include=*.json | wc -l` = 119 (not 121), confirmed
  against the live corpus; the "3-20"->"18-20" and "4-20"->"17-20" mappings (30 and 1) also
  reproduce. The exact post-fix count of files now reading the bare "20" (replacing "1-20")
  is not independently re-stated here, since a plain `"Text":"20"` grep is not selective to
  this fix (1045 files match today) -- 437 is the pre-fix "1-20" count, not re-verified as a
  post-fix partition.
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

- `cargo test --locked --lib` (root): 3398 passed, 0 failed, 16 ignored (state after this
  section's two `verify.sh` passes below; the fix-cycle addendum states the further-updated
  count after this cycle's own additional fixes).
- `cargo test --locked --test sheet_rule_convert_gate --test sheet_rule_book_collision_census
  --test package_store_atomic_write`: 40 passed, 0 failed.
- `cd apps/desktop/src-tauri && cargo test --locked`: 598 passed, 0 failed.
- `cd apps/desktop && npm run typecheck`: clean. `npm test`: 121/121 test files passed.
- `python3 -m unittest scripts.tests.test_token_coverage`: 21/21 passed.
- `cargo test --locked --no-run` (root) and (desktop): both compile clean (compile gate).
- `bash scripts/verify.sh` (`RETRO_ACTOR=sd36-epic-e`) ran TWICE, per AGENTS.md's Delivery
  Format and the brief's own Deliverable requirement for a verify stage table. **Corrected by
  the fix cycle** (review finding 8): the paragraph this replaced pointed at the wrong log and
  named only one of three failures.

  **First pass** (HEAD `2b538ce6a4`), log
  `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/epic-e-verify.log`:
  `RESULT: FAIL` -- `FAILED: 3 pcgen-residue-gate root-full clippy` (verbatim from the log's
  own summary line), not "one failure" as originally written here. All three are attributed
  to this cycle's own new code, per commit `96e883ec0e`:
  - `pcgen-residue-gate` (`live_files=2 live_hits=4`): new doc-comment prose in
    `apps/desktop/src/boundary/addTraitSelection.ts` and
    `apps/desktop/src/characterHub/CharacterSheet.tsx` literally quoted the PCGen `%LIST`
    token marker while explaining the open-choice-trait shape; reworded to "open-choice
    trait" in both files (no functional change).
  - `root-full` (`sd24_wired_integration_audit`): a test's `assert!` message used the bare
    word "placeholder" describing a real data shape; correctly flagged since it was not
    comment prose. Reworded.
  - `clippy`: 2 new lint warnings in this cycle's own code, fixed.

  Each was re-verified individually green
  (`bash scripts/verify.sh --only pcgen-residue-gate --only root-full --only clippy` ->
  PASS) before the second pass.

  **Second pass** (HEAD `96e883ec0e` -> `121ce526c8`), log
  `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/epic-e-verify2.log`:
  `RESULT: PASS`, all 46/46 stages. **This is the pass `fullPassGreen` in the structured
  return reports.** Full stage table (`grep -n 'PASS\|FAIL' epic-e-verify2.log`):

  | Stage | Result | Detail |
  |---|---|---|
  | preflight-disk … corpus-trap-audit-selftest (32 gate/selftest stages) | PASS | see log for each |
  | root-lib | PASS | 3398 passed |
  | root-full | PASS | 7872 passed across 414 suites, all 362 `tests/*.rs` suites executed |
  | desktop | PASS | 598 passed |
  | corpus-sweep | PASS | 48706/51523 records, 0 findings |
  | sheet-rules-check | PASS | records=49450 converted=49450 refused=0 rules=71862 |
  | corpus-trap-audit | PASS | 27681 examined, traps=407 at registered counts |
  | supersession-gate | PASS | 116 objects, all clean |
  | frontend-install / frontend-test / frontend-typecheck | PASS | 121/121 files, tsc clean |
  | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
  | class-dump | PASS | 31/31 computing |

  Both logs' `BASELINE NOTES` sections flagged `BASELINE_ROOT_LIB_TESTS`,
  `BASELINE_ROOT_FULL_TESTS`, `BASELINE_ROOT_TEST_BINARIES` and `BASELINE_DESKTOP_TESTS` as
  stale (3390/7855/412/592 recorded vs 3398/7872/414/598 measured) -- disposition in the
  fix-cycle addendum (review finding 13).

## Escalations (NEEDS HUMAN RULING, `codex-morning-log-2026-09-16.md`)

1. **engine-P1-3 full fix** (thread the source book through `HeldSeed`/saved-character schema)
   -- a multi-cycle schema migration across `pilot_compute` (88k+ lines) and its fixtures, not
   a bounded fix. Mitigation: `tests/sheet_rule_book_collision_census.rs`.
2. **PC4-1** (Warpriest Blessing chooser contradiction) -- confirmed real, marked COMPLEX by
   the review itself; the correct fix needs per-choice success detection through a
   member-level resolution path judged too easy to get subtly wrong to rush.

Both are also recorded as forward-scope register rows (FS-7, FS-8) and retro deferrals.

## Fix cycle (independent verifier findings, 2026-09-17)

An independent verifier reviewed the epic against the brief and this receipt and returned 14
findings. Disposition, most-severe first as reported:

1. **Working tree not clean** (`docs/retro/events/root.jsonl`, a `reclaim.sh` cron append
   sitting dirty across two of this epic's own commits) -- **FIXED**. Folded, commit
   `b793987d9e`, same convention as `a57ae67c`'s prior fold of the identical shared shard.
2. **engine-P1-4 second path** (`level_up_option_filter.rs`'s `filter_option_pool` built
   `EligibleOption`/`RefusedOption` labels from raw `rule.label.clone()`, bypassing
   `display_label`; `is_offerable` only filtered an *empty* label, not the placeholder) --
   **FIXED**. `display_label` promoted from private to `pub fn` in `sheet_rule.rs`;
   `filter_option_pool`'s three construction sites and `label_of` now call it. RED tests
   `a_placeholder_label_is_resolved_to_the_source_derived_name_not_printed_raw` and
   `label_of_resolves_a_placeholder_label_to_the_source_derived_name` confirmed failing
   (printed the raw `"Codex-Named Unit (feat_core_rulebook_x_lst_1)"`) before the fix, GREEN
   after.
3. **engine-P1-4 third and fourth paths** (`class_chassis_sheet_rules.rs`'s
   `ClassChassis::display_name: principal.label.clone()`; desktop
   `class_feature_feat_bridge.rs`'s `granted_feat: Some(feat.label.clone())`) -- **FIXED**.
   Both now call `display_label`/a thin `granted_feat_display` wrapper around it. RED tests
   `a_placeholder_principal_label_resolves_to_the_source_derived_name` (root) and
   `granted_feat_display_resolves_a_placeholder_label_to_the_source_derived_name` (desktop)
   confirmed failing (via a temporary revert of the fix, since both were written and fixed in
   the same edit) before GREEN. `label_of`'s own `sole_granted_feat`/`class_id`/module
   doc comments updated to stop claiming the raw `.label` is what's served.
   `class_chassis_sheet_rules.rs` was already outside the brief's literal write scope (see
   item 9); this is a direct correctness fix in the same already-widened file, not a new
   widening.
4. **desktop-P1-01 second bypass** (`characterHub/recomputeCharacter.rs` defined its OWN
   private `resolve_character_root` with no call to `character_hub::validate_character_id`;
   `recompute_character` used it) -- **FIXED**. Deleted the duplicate
   (`characters_root_from_app_data_dir`, `CHARACTERS_ROOT_DIR_NAME`, the local
   `resolve_character_root`, the now-unused `tauri::Manager` import); the command delegates to
   `crate::character_hub::resolve_character_root`, made reachable by making
   `character_hub::validate_character_id` `pub(crate)`. Added a permanent regression control
   (not just a fix): `this_module_defines_no_local_unvalidated_character_root_resolver`
   source-greps this file for a locally-assembled `"fn " + "resolve_character_root"` marker
   (assembled at runtime so the check does not match its own definition) and fails hard if a
   duplicate resolver is ever retyped -- the same technique `pcgen_residue_gate.py` and the
   wired-integration audit already use elsewhere (AGENTS.md rule 8: a warning is not a
   control). RED confirmed by reverting the fix and re-running.
5. **desktop-P2-01 orphan command left with no caller** (`list_wizard_school_options`
   registered in `main.rs`, zero frontend callers, only a doc-comment deferral) -- **FIXED**
   by DROPPING per the brief's own explicit fallback ("DROP the command and its tests"). No
   other Rust file referenced `wizard_school_picker`'s exports
   (`grep -rln "wizard_school_picker\|WIZARD_SCHOOL_\|WizardSchoolOptionDto\|
   list_wizard_school_options" apps/desktop/src-tauri/src/` -> only `main.rs`), and
   `apps/desktop/src/` had zero references at all -- clean removal. Deleted
   `wizard_school_picker.rs` (5 tests removed) and its `mod`/`use`/registration lines in
   `main.rs`. `add_trait_selection`, `remove_trait_selection` and `set_equipment_active_state`
   were independently confirmed still wired with real frontend callers
   (`CharacterSheet.tsx` -> `addTraitSelection`/`removeTraitSelection`/
   `setEquipmentActiveState` in `apps/desktop/src/boundary/`) -- 3 of 4 orphan commands wired,
   1 dropped, 0 left orphaned.
6. **Crit-range receipt figure did not reproduce** (478/121 for "1-20"/"2-20") --
   **CORRECTED** to 437/119, independently re-derived at the start SHA; see the corrected
   Regeneration figures section above for the full arithmetic (the per-pattern sum of 587
   exceeds the deduplicated combined total of 585 by 2, because 2 files carry more than one
   pattern).
7. **engine-P1-4 residual count missing from the receipt** -- **ADDED**. The data-layer
   placeholder count is unaffected by this fix (the fix is presentation-layer:
   `display_label` resolves placeholders at render/DTO time, it does not rewrite
   `data/sheet_rules/**`, which the brief reserves for the converter alone):
   `grep -rho '"label":"Codex-Named Unit[^"]*"' data/sheet_rules --include=*.json | wc -l` =
   **1748** label occurrences, `grep -rl '"label":"Codex-Named Unit' data/sheet_rules
   --include=*.json | wc -l` = **1488** files (re-derived independently this cycle; the
   original review reported 1748/1489 -- the occurrence count matches exactly, the file
   count is off by one in the review's own figure, not this receipt's). Every one of those
   1748 occurrences now resolves to a source-derived display name wherever `display_label`
   (or one of its 3 newly-fixed call sites) is on the read path; none of them reach a player
   as the raw ingest string through any of the 4 call sites this cycle audited.
8. **Verification run section pointed at the wrong log and undercounted failures** --
   **CORRECTED**; see the rewritten Verification run section above (now names both logs,
   states the real 3-stage first-pass failure list, and carries a full stage table for the
   clean second pass).
9. **8 paths outside the brief's declared write scope, not escalated** -- **NOTED** (the
   brief's write scope named `pilot_compute/mod.rs` only for PC8-2/PC8-1/PC4-1, and
   `character_hub.rs`/`main.rs`/frontend files for desktop; the prior pass also touched
   `pilot_compute/class_chassis_sheet_rules.rs`, `pilot_compute/generic_class_chassis.rs`,
   `class_catalog.rs`, `class_catalog_generic.rs`, `companion_pool_catalog.rs`,
   `wizard_school_picker.rs`, and 3 docs files outside `docs/release/SD-36-consolidation/**`).
   Each edit was a consequential test-pin/denominator correction forced by CONV-05's own
   in-scope fix rippling through those files' test assertions, not an independent scope
   expansion; this receipt now says so explicitly rather than leaving it silent. This fix
   cycle further touches 2 of the same already-widened files
   (`class_chassis_sheet_rules.rs` for item 3 above; `class_catalog_generic.rs` for item 12
   below) plus 2 genuinely NEW ones not in the original brief list:
   `class_feature_feat_bridge.rs` (item 3, a real engine-P1-4 leak the original pass missed
   entirely) and `character_hub.rs`'s `validate_character_id` visibility (item 4, `fn` ->
   `pub(crate) fn`, one-word change, no behavior change). Both are direct fixes for named,
   CONFIRMED findings in this fix-cycle's own instructions, the same standing under which the
   brief's own findings authorized touching files outside its literal list.
10. **CONV-06/07/08 deferral retro written to another actor's shard**
    (`docs/retro/events/sd31-transcribe.jsonl`, actor field `"sd31-transcribe"` -- a
    `RETRO_ACTOR` mis-set, since the event's own `task`/`tracked_at`/`summary` content is
    unambiguously this epic's) -- **FIXED**. Moved (not merely noted): removed from
    `sd31-transcribe.jsonl`, re-appended to `docs/retro/events/sd36-epic-e.jsonl` with
    `actor` corrected to `"sd36-epic-e"`, a fresh id (`...-sd36-epic-e-78bb64`, same
    timestamp and content otherwise) and a `note` field recording the move and why. The
    finding-disposition table's retro-id references (CONV-06/07/08 rows above) now cite the
    corrected id.
11. **GATE-02 must not be counted as a closed gate** -- already correctly marked **PARTIAL**
    in the disposition table above; no change needed (`git diff --name-only
    3954b4d9..121ce526c8 -- scripts/oracle_harness | wc -l` = 0, confirmed, the roster
    widening genuinely did not start).
12. **Two denominators conflated in the summary** (81 raw `(book, slug)` chassis rows vs 78
    distinct real classes after dedup) -- **FIXED at the source, not just documented**. This
    was a real, user-visible bug `engine-P1-4`'s own fix exposed:
    `class_catalog_generic.rs`'s `generic_catalog_entries_cover_every_class_with_no_overlap_
    into_crb_pu_names` test asserted 81 *distinct display names*, which was only ever true
    because each redacted class's raw placeholder label embeds its own source line and is
    therefore unique by construction -- an artifact of the bug, not a real invariant. Once
    `display_label` resolves real names, 3 slugs genuinely reprinted across two
    `CLASS_FAMILY_BOOKS` books each (`Cyphermage`, `Hellknight`, `Red Mantis Assassin` --
    re-derived by grouping `load_generic_class_progressions(&repo()).0` by `.name`) correctly
    collapse to one shared name apiece: 81 rows, 3 collapsed pairs, **78** distinct names --
    the SAME 78 `generic_class_chassis.rs`'s own `all_seventy_eight_conventional_classes_
    resolve` already asserted over the identical book set (that module dedupes by slug
    directly). The two populations now agree because they measure the same real classes, not
    two different denominators. Test assertion corrected 81 -> 78 with the full re-derivation
    in its own comment; `the_converted_package_carries_eighty_one_conventional_classes` and
    `all_81_generic_classes_reach_a_real_chassis_at_character_creation_altitude` are UNCHANGED
    and still correctly assert 81 -- they count raw rows (by slug, not by name) and are not
    affected by the naming fix.
13. **4 stale-baseline notices left unaddressed** -- see the dedicated subsection below
    (this fix cycle's own new tests move the counts further still, so the baselines are
    updated once, after this cycle's full verify pass, to the truly final numbers rather than
    twice).
14. **Atomic-save doc comment overclaimed durability** (no `fsync`, so the "OLD or NEW,
    never partial" guarantee holds for a process crash but not a power loss on a filesystem's
    default journaling mode) -- **FIXED by softening the comment** (chose this over adding
    `fsync`: the existing crash-only guarantee is real and tested, and the receipt's own
    accuracy was the actual defect, not the missing durability hardening, which is a
    deliberate future change per the finding's own framing). `saved_character/local_store.rs`,
    `campaign/local_store.rs` and `homebrew_authoring/package_store.rs` all now state the
    process-crash-vs-power-loss distinction explicitly.

### Baseline update (review finding 13)

Resolved by this fix cycle's own final verify passes (below), updated once to the numbers
those passes measured: `scripts/verify-baselines.env` `BASELINE_ROOT_LIB_TESTS` 3390 -> 3401,
`BASELINE_ROOT_FULL_TESTS` 7855 -> 7875, `BASELINE_ROOT_TEST_BINARIES` 412 -> 414,
`BASELINE_DESKTOP_TESTS` 592 -> 596. Full attribution (including the prior epic-e pass's own
unrecorded growth folded in at the same time) in that file's own new dated block.

### Fix cycle's final verification (two verify.sh passes)

**Pass 1** (full, HEAD `bb5a2e09fc`), log
`/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/epic-epice-fix1-verify.log`:
`RESULT: FAIL` -- `FAILED: 1 root-full`. All 45 other stages PASSED, including `root-lib
(3401 passed)`, `desktop (596 passed)`, `clippy (root:0 desktop:0 warnings)`,
`pcgen-residue-gate (live_files=0 live_hits=0)`. The single failure:
`sd24_wired_integration_audit.rs`'s `placeholder_findings_are_ui_text_prose_or_the_one_
documented_deferral` false-flagged this fix cycle's own new `let placeholder = ...` test
variables in `level_up_option_filter.rs`, `class_chassis_sheet_rules.rs` and
`class_feature_feat_bridge.rs` (11 hits: bare `placeholder` identifiers, none of the audit's
existing UI-text/deferred-finding/comment-prose/anti-fabrication/PCGen-`p.xx`/bucket-F
exclusions apply to a plain local variable name) -- a real, if narrow, audit finding against
this cycle's own code, not a pre-existing issue. Fixed at the source: renamed the variable
`placeholder` -> `redacted_label` everywhere it is a Rust identifier in those three files (11
sites); the surrounding doc comments and assert-message English prose, which also say
"placeholder" but are exempted (comment prose, or an incidental `"placeholder:"` substring
match against the UI-text bucket), were left untouched -- confirmed by re-implementing the
audit's own bucket logic in a standalone script over `git grep -nE '\bplaceholder\b' --
apps/desktop/ apps/desktop/src-tauri/ src/` after the rename: 0 unexplained hits.

Because the rename was made while pass 1's own `verify.sh` process was still running later
stages (`corpus-sweep` onward), those later stages' PASS results reflect the renamed code
(cargo picked up the file change and rebuilt), not a pre-rename snapshot -- but `root-full`'s
own FAIL was captured before the rename, so it needed an independent re-run rather than being
trusted from pass 1.

**Pass 2** (`--only root-full --only clippy`, after the rename), log
`/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d7b37005-8466-4968-b248-1a4983d15f82/scratchpad/epic-e-fix-rootfull-recheck.log`:
`RESULT: PASS` -- `root-full (7875 passed across 414 suites, all 362 tests/*.rs suites
executed)`, `clippy (root:0 desktop:0 warnings, 0 errors)`.

Together, passes 1 and 2 cover all 46 stages green on the final tree. The mid-run-edit timing
hazard itself (editing source while pass 1's later stages were still executing, so root-full's
FAIL had to be independently re-verified rather than trusted from that same process) is logged
as retro incident `1789664310234-sd36-epic-e-9097f7`.

## Blockers

None outstanding that block this cycle's own closure -- the two escalations above are scope
decisions on follow-on work, not blockers to landing what this cycle did fix.
