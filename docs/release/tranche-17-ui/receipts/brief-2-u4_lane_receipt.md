# Brief 2 — lane U4 receipt (follow-up after workflow run 1)

Worktree `.worktrees/brief2`, branch `ui-brief2`, base `8e95cbb0bb`. Not pushed.

## L4b — Create prints class features per level

- Rust: `draft_level_up_preview(request, class_id, from_level)` in
  `apps/desktop/src-tauri/src/character_hub.rs`, registered in `main.rs`. It runs
  `build_create_input(request, FeatPolicy::Trust)` and then
  `compute_level_up_grants_for_class(&input, class_id, from_level, from_level + 1)`, and returns
  `{ fromLevel, toLevel, automaticFeatures: string[] }`. The names are the engine's own, verbatim.
- TS boundary: `apps/desktop/src/boundary/draftLevelUpPreview.ts` (+ test). It refuses a malformed payload.
- `panels/LevelGains.tsx`: `loadLevelFeatures` makes one call per line (`fromLevel` = class level − 1).
  The panel appends `; class features: …` to each line. While the calls run it shows a loading
  note, and a refused call shows its message as a note. An empty engine answer prints
  `none reported by the engine`. `CreateCharacterForm.tsx` calls it in the draft-preview effect, keyed on the same request key.
- What the engine actually reports for Fighter 1→2 (draft and saved path, equal by test):
  `["Fighter base_attack_bonus at level 2", "Fighter fort_save at level 2", "bravery"]`
  (`cargo test --bin codex-desktop draft_level_up_preview`, red log
  `/tmp/claude-1000/brief2-logs/u4-red.log`). **No bonus-feat slot name.** The saved path does not
  report one either. That is the engine gap documented on `levelGrantsFeat` in
  `characterProgression.ts` and in `level_up/fighter.rs`, and it is not a gap between the draft and the saved path.
- Engine coverage, which is the same for the saved `preview_level_up`: the per-class level-up modules are gated
  (Barbarian and Fighter: single-class Human only, `src/rules_core/level_up/barbarian.rs:143`). So a
  non-Human, multiclass or non-Core draft prints `none reported by the engine`. In the dev app a
  Dwarf Fighter 3 did exactly that, and a Human Fighter 3 listed the features.

## L2b — 2x2 block cap

`SQUARE_BLOCK_STYLE` gained `maxWidth: '360px'`, `width: '100%'` and `justifySelf: 'start'`. The
style is shared by the sheet and Create through `SquareBlock`. It is pinned in `sheetLayout.test.ts`.
Dev app, 1920x1200: the block spans x=290–650 on the sheet and x=626–986 on Create, about 175px boxes, and the
stat block (Abilities/AC/Attack/Saves) is above the fold. Screenshots:
`/tmp/claude-1000/brief2-logs/u4-02-create.png`, `u4-03-sheet.png` (not committed).

## "29 lb carried" check — consistent, no change

Source: `pf1_adapter::compose_character_input` (`apps/desktop/src-tauri/src/pf1_adapter.rs:720-745`)
seeds every created character with an equipped longsword (4 lb) and chain shirt (25 lb). That is 29 lb,
whatever the class and whether or not equipment was chosen. Both the draft (`build_create_input`) and the saved
character go through it. Evidence:
- `cargo test --bin codex-desktop the_draft_preview_encumbrance_matches_the_saved_sheet` → `1 passed`.
- Dev app: a Human Fighter 3 draft showed `29 lb carried`, `Light / Med / Heavy 100 / 200 / 300 lb`.
  After `Create character` → Load, the saved sheet showed the same lines (probe bodyText).
The draft and the saved sheet agree, so there is no twin and the brief requires no change. Whether a
character with no chosen equipment should carry a seeded loadout at all is an operator question (the adapter is outside this brief's write scope).

## Verification

- `cargo test --bin codex-desktop draft_level_up_preview -- --test-threads=8` (apps/desktop/src-tauri,
  `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief2`): red with blanked names → `2 passed; 2 failed`.
  Green → `test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 803 filtered out`.
- `cargo build` (apps/desktop/src-tauri) → `Finished dev profile` (`/tmp/claude-1000/brief2-logs/u4-build.log`).
- `npm run typecheck` → clean. `npm test` → `164/164 test files passed.`
- Dev app under Xvfb (run-desktop driver, `RUN_DESKTOP_AGENT=brief2-u4`, UI probe). Create, Human Fighter 3:
  ```
  Fighter 1: +1 feat, +3 skills; class features: Fighter base_attack_bonus at level 1, Fighter fort_save at level 1, Fighter ref_save at level 1, Fighter will_save at level 1, corpus record bonus feats
  Fighter 2: +3 skills; class features: Fighter base_attack_bonus at level 2, Fighter fort_save at level 2, bravery
  Fighter 3: +1 feat, +3 skills; class features: Fighter base_attack_bonus at level 3, Fighter ref_save at level 3, Fighter will_save at level 3, armor training
  ```
- No-stub four-check audit (diff vs `HEAD` before commit, new files `git add -N`): `OK_NO_TOKENS`,
  `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`.

## Not covered

- Full desktop and root `cargo test` were not run here; the final verify agent runs them.
- Class features for non-Core classes, non-Human races and multiclass drafts are empty because the engine is gated. That is an engine boundary shared with the saved path.
- The grant names are raw engine names (`Fighter base_attack_bonus at level 2`). No display renaming was added.
