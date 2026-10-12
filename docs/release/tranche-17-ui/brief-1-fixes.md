# Brief 1 — Create-character fixes (items 1–6)

STATUS: APPROVED by operator 2026-10-11. Branch: `tranche-17-ui` @ 531e96b85e.

## Objective
Fix six defects/gaps on the Create-a-character screen and its Spells/Traits dialogs.

## Repo / workdir
`/home/ubuntu/workspace/repos/codex` (desktop UI: `apps/desktop/src/characterHub`, Rust: `apps/desktop/src-tauri/src/character_hub.rs`, engine: `src/rules_core`). One writer per tree: use `git worktree add` + own `CARGO_TARGET_DIR`; run `git status --porcelain` first.

## Tasks (each: failing test first, then smallest change)

### F1 — Stray "H2" in the title  (operator CONFIRMED 2026-10-11: visible on screen in the running app)
- Finding: the literal `H2` is NOT in any rendered source. `grep -rnE "H2" apps/desktop --include=*.tsx --include=*.ts --include=*.css --include=*.html` (excl. node_modules/target/dist) hits only a test comment (`composeBugReport.test.ts:76`). The title is `<h2 style={{margin:0}}>Create a character</h2>` at `CharacterHubPage.tsx:294`. INFERRED: it comes from the tool that read the screen (a tag-name dump), not from the app.
- It is real but not in the grep'd source, so diagnose in the running app first: use the `run-desktop` skill + the DEV DOM probe (`record_ui_probe`) to dump the DOM around the title and find the node whose text starts with `H2` (candidates: a wrapper that renders a tag name, a dev overlay, a font/icon ligature, a stale built bundle in `apps/desktop/dist`). Fix the source, not the symptom.
- Then add an exact-match title assertion (`innerText === 'Create a character'` for the heading node). The ui-smoke row `create-character-render` (`spec.json:340`) matches `Create a character` as a substring, so a prefix passes today.

### F2 — Racial Traits panel is confusing
- Innate list = `buildCreationRacialTraitsPreview` (`CreateCharacterForm.tsx:729`); count shown as `N innate`. "Manage" opens `TransferListDialog` "Racial traits: <race>" with `limit={null}` (no cap) over `alternateMenu` (`loadAlternateRacialTraitsRuntime`, line 641).
- Defect (INFERRED): the `N innate` count does not change when picks replace innate traits, and "Manage" has no label saying these are replacements.
- Change: show `N innate · M replaced by alternates` (or equivalent) and rename the control; keep the dialog notice. Test in `creationRacialTraitsPreview.test.ts` / `manageItems.test.ts`.
- Operator ruling needed only if you want the button hidden instead (default: keep, relabel).

### F3 — Drawbacks → third trait   (operator ruling 2026-10-11: stays IN this brief; largest item)
- Rule (PF1e): 2 traits at creation; may take ONE drawback to gain ONE extra trait.
- Data gap (VERIFIED): `grep -rli "Drawback ~" data/corpus/ultimate_campaign | wc -l` = 17 files (e.g. hedonistic, vain, paranoid). `src/rules_core/trait_effects.rs:239-241` declares all 30 `ability_content` units (Drawback/Retraining) out of scope. The picker offers only the 53 `trait_generic` traits. No `isDrawback` field exists. The limit `CHARACTER_TRAIT_LIMIT = 2` (`manageItems.ts:8`) is client-only; `selected_traits` has no server count check (`grep -n selected_traits apps/desktop/src-tauri/src/character_hub.rs`).
- Work: (a) model drawbacks in the engine (Rust) with their mechanical effect, per paper-sheet rule: compute only sheet-total effects, print the rest; (b) expose `isDrawback` on the trait DTO; (c) UI: limit = 2, or 3 when exactly one drawback is selected, max one drawback; (d) server-side count validation in `character_hub.rs`.
- Sheet's `TraitsSection` (`CharacterSheet.tsx:2766-2793`) does not use the limit; decide whether the sheet enforces it (default: no change).

### F4 — Sorcerer cannot select spells
- Root cause (VERIFIED by reading both regexes; not run): the form's regex (`CreateCharacterForm.tsx:530`) and `PER_DAY_ID` (`spellsPerDayModel.ts:23`) require ids starting `class_spell.`. The Sorcerer's rows are `class_chassis.sorcerer.spontaneous.…` (`class_sorcerer_wizard.rs:2184, 2340, 2381, 2419`), so no `class:sorcerer` token is derived and the dialog is empty.
- Also: Sorcerer rows exist only when `supported_sorcerer_level` is true (single-class Sorcerer). Multiclass Sorcerer stays unsupported unless widened.
- Test first: `character_hub.rs` has tests for cleric and gnome only (~line 7943); add a Sorcerer case, plus a front-end model test with Sorcerer row ids.
- Caution: `buildSpellsPerDaySurface` also feeds the sheet's per-day rows; widening the regex changes the sheet. Add a sheet test.

### F5 — Spell quota must be KNOWN spells and must grow with levels  (operator ruling 2026-10-11)
- Create screen uses KNOWN spells, not per-day.
- Why 13 Wizard levels gives 5 (best-supported, not run): (1) `spellKey` (`CreateCharacterForm.tsx:522`) depends on `heldClassKey` = class ids only (line 447), so adding a level of the same class does not reload quotas; (2) the quota is the per-day `count` (`spellDialogModel.ts:43-51`), where a level-1 Wizard with Int 12–13 gets 1 base + 1 specialist (hard-coded `class_wizard_prepared_spellbook.rs:1599`) + cantrips.
- Fix: (a) include level count per class in `spellKey`; (b) quota source = spells known. Sorcerer: `…spells_known.spell_level_N` rows exist. Wizard: NO spellbook-size quantity exists in the engine (only recorded `class_spell.wizard.spellbook_contents`, line 1545). Add one: all cantrips; 1st-level spells = 3 + Int mod at level 1, plus 2 new spellbook spells per wizard level after 1 (PF1e CRB). Cleric/Druid (and any caster that knows its whole list): NO picker on Create; show the engine's spells-per-day per level as read-only text (operator ruling 2026-10-11).
- Rust engine work. Batch all Rust changes, then ONE build+test pass (a cargo build is ~30 min).

### F6 — Spell dialog: one Options/Selected pair per spell level
- Today: one `TransferListDialog` (281 lines) with one Options + one Selected + Innate column. Items already carry `group="<class>:<level>"`, and `groupLimits`/`groupLabels`/`remainingByGroup` exist (`TransferListDialog.tsx:117-120`).
- Change: render one labelled pair per group, each showing "N of M picks left". Levels on the far left.
- Note: operator's example said a level-5 Wizard has levels 0/1/2. `wizard_base_spells_per_day(5)` (`class_wizard_prepared_spellbook.rs:1295`) makes 0–3 legal, so 3rd-level spells appear. Confirm.
- The same dialog serves Racial traits, Traits, Feats. Keep the single-pair mode for them (grouped mode only when groups exist).

### F7 — Known-spell quotas for the six classes that lost their picker (operator ruling 2026-10-11, option 3: add now, in this brief)
- Classes: Witch, Arcanist, Alchemist, Investigator, Hunter, Skald. Each has per-day rows but no known-spell quantity, so under F5 they fell back to read-only per-day text. Give each a `*_spells_known.spell_level_N` (or spellbook/formula-book size) explanation row and feed it through the same `known` path F5 built, so the picker returns for them.
- Producers (`grep -rnE "per_day\.spell_level_\{" src/rules_core/pilot_compute`): Witch and Arcanist in `class_summoner_witch.rs` (~1130, ~1645–1669); Alchemist and Investigator in `class_alchemist_investigator.rs`; Hunter in `class_hunter_cavalier_swashbuckler.rs` (~1662); Skald in `class_bard_skald.rs` (~2104; note `class_spell.skald.known_spells` at ~2039 counts the player's picks, it is not the quota).
- Rule source: the quota formula for each class comes from its corpus class record under `data/corpus/` (CRB/APG/ACG). Do not use a remembered figure; cite the corpus line in the explanation detail. Shapes: Witch/Arcanist = book size (cantrips all; 1st-level base + Int mod at level 1, + per level after); Alchemist/Investigator = formula book (no cantrips; base + Int mod at level 1, + per level); Hunter/Skald = spontaneous spells-known table by class level.
- Test first per class: a Rust test per class (level 1 and a higher level) + a front-end `spellDialogModel.test.ts` case with each class's row ids. Remove the "engine does not count known spells yet" line for these six once the rows exist.

### F3b — Sheet skill rows must read engine totals (lead ruling 2026-10-11, per §29.1 "a magnitude is not wired until it moves on the twin the player reads")
- Finding (lane, verified by reading): `CharacterSheet.tsx:702` sums each skill row on the frontend; only Climb/Intimidate/Swim read engine values. So Meticulous -2, and every existing trait skill bonus (Acrobat +1 etc.), never reach the sheet. The sheet skill rows are a twin (§29.2).
- Change: the sheet's skill rows render the engine's per-skill totals (the `explanations` rows) instead of re-summing; the skills DTO carries them. Smallest change; no layout work (Brief 2). Test first: a sheet skills-model test with Meticulous and with Acrobat; a Rust DTO test.
- Scope added (fifth widening): `apps/desktop/src/characterHub/CharacterSheet.tsx` (skill rows only, around line 702) + its skills model/test files, `apps/desktop/src-tauri/src/pf1_adapter.rs` (skills DTO), `character_hub.rs` (already in scope).

## Required reads
`CreateCharacterForm.tsx` 440–560, 641–730, 1069–1280; `TransferListDialog.tsx`; `transferListModel.ts`; `spellDialogModel.ts`; `spellsPerDayModel.ts`; `manageItems.ts`; `loadCharacterTraits.ts`; `character_hub.rs` `draft_spell_options_impl` (~1817); `class_sorcerer_wizard.rs` ~2184–2419; `class_wizard_prepared_spellbook.rs` ~1295, 1545, 1599; `trait_effects.rs` ~239.

## Allowed write scope
`apps/desktop/src/characterHub/{CreateCharacterForm.tsx,CharacterHubPage.tsx,TransferListDialog.tsx,transferListModel.ts,spellDialogModel.ts,spellsPerDayModel.ts,manageItems.ts,creationRacialTraitsPreview.ts,ManageBox.tsx}` (ManageBox added 2026-10-11 for the F2 `manageLabel` prop) and their `.test.ts`; `apps/desktop/src/boundary/{loadCharacterTraits.ts,draftSpellOptions.ts}`; `apps/desktop/src-tauri/src/character_hub.rs` (trait count check, known-spell rows); `src/rules_core/pilot_compute/{class_sorcerer_wizard.rs,class_wizard_prepared_spellbook.rs}`, `src/rules_core/{trait_effects.rs,skill_allocation.rs}`, `apps/desktop/src-tauri/src/trait_picker.rs` + tests (scope widened 2026-10-11: DTO lives in trait_picker.rs; Meticulous -2 needs skill_allocation.rs); `apps/desktop/scripts/ui-smoke/spec.json` and regenerated `docs/testing/ui-smoke-inventory.md` (`npm run ui-smoke:doc`); `apps/desktop/scripts/ui-smoke/run.mjs` only to add an exact-heading check kind for F1 (scope widened 2026-10-11, third widening); for F7: `src/rules_core/pilot_compute/{class_summoner_witch.rs,class_alchemist_investigator.rs,class_hunter_cavalier_swashbuckler.rs,class_bard_skald.rs}` + their tests (fourth widening, 2026-10-11, operator option 3). Anything else: stop and ask.

Lane note (2026-10-11): the `ui-brief1` worktree branched at 5f020b221b, before the three scope widenings. The lane must `git merge tranche-17-ui` into `ui-brief1` to read this version of the brief.

## Non-goals
No layout rework (Brief 2). No starter-character work (Brief 3). No simulation of spell effects (print-the-rule). No custom-menu work. No drawback effects beyond sheet totals.

## Verification (run ONCE, after all changes)
- `cd apps/desktop && npm run typecheck && npm test`
- Single file while iterating: `npx tsx src/characterHub/spellDialogModel.test.ts`
- `cd apps/desktop/src-tauri && cargo test the_draft_spell_options` (and new Sorcerer/known-spell/trait-count tests), scratch `CARGO_TARGET_DIR`, delete after.
- Root workspace AND `apps/desktop/src-tauri` (separate workspace) must both build.
- Every failing line attributed to its `Running` suite. Four-check Wired Integration audit (`docs/governance/no-stub-mvp-doctrine.md` §Per-cycle audit) into the cycle receipt.
- UNCOVERED by these proofs: real browser rendering (no browser on this box); multiclass casters; Cleric/Druid/Paladin/Ranger known-spell semantics.

## Models
Opus 5.5 for F3/F4/F5 (Rust + engine). Sonnet 5.5 for F2/F6 (well-specified UI). Haiku 4.5 for git/receipt housekeeping. Set `model` explicitly on every dispatch.

## Open questions for the operator
1. (RESOLVED: H2 is visible on screen — diagnose in the running app, F1.)
2. (RESOLVED: drawbacks stay in this brief, F3.)
3. (RESOLVED: Cleric/Druid show read-only per-day text, no picker.)
