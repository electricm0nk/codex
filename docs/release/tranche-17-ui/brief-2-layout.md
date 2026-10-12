# Brief 2 — Layout rework, sheet + Create (items 7–13)

STATUS: APPROVED by operator 2026-10-12 (Q3 = yes: build the engine draft-preview call). Brief 1 landed (29a04f716d). Anchors re-derived at tranche-17-ui @29a04f716d: `wc -l CharacterSheet.tsx` = 4895, `CreateCharacterForm.tsx` = 1712; `.sheet-cols` 4587; `IdentityPanel` 484, `AbilitiesPanel` 503, `InitiativeHpPanel` 556 (rendered 4775–4776 with `dexMod` from 4228), `EncumbrancePanel` 1752, `SkillsPanel` 4830; Create: `LevelsPanel` import 31, `heldClassKey` 456, `spellKey` 531, Racial Traits box 1082, Spells box 1255–1283. Brief 1 added `SkillsPanel` prop `explanations`, `TransferListDialog` `pairPerGroup`/`sharedLimits`, `ManageBox` `manageLabel`.

Run shape (operator 2026-10-12: "create a workflow for that"): a Workflow with two worktrees. Lane E (engine, Rust, `.worktrees/brief2-engine`, branch `ui-brief2-engine`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief2-engine`) does E1–E3 below. Lane U (UI, `.worktrees/brief2`, branch `ui-brief2`, `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief2`) does L0–L1 in parallel, then merges `ui-brief2-engine` and does L2–L5 serially. One final verification agent, one receipt agent.

## Objective
Regroup the character sheet and the Create screen into stacked, rearrangeable boxes; make Create mirror the sheet.

## Repo / workdir
`/home/ubuntu/workspace/repos/codex`, `apps/desktop/src/characterHub` + `apps/desktop/src/layout`. Own worktree, own `CARGO_TARGET_DIR` if Rust is touched.

## Findings (commands)
- Sheet is hard-coded JSX, inline styles, no section registry: `wc -l apps/desktop/src/characterHub/CharacterSheet.tsx` = 4885. Three columns (`.sheet-cols`, line 4578): left rail 240px (portrait, Level, class list); middle (from ~4763: IdentityPanel 4765, AbilitiesPanel 4766 with a 6-across grid at 502–519, InitiativeHpPanel 4767, AC/Attack left + Saves/Speed right at 4769–4778, then tabs); right 300px (Details 4818, Skills 4821).
- Create is a separate copy: `wc -l CreateCharacterForm.tsx` = 1683. Left (name, race, Level/HP/Alignment/Deity boxes, Physical Attributes, ManageBoxes incl. Skills at 1173–1182), middle `LevelsPanel` (1321–1328, own file, 166 lines), right Abilities (1330–~1500, inline JSX). Only `SkillAllocationDialog`, `CustomDialog`, `ManageBox` are shared with the sheet.
- No render/DOM tests exist. No browser on this box.

## Data readiness (per box)
| Box | Sheet | Create |
|---|---|---|
| Hit Points | EXISTS (`hp`, 4280; Durability panel 1466–1500) | EXISTS (`maxHp`, 440) |
| Initiative | PARTIAL: front-end uses Dex mod only (4219); ignores Improved Initiative etc. | MISSING |
| Encumbrance | EXISTS: `EncumbranceDto`, engine `src/rules_core/encumbrance.rs` (619 lines), `EncumbrancePanel` 1743 | MISSING (0 hits: `grep -c "corpusDerived\|ncumbrance" CreateCharacterForm.tsx`) — needs engine preview/IPC (INFERRED) |
| Conditions | MISSING: `CharacterCustom` has `grants, feats, equipment, spells, devices` only (`customModel.ts:12-41`) | MISSING |
| Per-level gains | PARTIAL: `buildLevelEntries` (feat, skills, ability increase) is pure front-end; class features need async `previewLevelUp` | same |

## Tasks (TDD; one pass)

### L0 — Layout registry (do first; this is the "easy to rearrange" requirement)
Export a declarative module: `{ id, column, order, boxStyle }[]` for the sheet and a second for Create. Panels render from it. Unit-test the config (no DOM): Abilities column is `flexDirection:'column'`; AC/Attack/Speed/Saves order; 2x2 block `gridTemplateColumns:'1fr 1fr'`, each box `aspectRatio:'1'`.
Also: `.sheet-mid` class is missing on the middle column, so the print rule in `printLayout.ts:67` matches nothing today — fix or note.

### L1 — Sheet: Abilities as one vertical column (item 10), AC / Attack / Speed / Saves as stacked own boxes (item 11).

### L2 — Top-left 2×2 square grid under the name (item 12): Hit Points, Initiative (prominent), Encumbrance, Conditions.
- Encumbrance (sheet): reuse `describeEncumbrance`.
- Conditions: shows **Normal** when the list is empty (operator ruling). To avoid a hard-coded stub, read from a real `conditions: string[]` on `CharacterCustom` (default `[]`, serde default on the Rust side so old saves load). Nothing writes it yet; the custom menu will. (Operator approved 2026-10-11: add `conditions: string[]` to `CharacterCustom` now, serde default on the Rust side.)
- Initiative (operator ruling 2026-10-11): compute it in the engine as a sheet total (Dex mod + Improved Initiative + every other grounded initiative bonus) and render the engine row on the sheet and in the Create preview; retire the Dex-only front-end shortcut at `CharacterSheet.tsx:4219`.

### L3 — Create screen: reorder to match the sheet (items 7, 8, 13)
Far-left: level-add dialog + HP roller (`LevelsPanel`, `rerollLevel`/`rollHitDie` in `levelsModel.ts`; HP roller stays). Same column as Abilities: a Skills box listing known skills. Reuse the sheet's panels instead of copies where possible (extract Abilities/AC/etc. into shared components).

### L4 — Create: read-only per-level summary under the HP roller (item 9)
`Monk 1: +1 feat, +N skills, +1 Str…`. Source: `heldClassesOf(levels)` (`levelsModel.ts`) → `buildLevelEntries` (`characterProgression.ts`) for feat / skills / ability increase (pure front-end). Class features per level need `previewLevelUp` (async, per class). Informational only. Do not alter the HP roller.

### L5 — Create: Encumbrance + Initiative boxes
Operator ruled yes (Q3). Lane E builds the call; lane U renders the two boxes on Create from it, matching the sheet's 2x2 block.

### Lane E — engine tasks (Rust first, one build pass)
- E1 Initiative as an engine sheet total: one explanation row (id `initiative.total`, or the existing standalone initiative record if one exists; `grep -rn initiative src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs`) = Dex mod + `initiative_bonus_from_feats` (`src/rules_core/feat_effects.rs:484`) + every other grounded initiative bonus (trait rows, class rows such as Inquisitor cunning initiative). Sheet DTO carries it; `CharacterSheet.tsx:4228` `dexMod` shortcut is retired by lane U.
- E2 `conditions: Vec<String>` on the custom record (`apps/desktop/src-tauri/src/character_custom.rs:67` `CharacterCustomDto`, `:217` request) with `#[serde(default)]` so old `custom.json` files load; TS `CharacterCustom.conditions: string[]` default `[]` (`customModel.ts:33–43`). Nothing writes it yet; the sheet shows `Normal` when empty.
- E3 Draft preview command `draft_sheet_preview` (new Tauri command in `character_hub.rs`, registered in `main.rs` `invoke_handler`; boundary file `apps/desktop/src/boundary/draftSheetPreview.ts`): takes the same `CreateCharacterRequest` as `draft_spell_options_impl` (`character_hub.rs:1821`) and returns `{ initiative, encumbrance: EncumbranceDto }` computed by the real engine (`compute_encumbrance`, `src/rules_core/encumbrance.rs:370`; `map_encumbrance_dto`, `character_hub.rs:1251`). Tests first: Rust (Fighter draft with chain shirt + longsword → light/medium load matches the sheet's value for the same saved character; Improved Initiative draft → +4), TS boundary test with the wire shape.

### Follow-up after workflow run 1 (lead rulings 2026-10-12; lane U4, single worktree `.worktrees/brief2`, Rust allowed)
- L4b (blocker cleared, in DoD): `preview_level_up` (`character_hub.rs:2798`) needs a saved character, so Create cannot get class features per level. Add `draft_level_up_preview(request: CreateCharacterRequest, class_id, from_level)` in `character_hub.rs` (registered in `main.rs`): `build_create_input(request, FeatPolicy::Trust)` → `compute_level_up_grants_for_class(&input, class_id, from_level, from_level+1)` → return the automatic feature names. TS boundary `apps/desktop/src/boundary/draftLevelUpPreview.ts` (+ test); `LevelGainsPanel` calls it per level and prints the features in the line. Tests first (Rust: Fighter 1→2 gives the bonus feat slot/feature names the saved path gives; TS wire shape).
- L2b (sizing, lead judgment; operator may retune, "easy in code"): the sheet's 2x2 block at full column width (~640px squares at 1920x1200) pushes the stat block below the fold. Cap the block: `SQUARE_BLOCK_STYLE.maxWidth` ≈ 360px (boxes ≈ 170px), left-aligned under the name, on both sheet and Create. Config test pins the cap.
- Check (not a change unless wrong): Create's draft preview shows "29 lb carried" with no equipment chosen. Find the source (`build_create_input` default kit?). If the saved character from the same draft shows the same figure, it is consistent and stays; if the draft and the saved sheet differ, that is a twin and must be fixed in the draft path. Record the finding in the receipt.
- Scope for lane U4: everything in lane U plus `apps/desktop/src-tauri/src/{character_hub.rs,main.rs}`, new `apps/desktop/src/boundary/draftLevelUpPreview.ts` (+ test).

## Required reads
`CharacterSheet.tsx` 458–620, 1466–1500, 1743, 4219, 4280, 4578–4830; `CreateCharacterForm.tsx` 917–1500; `LevelsPanel.tsx`; `levelsModel.ts`; `characterProgression.ts`; `encumbranceTabModel.ts`; `customModel.ts`; `printLayout.ts`; `layout/screenLayout.ts`; `src/rules_core/encumbrance.rs` (~179).

## Allowed write scope
Lane U: `apps/desktop/src/characterHub/{CharacterSheet.tsx,CreateCharacterForm.tsx,LevelsPanel.tsx,levelsModel.ts,customModel.ts,printLayout.ts,characterProgression.ts,encumbranceTabModel.ts}` + new `sheetLayout.ts`, new shared panel files (`apps/desktop/src/characterHub/panels/*`) + tests; `apps/desktop/src/layout/*`; `apps/desktop/src/boundary/draftSheetPreview.ts` (consume only); `apps/desktop/scripts/ui-smoke/spec.json` + generated `docs/testing/ui-smoke-inventory.md`.
Lane E: `apps/desktop/src-tauri/src/{character_hub.rs,character_custom.rs,main.rs}`, new `apps/desktop/src/boundary/draftSheetPreview.ts` (+ test), `src/rules_core/{encumbrance.rs,feat_effects.rs}`, `src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs` (+ tests), and the one pilot_compute file that emits the initiative total if it lives elsewhere (name it in the receipt).
Anything else: stop and report the exact path and defect; the lead widens scope or escalates.

## Non-goals
No drag-and-drop (operator: "easy in code"). No custom-menu UI. No conditions editing or condition effects. No spell/trait changes (Brief 1). No starter characters (Brief 3).

## Verification (once)
`cd apps/desktop && npm run typecheck && npm test`; layout-config tests; ui-smoke labels unchanged (`Add level` at `spec.json:341,356,1104`; `Initiative` at 1467), then `npm run ui-smoke:doc`. Rust: `cargo test` in `apps/desktop/src-tauri` (separate workspace) + root.
UNCOVERED: real pixel layout, squareness, overflow, print layout — no browser on this box. **Operator must eyeball the running app.** Panels are plain function components, so tests may inspect returned element trees; this is brittle.

## Models
Opus 5.5 for L0/L3/L5. Sonnet 5.5 for L1/L2/L4. Haiku 4.5 for housekeeping.

## Open questions
1. (RESOLVED: engine-computed Initiative.)
2. (RESOLVED: add `conditions` field now.)
3. (RESOLVED 2026-10-12: operator approved building the engine draft-preview call, E3.)
