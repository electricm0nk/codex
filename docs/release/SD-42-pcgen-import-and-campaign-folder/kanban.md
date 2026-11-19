---
canonical: true
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
board: local-file ./kanban.md
status: planning-ready
---

# SD-42 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a card edits
that card's row in its own closing commit. A row moves to `complete` only together with a receipt and its
commit SHA.

**Status vocabulary:** `ready` · `waiting` · `in-progress` · `complete` · `partial` (blocks closure) ·
`blocked-escalated` (operator ruling needed; blocks closure).

**Column order is load-bearing:** E5.2's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the script dispatches a card only when every dependency is `complete`.
`SD-41 merged` on E4.1 is a dependency the **lead** clears by hand (replacing it with the merge SHA after the
catch-up rebase, `decisions.md §4`, SD-m). Row order is dispatch order within a lane; the three Phase-1 lanes
(`import` E1, `campaign` E2, `editor` E3) run in parallel. **Closure rule:** no PR while any row other than
E5.4 is anything but `complete`. **One operator gate:** E5.G stays `partial` until the operator's three
`usable: yes` lines exist.

**Same-ID check** (`epic-breakdown.md` ↔ this file):

```bash
cd docs/release/SD-42-pcgen-import-and-campaign-folder
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(grep -oE '^\| (C|E)[0-9][0-9.A-Za-z]*' epic-breakdown.md | sed 's/^| //' | sort -u) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 54: 37 at authoring (2 + 8 + 12 + 4 + 6 + 5) + E1.8 (discovered by E1.2) + E1.9 (discovered by E1.5) + E2.14 (discovered by E2.3) + E2.12, E2.13, E4.6 (operator, 2026-11-17, decisions.md §26) + E4.7, E4.8, E4.9, E4.10, E4.11 (operator, 2026-11-17, decisions.md §27) + E4.12 (issue #409, operator 2026-11-17, decisions.md §28) + E1.10, E4.13 (discovered by E1.7) + E4.14 (discovered by E1.10) + E2.15 (discovered by E2.10) + E4.15 (discovered by E4.2)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the lead 2026-11-16 from `origin/tranche-21` @ `1046daefc0` |
| C1 | 0.22.0 on 14 surfaces + notes pointer | haiku | complete | C0 | commit `589815b65f`; receipt `artifacts/cycle_0/C1_cycle_receipt.md`; Phase 1, first |
| E1.1 | Diagnosis run over the 8 files; H1–H7 proved or refuted | opus | complete | C1 | receipt `artifacts/epic_1/E1.1_cycle_receipt.md`; H1 H3 H4 H7 proved, H2 proved (a narrower: 1 slot conflict, not 3), H5 refuted, H6 not exercised; operator table local |
| E1.2 | One stack per line; `equipmentSets` on the wire; locations from the set tree; `QUANTITY:0` noted; no cap | opus | complete | E1.1 | commit: the one adding the E1.2 receipt; receipt `artifacts/epic_1/E1.2_cycle_receipt.md`; 6/6 `pcg_import_stacks`; discovered E1.8 |
| E1.3 | Custom names and every property; `CopyFacts` gains four fields | opus | complete | E1.2 | commit: the one adding the E1.3 receipt (code `0b494a0bfe`); receipt `artifacts/epic_1/E1.3_cycle_receipt.md`; 6/6 `pcg_import_custom`, 2/2 `copy_facts_` |
| E1.4 | Unmappable items → Create items at their location | opus | complete | E1.3 | commit: the one adding the E1.4 receipt (code `d7cacc4343`); receipt `artifacts/epic_1/E1.4_cycle_receipt.md`; 3/3 `pcg_import_unmapped` |
| E1.5 | The Notes block in `bio.notes` | opus | complete | E1.4 | commit: the one adding the E1.5 receipt (code `f4b23af04b`); receipt `artifacts/epic_1/E1.5_cycle_receipt.md`; 3/3 `pcg_import_notes`; discovered E1.9 |
| E1.6 | Starfinder import; the drone per Q-A | opus | complete | E1.5 | commit: the one adding the E1.6 receipt (code `826b421762`); receipt `artifacts/epic_1/E1.6_cycle_receipt.md`; 3/3 `pcg_import_sf`, 1/1 `pcg_import_fixtures_a_starfinder`; drone refused (Q-A second branch) |
| E1.7 | Whole-character parity harness + gate; `explained.tsv` | opus | complete | E1.6 | commit: the one adding the E1.7 receipt (code `75f26c7ebb`); receipt `artifacts/epic_1/E1.7_cycle_receipt.md`; 1/1 `pcg_import_parity` (7 of 7 samples + operator file, 39 ledger rows); discovered E1.10, E4.13 |
| E1.8 | Bundle items: PCGen `QUANTITY` counts bundles; pieces = × `base_quantity` | opus | complete | E1.7 | commit: the one adding the E1.8 receipt (code `d3546f488b`); receipt `artifacts/epic_1/E1.8_cycle_receipt.md`; 2/2 `stacks_bundle` (pf_Rogue bullets, pf_Cleric bolts: 20 pieces each); parity unchanged 257 equal / 258 explained |
| E1.9 | `pcg_import_diagnosis` stops rewriting E1.1's committed artifacts on every run | sonnet | complete | E1.5 | commit: the one adding the E1.9 receipt (code `ea52e721f4`); receipt `artifacts/epic_1/E1.9_cycle_receipt.md`; 2/2 `pcg_import_diagnosis`, E1.1 dir status count 4 → 0 |
| E1.10 | The parity ledger's `fixable` rows naming E1.10: diagnose each, fix converter-side or re-classify with evidence | opus | complete | E1.7 | commit: the one adding the E1.10 receipt; receipt `artifacts/epic_1/E1.10_cycle_receipt.md`; `E1.10` rows 22 → 0 (4 fixed and deleted, 1 codex-rule, 5 oracle-departs, 12 → E4.14); 1/1 `pcg_import_parity`; discovered E4.14 |
| E1.MC | E1 merge check + Epic report | opus | complete | E1.7, E1.8, E1.9, E1.10 | commit: the one adding the E1.MC receipt; receipt `artifacts/epic_1/E1.MC_cycle_receipt.md`; 2 Q5 defects fixed (custom-feat lines and Create-item customization now in Notes); codex-ingest 187 suites 1912 passed 0 failed; root lib 3156 passed; residue PASS; import lane ended |
| E2.1 | Folder model (root) + `campaign_folder.rs` commands + known folders | opus | complete | C1 | code `039dd404a5`; receipt `artifacts/epic_2/E2.1_cycle_receipt.md`; root 5 + desktop 3 tests |
| E2.2 | Member #1; typed members; Create / Edit / Manager over the folder | opus | complete | E2.1 | code `2c4fed1ad0`; receipt `artifacts/epic_2/E2.2_cycle_receipt.md`; 4 member cases + desktop `campaign_folder::` 4; `save_campaign_json` fence exit (§20) |
| E2.3 | Drive removal; folder-backed Manager; hub modes carry a folder | opus | complete | E2.2 | commit: the one adding the E2.3 receipt (code 80692e2ac0); receipt `artifacts/epic_2/E2.3_cycle_receipt.md`; NR-6 grep 70 → 0; desktop 1373 → 1366 (−7 `campaign_drive`); discovered E2.14 |
| E2.4 | Migration screen + command + backup | opus | complete | E2.3 | commit: the one adding the E2.4 receipt (code `bef3e6c2c8`); receipt `artifacts/epic_2/E2.4_cycle_receipt.md`; root `campaign_migrate_` 4 passed; desktop `campaign_folder::` 6; TS 3 cases; `§20` test names |
| E2.5 | Help screen; `opener:default`; Obsidian link; live tree | opus | complete | E2.4 | commit: the one adding the E2.5 receipt (code `20257d121c`); receipt `artifacts/epic_2/E2.5_cycle_receipt.md`; `campaignHelpModel.test.ts` 11 cases; `@tauri-apps/plugin-opener` 2.5.3 (package.json fence exit); desktop 1508 passed |
| E2.6 | Party Resources tab | opus | complete | E2.5, E3.1 | commit: the one adding the E2.6 receipt (code `fad0509fa6`); receipt `artifacts/epic_2/E2.6_cycle_receipt.md`; root `party_resources_` 2 passed; desktop 1509 passed; npm 209/209; 4 screenshots looked at; fence exit `create_party_resources_starter` |
| E2.7 | Adventure Log tab + frontmatter parser | opus | complete | E2.6 | commit: the one adding the E2.7 receipt (code `e9914545d3`); receipt `artifacts/epic_2/E2.7_cycle_receipt.md`; root `adventure_log_` 5 passed, `frontmatter_` 7 passed; `adventureLogModel.test.ts` 4 cases; desktop 1510 passed; npm 210/210; 4 screenshots looked at; fence exits `CampaignSheet.tsx`, `campaignFiles.ts`, `PartyResourcesTab.tsx` |
| E2.8 | Maps tab with pins | opus | complete | E2.7 | commit: the one adding the E2.8 receipt (code `a301b42f03`); receipt `artifacts/epic_2/E2.8_cycle_receipt.md`; root `campaign::maps::tests::maps_` 4 passed; `mapsModel.test.ts` 4 cases; desktop 1511 passed; npm 211/211; 6 screenshots looked at (two pins); fence exits `CampaignSheet.tsx`, `frontmatter.rs` (1 line) |
| E2.9 | Wiki tab: folders, pages, links, backlinks, search | opus | complete | E2.8 | commit: the one adding the E2.9 receipt (code `abec3b9380`, `f2172793af`); receipt `artifacts/epic_2/E2.9_cycle_receipt.md`; root `--lib wiki_` 8 passed; `wikiModel.test.ts` 4 cases; desktop 1512 passed; npm 212/212; 7 screenshots looked at; fence exits `CampaignSheet.tsx`, the E2.6–E2.8 tabs/models (one link resolver, one page writer) |
| E2.10 | `Characters/` backup model: master.json, counter, stand-in, move master, diff | opus | complete | E2.9 | commit: the one adding the E2.10 receipt (code `8d16f91c37`, `4380ce7bec`); receipt `artifacts/epic_2/E2.10_cycle_receipt.md`; root `--lib characters_backup_` 8 passed; desktop `import_campaign_character` 2 passed, `campaign_folder::` 13; root lib 3186 passed; desktop 1515 passed; npm 213/213; fence exits `character_hub.rs`, `cpr_adapter.rs` (test lines + one `pub(crate)`); discovered E2.15 |
| E2.11 | DM toolkit party from the folder | opus | complete | E2.10 | commit: the one adding the E2.11 receipt (code `573b179d50`); receipt `artifacts/epic_2/E2.11_cycle_receipt.md`; desktop `rate_encounter_from_campaign` 2 passed, `encounter_rating` 16; desktop 1517 passed 0 failed; npm 213/213; no fence exit |
| E2.12 | "Main menu" is the first ☰ entry on every screen but the main one | opus | complete | E2.11 | commit: the one adding the E2.12 receipt (code `a3edf841f6`); receipt `artifacts/epic_2/E2.12_cycle_receipt.md`; `AppMenu.test.ts` passed (5 new named cases + 2 source assertions); `grep -c "Main menu" AppMenu.tsx` 3; `layout/AppMenu` users 3; npm 213/213; no fence exit |
| E2.13 | The Manager's open-campaign button: full column width, the main screen's Campaign Manager style | opus | complete | E2.12 | commit: the one adding the E2.13 receipt (code `cd33e7d82d`); receipt `artifacts/epic_2/E2.13_cycle_receipt.md`; `campaignManagerScreen.test.ts` 3 named cases passed; `linear-gradient` in the Manager 0; LandingScreen exports 2; npm 214/214; typecheck clean; no fence exit; operator UI-2 2026-11-17, `decisions.md §26.2` |
| E2.14 | Print dialog and DM console read the campaign folders, not the legacy localStorage campaigns | opus | complete | E2.13 | commit: the one adding the E2.14 receipt (code `d545ead414`); receipt `artifacts/epic_2/E2.14_cycle_receipt.md`; `getLegacyCampaigns` grep 2 → 0; `PrintDialog.test.ts` + `dmRecordModel.test.ts` passed; desktop `load_character_campaigns` 1 passed, suite 1518 passed 0 failed; npm 214/214; fence exits `campaign_folder.rs`, `main.rs` (2 lines), `boundary/campaignFolder/characters.ts` (`decisions.md §20`); discovered by E2.3 |
| E2.15 | This device's name editable in Settings (the `deviceName` a backup's `master.json` shows) | sonnet | complete | E2.10 | commit: the one adding the E2.15 receipt (code `8bc6066c6e`); receipt `artifacts/epic_2/E2.15_cycle_receipt.md`; root `--lib characters_backup_` 10 passed; desktop `campaign_folder::` 13; npm 213/213; discovered by E2.10; `decisions.md §16` item 2; `epic-breakdown.md` E2.15 |
| E2.MC | E2 merge check + Epic report (≥ 10 screenshots, looked at) | opus | complete | E2.14, E2.15 | commit: the one adding the E2.MC receipt; receipt `artifacts/epic_2/E2.MC_cycle_receipt.md`; root lib 3188 → 3191 passed; desktop 1518 passed 0 failed; npm 214/214; NR-6 0; partition 1 (E2.10 post-merge fence exit, explained); 19 PNGs looked at; fixed: SD-r retry in the backup mirror, `TEMPLATE_TREE` pinned, Friends copy; the campaign lane ends |
| E3.1 | Editor core: packages, byte-stable round trip, live preview, reading view | opus | complete | C1 | lane `editor`; `decisions.md §18`; commit `75e9feca1d`; receipt `artifacts/epic_3/E3.1_cycle_receipt.md` |
| E3.2 | `[[` completion; link clicks | opus | complete | E3.1 | commit `931098912b`; receipt `artifacts/epic_3/E3.2_cycle_receipt.md` |
| E3.3 | Toolbar, auto-save, disk-change warning, drag-in images | opus | complete | E3.2 | code `084394b746`; receipt `artifacts/epic_3/E3.3_cycle_receipt.md`; fence exit `save_attachment` (campaign_folder.rs + main.rs) |
| E3.MC | E3 merge check + Epic report | opus | complete | E3.3 | adversarial; the editor lane ends; fix `05aa922dcd` (reading-view tab-split `javascript:` link; fixture = real starter); receipt `artifacts/epic_3/E3.MC_cycle_receipt.md` |
| E4.1 | Import writes `equipment_sets.json`, Create items, Notes, `game_system`; a new root every import | opus | complete | E1.MC, E2.MC, E3.MC, d6890a6c73 | commit: the one adding the E4.1 receipt (code `c3ab5aa9a2`); receipt `artifacts/epic_4/E4.1_cycle_receipt.md`; 7/7 `import_writes_equipment_sets`; desktop 1526 passed 0 failed; PF seeds byte-identical; Create item stacks weighed by pieces (load fix); fence exit `sf_adapter.rs` (2 `pub(crate)`) |
| E4.2 | Copy overrides end to end (engine, DTOs, screen, Weapons tab, print) | opus | complete | E4.1 | commit: the one adding the E4.2 receipt (code `5083358d54`); receipt `artifacts/epic_4/E4.2_cycle_receipt.md`; root `copy_override_` 4 passed, desktop `copy_overrides` 3, `print_pf_blocks_gear_properties` 1; root lib 3195, ingest 1917, desktop 1530 passed 0 failed; npm 214/214; G-1 SAME ×3; G-2 30/30 PDFs byte-identical; 4 images looked at; fence exits `print_pf_blocks.rs` (test), `loadSavedCharacterDetail.ts`, `weaponsTabModel.ts`; discovered E4.15 |
| E4.3 | Mirror hooks: `after_write` with stand-in + counter; membership dialog over folders; import choice | opus | complete | E4.2 | commit: the one adding the E4.3 receipt (code `f496612b6d`); receipt `artifacts/epic_4/E4.3_cycle_receipt.md`; `set_character_campaigns` keeps `stand_in` + counter from the file; desktop `mirror_two_master` 3 passed (RED 1/3 on the base); model 5 cases; desktop 1390 + 1 + 139 + 1 + 2 passed 0 failed; npm 214/214; clippy PASS, sd24 5 passed; fence exits `setCharacterCampaigns.ts`, `CharacterHubPage.tsx` |
| E4.7 | Weightless container contents: PCGen `CONTAINS:*N` / `N%` parsed; Bag of Holding and Handy Haversack contents add nothing to the carried total; nesting; capacity check still counts | opus | complete | E4.3 | commit: the one adding the E4.7 receipt (code `8f81981c89`); receipt `artifacts/epic_4/E4.7_cycle_receipt.md`; root `weightless_` 5 passed, ingest `container_capacity` 2 passed, desktop `move_equipment_into_bag_of_holding` 1 passed (RED 0 matches each); 5 settled bundles regenerated (48 flagged records, CRB 20), `--check` PASS; root lib 3200, ingest 1914, desktop 1391 + 1 + 139 + 2, npm 214/214, 0 failed; seeds SAME; fence exits `damage_total.rs` (test line), `loadSavedCharacterDetail.ts` |
| E4.8 | Slot kinds: a copy is offered and accepted only in a slot of its own kind (menu, drag, backend); the slot override lifts it; a saved wrong slot loads with a warning | opus | complete | E4.7 | operator SLOT-1 2026-11-17; `decisions.md §27.2` |
| E4.9 | Customize a held copy: one button per equip row / container-table row opens rename (per copy, `CopyFacts.name`) + attach modifier (the existing flow, now per copy); the redundant list under the equip section and everything only it used deleted; print `Camping Gear (Sack)` | opus | waiting | E4.8 | operator REN-1 + CUS-1 2026-11-17; `decisions.md §27.3` |
| E4.10 | Gear detail panel: selecting a row shows the record (weight, cost, type, slot, weapon/armor stats, capacity, description, book) and the copy (name, size, modifiers, overrides, quantity, location); `—` for absent | opus | waiting | E4.9 | operator DET-1 2026-11-17; `decisions.md §27.4` |
| E4.11 | Stacks: identical copies at one place are one row with the count everywhere (screen tree, print); a container copy holding anything, or a renamed one, is its own row; a stack splits by Move to's "how many?" and the parts move on their own; parts rejoin when put back together | opus | waiting | E4.10 | operator STK-1 2026-11-17; `decisions.md §27.5` |
| E4.4 | ui-smoke rows: import + campaign (new + 8 rewritten) + E2.12/E2.13's two rows + E4.7–E4.11's six rows; inventory | opus | waiting | E4.11 | the only `spec.json` writer (R24); `decisions.md §26.4, §27.6` |
| E4.5 | Proof runs: 7 samples (CI shape) + the operator's file (local, counts) | opus | waiting | E4.4 | `decisions.md §10`; P-8 |
| E4.6 | Linux AppImage writes its own menu entry + icon on every start; re-points after a move or update; never for .deb/.rpm/Windows/macOS | opus | waiting | E4.5 | operator LNX-1 / Q27 2026-11-17; `decisions.md §26.3`; `desktop/update/self_install.rs` only |
| E4.12 | Linux release builds against glibc 2.35 (`container: ubuntu:22.04`; fallback `runs-on: ubuntu-22.04`); `tools/release/check_glibc_floor.py` gates every shipped Linux ELF (`.deb` + AppImage) at the one stated floor, with a self-test; `linux-build-check.yml` proves it on the branch before release | opus | waiting | E4.6 | GitHub issue #409, operator 2026-11-17; `decisions.md §28`; the only SD-42 card touching `.github/workflows/`; no cargo slot |
| E4.13 | Pathfinder whole-character parity at the desktop: every `not-computed` / E4.13 ledger row compared against the desktop's own totals | opus | waiting | E4.1, E1.MC | discovered by E1.7; `epic-breakdown.md` E4.13 |
| E4.14 | Engine rows the import ledger names: Starfinder Toughness/Extra Resolve/personal upgrades, Armory melee weapons' `Melee` tag, the paladin's Ultimate Combat spells | opus | waiting | E4.1, E1.MC | discovered by E1.10; `epic-breakdown.md` E4.14 |
| E4.15 | Per-copy weapon rows: two copies of one base (Mourn beside a plain Longsword) each print their own die and totals | opus | waiting | E4.2 | discovered by E4.2; `epic-breakdown.md` E4.15 |
| E4.MC | E4 merge check + Epic report + E5.G's screenshots | opus | waiting | E4.5, E4.6, E4.7, E4.8, E4.9, E4.10, E4.11, E4.12, E4.13, E4.14, E4.15 | adversarial |
| E5.G | OPERATOR GATE: import + Campaign Manager + editor, `usable: yes` ×3 | sonnet | waiting | E4.MC | the run ends here until the operator signs |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E5.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-22 → develop (final action) | sonnet | waiting | E5.3 | the PR body carries `Fixes #409` (`decisions.md §28`) |
