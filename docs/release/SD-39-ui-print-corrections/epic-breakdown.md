---
canonical: false
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready — acceptance commands are authored, not yet run against a tree that has the change; each card's first step runs its own command on the pre-change tree and records the RED output
---

# SD-39 Epic Breakdown — UI and Print Corrections

## §0 Parallel / serial map

This table leads the file because the Workflow script reads it (`workflow-instruction.md §2.4`, `§3`).
**Tier** is the `model` each `agent()` sets. **Parallel?** is `yes` only where file sets are disjoint.
Every parallel lane works in its own worktree (`.worktrees/sd39-<lane>`) with its own
`CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd39-<lane>`. **At most 3 cargo lanes.** **Cargo?** says
whether the card holds a cargo slot.

> In a Markdown table cell `\|` is the escape for `|`. Unescape before running a command copied from a
> cell. Paths are repo-root-relative unless the command `cd`s first. A check whose input is missing must
> exit non-zero, never print nothing.

**The main lane is serial.** `CharacterSheet.tsx`, `character_hub.rs`, `main.rs` and `spec.json` are touched by most epics, so E1–E8 run one card at a time in
`.worktrees/sd39-main` unless a row says otherwise. The three allowed side lanes are named below.

| Epic/Cycle | Parallel? | Tier | Cargo? | Notes |
|---|---|---|---|---|
| C0 package | no | lead session | no | this folder, committed on `develop` or on `tranche-19` after C1 |
| C1 cut `tranche-19` + 0.19.0 (14 surfaces) + `current-release.json` | no, first | haiku | no | one commit, 15 files |
| E1.0 slot table + `slot_of`; proof that `repeatable` and tags already carry the data | no | opus | yes | `src/rules_core/body_slots.rs` (new), `data/equipment_types.json`; a converter re-run only if the proof fails (`decisions.md §25`) |
| E1.1 feat budget rows + refusal | no | opus | yes | `src/rules_core/feat_budget.rs` (new), `character_hub.rs` |
| E1.2 duplicate guard | no | opus | yes | `character_hub.rs`; reads `repeatable` |
| E1.3 hide-unmet checkbox | no | opus | yes (npm + command) | `CharacterSheet.tsx` feat picker |
| E1.4 Add Custom chip styling | **yes ∥ E1.1–E1.3** (side lane `ui-a`) | sonnet | no (npm) | `CustomDialog.tsx` + `customChip.ts` + its test; the `spec.json` row landed in E1.5 |
| E1.5 ui-smoke feat rows (discovered by E1.3) | no | sonnet | yes (ui-smoke) | `spec.json` + generated inventory |
| E1.MC | no | opus | yes | adversarial |
| E1.6 feat pools a class grants by `offers`; Hunter counted once (discovered by E1.MC) | no | opus | yes | `src/rules_core/feat_budget.rs`, `character_hub.rs` tests |
| E2.1 registry + Abilities/Details moves + title | no | opus | yes (npm) | `sheetLayout.ts`, `CharacterSheet.tsx` |
| E2.2 combat row 2x2 + 1x4, wrap | no | opus | yes (npm) | same files |
| E2.3 Show Progression drawer shell | no | opus | yes (npm) | `ProgressionDrawer.tsx` (new), `CharacterSheet.tsx` |
| E2.4 ui-smoke rows + inventory | no | sonnet | yes (app build) | `spec.json`, `docs/testing/ui-smoke-inventory.md` |
| E2.MC | no | opus | yes | adversarial |
| E3.1 `list_market` + category fix + `+N` variants | no | opus | yes | `equipment_catalog.rs`, `main.rs`; `+N` from `equipment_modifier/plus1a…`, `plus1w…` |
| E3.2 sell + `+N` purchase on the existing `purchase_equipment` | no | opus | yes | `character_hub.rs`, `boundary/purchaseEquipment.ts`; no new buy command |
| E3.3 Market dialog replaces the three Add buttons | no | opus | yes (npm) | `MarketDialog.tsx` (new), `CharacterSheet.tsx`, `boundary/*.ts`, `spec.json` |
| E3.4 `+N` reaches AC / attack / damage rows, worn or wielded only | no | opus | yes | the eqmod consumer path; `src/rules_core/{equipment_resolver,damage_total}.rs`, `pilot_compute_corpus.rs` only if the proof fails |
| E3.5 SF + CPR Market shell | **yes ∥ E3.4** (side lane `sf-cpr`) | opus | yes | `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`, `sf_adapter.rs`, `cpr_adapter.rs` (currency label); their seed deltas listed |
| E3.MC | no | opus | yes | adversarial |
| E4.1 slot check on the existing `set_equipment_active_state`; wear-if-free on buy | no | opus | yes | `src/rules_core/body_slots.rs`, `character_hub.rs` |
| E4.2 proof per consumer that carried items add weight only | no | opus | yes | the 5 `ActiveState` consumers (F-11), only where a proof fails |
| E4.3 Equipment panel | no | opus | yes (npm) | `EquipmentPanel.tsx` (new), `CharacterSheet.tsx` Gear tab, `boundary/*.ts`, `spec.json` |
| E4.4 custom bonuses + slot | no | opus | yes | `character_custom.rs`, `CustomTab.tsx`, engine consumer for custom bonuses |
| E4.MC | no | opus | yes | adversarial |
| E5.1 ledger file beside `custom.json`, stamped on every add | no | opus | yes | `character_ledger.rs` (new), `character_hub.rs` add paths, `main.rs` |
| E5.2 legacy ledger built once by slot order | no | opus | yes | `character_ledger.rs` |
| E5.3 drawer content from `list_progression_ledger` | no | opus | yes | `character_hub.rs`, `main.rs`, `ProgressionDrawer.tsx`, `boundary/*.ts` |
| E5.MC | no | opus | yes | adversarial |
| E6.1 Overrides toggles + enforcement | no | opus | yes | `character_custom.rs`, `CustomTab.tsx`, every enforcing command |
| E6.2 tagging in UI + DTO | no | opus | yes (npm) | `OverrideTag.tsx` (new), selection DTOs |
| E6.MC | no | opus | yes | adversarial |
| E7.1 `.pcg` reader + mapping + fixtures | **yes ∥ E4–E6** (side lane `ingest`) | opus | yes | `crates/codex-ingest/src/pcg_import/**` (new), `bin/pcg_import.rs` (new), fixtures; no desktop files |
| E7.2 sidecar + Import button + report | no (main lane, after E6.MC and E7.1) | opus | yes | `tauri.sidecar.json` (new), `.github/workflows/publish-tester-release.yml`, `character_hub.rs`, `main.rs`, `LoadCharacterScreen.tsx`, `boundary/importPcg.ts` (new) |
| E7.MC | no | opus | yes | adversarial |
| E8.1 print core: Typst world, fonts, frame, PF layout file, Save PDF command | no | opus | yes | `apps/desktop/src-tauri/src/print/**` (new), `resources/fonts/**` (new), `resources/print/*.typ` (new), `Cargo.toml` |
| E8.2 PF blocks complete + three options | no | opus | yes | `print/blocks/**`, `print/layout/pf1e.rs` |
| E8.3 SF + CPR layout files | no | opus | yes | `print/layout/{sf1e,cpr}.rs`; reads `sf_sheet_print.rs`, `cpr_sheet_print.rs`; **builds FSR-C1's out-dir control if the incident fires (R14)** |
| E8.4 Settings paper + print dialog + Save PDF / Print buttons on 3 sheets + retire CSS print | no | opus | yes (npm) | `settings/**`, `PrintDialog.tsx` (new), the three sheet files, delete `printLayout.ts` + its test, `spec.json` |
| E8.5 golden tests 3×2×4 + overflow-zero + determinism | no | opus | yes (long) | `apps/desktop/src-tauri/tests/print_golden.rs` (new), fixtures |
| E8.MC | no | opus | yes | adversarial |
| E9.1 packaged resources (fonts, sidecar, `.typ`) + clean-checkout build | no | opus | yes | `tauri.conf.json`, `packaged_resources.rs`, `scripts/verify.sh` tauri-resources-tracked block |
| E9.2 fresh-install acceptance (.deb, AppImage) incl. one printed PDF | no | sonnet | yes (long) | SD-l if the box cannot install |
| E9.3 widest verify, both workspaces + npm + `verify.sh --full` + seed deltas | no | sonnet | yes (long) | one pass |
| E9.4 **operator eyeball**: three printed sheets, "usable" ×3 recorded | no | sonnet (prepares), **operator signs** | yes (renders) | `partial` until the operator's line is in the receipt; defects → discovered E8 cards |
| E9.5 final-acceptance scan | no | opus | no | any short → stop, no PR |
| E9.6 retrospective written + cited | no | sonnet | no | |
| E9.7 worktree/branch sweep (`sd39-*` only) | no | haiku | no | |
| E9.8 release notes | no | sonnet | no | removes `SD-39 INTERIM NOTES` |
| E9.9 architecture truth-up + claims critic | no | sonnet + opus | no | two `agent()` calls |
| E9.10 graphify LAST | no | sonnet | no | clean tree, HEAD = `origin/tranche-19` |
| E9.11 PR (final action) + `pr-tests` wait | no | sonnet | no | operator merges |

**Card count: 53** (C 2, E1 8, E2 5, E3 6, E4 5, E5 4, E6 3, E7 3, E8 6, E9 11).
`kanban.md` holds the `diff … && echo SAME_IDS` command proving its rows and this file's IDs match.

**Side lanes (the only parallelism):** `ui-a` (E1.4, `CustomDialog.tsx` + `customChip.ts`), `sf-cpr` (E3.5), `ingest`
(E7.1, `crates/codex-ingest` only). Each is disjoint from the main lane's files by construction; each
lane's `spec.json` row, if any, is added by the next main-lane card that owns `spec.json`, never by the lane.

**Shared-file writer order (one writer at a time, enforced by `Depends on`):**
`CharacterSheet.tsx` E1.3 → E2.1 → E2.2 → E2.3 → E3.3 → E4.3 → E5.3 → E8.4;
`character_hub.rs` E1.1 → E1.2 → E1.6 → E3.1 → E3.2 → E4.1 → E5.1 → E5.3 → E6.1 → E7.2;
`main.rs` E3.1 → E5.1 → E5.3 → E7.2 → E8.1;
`spec.json` E1.4 → E2.4 → E3.3 → E4.3 → E6.2 → E8.4; `docs/testing/ui-smoke-inventory.md` regenerated by each;
`tauri.conf.json` E8.1 → E9.1.

### Cross-bundle section

**No other SD bundle is live.** SD-38 closed (PR #399). Live foreign work on 2026-11-01: none: the merged `.worktrees/fix-perform-install` was removed by operator ruling R20 at launch. `tranche-17-ui` and `tranche-18` remain on origin as merged history; not written to.

---

## Card format

| ID | Criterion | FILES YOU OWN | Acceptance command (must be able to fail) | Does not cover |

Every FILES YOU OWN list is **closed under the change** (R15): the card checks `boundary/*.ts wire()`
for every new request field and any second call site before it starts, and names any missing file in
its receipt as a fence exit.

## Epic C — package and cut

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| C0 | This package is committed with 16 chassis files and the Cycle 0 receipt. | `docs/release/SD-39-ui-print-corrections/**` | `find docs/release/SD-39-ui-print-corrections -maxdepth 2 -name '*.md' \| awk 'END{print NR}'` → 16; `grep -c '^## Cycle 0' docs/release/SD-39-ui-print-corrections/progress.md` → 1 | content review |
| C1 | `tranche-19` exists on origin, cut from `origin/develop` ⊇ `2b14020bc7`; one commit sets `0.19.0` on the 14 surfaces (F-1) and points `current-release.json` at this package's interim notes, which resolve. | the 14 F-1 files; `docs/release/current-release.json` | `git merge-base --is-ancestor 2b14020bc7 origin/tranche-19`; `git show --name-only --format= <sha> \| awk 'NF' \| wc -l` → 15; `grep -rlE '0\.18\.' --include=*.json --include=*.toml --include=*.ts --include=*.yml --include=*.py apps .github tools docs/architecture docs/release/current-release.json \| grep -v node_modules \| grep -v target` → empty; `awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock` → `version = "0.19.0"` (the lockfile's other 0.18.x lines are third-party pins and stay); `python3 -c "import json;d=json.load(open('docs/release/current-release.json'));assert d=={'tranche':19,'notes_path':'docs/release/SD-39-ui-print-corrections/release-notes.md'}"`; `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0; `cd apps/desktop && npx tsx src/release/buildVersionTriple.test.ts && npx tsx src/releaseChecks/buildVersionTriple.test.ts` | the published `0.19.<run>` |

## Epic E1 — guards and the chip fix

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E1.0 | `slot_of(record)` returns a `BodySlot` from the EQSLOT table (converted from `$PCGEN_REPO_DIR/system/gameModes/Pathfinder/equipmentslots.lst` into `data/equipment_types.json`) over the record's `tags`; the book's `Slot:` prose line wins where it disagrees, and "Slot: none" means no slot; rings take 2 of the ring slot. `repeatable` is proved equal to PCGen `MULT:YES` on the pinned feats. `enhanceable(record)` comes from the `Armor`, `Shield`, `Weapon` tags and excludes ammunition, barding and already-magic records. `data/sheet_rules/**` stays unchanged unless the proof fails. | `src/rules_core/body_slots.rs` (new), `src/rules_core/mod.rs`, `data/equipment_types.json`, `crates/codex-ingest/src/bin/gen_equipment_types.rs` + `crates/codex-ingest/src/pcgen_import/equipment_types.rs` (that file's generator; closed by E1.0, `decisions.md §25`), the rest of `crates/codex-ingest/**` + `data/sheet_rules/**` only on a failed proof | `cargo test --locked -j 8 slot_of -- --test-threads=8` asserts the slot of Ring of Protection +1, Headband of Mental Prowess, Hat of Disguise, Amulet of Natural Armor +1, Leather Armor, and no slot for rope; a test lists the 27 tag/prose disagreements (F-12: the 19 + 8 `Slot: none` weapons) and resolves each; the `repeatable` proof prints agree / disagree of N; `bash scripts/verify.sh --only pcgen-residue-gate` PASS | SF data (DEF-2) |
| E1.1 | The engine emits `feats.budget.<pool>.{granted,spent,remaining}` rows for every pool the class catalog and race grant, moving any count the frontend computes today (`ManageBox` "1 of 3 feats remaining", `characterProgression.ts`) onto those rows; `add_feat_selection` refuses when the consumed pool's `remaining` is 0 with an error naming the pool; the Feats tab shows a red "over budget by N" badge and blocks adds for an over-budget save; nothing is deleted. Aldric's pools are re-derived by the card. | `src/rules_core/feat_budget.rs` (new), `src/rules_core/mod.rs`, `apps/desktop/src-tauri/src/character_hub.rs`, `CharacterSheet.tsx` Feats tab, `CreateCharacterForm.tsx`, `characterProgression.ts`, `manageItems.ts`, `classRoster.ts`, `apps/desktop/src-tauri/src/main.rs`, `boundary/draftSheetPreview.ts` (closed by E1.1, `decisions.md §25`) + their tests | `cargo test --locked -j 8 feat_budget -- --test-threads=8` in each workspace (RED first: no module); a test adding one feat past the budget asserts the error contains `pool`; seed deltas listed | bonus pools from archetypes not in the class catalog (counted and listed in the receipt) |
| E1.2 | Adding a feat already held (same id, and same target when targeted) is refused unless `repeatable` (the record's field; a feat that takes a target may be repeated only on a new one); Weapon Focus (longsword) + Weapon Focus (dagger) allowed; Toughness twice refused. | `character_hub.rs` | `cargo test --locked -j 8 feat_duplicate -- --test-threads=8` | — |
| E1.3 | The feat picker has "Hide feats I don't qualify for", default ticked, persisted in app settings; unticked, unmet feats render greyed with the unmet list and a disabled Add. | `CharacterSheet.tsx` (`decisions.md §25`), `ItemPickerModal.tsx`, `itemPickerFilter.ts` and their tests, `settings/**` key, `boundary/*.ts` if a field is added | `cd apps/desktop && npm run typecheck && npm test` with a new test on the picker filter function (unmet hidden when ticked; greyed + disabled when not) | prerequisite evaluation itself (exists: `feat_catalog.rs:33–69`) |
| E1.4 | Add Custom's top-row chips: deselecting returns the chip to its resting style in both themes; exactly one selected; focus ring distinct from selection. | `apps/desktop/src/characterHub/CustomDialog.tsx` + `customChip.ts` (the chips live in the dialog, not CustomTab.tsx; + the shared chip component if one exists), its test | a unit test on the chip class/style function for (selected→deselected) equals the resting style; `npm run typecheck && npm test` | pixel rendering (E9.4 eyeball covers the dialog once) |
| E1.5 | The ui-smoke feat rows reach the picker. `sheet-action-add-feat` removes a feat before clicking Add Feat (a fresh Wizard 1 has no room since E1.1, so the row is green today with no picker open), sets `marker` to the checkbox label, and expects `hidden (prerequisites not met)`. A new row clicks the checkbox and asserts `unavailable (prerequisites not met)` and `Unavailable —`, reopens the picker, asserts the same, and ends unticked (the harness reads the screen once, after the last step). The row adds `allowGlobalForbid: ["failure"]` because feat descriptions contain that word. E1.4's wanted chip row (its receipt) lands here too. | `apps/desktop/scripts/ui-smoke/spec.json`, `docs/testing/ui-smoke-inventory.md` | `cd apps/desktop && RUN_DESKTOP_AGENT=sd39-e1.5 npm run ui-smoke -- --only <the three ids> && npm run ui-smoke:doc`; the step list that ran green is in `artifacts/epic_1/E1.3_cycle_receipt.md` | a level-up feat pick; picking a feat through to the sheet |
| E1.MC | Adversarial re-run of E1.0–E1.5 commands on `origin/tranche-19`; seed deltas explained; Epic report written. | receipts, `progress.md`, `kanban.md` | all E1 commands green; `grep -c 'Epic report — E1' progress.md` → 1 | — |
| E1.6 | `feats.budget.*` counts every feat a class grants, the book's number once. (a) A class whose grant is an `offers` row with no `Pool` target gets a pool: Swashbuckler (`swashbuckler_bonus_feats`) 1 at 4, 5 at 20; Gunslinger (`gunslinger_bonus_feats`) 1 at 4, 5 at 20; each restricted as the book says (combat feats; combat or grit feats: `gunslinger_feat` lists `GunslingerFeat`, `Combat`, `Grit`, and `FeatPool::admits` reads a tag set as all-of, so the card settles any-of for this list). The same shape on an archetype (`daring_infiltrator_bonus_feats`, `battle_host_bonus_feats`) is read by the same rule, and a feat's own chooser (`martial_versatility`, `point_blank_master`) and a temporary pick (`martial_flexibility_feat`) are not pools. (b) Hunter reads 1 at 3 and 6 at 18, not 2 and 12: `hunter_teamwork_feats#bonus6..11` repeat `#bonus0..5` with the same target, value and gate. (c) A grant the record holds only as prose and `granted_by` gates gets its number from those gates or is listed by name in the receipt: Dragon Disciple (`dragon_disciple_bloodline_feat`, 2nd, 5th, 8th), Sentinel (`sentinel_bonus_feat`, 2nd, 7th); both read `{}` today. `data/sheet_rules/**` stays unchanged. Seed deltas: none expected. | `src/rules_core/feat_budget.rs`, `apps/desktop/src-tauri/src/character_hub.rs` (tests only) | `cargo test --locked -j 8 feat_budget -- --test-threads=8` in each workspace (RED first, from `artifacts/epic_1/E1.MC/probe-tests.rs.txt`): the numbers above; an Elf Swashbuckler 4 with its seeds takes Toughness and Improved Initiative, and a third non-combat feat is refused naming the pool | the 82 pools of `E1.1/feat-pools-census.txt` no census class holds; the level-up prompt on a Swashbuckler's or Fighter's even level (`levelGrantsFeat`'s known gap, E1.1's receipt) |

## Epic E2 — layout

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E2.1 | `sheetLayout.ts` registers `leftRail`, `combatRow`, `rightColumn`; the left rail holds portrait, Level/XP, Class, **Character Details** (moved), **Show Progression** button; the right column holds **Abilities** (moved) then Skills; the left-rail heading reads "Character Details"; `CharacterSheet.tsx` renders every region from the registry. | `sheetLayout.ts`, `sheetLayout.test.ts`, `CharacterSheet.tsx`, `panels/squareBlock.test.ts` (its source-order check reads `SHEET_COLUMNS.map`; closed by E2.1, `decisions.md §25`) | `sheetLayout.test.ts` asserts the box→region map above; `grep -c '>Progression<\|Progression$' CharacterSheet.tsx` → 0 for the heading; `npm run typecheck && npm test` | pixel layout |
| E2.2 | The 2x2 (HP / Initiative / Encumbrance / Conditions) and a 1x4 (Armor Class / Attack / Speed / Saving Throws) share one `combatRow`; below the breakpoint the 1x4 wraps under. | `sheetLayout.ts`, `CharacterSheet.tsx` | `sheetLayout.test.ts` asserts `combatRow` order and the wrap style token; `npm test` | pixel layout |
| E2.3 | Show Progression opens a left drawer (`role="dialog"`, `aria-modal`) with the former rail content (levels, grants, next-level text); closes by X, click-outside, Escape; focus returns to the button. | `ProgressionDrawer.tsx` (new), `CharacterSheet.tsx` | a unit test on the drawer's open/close reducer (three close paths); `npm test`; ui-smoke row in E2.4 | ledger content (E5.3) |
| E2.4 | ui-smoke rows for: title text, Abilities in right column, Details in left rail, drawer open/close; `load-seed-wizard-fireball` expects `Skill points: 7`, which only the level history prints and which sits behind Show Progression since E2.1, so that row opens it or stops expecting it; inventory regenerated; `npm run ui-smoke` on the new rows green; the full-suite red-row list recorded (FSR-C2). | `spec.json`, `docs/testing/ui-smoke-inventory.md` | `RUN_DESKTOP_AGENT=sd39-e2.4 npm run ui-smoke -- --only <new ids>` green; `npm run ui-smoke:doc && git status --porcelain docs/testing/ui-smoke-inventory.md` empty | pre-existing red rows |
| E2.MC | Adversarial; PF seed JSON unchanged by E2 (layout only); Epic report. | receipts | PF render-hash diff empty; E2 commands green | — |

## Epic E3 — Market

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E3.1 | `list_market(MarketFilter)` returns only the requested category (the Armor list has 0 ammunition, weapon or rod records); for `enhanceable` PF records it offers `+1…+5` with the price `base + masterwork + N²×(1000 armor/shield \| 2000 weapon)` gp; the offer maps to the existing eqmods (`plus1a`, `plus1w` …) so one code path attaches the bonus. | `equipment_catalog.rs`, `src/rules_core/body_slots.rs` (`enhanceable`), `main.rs` (register), `boundary/listMarket.ts` (new) | `cargo test --locked -j 8 market_category_filter -- --test-threads=8` and `cargo test --locked -j 8 enhancement_price_table -- --test-threads=8`; a count test: Armor category cross-type records = 0 of N | special abilities (DEF-1) |
| E3.2 | `purchase_equipment` (exists: atomic, pre-checks money, honours `PriceMode`) accepts an optional `+N` and attaches the eqmod in the same save at the E3.1 price; selling uses `remove_equipment_selection` (exists: 50% / 100% / 0 by mode); Cashless moves no money; a buy you cannot afford is refused. | `character_hub.rs`, `boundary/purchaseEquipment.ts` | `cargo test --locked -j 8 market_buy_sell -- --test-threads=8` (9 cases: 3 modes × buy ok / buy refused / sell) | — |
| E3.3 | One **Market** button on Weapons, Defense and Gear tabs; Add Weapon / Add Armor / Add Gear are gone; the dialog has category tabs, search, `+N` picker, price with currency label, Buy, and a Sell action on inventory rows; price-mode control reused. | `MarketDialog.tsx` (new), `CharacterSheet.tsx`, `itemPickerFilter.ts`, `spec.json` | `grep -c 'Add Weapon\|Add Armor\|Add Gear' apps/desktop/src/characterHub/CharacterSheet.tsx` → 0; ui-smoke rows for open Market, filter Armor, buy, sell green; `npm run typecheck && npm test` | SF/CPR (E3.5) |
| E3.4 | A worn `+N` armor or shield adds an `enhancement` AC row; a wielded `+N` weapon adds `enhancement` to attack and damage rows; carried or unwielded `+N` items add nothing. First prove what the existing eqmod path already does; change only what fails. | the eqmod consumer path; `src/rules_core/{equipment_resolver,damage_total}.rs`, `pilot_compute_corpus.rs` on a failed proof | `cargo test --locked -j 8 enhancement_rows -- --test-threads=8`: Aldric + longsword `+1` wielded → attack +1, damage +1; stowed → +0 | — |
| E3.5 | Starfinder and Cyberpunk sheets get the same Market button and dialog with their catalogs and currency (credits / eddies); their Add buttons are gone; no `+N` picker; their seed deltas listed. | `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`, `sf_adapter.rs`, `cpr_adapter.rs`, `sf_catalog.rs`, `cpr_catalog.rs` | `grep -c 'Add Weapon\|Add Armor\|Add Gear' apps/desktop/src/characterHub/{Starfinder,Cyberpunk}CharacterSheet.tsx` → 0; SF + CPR seed outputs diffed and every delta listed | SF/CPR slots (DEF-2) |
| E3.MC | Adversarial; every tab on every system still has an add path (R-11); Epic report. | receipts | E3 commands green; a grep that each sheet file references `MarketDialog` | — |

## Epic E4 — slots and equipping

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E4.1 | `set_equipment_active_state` (exists) refuses `EquippedActive` when the slot is held, with `"<slot> slot is taken by <item>"`; two rings allowed, a third refused; shields take `shield` + off hand. `purchase_equipment` from Market wears the item if its slot is free, else leaves it carried and returns the reason; an item with no slot is carried (operator ruling R18). Weapons: main / off / two-handed, from the settled `wield_category` on the corpus equipment record (`slot_of` reads only the `TwoHanded` tag and returns `wield_main` otherwise; F-14). A `Slot: none` item, such as a weapon rod, never meets a slot refusal. | `src/rules_core/body_slots.rs`, `character_hub.rs`, `boundary/setEquipmentActiveState.ts`, `CharacterSheet.tsx` caller at `handleAddEquipment` | `cargo test --locked -j 8 slot_conflict -- --test-threads=8` (helmet×2 refused; ring×2 ok; ring×3 refused; bought helmet with head free = worn; with head held = carried + reason) | the Equipment panel (E4.3) |
| E4.2 | Proof, then fix only what fails: only `EquippedActive` items feed AC, attack, saves, ability and skill bonuses, and weight counts both states. The stow control and its engine filter already exist (`CharacterSheet.tsx:1642`, `pilot_compute_corpus.rs:280`). Saved items keep their saved state; no saved sheet changes. | the 5 F-11 files on a failed proof | `cargo test --locked -j 8 worn_vs_carried -- --test-threads=8`: Aldric AC with armor stowed = AC with no armor; a stowed Ring of Protection +1 adds 0, worn adds 1; weight equal; FSR-C3 "29 lb" cause recorded | — |
| E4.3 | Gear tab has an **Equipment** control: Worn (by slot) and Carried lists built on the existing stow and re-equip actions, the refusal message shown verbatim, the override tag. | `EquipmentPanel.tsx` (new), `CharacterSheet.tsx`, `spec.json` | ui-smoke rows: equip, conflict message, unequip; `npm run typecheck && npm test` | — |
| E4.4 | Custom items may declare `bonuses[] {target, bonus_type, value}` and an optional `slot`, reusing the target vocabulary `CustomGrantDto` already has where it fits; a worn custom item with `ac +1 deflection` raises AC by 1; carried adds 0; a text-only custom item is unchanged. | `character_custom.rs`, `CustomTab.tsx`, the engine consumer of custom bonuses, `boundary/*.ts` | `cargo test --locked -j 8 custom_item_bonuses -- --test-threads=8`; `npm test` on the CustomTab form model | — |
| E4.MC | Adversarial; Epic report. | receipts | E4 commands green | — |

## Epic E5 — level ledger

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E5.1 | A `ledger.json` beside `custom.json` (the `character_custom.rs` pattern) records `{kind, id, target, taken_at_level, assigned_on_upgrade}` for each feat, spell, skill rank and ability increase; every add and remove path keeps it in step. Engine input (`selected_feats: Vec<String>`) does not change, so no saved-character schema changes. | `character_ledger.rs` (new), `character_hub.rs` add paths, `main.rs` | `cargo test --locked -j 8 ledger_stamp -- --test-threads=8`; a character with no ledger file still loads | — |
| E5.2 | On first load with no `ledger.json`, the ledger is built by slot order (R8) with `assigned_on_upgrade = true`; extras past the budget get `None` (SD-o); running it twice gives the same file; the character's own files are read, never rewritten. | `character_ledger.rs` | `cargo test --locked -j 8 ledger_migration -- --test-threads=8` (Elowen fixture; run twice = same; sha256 of the saved character file unchanged) | — |
| E5.3 | `list_progression_ledger` returns per level: class level, feats (with targets), spells learned, ranks spent, class features, ability increase, unspent slots, `assigned_on_upgrade`; the drawer renders it; "next level grants" stays. | `character_hub.rs`, `main.rs`, `ProgressionDrawer.tsx`, `boundary/listProgressionLedger.ts` (new), `characterProgression.ts` | `cargo test --locked -j 8 progression_ledger -- --test-threads=8`; ui-smoke row: drawer shows "Level 1" and a feat name for Aldric | editing (DEF-6) |
| E5.MC | Adversarial; Epic report. | receipts | E5 commands green | — |

## Epic E6 — overrides

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E6.1 | Custom tab has an **Overrides** group with the eight toggles (`decisions.md §10`), saved on the character; each enforcing command admits the matching case when its flag is on and refuses when off. | `character_custom.rs`, `CustomTab.tsx`, `character_hub.rs`, `equipment_resolver.rs` (proficiency), spell add path, `boundary/*.ts` | `cargo test --locked -j 8 overrides_ -- --test-threads=8` (16 cases: 8 flags × on/off) | — |
| E6.2 | Every record admitted under an override carries `override: Some(flag)`; the UI shows `OverrideTag`; the DTO reaches the print model. | `OverrideTag.tsx` (new), selection DTOs, the Feats/Gear/Spells tabs, `spec.json` | ui-smoke row: admit a feat without prereqs → tag visible; `npm test` | the printed mark (E8.2 consumes) |
| E6.MC | Adversarial; Epic report. | receipts | E6 commands green | — |

## Epic E7 — `.pcg` import

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E7.1 | `pcg_import <file> --out <json>` converts every `.pcg` fixture (F-16) to the JSON `import_character` already accepts: race, classes+levels, stats, feats+targets, ranks, spells, equipment with location → worn or carried, money, languages, alignment, deity, name/bio; unknown keys → custom items + a report; a mapping table doc. Reuse any `.pcg` reading `crates/codex-ingest/src/oracle_validation/` holds. | `crates/codex-ingest/src/pcg_import/**` (new), `crates/codex-ingest/src/bin/pcg_import.rs` (new), `crates/codex-ingest/Cargo.toml`, `docs/architecture/` import note | `cargo test --locked -p codex-ingest -j 8 pcg_import_fixtures -- --test-threads=8`; `bash scripts/verify.sh --only pcgen-residue-gate --only crate-wall` PASS | PCGen XML (DEF-3) |
| E7.2 | The binary ships via `tauri.sidecar.json` (`bundle.externalBin`), passed as `--config` only at bundling, because `tauri-build` fails when an `externalBin` is absent and that would break every `cargo test`. All three publish jobs (Linux, macOS, Windows) build `pcg_import` first and copy it under its target-triple name. `LoadCharacterScreen.tsx` Import offers "Codex file \| PCGen file"; a `.pcg` runs the sidecar with `std::process::Command` (no new plugin), goes through `import_character`, loads, and the report is shown; a missing sidecar gives a named error. `src/` and `apps/desktop` contain no `.pcg` parser. | `apps/desktop/src-tauri/tauri.sidecar.json` (new), `.github/workflows/publish-tester-release.yml`, `character_hub.rs`, `main.rs`, `LoadCharacterScreen.tsx`, `boundary/importPcg.ts` (new) | `grep -rln 'pcg' src apps/desktop/src apps/desktop/src-tauri/src` lists only the sidecar caller and its boundary, each read by hand in the receipt; a Rust test runs a built `pcg_import` on a fixture through the import path; ui-smoke row: import fixture → character listed; `npx tauri build --config tauri.sidecar.json --bundles deb` | Windows and macOS runs (CI at publish) |
| E7.MC | Adversarial; Epic report. | receipts | E7 commands green | — |

## Epic E8 — print

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E8.1 | `print/` module: Typst world with bundled OFL fonts only, frame template, `SheetDocument`, PF layout file with Identity + Combat + Abilities blocks, `save_sheet_pdf` command writing a real PDF for Aldric on A4 and Letter; versions recorded. | `apps/desktop/src-tauri/src/print/**` (new), `resources/fonts/**` (new), `resources/print/*.typ` (new), `apps/desktop/src-tauri/Cargo.toml`, `main.rs` | P-4; `cargo test --locked -j 8 print_smoke -- --test-threads=8` asserts a PDF > 1 KiB whose extracted text contains "Aldric" and "Armor Class"; two runs byte-equal (NR-3) | the other blocks |
| E8.2 | All 15 blocks for PF; the three options: Spellbook (full text per known spell), Show the math (every explanations row behind a printed value, "(no derivation recorded)" where none), Bio and campaign notes (the character bio + the campaign's description, the fields that exist, operator ruling R17); override tags printed in "full". | `print/blocks/**`, `print/layout/pf1e.rs` | `cargo test --locked -j 8 print_pf_blocks -- --test-threads=8`: extracted text contains each block heading in layout order; option on/off changes page count; `(no derivation recorded)` count per seed listed | — |
| E8.3 | SF and CPR layout files from their printed-lines modules; both render all seeds on both papers. | `print/layout/{sf1e,cpr}.rs`, `sf_sheet_print.rs` / `cpr_sheet_print.rs` only if the out-dir control is built (R14) | `cargo test --locked -j 8 print_sf -- --test-threads=8` and `cargo test --locked -j 8 print_cpr -- --test-threads=8` | — |
| E8.4 | Settings → Print paper (A4 default, Letter); a Print dialog on all three sheets with paper override and the three checkboxes; **Save PDF** (save-as, then opens) and **Print** (writes a temp PDF and opens it in the system PDF viewer, operator ruling R19); `printLayout.ts` + test deleted; no `window.print`. | `settings/**`, `PrintDialog.tsx` (new), the three sheet `.tsx`, delete `printLayout.ts`, `printLayout.test.ts`, `boundary/saveSheetPdf.ts` (new), `spec.json` | `grep -rn 'window.print\|printLayout' apps/desktop/src \| awk 'END{print NR}'` → 0; ui-smoke rows: dialog opens, Save PDF produces a file; `npm run typecheck && npm test` | — |
| E8.5 | Golden suite: 3 systems × 2 papers × 4 option sets = 24 PDFs per run; each asserts value presence, heading order, page count, Typst diagnostics = 0, determinism. | `apps/desktop/src-tauri/tests/print_golden.rs` (new), fixtures | `cargo test --locked -j 8 --test print_golden -- --test-threads=8` → 24 passed | pixels (DEF-4) |
| E8.MC | Adversarial: NR-2 grep, every sheet value on screen appears in the basic PDF for each seed; Epic report. | receipts | `grep -n 'fn compute\|calc(' apps/desktop/src-tauri/src/print/` → 0; E8 commands green | — |

## Epic E9 — closure

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E9.1 | Fonts and `.typ` templates are in `bundle.resources`, asserted in `packaged_resources.rs`; the sidecar config is asserted present; `verify.sh` tauri-resources-tracked knows them; a clean-checkout `tauri build --config tauri.sidecar.json` succeeds. | `tauri.conf.json`, `packaged_resources.rs`, `scripts/verify.sh` | `cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources`; `bash scripts/verify.sh --only tauri-resources-tracked` PASS; `ls` the bundle output | — |
| E9.2 | Fresh install (.deb and AppImage, clean profile): Settings → Update → Check shows no "unknown"; open a seed, Save PDF works and the PDF opens. | receipt | observed values pasted; SD-l if impossible | — |
| E9.3 | Widest verify once on the final code: root + desktop `cargo test`, `npm run typecheck && npm test && npm run ui-smoke:doc`, `bash scripts/verify.sh --full --show-actuals -j 8`; every FAILED attributed; PF/SF/CPR seed deltas listed. | receipt, `scripts/verify-baselines.env` if floors move | all green or attributed; `grep -c 'FAIL' <log>` → 0 | — |
| E9.4 | **Operator gate.** Three PDFs (Aldric PF full, SF-Soldier-3 basic, Streetrat Solo basic) saved to `artifacts/epic_9/E9.4/`; the operator prints them and writes `usable: yes` ×3 (or the defects) in the receipt. | `artifacts/epic_9/E9.4/**` | `grep -c 'usable: yes' artifacts/epic_9/E9.4_cycle_receipt.md` → 3 | — |
| E9.5 | Final-acceptance scan (`workflow-instruction.md §11` step 1): 53 rows, every non-exempt row `complete`, every DEF/FSR revisit command run and pasted. | `progress.md` | the §11 awk prints nothing | — |
| E9.6 | `docs/retro/sd39-retrospective.md` written from `retro.py summary --since 2026-11-01 --json`, cited from `references/README.md` in the same commit. | `docs/retro/sd39-retrospective.md`, `references/README.md` | `grep -c sd39-retrospective references/README.md` ≥ 1 | — |
| E9.7 | Every `sd39-*` worktree and `sd39/*` branch except `sd39-main` removed, every `cargo-target/sd39-*` except `sd39-main` deleted; counts found/removed. The script's last step removes `sd39-main` and its target dir after the PR. | — | `git worktree list \| grep -c sd39` → 1; `ls /home/ubuntu/cargo-target \| grep -c sd39` ≤ 1 | — |
| E9.8 | `release-notes.md` final body; sentinel gone; resolver passes. | `release-notes.md` | `grep -c 'SD-39 INTERIM NOTES' release-notes.md` → 0; `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0; `python3 scripts/tranche/validate-tranche-notes.py --release-notes-path docs/release/SD-39-ui-print-corrections/release-notes.md` | — |
| E9.9 | Architecture truth-up receipt + Opus claims critic. | `docs/architecture/**`, `receipts.md` | `grep -c 'architecture-truth-up' receipts.md` ≥ 1 | — |
| E9.10 | Graphify last, clean tree, HEAD = `origin/tranche-19`; indexed SHA recorded; node-count guard respected. | graph outputs, `receipts.md` | `git status --porcelain \| wc -l` → 0 before; `grep -c 'graphify:update' receipts.md` ≥ 1 | — |
| E9.11 | PR `tranche-19 → develop` opened as the final action; `pr-tests` green; the operator merges. | `kanban.md` row | `gh pr view <n> --json state,statusCheckRollup` | the merge |
