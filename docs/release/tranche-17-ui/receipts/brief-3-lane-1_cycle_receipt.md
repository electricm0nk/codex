# Brief 3 — lane 1 cycle receipt (S1, S2, S2b; Rust + TS)

- Brief: `docs/release/tranche-17-ui/brief-3-starters.md` (approved 2026-10-11)
- Worktree / branch: `.worktrees/brief3`, `ui-brief3`, base `tranche-17-ui` @ `6d08b90324`
- `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief3` (kept for the next lane, per dispatch)
- Status: **complete** for S1, S2, S2b. S3 (docs, ui-smoke spec, inventory, architecture docs) is NOT done here; it is the next lane.

## Commits

| sha | subject |
|---|---|
| `fec287a994` | feat(tranche-17-ui,brief3): third starter seed (Cleric 3 / Fighter 2), visible seed errors, restore command |
| `578ed2b9af` | test(tranche-17-ui,brief3): regenerate the F6e starter-seed list wire pin for three seeds |
| `db2d6bac53` | feat(tranche-17-ui,brief3): Restore starter characters button and seed-error notice on the Load screen |

S1, S2 and S2b Rust share one commit: all three change `seed_one_starter` / the seeding pass in one file, and `git add -p` is not available in this harness.

## Files changed

- `apps/desktop/src-tauri/src/character_hub.rs` — `StarterSeed.additional_levels`; third seed; `SeedMarkers`, `StarterSeedPass`, `seed_starters_at`; `seed_one_starter` returns `Result<bool, String>` and applies `additional_levels` via `pf1_adapter::apply_level_up`; `seed_default_character_if_needed` now returns `Vec<String>` (the errors); `StarterSeedErrors` state; commands `list_starter_seed_errors`, `restore_starter_characters` (+ `restore_starter_characters_at`); tests.
- `apps/desktop/src-tauri/src/main.rs` — setup keeps the seed errors as `StarterSeedErrors` state (still `eprintln!`s them); both commands registered in `invoke_handler`.
- `apps/desktop/src/boundary/restoreStarterCharacters.ts` (new) — `restoreStarterCharacters()`, `listStarterSeedErrors()`.
- `apps/desktop/src/characterHub/starterCharactersModel.ts` (new) + `.test.ts` (new) — pure message text.
- `apps/desktop/src/characterHub/LoadCharacterScreen.tsx` — seed-error notice under the heading; "Restore starter characters" button in the action bar (calls the command, sets the status line, calls `props.onRefresh()` to reload the list).
- `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6e-starter-seed-list-wire.json` — regenerated (operator-approved, brief open question 3).
- `apps/desktop/src/characterHub/buildCharacterHubListSurface.test.ts` — the Load-list test that reads that pin now expects three seeds (it went red from the regenerated pin; fixed here, not left for S3).
- `apps/desktop/src/testSupport/starterSeedListWire.ts` — doc comment only ("both" → "all three").

## Decisions

- **Class order:** Cleric is the level-1 class (`class_id: class:cleric`, `level: 3`), then `additional_levels: [class:fighter, class:fighter]`. Stored `class_levels` = `[cleric 3, fighter 2]`.
- **Domain / deity:** none chosen by the seed. The engine needs only the Cleric's canonical seed, which `compose_character_input` already records (`class_seeds::canonical_seeds_for("cleric")` = one choice `choice:cleric_domain` → `domain:good`). No deity. The build reaches `Computed` with that (test `the_third_seed_is_a_cleric_3_fighter_2_with_spells_prepared`).
- **Spells:** Cure Light Wounds (Cleric 1) and Spiritual Weapon (Cleric 2), recorded and prepared. Bless was not used: its converted record label is `Bless (attack)`, not a plain key. Pinned by `the_cleric_seed_spell_ids_are_converted_crb_records` (SPELL_LIST key, `class_spell_level`, converted record label + `ClassSpellList cleric` level).
- **Skill ranks:** Climb 1, Intimidate 1, Swim 1, Heal 5, Knowledge (religion) 5, Sense Motive 2 = 15 = (2 + 0 + 1) × 5. Zero unallocated; max 5; non-GE-06 skills are Cleric class skills.
- **S1 surface:** Tauri-managed `StarterSeedErrors` (latest pass: launch, or last Restore) read by the new `list_starter_seed_errors` command, shown as a notice on the Load screen. `ListSavedCharactersResponse` was not changed (its producers live in `pf1_adapter.rs` / `sf_adapter.rs` / `rule_system_adapter.rs`, outside the write scope).
- **Restore result shape:** `{ restored: string[], errors: string[] }` — no success flag. A failed seed is in `errors`; the UI says `Could not restore: <error>`, never "already here".
- `previewData.ts:14-23` still describes Aldric as Fighter 3 / Wizard 1 (hand-written preview data, disagrees with the seed). Not changed, per the brief.

## Tests added or updated (`character_hub::starter_seed_tests`)

New: `the_cleric_seed_spell_ids_are_converted_crb_records`, `the_third_seed_is_a_cleric_3_fighter_2_with_spells_prepared`, `brother_tomas_loads_with_zero_unallocated_skill_points`, `an_existing_install_with_two_markers_gains_the_third_seed_without_touching_aldric`, `a_deleted_third_seed_does_not_come_back`, `restore_brings_back_a_deleted_seed_and_leaves_an_edited_seed_untouched`, `restore_rewrites_the_marker_of_the_seed_it_restores`, `a_seed_that_fails_to_compute_reports_its_error`, `the_starter_seed_error_state_holds_the_latest_pass`.
Renamed + widened: `a_fresh_install_seeds_both_characters_and_both_markers` → `a_fresh_install_seeds_all_three_characters_and_all_three_markers` (labels in id order, classes per seed, three markers).
TS: `src/characterHub/starterCharactersModel.test.ts` (new); `buildCharacterHubListSurface.test.ts` function `verifiesAFreshInstallListsBothStarterSeeds` → `verifiesAFreshInstallListsAllThreeStarterSeeds`.

## Commands run and results

All cargo commands from `apps/desktop/src-tauri` with `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief3`.

1. RED (TS): `npx tsx src/characterHub/starterCharactersModel.test.ts` → `ERR_MODULE_NOT_FOUND ... starterCharactersModel` (model absent).
2. RED (Rust): `cargo test -p codex-desktop --no-run` (log `/home/ubuntu/cargo-target/brief3/logs/warmup.log`) → `could not compile codex-desktop (bin "codex-desktop" test) due to 23 previous errors`, every one a missing new name (`THIRD_SEED_CHARACTER_ID` ×7, `restore_starter_characters_at` ×3, `StarterSeed` has no field `additional_levels`, `SeedMarkers`, `StarterSeedErrors`, `seed_starters_at`, spell/class constants). Re-derive: `grep -E '^error' /home/ubuntu/cargo-target/brief3/logs/warmup.log | sort | uniq -c`. This is a compile-level RED; the count assertions (2 → 3 seeds) were not seen failing as assertions.
3. GREEN (Rust, targeted): `cargo test -p codex-desktop starter_seed_tests -- --test-threads=8` → `test result: FAILED. 19 passed; 1 failed; ... 771 filtered out`. The one failure: `the_starter_seed_list_wire_matches_the_committed_artifact` (expected pin drift).
4. Pin: `CODEX_WRITE_F6E_WIRE=1 cargo test -p codex-desktop the_starter_seed_list_wire_matches_the_committed_artifact` → `1 passed`; then without the env var → `1 passed; 0 failed`.
5. Wide Rust: `cargo test -p codex-desktop -- --test-threads=8` (log `/home/ubuntu/cargo-target/brief3/logs/full.log`). Every `test result` line with its `Running` line (`awk '/Running/{r=$0} /^test result/{print r " => " $0}' /home/ubuntu/cargo-target/brief3/logs/full.log`):
   - `unittests src/main.rs` → `ok. 789 passed; 0 failed; 2 ignored`
   - `tests/packaged_resources.rs` → `ok. 1 passed; 0 failed; 0 ignored`
   - No `FAILED` line.
6. Frontend, from `apps/desktop`: `npm run typecheck` → exit 0. `npm test` → `155/155 test files passed.` (log `/home/ubuntu/cargo-target/brief3/logs/npm-test2.log`). The first `npm test` run, before the list-test fix, was `154/155` with `FAIL src/characterHub/buildCharacterHubListSurface.test.ts` (the regenerated pin).

## Wired Integration audit (four checks, base `6d08b90324...HEAD`)

```
1:
OK_NO_TOKENS
2:
OK_NO_NOOP_HANDLERS
3:
OK_NO_MOCK_LEAKS
4:
OK_NO_WOULD_STRINGS
```

Commands: the four in `docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit", with `develop...HEAD` replaced by `6d08b90324...HEAD` (this lane's base).

## What this proof does NOT cover

- The operator's real app-data directory (S0 diagnosis is still the operator's two `ls` commands).
- A built, launched app: the two new commands are registered and compile, but no test invokes them through Tauri IPC; `StarterSeedErrors` being managed before a command reads it is shown by code, not by a run. The Load-screen button and notice have no render test (only the pure message model is tested).
- ui-smoke: no row for the button yet (S3 lane).
- `pf_seed_render_hash.rs` still renders only Aldric and Elowen (`PF_SEEDS` has 2 entries); Tomas is not in that hash harness.
- The broken-seed test uses an unknown class id (`class:not_a_real_class`). It proves the error path, not every way a real seed could fail.
- Cleric 3 / Fighter 2 is proven `Computed` with this one build (these scores, domain, feats, spells). Other Cleric/Fighter builds are not covered.

## Blockers

None.

## For the S3 lane

- New seed: `Brother Tomas Vane`, id `00000000-0000-0000-0000-000000000003`, marker `.default_character_seeded_3`, classes `class:cleric:3,class:fighter:2` (prints `Cleric 3 / Fighter 2`).
- New Tauri commands: `restore_starter_characters`, `list_starter_seed_errors`. Button label `Restore starter characters` (busy: `Restoring…`). Messages: `Restored: <names>`, `All starter characters are already here.`, `Could not restore: <errors>`, notice `A starter character could not be created: <errors>`.
- Renamed tests: Rust `a_fresh_install_seeds_both_characters_and_both_markers` → `a_fresh_install_seeds_all_three_characters_and_all_three_markers`; TS `verifiesAFreshInstallListsBothStarterSeeds` → `verifiesAFreshInstallListsAllThreeStarterSeeds`.
- Already done here (do not redo): the wire JSON pin, `starterSeedListWire.ts` comment, `buildCharacterHubListSurface.test.ts`.
- Still S3: `ui-smoke/spec.json` (`load-seed-list-both`, a row for the button), generated inventory, `docs/architecture/persistence.md`, `docs/architecture/desktop-app.md`, `appDataIsolation.mjs`, `run-desktop/SKILL.md`.

## S3 lane (docs / ui-smoke / render-hash)

Files changed: `apps/desktop/scripts/ui-smoke/spec.json` (`load-seed-list-both` widened to three seeds, new row `load-restore-starter-characters`, `open-seed-aldric` note), `docs/testing/ui-smoke-inventory.md` (regenerated), `docs/architecture/persistence.md`, `docs/architecture/desktop-app.md`, `apps/desktop/scripts/ui-smoke/lib/appDataIsolation.mjs`, `apps/desktop/.claude/skills/run-desktop/SKILL.md`, `apps/desktop/src-tauri/src/pf_seed_render_hash.rs` (third seed `tomas`), `apps/desktop/src-tauri/src/character_hub.rs` (one stale comment, "both seeds" to "all three seeds").

Commands and results (cargo from `apps/desktop/src-tauri`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief3`):
- RED: `cargo test --locked -j 8 -p codex-desktop --bins pf_seed_render_hash -- --test-threads=8` with the names assertion widened first: FAILED, `left: ["aldric","elowen"] right: ["aldric","elowen","tomas"]`.
- GREEN: same command after adding `("tomas", THIRD_SEED_CHARACTER_ID)`: `1 passed; 0 failed`. The test is not a pinned-hash test (no committed hash), so nothing was regenerated. Hashes it printed: aldric `1d830682...a5a569` (55037 bytes), elowen `8d1a711c...b100f2` (81136), tomas `bc50abd2...18ccdf` (79490).
- Test count in docs: `cargo test --locked -p codex-desktop --bins starter_seed_tests -- --list | grep -c ": test"` printed 20. Marker count: `grep -c "marker: " apps/desktop/src-tauri/src/character_hub.rs` printed 5.
- `cd apps/desktop && npm run ui-smoke:doc`: row count 103 to 104. The regeneration also refreshed 4 create-character/encounter rows that were already stale against HEAD's spec.json ("Add level" form); that drift is not from this lane.
- `npm run typecheck && npm test`: exit 0, `155/155 test files passed`.
- Leftover wording grep (`both seeds|two seeds|both starter|two starter|2 seeds`): only `pf1_adapter.rs:3063` remains; it is about two arcanist spell seeds, unrelated.

Not covered: no built-app / ui-smoke run (the new row `load-restore-starter-characters` and the widened `load-seed-list-both` are unexecuted); the row only proves the "nothing to restore" path, the restore-after-delete path is covered by the Rust tests only; `Cleric 3` is expected as a substring of Tomas's list row, unconfirmed live.
