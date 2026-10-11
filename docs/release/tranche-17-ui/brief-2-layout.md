# Brief 2 — Layout rework, sheet + Create (items 7–13)

STATUS: DRAFT for operator approval. No code written. Sequence AFTER Brief 1 lands (both edit `CreateCharacterForm.tsx` ~1069–1300). Line numbers below were derived at tranche-17-ui @e43e9e89c8 (before Brief 1); re-derive every one at dispatch time.

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
Blocked on an engine preview call (Q3). Per blocker doctrine this is a DoD item, not a deferral: either the engine call is built here, or the operator rules otherwise.

## Required reads
`CharacterSheet.tsx` 458–620, 1466–1500, 1743, 4219, 4280, 4578–4830; `CreateCharacterForm.tsx` 917–1500; `LevelsPanel.tsx`; `levelsModel.ts`; `characterProgression.ts`; `encumbranceTabModel.ts`; `customModel.ts`; `printLayout.ts`; `layout/screenLayout.ts`; `src/rules_core/encumbrance.rs` (~179).

## Allowed write scope
`apps/desktop/src/characterHub/{CharacterSheet.tsx,CreateCharacterForm.tsx,LevelsPanel.tsx,levelsModel.ts,customModel.ts,printLayout.ts}` + new `sheetLayout.ts`, new shared panel files + tests; `apps/desktop/src/layout/*`; `ui-smoke/spec.json` + generated inventory; Rust only for `conditions` serde default and an encumbrance/initiative draft preview (`character_hub.rs`, `src/rules_core/encumbrance.rs`). Anything else: stop.

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
3. Create-time Encumbrance/Initiative: build the engine draft-preview call in this brief? Recommended: YES (L5 is in the DoD; without the call the Create screen cannot mirror the sheet's 2x2 grid).
