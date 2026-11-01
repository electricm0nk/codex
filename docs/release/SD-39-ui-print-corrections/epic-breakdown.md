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

**The main lane is serial.** `CharacterSheet.tsx`, `character_hub.rs`, `main.rs`, `spec.json` and
`src/saved_character/mod.rs` are touched by most epics, so E1–E8 run one card at a time in
`.worktrees/sd39-main` unless a row says otherwise. The three allowed side lanes are named below.

| Epic/Cycle | Parallel? | Tier | Cargo? | Notes |
|---|---|---|---|---|
| C0 package | no | lead session | no | this folder, committed on `develop` or on `tranche-19` after C1 |
| C1 cut `tranche-19` + 0.19.0 (14 surfaces) + `current-release.json` | no, first | haiku | no | one commit, 15 files |
| E1.0 converter: `stackable`, `slot`, `enhanceable` + ONE re-run | no | opus | yes | `crates/codex-ingest`, `data/sheet_rules/**`, `data/equipment_types.json`; the only data re-run in SD-39 |
| E1.1 feat budget rows + refusal | no | opus | yes | `src/rules_core/feat_budget.rs` (new), `character_hub.rs` |
| E1.2 duplicate guard | no | opus | yes | `character_hub.rs`; reads `stackable` |
| E1.3 hide-unmet checkbox | no | opus | yes (npm + command) | `CharacterSheet.tsx` feat picker |
| E1.4 Add Custom chip styling | **yes ∥ E1.1–E1.3** (side lane `ui-a`) | sonnet | no (npm) | `CustomTab.tsx` only + its test + `spec.json` row (held to the end of E1 by the spec.json writer order) |
| E1.MC | no | opus | yes | adversarial |
| E2.1 registry + Abilities/Details moves + title | no | opus | yes (npm) | `sheetLayout.ts`, `CharacterSheet.tsx` |
| E2.2 combat row 2x2 + 1x4, wrap | no | opus | yes (npm) | same files |
| E2.3 Show Progression drawer shell | no | opus | yes (npm) | `ProgressionDrawer.tsx` (new), `CharacterSheet.tsx` |
| E2.4 ui-smoke rows + inventory | no | sonnet | yes (app build) | `spec.json`, `docs/testing/ui-smoke-inventory.md` |
| E2.MC | no | opus | yes | adversarial |
| E3.1 `list_market` + category fix + `+N` variants | no | opus | yes | `equipment_catalog.rs`, `src/rules_core/enhancement.rs` (new), `main.rs` |
| E3.2 `buy_item` / `sell_item` + price modes | no | opus | yes | `character_hub.rs`, `main.rs` |
| E3.3 Market dialog replaces the three Add buttons | no | opus | yes (npm) | `MarketDialog.tsx` (new), `CharacterSheet.tsx`, `boundary/*.ts`, `spec.json` |
| E3.4 enhancement bonus → AC / attack / damage rows | no | opus | yes | `src/rules_core/{enhancement,equipment_resolver,damage_total}.rs`, `pilot_compute_corpus.rs` |
| E3.5 SF + CPR Market shell | **yes ∥ E3.4** (side lane `sf-cpr`) | opus | yes | `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`, `sf_adapter.rs`, `cpr_adapter.rs` (currency label); their seed deltas listed |
| E3.MC | no | opus | yes | adversarial |
| E4.1 `BodySlot` + equip/unequip engine | no | opus | yes | `src/rules_core/body_slots.rs` (new), `character_input.rs`, `character_hub.rs`, `main.rs` |
| E4.2 worn vs carried consumption + legacy items stay worn | no | opus | yes | the 5 `ActiveState` consumers (F-11), `local_store.rs` migration |
| E4.3 Equipment panel | no | opus | yes (npm) | `EquipmentPanel.tsx` (new), `CharacterSheet.tsx` Gear tab, `boundary/*.ts`, `spec.json` |
| E4.4 custom bonuses + slot | no | opus | yes | `character_custom.rs`, `CustomTab.tsx`, engine consumer for custom bonuses |
| E4.MC | no | opus | yes | adversarial |
| E5.1 `taken_at_level` stamp + schema version | no | opus | yes | `src/saved_character/mod.rs`, `character_hub.rs` add paths |
| E5.2 legacy migration by slot order | no | opus | yes | `local_store.rs`, `src/saved_character/migrate_ledger.rs` (new) |
| E5.3 drawer content from `list_progression_ledger` | no | opus | yes | `character_hub.rs`, `main.rs`, `ProgressionDrawer.tsx`, `boundary/*.ts` |
| E5.MC | no | opus | yes | adversarial |
| E6.1 Overrides toggles + enforcement | no | opus | yes | `character_custom.rs`, `CustomTab.tsx`, every enforcing command |
| E6.2 tagging in UI + DTO | no | opus | yes (npm) | `OverrideTag.tsx` (new), selection DTOs |
| E6.MC | no | opus | yes | adversarial |
| E7.1 `.pcg` reader + mapping + fixtures | **yes ∥ E4–E6** (side lane `ingest`) | opus | yes | `crates/codex-ingest/src/pcg_import/**` (new), `bin/pcg_import.rs` (new), fixtures; no desktop files |
| E7.2 sidecar + Import button + report | no (main lane, after E6.MC and E7.1) | opus | yes | `tauri.conf.json` (`externalBin`), `character_hub.rs`, `main.rs`, Import UI, `packaged_resources.rs` |
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
| E9.7 worktree/branch sweep (`sd39-*` only) | no | haiku | no | never the peer's `fix-perform-install` |
| E9.8 release notes | no | sonnet | no | removes `SD-39 INTERIM NOTES` |
| E9.9 architecture truth-up + claims critic | no | sonnet + opus | no | two `agent()` calls |
| E9.10 graphify LAST | no | sonnet | no | clean tree, HEAD = `origin/tranche-19` |
| E9.11 PR (final action) + `pr-tests` wait | no | sonnet | no | operator merges |

**Card count: 51** (C 2, E1 6, E2 5, E3 6, E4 5, E5 4, E6 3, E7 3, E8 6, E9 11).
`kanban.md` holds the `diff … && echo SAME_IDS` command proving its rows and this file's IDs match.

**Side lanes (the only parallelism):** `ui-a` (E1.4, `CustomTab.tsx` only), `sf-cpr` (E3.5), `ingest`
(E7.1, `crates/codex-ingest` only). Each is disjoint from the main lane's files by construction; each
lane's `spec.json` row, if any, is added by the next main-lane card that owns `spec.json`, never by the lane.

**Shared-file writer order (one writer at a time, enforced by `Depends on`):**
`CharacterSheet.tsx` E1.3 → E2.1 → E2.2 → E2.3 → E3.3 → E4.3 → E5.3 → E8.4;
`character_hub.rs` E1.1 → E1.2 → E3.2 → E4.1 → E5.1 → E5.3 → E6.1 → E7.2;
`main.rs` E3.1 → E3.2 → E4.1 → E5.3 → E7.2 → E8.1;
`spec.json` E1.4 → E2.4 → E3.3 → E4.3 → E6.2 → E8.4; `docs/testing/ui-smoke-inventory.md` regenerated by each;
`src/saved_character/mod.rs` E4.1 → E5.1 → E5.2 → E6.1; `tauri.conf.json` E7.2 → E8.1 → E9.1.

### Cross-bundle section

**No other SD bundle is live.** SD-38 closed (PR #399). Live foreign work on 2026-11-01: worktree
`.worktrees/fix-perform-install` (branch `fix/perform-install-args`, another session) — never touched
(SD-f). `tranche-17-ui` and `tranche-18` remain on origin as merged history; not written to.

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
| C1 | `tranche-19` exists on origin, cut from `origin/develop` ⊇ `2b14020bc7`; one commit sets `0.19.0` on the 14 surfaces (F-1) and points `current-release.json` at this package's interim notes, which resolve. | the 14 F-1 files; `docs/release/current-release.json` | `git merge-base --is-ancestor 2b14020bc7 origin/tranche-19`; `git show --name-only --format= <sha> \| awk 'NF' \| wc -l` → 15; `grep -rln '0\.18\.' <F-1 globs> \| grep -v test_resolve_release_notes` → empty; `python3 -c "import json;d=json.load(open('docs/release/current-release.json'));assert d=={'tranche':19,'notes_path':'docs/release/SD-39-ui-print-corrections/release-notes.md'}"`; `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0; `cd apps/desktop && npx tsx src/release/buildVersionTriple.test.ts && npx tsx src/releaseChecks/buildVersionTriple.test.ts` | the published `0.19.<run>` |

## Epic E1 — guards and the chip fix

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E1.0 | Every PF equipment record whose PCGen source carries `SLOTS:` has `slot` ∈ `BodySlot`; every feat with `MULT:YES` has `stackable: true`; armor/shield/weapon records carry `enhanceable`; one converter run; `--check` passes for both systems; the data diff touches only those three keys. | `crates/codex-ingest/src/pcgen_import/{lst_parser/equipment.rs,lst_parser/feat*.rs,sheet_rule/convert.rs,sheet_rule/table.rs}`, `data/sheet_rules/**`, `data/equipment_types.json`, `docs/architecture/` schema note | F-13/F-14/F-15 commands with values written back to CUI; `cargo run --locked -p codex-ingest --bin sheet_rule_convert -- --check` and `-- --system starfinder-1e --check` exit 0; `git diff --name-only <base>..HEAD -- data \| xargs -I{} python3 scripts/<new>/json_key_diff.py <base> {} \| sort -u` prints only `slot`, `stackable`, `enhanceable`; `bash scripts/verify.sh --only pcgen-residue-gate` PASS | SF data (DEF-2) |
| E1.1 | The engine emits `feats.budget.<pool>.{granted,spent,remaining}` rows for every pool the class catalog and race grant; `add_feat_selection` refuses when the consumed pool's `remaining` is 0 with an error naming the pool; Aldric (Fighter 3) shows level pool 2/2 and combat pool 2/2 (re-derived by the card, not copied). | `src/rules_core/feat_budget.rs` (new), `src/rules_core/mod.rs`, `apps/desktop/src-tauri/src/character_hub.rs`, `CharacterSheet.tsx` Feats tab badge | `cargo test --locked -j 8 feat_budget -- --test-threads=8` (RED first: no module); a test adding a 5th feat to a 4-slot character asserts the error contains `pool`; seed deltas listed | bonus pools from archetypes not in the class catalog (counted and listed in the receipt) |
| E1.2 | Adding a feat already held (same id, and same target when targeted) is refused unless `stackable`; Weapon Focus (longsword) + Weapon Focus (dagger) allowed; Toughness twice refused. | `character_hub.rs` | `cargo test --locked -j 8 feat_duplicate -- --test-threads=8` | — |
| E1.3 | The feat picker has "Hide feats I don't qualify for", default ticked, persisted in app settings; unticked, unmet feats render greyed with the unmet list and a disabled Add. | `CharacterSheet.tsx` picker, `settings/**` key, `boundary/*.ts` if a field is added | `cd apps/desktop && npm run typecheck && npm test` with a new test on the picker filter function (unmet hidden when ticked; greyed + disabled when not) | prerequisite evaluation itself (exists: `feat_catalog.rs:33–69`) |
| E1.4 | Add Custom's top-row chips: deselecting returns the chip to its resting style in both themes; exactly one selected; focus ring distinct from selection. | `apps/desktop/src/characterHub/CustomTab.tsx` (+ the shared chip component if one exists), its test | a unit test on the chip class/style function for (selected→deselected) equals the resting style; `npm run typecheck && npm test` | pixel rendering (E9.4 eyeball covers the dialog once) |
| E1.MC | Adversarial re-run of E1.0–E1.4 commands on `origin/tranche-19`; seed deltas explained; Epic report written. | receipts, `progress.md`, `kanban.md` | all E1 commands green; `grep -c 'Epic report — E1' progress.md` → 1 | — |

## Epic E2 — layout

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E2.1 | `sheetLayout.ts` registers `leftRail`, `combatRow`, `rightColumn`; the left rail holds portrait, Level/XP, Class, **Character Details** (moved), **Show Progression** button; the right column holds **Abilities** (moved) then Skills; the left-rail heading reads "Character Details"; `CharacterSheet.tsx` renders every region from the registry. | `sheetLayout.ts`, `sheetLayout.test.ts`, `CharacterSheet.tsx` | `sheetLayout.test.ts` asserts the box→region map above; `grep -c '>Progression<\|Progression$' CharacterSheet.tsx` → 0 for the heading; `npm run typecheck && npm test` | pixel layout |
| E2.2 | The 2x2 (HP / Initiative / Encumbrance / Conditions) and a 1x4 (Armor Class / Attack / Speed / Saving Throws) share one `combatRow`; below the breakpoint the 1x4 wraps under. | `sheetLayout.ts`, `CharacterSheet.tsx` | `sheetLayout.test.ts` asserts `combatRow` order and the wrap style token; `npm test` | pixel layout |
| E2.3 | Show Progression opens a left drawer (`role="dialog"`, `aria-modal`) with the former rail content (levels, grants, next-level text); closes by X, click-outside, Escape; focus returns to the button. | `ProgressionDrawer.tsx` (new), `CharacterSheet.tsx` | a unit test on the drawer's open/close reducer (three close paths); `npm test`; ui-smoke row in E2.4 | ledger content (E5.3) |
| E2.4 | ui-smoke rows for: title text, Abilities in right column, Details in left rail, drawer open/close; inventory regenerated; `npm run ui-smoke` on the new rows green; the full-suite red-row list recorded (FSR-C2). | `spec.json`, `docs/testing/ui-smoke-inventory.md` | `RUN_DESKTOP_AGENT=sd39-e2.4 npm run ui-smoke -- --rows <new ids>` green; `npm run ui-smoke:doc && git status --porcelain docs/testing/ui-smoke-inventory.md` empty | pre-existing red rows |
| E2.MC | Adversarial; PF seed JSON unchanged by E2 (layout only); Epic report. | receipts | PF render-hash diff empty; E2 commands green | — |

## Epic E3 — Market

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E3.1 | `list_market(MarketFilter)` returns only the requested category (Armor list has 0 ammunition/weapon/rod records); for `enhanceable` PF records it returns `+1…+5` variants with CRB prices (NR-8) and ids `<base>+<N>`. | `equipment_catalog.rs`, `src/rules_core/enhancement.rs` (new), `main.rs` (register), `boundary/listMarket.ts` (new) | `cargo test --locked -j 8 market_category_filter enhancement_price_table -- --test-threads=8`; a count test: Armor category cross-type records = 0 of N | special abilities (DEF-1) |
| E3.2 | `buy_item` deducts price in Standard/CharacterBuild, refuses `Insufficient funds` when `gold < price`, optional `equip_now`; `sell_item` credits 50% / 100% / 0 by mode; Cashless never moves gold. | `character_hub.rs`, `main.rs`, `boundary/{buyItem,sellItem}.ts` (new) | `cargo test --locked -j 8 market_buy_sell -- --test-threads=8` (9 cases: 3 modes × buy ok / buy refused / sell) | — |
| E3.3 | One **Market** button on Weapons, Defense and Gear tabs; Add Weapon / Add Armor / Add Gear are gone; the dialog has category tabs, search, `+N` picker, price with currency label, Buy, and a Sell action on inventory rows; price-mode control reused. | `MarketDialog.tsx` (new), `CharacterSheet.tsx`, `itemPickerFilter.ts`, `spec.json` | `grep -c 'Add Weapon\|Add Armor\|Add Gear' apps/desktop/src/characterHub/CharacterSheet.tsx` → 0; ui-smoke rows for open Market, filter Armor, buy, sell green; `npm run typecheck && npm test` | SF/CPR (E3.5) |
| E3.4 | A worn `+N` armor/shield adds an `enhancement` AC row; a wielded `+N` weapon adds `enhancement` to attack and damage rows; two enhancement bonuses do not stack; carried/unwielded `+N` items add nothing. | `src/rules_core/{enhancement,equipment_resolver,damage_total,pilot_compute_corpus}.rs` | `cargo test --locked -j 8 enhancement_rows -- --test-threads=8` with Aldric + `longsword+1` wielded → attack +1, damage +1; carried → +0 | — |
| E3.5 | Starfinder and Cyberpunk sheets get the same Market button and dialog with their catalogs and currency (credits / eddies); their Add buttons are gone; no `+N` picker; their seed deltas listed. | `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`, `sf_adapter.rs`, `cpr_adapter.rs`, `sf_catalog.rs`, `cpr_catalog.rs` | `grep -c 'Add Weapon\|Add Armor\|Add Gear' apps/desktop/src/characterHub/{Starfinder,Cyberpunk}CharacterSheet.tsx` → 0; SF + CPR seed outputs diffed and every delta listed | SF/CPR slots (DEF-2) |
| E3.MC | Adversarial; every tab on every system still has an add path (R-11); Epic report. | receipts | E3 commands green; a grep that each sheet file references `MarketDialog` | — |

## Epic E4 — slots and equipping

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E4.1 | `BodySlot` enum + `equip`/`unequip`; equip refuses with `"<slot> slot is taken by <item>"`; rings allow two; shields take `shield` + off hand; `equip_item` / `unequip_item` commands; `add_equipment_selection` defaults **new** items to carried (behaviour change stated). | `src/rules_core/body_slots.rs` (new), `character_input.rs`, `character_hub.rs`, `main.rs`, `boundary/{equipItem,unequipItem}.ts` (new) | `cargo test --locked -j 8 slot_conflict -- --test-threads=8` (helmet×2 refused; ring×2 ok; ring×3 refused; override flag admits + tags) | the Equipment panel (E4.3) |
| E4.2 | Only `EquippedActive` items feed AC, attack, saves, ability and skill bonuses; weight counts both states; legacy saves' existing items are migrated to `EquippedActive` (worn) so no saved sheet changes silently; Aldric's AC with armor carried = AC without armor. | the 5 F-11 files, `local_store.rs`, `src/saved_character/mod.rs` | `cargo test --locked -j 8 worn_vs_carried -- --test-threads=8`; seed deltas = none beyond the new `slot`/state fields (listed); FSR-C3 "29 lb" cause recorded | — |
| E4.3 | Gear tab has an **Equipment** control: Worn (by slot) and Carried lists, Equip / Unequip, the refusal message shown verbatim, the override tag. | `EquipmentPanel.tsx` (new), `CharacterSheet.tsx`, `spec.json` | ui-smoke rows: equip, conflict message, unequip; `npm run typecheck && npm test` | — |
| E4.4 | Custom items may declare `bonuses[] {target, bonus_type, value}` from the fixed target list and an optional `slot`; a worn custom item with `ac +1 deflection` raises AC by 1; carried adds 0; a text-only custom item is unchanged. | `character_custom.rs`, `CustomTab.tsx`, the engine consumer for custom bonuses, `boundary/*.ts` | `cargo test --locked -j 8 custom_item_bonuses -- --test-threads=8`; `npm test` on the CustomTab form model | — |
| E4.MC | Adversarial; Epic report. | receipts | E4 commands green | — |

## Epic E5 — level ledger

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E5.1 | Feat, spell, skill-rank and ability-increase selection records carry `taken_at_level: Option<u8>` and `assigned_on_upgrade: bool`; every add path stamps the current level; schema version bumped; old files still load. | `src/saved_character/mod.rs`, `character_hub.rs` add paths | `cargo test --locked -j 8 ledger_stamp -- --test-threads=8`; an old-version fixture loads | — |
| E5.2 | On first load of an older save, `migrate_ledger()` assigns levels by slot order (R8) and sets `assigned_on_upgrade`; over-budget extras get `None` (SD-o); idempotent. | `src/saved_character/migrate_ledger.rs` (new), `local_store.rs` | `cargo test --locked -j 8 ledger_migration -- --test-threads=8` (Elowen fixture: 5 levels, feats at 1, 3, 5 + wizard bonus at 5; run twice = same) | — |
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
| E7.1 | `pcg_import <file> --out <json>` converts every `.pcg` fixture (F-16) to a Codex saved character: race, classes+levels, stats, feats+targets, ranks, spells, equipment with location → `slot`/state, money, languages, alignment, deity, name/bio; unknown keys → custom items + a report; a mapping table doc. | `crates/codex-ingest/src/pcg_import/**` (new), `crates/codex-ingest/src/bin/pcg_import.rs` (new), `crates/codex-ingest/Cargo.toml`, `docs/architecture/` import note | `cargo test --locked -p codex-ingest -j 8 pcg_import_fixtures -- --test-threads=8`; `bash scripts/verify.sh --only pcgen-residue-gate crate-wall` PASS | PCGen XML (DEF-3) |
| E7.2 | The binary ships as a sidecar (`externalBin`); the Import button offers "Codex file \| PCGen file"; a `.pcg` imports and loads; the report is shown; `src/` and `apps/desktop` contain no `.pcg` parser. | `tauri.conf.json`, `Cargo.toml` (`tauri-plugin-shell`), `character_hub.rs`, `main.rs`, Import UI file, `packaged_resources.rs`, `boundary/importPcg.ts` (new) | `cargo test --test packaged_resources`; `grep -rn 'pcg' src apps/desktop/src apps/desktop/src-tauri/src \| grep -v 'sidecar\|externalBin\|import_character_from_pcg'` → empty; ui-smoke row: import fixture → character listed | — |
| E7.MC | Adversarial; Epic report. | receipts | E7 commands green | — |

## Epic E8 — print

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E8.1 | `print/` module: Typst world with bundled OFL fonts only, frame template, `SheetDocument`, PF layout file with Identity + Combat + Abilities blocks, `save_sheet_pdf` command writing a real PDF for Aldric on A4 and Letter; versions recorded. | `apps/desktop/src-tauri/src/print/**` (new), `resources/fonts/**` (new), `resources/print/*.typ` (new), `apps/desktop/src-tauri/Cargo.toml`, `main.rs` | P-4; `cargo test --locked -j 8 print_smoke -- --test-threads=8` asserts a PDF > 1 KiB whose extracted text contains "Aldric" and "Armor Class"; two runs byte-equal (NR-3) | the other blocks |
| E8.2 | All 15 blocks for PF; the three options: Spellbook (full text per known spell), Show the math (every explanations row behind a printed value, "(no derivation recorded)" where none), Bio and campaign notes; override tags printed in "full". | `print/blocks/**`, `print/layout/pf1e.rs` | `cargo test --locked -j 8 print_pf_blocks -- --test-threads=8`: extracted text contains each block heading in layout order; option on/off changes page count; `(no derivation recorded)` count per seed listed | — |
| E8.3 | SF and CPR layout files from their printed-lines modules; both render all seeds on both papers. | `print/layout/{sf1e,cpr}.rs`, `sf_sheet_print.rs` / `cpr_sheet_print.rs` only if the out-dir control is built (R14) | `cargo test --locked -j 8 print_sf print_cpr -- --test-threads=8` | — |
| E8.4 | Settings → Print paper (A4 default, Letter); a Print dialog on all three sheets with paper override and the three checkboxes; **Save PDF** and **Print** buttons; `printLayout.ts` + test deleted; no `window.print`. | `settings/**`, `PrintDialog.tsx` (new), the three sheet `.tsx`, delete `printLayout.ts`, `printLayout.test.ts`, `boundary/{saveSheetPdf,printSheet}.ts` (new), `spec.json` | `grep -rn 'window.print\|printLayout' apps/desktop/src \| awk 'END{print NR}'` → 0; ui-smoke rows: dialog opens, Save PDF produces a file; `npm run typecheck && npm test` | — |
| E8.5 | Golden suite: 3 systems × 2 papers × 4 option sets = 24 PDFs per run; each asserts value presence, heading order, page count, Typst diagnostics = 0, determinism. | `apps/desktop/src-tauri/tests/print_golden.rs` (new), fixtures | `cargo test --locked -j 8 --test print_golden -- --test-threads=8` → 24 passed | pixels (DEF-4) |
| E8.MC | Adversarial: NR-2 grep, every sheet value on screen appears in the basic PDF for each seed; Epic report. | receipts | `grep -n 'fn compute\|calc(' apps/desktop/src-tauri/src/print/` → 0; E8 commands green | — |

## Epic E9 — closure

| ID | Criterion | FILES YOU OWN | Acceptance command | Does not cover |
|---|---|---|---|---|
| E9.1 | Fonts, `.typ` templates and the sidecar are in `bundle.resources`/`externalBin`, asserted in `packaged_resources.rs`; `verify.sh` tauri-resources-tracked knows them; a clean-checkout `tauri build` succeeds. | `tauri.conf.json`, `packaged_resources.rs`, `scripts/verify.sh` | `cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources`; `bash scripts/verify.sh --only tauri-resources-tracked` PASS; `ls target/release/bundle/` | — |
| E9.2 | Fresh install (.deb and AppImage, clean profile): Settings → Update → Check shows no "unknown"; open a seed, Save PDF works and the PDF opens. | receipt | observed values pasted; SD-l if impossible | — |
| E9.3 | Widest verify once on the final code: root + desktop `cargo test`, `npm run typecheck && npm test && npm run ui-smoke:doc`, `bash scripts/verify.sh --full --show-actuals -j 8`; every FAILED attributed; PF/SF/CPR seed deltas listed. | receipt, `scripts/verify-baselines.env` if floors move | all green or attributed; `grep -c 'FAIL' <log>` → 0 | — |
| E9.4 | **Operator gate.** Three PDFs (Aldric PF full, SF-Soldier-3 basic, Streetrat Solo basic) saved to `artifacts/epic_9/E9.4/`; the operator prints them and writes `usable: yes` ×3 (or the defects) in the receipt. | `artifacts/epic_9/E9.4/**` | `grep -c 'usable: yes' artifacts/epic_9/E9.4_cycle_receipt.md` → 3 | — |
| E9.5 | Final-acceptance scan (`workflow-instruction.md §11` step 1): 51 rows, every non-exempt row `complete`, every DEF/FSR revisit command run and pasted. | `progress.md` | the §11 awk prints nothing | — |
| E9.6 | `docs/retro/sd39-retrospective.md` written from `retro.py summary --since 2026-11-01 --json`, cited from `references/README.md` in the same commit. | `docs/retro/sd39-retrospective.md`, `references/README.md` | `grep -c sd39-retrospective references/README.md` ≥ 1 | — |
| E9.7 | Every `sd39-*` worktree and `sd39/*` branch removed, `cargo-target/sd39-*` deleted; counts found/removed; the peer's tree untouched. | — | `git worktree list \| grep -c sd39` → 0; `ls /home/ubuntu/cargo-target \| grep -c sd39` → 0; `git worktree list \| grep -c fix-perform-install` unchanged | — |
| E9.8 | `release-notes.md` final body; sentinel gone; resolver passes. | `release-notes.md` | `grep -c 'SD-39 INTERIM NOTES' release-notes.md` → 0; `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0; `python3 scripts/tranche/validate-tranche-notes.py --release-notes-path docs/release/SD-39-ui-print-corrections/release-notes.md` | — |
| E9.9 | Architecture truth-up receipt + Opus claims critic. | `docs/architecture/**`, `receipts.md` | `grep -c 'architecture-truth-up' receipts.md` ≥ 1 | — |
| E9.10 | Graphify last, clean tree, HEAD = `origin/tranche-19`; indexed SHA recorded; node-count guard respected. | graph outputs, `receipts.md` | `git status --porcelain \| wc -l` → 0 before; `grep -c 'graphify:update' receipts.md` ≥ 1 | — |
| E9.11 | PR `tranche-19 → develop` opened as the final action; `pr-tests` green; the operator merges. | `kanban.md` row | `gh pr view <n> --json state,statusCheckRollup` | the merge |
