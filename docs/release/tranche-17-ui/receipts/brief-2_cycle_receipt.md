# Brief 2 — cycle receipt (sheet/Create layout, engine initiative total, draft previews)

Branch `ui-brief2` (worktree `.worktrees/brief2`), base `tranche-17-ui` @ d049f2251e
(`git merge-base tranche-17-ui ui-brief2`). Engine lane built on `ui-brief2-engine` and merged into
`ui-brief2` as 6884228486 (clean, ort). Not pushed.

**Status: complete**, except the items under "Uncovered" and "Not done". L4's class-feature part was
blocked in lane U3 (the existing `preview_level_up` needs a saved character) and closed in the U4
follow-up as L4b. The final verification agent ran green (below). Lane detail for U4:
`docs/release/tranche-17-ui/receipts/brief-2-u4_lane_receipt.md`.

## Commits (`git log --oneline tranche-17-ui..ui-brief2 --no-merges`)

| sha | subject |
|---|---|
| 548d199da6 | feat(tranche-17-ui,brief2-L4b/L2b): Create per-level class features from draft_level_up_preview; 2x2 block capped at 360px |
| f2ed6837c4 | feat(tranche-17-ui,brief2-L4/L5): Create per-level gains summary; Create 2x2 block from draft_sheet_preview |
| a44b1c5f03 | feat(tranche-17-ui,brief2-L2/L3): sheet 2x2 block (engine initiative, conditions); Create from CREATE_LAYOUT |
| 6e6750c3eb | feat(tranche-17-ui,brief2-E): engine initiative total, custom conditions, draft sheet preview |
| 581a51790f | feat(tranche-17-ui,brief2-L0/L1): layout registry; sheet Abilities column + stacked combat boxes |

Plus merge commit 6884228486 (`ui-brief2-engine` into `ui-brief2`; 9 files changed, 408 insertions, 2 deletions).

## Lane E — engine (6e6750c3eb)

- **E1 `initiative.total` explanation row.** Dexterity modifier plus every grounded, unconditional
  initiative record: Improved Initiative, traits (Tactician/Arcane Temper), Inquisitor Cunning
  Initiative, Oracle Deaf-curse penalty, Duelist Improved Reaction, Pathfinder Delver Vigilant
  Combatant. In a multiclass mix the `multiclass.<class>.<id>` copies count. Gunslinger Initiative
  (needs a grit point) and Swashbuckler Initiative (needs a panache point) are left out by design and
  keep their own rows. Function, `INITIATIVE_TOTAL_TERMS` and public `INITIATIVE_TOTAL_ID` are in
  `src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs`. The one `pilot_compute` file
  that emits the row is `src/rules_core/pilot_compute/class_shared_core.rs` (one line, last in
  `compute_pilot_base_chassis`). The saved sheet's explanations now carry the row.
- **E2 conditions.** `CharacterCustomDto.conditions: Vec<String>` with `#[serde(default)]`
  (`apps/desktop/src-tauri/src/character_custom.rs`); one test literal in `character_hub.rs` got
  `conditions: Vec::new()`. TS: `CharacterCustom.conditions: string[]`, `EMPTY_CUSTOM.conditions = []`
  (`apps/desktop/src/characterHub/customModel.ts`). One assertion line added to `customModel.test.ts`
  (not named in lane E's scope; smallest place for the failing test).
- **E3 `draft_sheet_preview`.** New Tauri command in `character_hub.rs`, registered in `main.rs`.
  Takes `CreateCharacterRequest`, returns `DraftSheetPreviewDto { initiative: ExplanationDto,
  encumbrance: EncumbranceDto }` through `build_create_input` (Trust) -> `compute_pilot_with_corpus`
  -> `map_encumbrance_dto`, the saved-sheet path. Errs if the draft cannot be built or the engine
  produced no initiative row. TS boundary `apps/desktop/src/boundary/draftSheetPreview.ts`
  (`loadDraftSheetPreview`, `readDraftSheetPreview`) + test.

## Lane U1 — L0/L1 (581a51790f)

- **L0** `apps/desktop/src/characterHub/sheetLayout.ts`: `SHEET_LAYOUT`, `CREATE_LAYOUT`
  (`{ id, column, order, boxStyle }`), `SHEET_COLUMNS`, `SHEET_COLUMN_STYLES`, `boxesInColumn` (sorts by
  `order`), `layoutBox` (throws on unknown id), `SQUARE_BLOCK_STYLE`, `SQUARE_BOX_STYLE`.
- **L1** The sheet's middle stat block renders from `SHEET_LAYOUT` through `renderSheetBox`. Abilities
  is one vertical column of six rows (STR to CHA); Armor Class, Attack, Speed and Saving Throws are
  separate titled boxes stacked to its right. Shared panels moved to
  `apps/desktop/src/characterHub/panels/` (`StatBox.tsx`, `AbilitiesPanel.tsx`, `CombatPanels.tsx`).
- **`.sheet-mid` fix.** The middle column now carries `className="sheet-mid"`, so the print rule at
  `printLayout.ts:69` matches; a test checks every print column class appears in the sheet source.

## Lane U2 — L2/L3 (a44b1c5f03)

- **L2** `panels/SquareBlock.tsx`: Hit Points, Initiative, Encumbrance, Conditions as a 2x2 grid
  (order from `SQUARE_BLOCK_LAYOUT`), directly under IdentityPanel. Initiative shows the engine's
  `initiative.total` value and detail verbatim, em dash if the row is missing (never +0). Encumbrance
  uses `describeEncumbrance` on `corpusDerived.encumbrance`. Conditions shows `custom.conditions`, or
  `Normal` when empty. The Dex-only `InitiativeHpPanel` and `dexMod` shortcut are removed.
- **L3** Create builds its columns from `CREATE_COLUMNS = ['left','details','abilities']` and
  `CREATE_LAYOUT`: levels far left, details in the middle, ability editor on the right with the Skills
  box and `SkillAllocationDialog` below it. `CreateCharacterFields` is now exported for a render test.
  `git diff -w` for the four modified files: +111/-74.

## Lane U3 — L4/L5 (f2ed6837c4)

- **L4** `panels/LevelGains.tsx` (`LevelGainsPanel`, `CREATE_LAYOUT` left, order 1) prints one line per
  level with `describeLevelGains` (e.g. "Barbarian 4: +4 skills, +1 ability score"). The class-feature
  part was blocked here (see "Blocker closed by L4b").
- **L5** `CREATE_LAYOUT` details column: `identity`, `sheetBlock`, `details`. `sheetBlock` renders the
  sheet's `SquareBlock` via `squareBlockForDraft` (`panels/createSheetBlock.ts`), with Initiative and
  Encumbrance from `loadDraftSheetPreview(buildRequest())`, recomputed when `draftPreviewKey` changes.
  `SquareBlock` gained an optional `absent` prop (loading, no level yet, or a refusal shown verbatim).
  The sheet passes none, so it keeps its em dash.
- Xvfb probe (agent brief2-u3): Create with no level shows "Add a level first: the engine computes this
  from the class levels."; Dwarf Barbarian 5, Dex 14: Hit Points 55 / 55, Initiative +2, Encumbrance
  Light Load, 29 lb carried, 76 / 153 / 230 lb; sheet Aldric Ironhand (Fighter 3): HP 28 / 28,
  Initiative +1, 29 lb carried, 116 / 233 / 350 lb.

## Lane U4 — L4b, L2b, 29 lb check (548d199da6)

- **L4b** `draft_level_up_preview(request, class_id, from_level)` in `character_hub.rs`, registered in
  `main.rs`: `build_create_input(request, Trust)` then
  `compute_level_up_grants_for_class(input, class_id, from, from+1)`, returns
  `{fromLevel, toLevel, automaticFeatures}`. TS boundary `boundary/draftLevelUpPreview.ts` (+ test);
  `LevelGainsPanel` gained `classFeatures`/`loadLevelFeatures`, one call per line. Human Fighter 3 in
  the dev app: `Fighter 2: +3 skills; class features: Fighter base_attack_bonus at level 2, Fighter fort_save at level 2, bravery`.
- **L2b** `SQUARE_BLOCK_STYLE` gained `maxWidth: '360px'`, `width: '100%'`, `justifySelf: 'start'`;
  pinned in `sheetLayout.test.ts`. Dev app at 1920x1200: block x=626-986 on Create, x=290-650 on the sheet.
- **29 lb check.** Source is `pf1_adapter::compose_character_input`, which seeds every character with
  an equipped longsword (4 lb) and chain shirt (25 lb). Draft and saved sheet agree; nothing changed.

## Tests added

- Rust root lib: `initiative_total_tests` (7).
- Rust desktop: 2 conditions tests, 4 draft-preview tests (Fighter chain shirt + longsword encumbrance
  JSON equals the saved sheet's; Improved Initiative adds +4, plain draft 2 at Dex 14; camelCase wire
  keys; unbuildable draft errors), 4 `draft_level_up_preview` tests,
  `the_draft_preview_encumbrance_matches_the_saved_sheet`.
- TS: `sheetLayout`, `panels/statPanels`, `panels/squareBlock`, `createLayout`, `panels/levelGains`,
  `panels/createSheetBlock`, `boundary/draftSheetPreview`, `boundary/draftLevelUpPreview`; one line in
  `customModel.test.ts`.

## Red evidence

- E: compile errors for the missing `INITIATIVE_TOTAL_ID`, `draft_sheet_preview_impl` and `conditions`
  field; TS tests failed on the missing module and undefined `conditions`.
- U1: `sheetLayout.test.ts` -> `CharacterSheet.tsx applies className="sheet-mid" for the print rule`;
  `statPanels.test.ts` -> `the six scores stack top to bottom: expected column, got undefined`.
- U2: `squareBlock.test.ts` -> `ERR_MODULE_NOT_FOUND ./SquareBlock`; `createLayout.test.ts` ->
  `'./sheetLayout' does not provide an export named 'CREATE_COLUMNS'`. Mutations: `conditionsLabel`
  returning '' failed `squareBlock.test.ts`; skills moved to column `details` failed with
  `Abilities, then Skills, in one column: expected abilities|skills, got abilities`; both restored.
- U3: `levelGains.test.ts` and `createSheetBlock.test.ts` -> `ERR_MODULE_NOT_FOUND`; `createLayout.test.ts`
  -> `expected levels|levelGains, got levels`.
- U4: `cargo test --bin codex-desktop draft_level_up_preview -- --test-threads=8` with names blanked ->
  `test result: FAILED. 2 passed; 2 failed` (log `/tmp/claude-1000/brief2-logs/u4-red.log`); TS:
  `draftLevelUpPreview.test.ts` `ERR_MODULE_NOT_FOUND`, `levelGains.test.ts` no export
  `LEVEL_FEATURES_LOADING`, `sheetLayout.test.ts` `maxWidth expected 360px, got undefined`.

## Verification (final verify agent, tree clean at start: `git -C .worktrees/brief2 status --porcelain` empty)

| command | result |
|---|---|
| `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief2 cargo test --no-fail-fast -- --test-threads=8` (root, log `/tmp/claude-1000/brief2-logs/v-root.log`) | awk over `test result` lines: 299 result lines, passed 6558, failed 0, no compile errors |
| same command in `apps/desktop/src-tauri` (log `v-desktop.log`) | awk: 2 suites, passed 806, failed 0, no compile errors |
| `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` | exit 0; `164/164 test files passed`; ui-smoke inventory 104 rows, 18 screens |
| `git status --porcelain` after `ui-smoke:doc` | empty (inventory doc unchanged) |

Lane-level results behind that:

- Lane E: desktop unittests `test result: ok. 801 passed; 0 failed; 2 ignored` + `packaged_resources`
  `ok. 1 passed`; root `cargo test --lib initiative_total_tests -- --test-threads=8` -> `ok. 7 passed; 0 failed; 2880 filtered out`;
  root filtered run (lib + `ge06_pilot_combat_baseline`, `sd13_ranger_second_favored_terrain`,
  `sd13_ranger_favored_terrain_choice`, `sd27_alternate_racial_trait_reachability`,
  `sd24_apg_class_coverage_audit`, `sd36_f3_polish`) -> lib 2881 passed, others 4, 7, 9, 3, 15, 7 passed, 0 failed;
  root `cargo build` finished (1 warning, `wizard_spellbook_size_rows` dead code, pre-existing);
  `npm test` 157/157.
- U4: `draft_level_up_preview` green `ok. 4 passed; 0 failed; 803 filtered out`; desktop `cargo build` finished.
- Frontend file counts by lane: 158 (U1), 159 (merge), 161 (U2), 163 (U3), 164 (U4).
- Figures derive from `awk '/^test result/{p+=$4; fl+=$6} END{print p, fl}' <log>` over the logs above.
  The root total sums every `test result` line, so it includes doctests and 0-test suites.

## Decisions

- `initiative.total` adds only rows the engine already grounds; conditional (grit/panache) bonuses stay
  as their own rows.
- Create reuses the sheet's `SquareBlock` and the engine path (`build_create_input` Trust), so draft
  and sheet cannot drift; a refused draft shows the engine's message, not a made-up number.
- Create's ability box stays an editor; no `ArmorClassPanel` etc. on Create (no engine AC/attack/save
  values exist for a draft, so drawing them would invent numbers).
- Block sized at `maxWidth: 360px` after the uncapped squares (~640px on the sheet, ~380px on Create at
  1920x1200) pushed the stat block below the fold.
- The class-feature blocker was resolved by adding a draft variant (`draft_level_up_preview`), not by
  changing the saved-character `preview_level_up`.

## Four-check Wired Integration audit

Final verify, `git diff tranche-17-ui...HEAD --stat`: 29 files changed, 2661 insertions, 859 deletions.

```
1: OK_NO_TOKENS
2: OK_NO_NOOP_HANDLERS
3: OK_NO_MOCK_LEAKS
4: OK_NO_WOULD_STRINGS
```

Checks 3 and 4 were re-run with brace-free expanded pathspecs (git does not expand `{ts,tsx}`): both OK.
Each lane also ran the four checks on its own diff; all OK.

## Uncovered (what the proofs do not cover)

- **No pixel check beyond Xvfb screenshots at 1920x1200.** Row widths, overflow, whether the 2x2
  boxes are square (`aspectRatio: 1` with `overflow: auto`; a long initiative detail may scroll inside
  its box), Create column wrapping at narrow widths, and how page 1 prints with the taller Abilities
  column are not verified. The operator should look at the running app.
- Element-tree and source-text tests (`statPanels`, `squareBlock`, sheet source reads) are brittle and
  do not render DOM; the sheet component itself is never rendered in a test, and the `useEffect`
  recompute has no DOM test (only the pure `draftPreviewKey` test plus the live run).
- Create render test covers Human, one Fighter, no levels; the ranked-skills state is not rendered in a test.
- The audit greps cover only added lines in the listed pathspecs; they do not scan docs or scripts.
- Racial initiative bonuses (e.g. Half-Elf Mismatched) are not in the total: the engine grounds no row.
- Starfinder characters use `sf.initiative` from `sf_defense`; E1 was not checked against that path.
- `initiative.total` is a new row on every PF1 character. Only `sd36_f3_polish` and the sheet_rule
  HeldSeed iteration were inspected (both read class_feature/class_spell/class_chassis rows only);
  other whole-explanation-set pins were covered only by the final root sweep above.
- The engine reports no bonus-feat slot name for Fighter 1->2, in the draft or the saved
  `preview_level_up` (gap in `level_up/fighter.rs`). Per-class level-up modules are gated (Barbarian and
  Fighter: single-class Human only, `src/rules_core/level_up/barbarian.rs:143`), so non-Human,
  multiclass and non-Core drafts print `none reported by the engine`.
- Grant names are raw engine names (e.g. `Fighter base_attack_bonus at level 2`), not display-renamed.
- Create's L4 line says "+1 ability score" (not "+1 Str") because Create records no target ability.
- The seeded 29 lb loadout (longsword + chain shirt for every class, `pf1_adapter.rs`) is consistent
  between draft and sheet; whether it should exist at all is an operator question.
- Nothing writes `conditions` yet (non-goal), so only `Normal` can appear on a real save.
- `spec.json` is unchanged, so `ui-smoke` app rows were not run; only `ui-smoke:doc` was.

## Not done

- Nothing deferred from the brief. Housekeeping left for the lead: delete
  `/home/ubuntu/cargo-target/brief2` and `/home/ubuntu/cargo-target/brief2-engine`; logs are in
  `/tmp/claude-1000/brief2-logs/`. Branch not pushed.
