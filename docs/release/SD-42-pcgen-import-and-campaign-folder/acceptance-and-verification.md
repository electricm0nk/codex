---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready
---

# SD-42 Acceptance and Verification

Gates G-1…G-9 map the Definition of Done (`scope-draft.md §2`) to commands. Root-crate filters run from the
repo root; `codex-ingest` filters from `crates/codex-ingest`; desktop filters from `apps/desktop/src-tauri`;
frontend commands from `apps/desktop`. Every figure a receipt quotes carries its command and denominator.

## G-1 PF seed renders: no existing value moves (DoD 2; `decisions.md §24`) — E4.1, E4.2, E4.3, every `*.MC` from E4 on, E5.1

As SD-41 G-1: render the PF seeds (Aldric Fighter 3, Elowen Wizard 5) on the base and on the card's tree;
`SAME_EXCEPT_NAMED <keys>` per seed, where the only named keys are E4.2's new DTO fields (`properties`,
`damageOverride`, `weightOverrideLbs`, `costOverrideCopper`); any `VALUE_MOVED` is a defect.

## G-2 SF1e and CPR golden output unchanged — E4.2, E4.MC, E5.1

SF ×8 and CPR ×8 golden text and page counts `diff` empty; PF golden rows re-recorded only for the
properties lines E4.2 adds, each listed before → after.

## G-3 The partition held (`decisions.md §4`) — E1.MC, E2.MC, E3.MC

```bash
git diff --name-only <C1 sha>..HEAD -- apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src/characterHub/BuyCustomDialog.tsx docs/architecture scripts/verify-baselines.env | awk 'END{print NR}'   # 0 on every Phase-1 lane
git diff --name-only <C1 sha>..HEAD -- apps | awk 'END{print NR}'   # 0 for the import lane (E1 touches nothing under apps/)
```

## G-4 Import: stacks, locations, names, properties, nothing dropped (DoD 1–3) — E1.2–E1.5, E4.1, E4.2

```bash
cd crates/codex-ingest
cargo test --locked -j 8 --test pcg_import_stacks --test pcg_import_custom --test pcg_import_unmapped --test pcg_import_notes -- --test-threads=8   # every test passed; counts quoted
grep -c 'MAX_COPIES' src/pcg_import/map.rs   # 0
python3 -I -c "import json;d=json.load(open('../../docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_1/E1.MC/summary.json'));bad=[k for k,v in d.items() if v['copies_emitted']!=v['lines']-v['zero_lines']];print('COPIES_MISMATCH',bad) if bad else print('ONE_COPY_PER_LINE', len(d))"   # ONE_COPY_PER_LINE 7
cd ../../apps/desktop/src-tauri && cargo test --locked -j 8 --test import_writes_equipment_sets -- --test-threads=8   # ≥ 5 passed
```

## G-5 Starfinder and parity (DoD 4, 5) — E1.6, E1.7

```bash
cd crates/codex-ingest
cargo test --locked -j 8 --test pcg_import_sf -- --test-threads=8                       # ≥ 3 passed
cargo test --locked -j 8 --test pcg_import_parity -- --test-threads=8 --nocapture        # 1 passed; per build: fields compared / equal / explained
awk -F'\t' 'NR>1 && NF!=6' ../../scripts/oracle_harness/pcg_import_parity/explained.tsv | awk 'END{print NR}'   # 0
grep -c 'cannot compute a Starfinder soldier' src/pcg_import/fixtures_tests.rs            # 0
```

## G-6 The campaign folder (DoD 7–16) — E2.1–E2.13, E2.MC, E4.3

```bash
cargo test --locked -j 8 --lib campaign -- --test-threads=8                              # root: every campaign_folder_/campaign_migrate_/adventure_log_/frontmatter_/maps_/wiki_/characters_backup_ test passed, counts quoted
cd apps/desktop/src-tauri && cargo test --locked -j 8 campaign_folder -- --test-threads=8 && cargo test --locked -j 8 rate_encounter_from_campaign -- --test-threads=8 && cargo test --locked -j 8 --test mirror_two_master -- --test-threads=8
grep -rn 'codex\.campaigns\|drive_list_campaigns\|drive_save_campaign\|drive_load_campaign\|drive_delete_campaign\|write_campaign_drive\|googleDrive\|GoogleDrivePanel\|campaignManagerAccessGate' apps/desktop/src apps/desktop/src-tauri/src | grep -v campaignMigration | awk 'END{print NR}'   # 0 (NR-6)
test ! -f apps/desktop/src-tauri/src/campaign_drive.rs && echo DRIVE_GONE
python3 -I -c "import json;p=json.load(open('apps/desktop/src-tauri/capabilities/default.json'))['permissions'];assert 'opener:default' in p and len(p)==3,p"   # core, dialog, opener only
cd apps/desktop && npx tsx src/campaign/campaignHelpModel.test.ts                           # ≥ 7 cases (the required sentences)
cd apps/desktop && npx tsx src/layout/AppMenu.test.ts && npx tsx src/campaign/campaignManagerScreen.test.ts   # E2.12 (≥ 4 new cases), E2.13 (≥ 3 cases); decisions.md §26
cd apps/desktop/src-tauri && cargo test --locked -j 8 self_install -- --test-threads=8       # E4.6: 14 + ≥ 4 passed (the Linux AppImage menu entry + icon; decisions.md §26.3)
```

## G-7 The editor (DoD 17) — E3.1–E3.3, E3.MC

```bash
cd apps/desktop
npx tsx src/editor/roundTrip.test.ts && npx tsx src/editor/livePreview.test.ts && npx tsx src/editor/wikilinks.test.ts && npx tsx src/editor/toolbar.test.ts && npx tsx src/editor/autosave.test.ts && npx tsx src/editor/imageDrop.test.ts
git diff <C1 sha>..HEAD -- package.json | grep '^+' | grep -E '"@codemirror/|"@lezer/|"marked"|"@tauri-apps/plugin-opener"' | awk 'END{print NR}'   # 12 (eleven editor packages + the opener)
for d in node_modules/@codemirror/* node_modules/@lezer/* node_modules/marked; do python3 -I -c "import json,sys;print(sys.argv[1].split('node_modules/')[1], json.load(open(sys.argv[1]+'/package.json'))['license'])" "$d"; done | grep -vc ' MIT$'   # 0
```

## G-8 No PCGen in live code; the operator's file never in the repo (`decisions.md §10, §22`) — every `*.MC`, E5.1

The lead sets two shell variables **locally, never in a doc**: `SD42_CHAR` = the test character's name and
`SD42_PII` = the player's name as the file's `PLAYERNAME:` line holds it (`awk -F: '/^PLAYERNAME:/{print $2}' "$CODEX_SD42_PCG"`).

```bash
git diff --name-only <C1 sha>..HEAD -- apps src crates data resources scripts | grep -v 'scripts/oracle_harness/pcg_import_parity/' | xargs -r grep -lis 'pcgen' | awk 'END{print NR}'   # 0 outside the parity exports (a doc-comment citing the pinned oracle's docs is allowed only in this package)
bash scripts/verify.sh --only pcgen-residue-gate -j 8   # PASS
git log <C1 sha>..HEAD -p -S"$SD42_CHAR" -- . ':!docs/release/SD-42-pcgen-import-and-campaign-folder' | awk 'END{print NR}'   # 0 (NR-4: the character name appears only in this package's docs)
git log <C1 sha>..HEAD -p -S"$SD42_PII" -- . | awk 'END{print NR}'   # 0 (the player name appears nowhere, this package included)
```

## G-10 The gear screen's five operator items of 2026-11-17 (`decisions.md §27`) — E4.7–E4.11, E4.4, E4.MC

```bash
cargo test --locked -j 8 --lib weightless_ -- --test-threads=8                          # E4.7: ≥ 5 passed
cargo run --locked --bin gen_settled_corpus -- --check                                  # E4.7: exit 0 (30 settled bundles regenerated, no drift)
cd apps/desktop/src-tauri && cargo test --locked -j 8 slot_kind -- --test-threads=8     # E4.8: ≥ 5 passed; equipment.wrong_slot refused with words
cd apps/desktop/src-tauri && cargo test --locked -j 8 rename_copy_ -- --test-threads=8  # E4.9: ≥ 4 passed; print block reads "Camping Gear (Sack)"
cd apps/desktop/src-tauri && cargo test --locked -j 8 attach_modifier_to_copy -- --test-threads=8   # E4.9: ≥ 2 passed; the pick lands on the chosen copy
grep -c 'Attach Modifier' apps/desktop/src/characterHub/CharacterSheet.tsx                 # 0: the redundant list is gone (E4.9)
cd apps/desktop/src-tauri && cargo test --locked -j 8 describe_held_ -- --test-threads=8 # E4.10: ≥ 5 passed; unstated values null
grep -cE '(weightLbs|costGp|quantity|acBonus|maxDex) *[-+*/]' apps/desktop/src/characterHub/GearDetail.tsx   # 0: the panel derives nothing (NR-2)
cd apps/desktop/src-tauri && cargo test --locked -j 8 stack_rows_ -- --test-threads=8     # E4.11: ≥ 5 passed; four sacks one row; a filled one its own; split 50/50 and rejoin; no weight moves
cd apps/desktop && npx tsx src/characterHub/gearScreenModel.test.ts && npx tsx src/characterHub/CustomizeCopyDialog.test.tsx && npx tsx src/characterHub/GearDetail.test.tsx   # E4.8 ≥ 5, E4.9 ≥ 4, E4.10 ≥ 3 new cases
python3 -I -c "import json;ids={x['id'] for x in json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']};print(sum(i in ids for i in ['sheet-gear-bag-of-holding-weightless','sheet-gear-move-to-slot-kinds','sheet-gear-customize','sheet-gear-detail-panel','sheet-gear-sacks-one-row','sheet-gear-arrows-split-quiver-backpack']))"   # 6 (E4.4)
```

## G-11 The Linux build's glibc floor (GitHub issue #409; `decisions.md §28`) — E4.12, E4.MC, E5.1

```bash
python3 -m pytest tools/release/test_check_glibc_floor.py -q                                   # ≥ 5 passed (E4.12)
python3 tools/release/check_glibc_floor.py --deb "$SCRATCH/Codex_0.21.153_amd64.deb"; echo "exit $?"   # codex-desktop max GLIBC_2.43 floor 2.35 ABOVE; exit 1 (the gate catches the shipped defect)
grep -c 'GLIBC_FLOOR = "2.35"' tools/release/check_glibc_floor.py                            # 1: the floor's one home
grep -n '2\.35' .github/workflows/publish-tester-release.yml .github/workflows/linux-build-check.yml | awk 'END{print NR}'   # 0: the YAML names the image, never the number
awk '/^  publish-tester-release:$/,/^    steps:/' .github/workflows/publish-tester-release.yml | grep -c 'image: ubuntu:22.04'   # 1 (or the fallback's `runs-on: ubuntu-22.04`, with its §20 row)
gh run list -w linux-build-check.yml -b tranche-22 -L 1 --json conclusion,url                # success; the URL is in E4.12's receipt with the gate's OK lines for both bundles
```

## G-9 Closure (DoD 18–20) — E5.G, E5.1–E5.4

```bash
cd docs/release/SD-42-pcgen-import-and-campaign-folder
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 52) print "ROW_COUNT " n }' kanban.md   # 52: 37 + E1.8 + E1.9 + E2.14 + E2.12, E2.13, E4.6 (decisions.md §26) + E4.7–E4.11 (decisions.md §27) + E4.12 (decisions.md §28) + E1.10, E4.13 (discovered by E1.7) + E4.14 (discovered by E1.10)
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E5\.[2-4] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
grep -c '^- \(Import ([^)]*)\|Campaign Manager ([^)]*)\|Editor\) — usable: yes' artifacts/epic_5/E5.G_cycle_receipt.md   # 3 (anchored; the receipt's own instructions must not match)
test -s artifacts/epic_4/E4.5_cycle_receipt.md && grep -c 'copies_emitted' artifacts/epic_4/E4.5_cycle_receipt.md   # ≥ 1: the local proof run's counts are in the receipt (Q9)
grep -c 'SKIPPED: operator .pcg absent' artifacts/epic_4/E4.5_cycle_receipt.md   # 0 (the local run actually ran)
grep -c 'SD-42 INTERIM NOTES' release-notes.md                        # 0 at E5.3+
python3 ../../../tools/release/resolve_release_notes.py --repo-root ../../..
grep -L 'Looked at' artifacts/epic_[24]/*_cycle_receipt.md | grep -v 'E2.1_\|E2.2_\|E2.3_\|E2.4_\|E2.5_\|E2.7_\|E2.10_\|E2.11_\|E4.1_\|E4.3_\|E4.4_\|E4.5_'   # prints nothing (E2.6, E2.8, E2.9, E2.MC, E4.2, E4.MC looked)
grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md                 # prints nothing (R15)
```

Plus the template §6 step 1a items: `cargo test --test packaged_resources` (desktop), `cargo test --test
no_baked_manifest_dir_in_production` (root), `python3 -m pytest tools/release -q`, and the fresh-install check
recorded in E5.1's receipt.

## Traceability — every operator ruling → card → acceptance row

| Ruling | Card(s) | Acceptance row (gate / command) |
|---|---|---|
| Intake 1 (gear wrong: weapons, armor) | E1.1, E1.2, E1.3, E4.1, E4.2 | G-4; E1.1's `summary.json` marks H1, H2 proved/refuted; E5.G line 1 |
| Intake 2 (stacks repeated) | E1.2, E4.1 | G-4 `ONE_COPY_PER_LINE 7`; E4.5 `quantity == 84` |
| Q2 custom name + every property | E1.3, E4.2 | G-4 `pcg_import_custom` ≥ 6 passed; G-1 `NEW_KEYS`; E4.5: 36 names, 17 property sets |
| Q3 proof set = operator file + 7 samples | E1.1, E1.7, E4.5 | G-5 parity per build; G-9 `copies_emitted` in E4.5's receipt; P-6 → 7 files |
| Q4 whole character vs PCGen | E1.7 | G-5 `pcg_import_parity` 1 passed; `explained.tsv` 6 fields per row |
| Q5 unmappable kept + Notes | E1.4, E1.5, E4.1 | G-4 `pcg_import_unmapped` ≥ 3, `pcg_import_notes` ≥ 2; `import_notes_land_in_bio` |
| Q6 stacks one row, splittable, all stacks | E1.2, E4.1 (SD-41 E2.5 for Market buys) | G-4; `grep -c 'quantity = Some(row.quantity.saturating_add(pieces))' apps/desktop/src-tauri/src/character_hub.rs` → 1 (the Market merge, re-verified by E4.1's receipt) |
| Q7 Pathfinder and Starfinder | E1.6, E1.7 | G-5 `pcg_import_sf` ≥ 3; `fixtures_tests.rs` grep → 0 |
| Q8 new character; test file never a default | E4.1, E4.5 | `import_never_touches_another_root`; G-8 `-S"$SD42_CHAR"` → 0 outside this package |
| Q9 file local only; skip loudly; local run is evidence | E4.5, E5.G | G-9 `SKIPPED` grep → 0 in the receipt; `CODEX_SD42_PCG=/nonexistent` run prints `SKIPPED`; G-8 |
| CM-1 / Q11 member #1, no roles | E2.2 | G-6 `campaignMembersModel.test.ts` ≥ 4; NR-13 |
| CM-2 folder, Markdown, Obsidian-compatible, simple editor | E2.1, E2.6–E2.9, E3.1 | G-6 root `campaign` tests; NR-18 checks in E2.MC; G-7 |
| Q12 Drive removed; typed members; help screen | E2.3, E2.2, E2.5 | G-6 NR-6 grep → 0, `DRIVE_GONE`; `campaignHelpModel.test.ts` ≥ 7 |
| Q13 migration with backup; help shows the tree + Obsidian link | E2.4, E2.5 | G-6 `campaign_migrate_` ≥ 4; help test cases "https://obsidian.md", live tree |
| Q14 Party Resources starter, free text | E2.6 | G-6 `party_resources_` 2 passed |
| Q15 Adventure Log: one file per session per author; "+"; frontmatter; tolerant | E2.7 | G-6 `adventure_log_` ≥ 5 (`…missing_frontmatter_is_listed_not_hidden`) |
| Q16 `session` real date or range sorts; `in_game_date` free text | E2.7 | `adventure_log_sorts_by_session_start_with_ranges` |
| Q17 Maps: image + notes; pins now; pins in frontmatter | E2.8 | G-6 `maps_` ≥ 4 (`maps_pins_round_trip_as_fractions`) |
| Q18 Wiki: folders, "+" page, links, backlinks, search | E2.9 | G-6 `wiki_` ≥ 7 |
| Q19 no secrets; help says so | E2.5 | help test case "Everything in this folder is shared" |
| Q20 `Characters/`; DM toolkit reads the folder | E2.10, E2.11, E4.3 | G-6 `characters_backup_target_is_capital_c…`; `rate_encounter_from_campaign` ≥ 2 |
| Q21/Q22 two-master checks; stand-in default; counter; local save succeeds; diff no merge | E2.10, E4.3 | G-6 `characters_backup_counter_mismatch_does_not_overwrite_and_warns`, `…stand_in_never_writes`, `…move_master_takes_over`, `…diff_lists_changed_files`; `mirror_two_master` ≥ 3 |
| Q24 CodeMirror 6 live preview; byte-stable; `[[` suggests | E3.1, E3.2 | G-7 `roundTrip.test.ts` sha256 equal; `livePreview.test.ts` ≥ 8; `wikilinks.test.ts` ≥ 4; licences MIT ×11 |
| Q25 toolbar; auto-save + disk warning; drag-in images | E3.3 | G-7 `toolbar.test.ts` 6, `autosave.test.ts` ≥ 3, `imageDrop.test.ts` ≥ 2 |
| UI-1 (2026-11-17) "Main menu" first on every screen that shows the menu; hidden on the main screen | E2.12, E4.4 | G-6 `AppMenu.test.ts` ≥ 4 new cases; `grep -rl "layout/AppMenu"` → 3; ui-smoke `main-menu-first` |
| UI-2 (2026-11-17) the open-campaign button: full column width, the main screen's Campaign Manager style | E2.13, E4.4 | G-6 `campaignManagerScreen.test.ts` ≥ 3; `linear-gradient` grep → 0 in the Manager; ui-smoke `campaign-open-button` |
| LNX-1 / Q27 (2026-11-17) the AppImage writes its own menu entry + icon; re-points after an update; never for .deb/.rpm/Windows/macOS | E4.6 | `cargo test self_install` 14 + ≥ 4 passed; E4.MC names the four tests |
| BAG-1 / HVS-1 (2026-11-17) a bag of holding's (and a handy haversack's) contents are weightless, not cumulative | E4.7, E4.4 | G-10 `weightless_` ≥ 5 (bag + haversack + nested backpack + flag parse + plain sack); `gen_settled_corpus --check` 0; ui-smoke `sheet-gear-bag-of-holding-weightless` |
| SLOT-1 (2026-11-17) anything can go in any slot (a backpack in Shield) | E4.8, E4.4 | G-10 `slot_kind` ≥ 5 (menu, drag, backend refusal with words, override, saved wrong slot loads with a warning); ui-smoke `sheet-gear-move-to-slot-kinds` |
| REN-1 (2026-11-17) rename an object in inventory (Sack #1 → Camping Gear) | E4.9, E4.4 | G-10 `rename_copy_` ≥ 4; print `Camping Gear (Sack)`; ui-smoke `sheet-gear-customize` |
| CUS-1 (2026-11-17) a Customize button on the equip rows (naming + modifiers); the redundant list with the modifier buttons dropped | E4.9, E4.4 | G-10 `attach_modifier_to_copy` ≥ 2; `CustomizeCopyDialog.test.tsx` ≥ 4; `grep -c 'Attach Modifier' CharacterSheet.tsx` → 0; ui-smoke `sheet-gear-customize` |
| DET-1 (2026-11-17) selecting gear shows all its information (weight / cost / stats / description / slots) | E4.10, E4.4 | G-10 `describe_held_` ≥ 5; `GearDetail.test.tsx` ≥ 3; the no-arithmetic grep → 0; ui-smoke `sheet-gear-detail-panel` |
| LNX-2 / #409 (2026-11-17) "Requires glibc-2.43 on Ubuntu 24" — "add an SD-42 card. post now that this has been assigned to relase v0.22.x." | E4.12, E5.3, E5.4 | G-11: the self-test ≥ 5; the gate exits 1 on the 0.21.153 `.deb`; `linux-build-check.yml` green on `tranche-22` with the gate's `OK` lines; `grep -c '#409' release-notes.md` ≥ 1; `Fixes #409` in the PR body |
| STK-1 (2026-11-17) four sacks read as one `Sack ×4`, a filled one as its own row; stacks split (50/50 into quiver and backpack) and rejoin | E4.11, E4.4 | G-10 `stack_rows_` ≥ 5; `print_pf_blocks_gear_stacks` ≥ 2; model test one node per stack; ui-smoke `sheet-gear-sacks-one-row`, `sheet-gear-arrows-split-quiver-backpack` |

## Artifact map

| Gate | Receipt(s) | Evidence path |
|---|---|---|
| G-1 | E4.1–E4.3, E4.MC, E5.1 | the receipt's "Seed deltas" |
| G-2 | E4.2, E4.MC, E5.1 | the receipt's "Golden deltas" |
| G-3 | E1.MC, E2.MC, E3.MC | the receipt's partition line |
| G-4 | E1.1–E1.5, E1.MC, E4.1 | `artifacts/epic_1/E1.1/*.tsv`, `summary.json`; `artifacts/epic_1/E1.MC/summary.json` |
| G-5 | E1.6, E1.7 | `scripts/oracle_harness/pcg_import_parity/{exports,explained.tsv}`; the receipt's per-build counts |
| G-6 | E2.1–E2.13, E2.MC, E4.3, E4.6 | receipts; `artifacts/epic_2/E2.MC/*.png`; E4.6's receipt (the code path the operator's build took) |
| G-7 | E3.1–E3.3, E3.MC | receipts; the licence loop output |
| G-8 | every MC, E5.1 | receipts |
| G-10 | E4.7–E4.11, E4.4, E4.MC | receipts; `artifacts/epic_4/E4.MC/*.png` (the bag's table, a Backpack's Move to menu, a renamed Sack, the detail panel, four Sacks as one node) |
| E5.G | E4.MC, E5.G | `artifacts/epic_5/E5.G/` (≥ 12 PNGs, 1 PDF, the signed receipt with three lines) |
| G-9 | E5.1–E5.4 | `artifacts/epic_5/`, `receipts.md`, `docs/retro/sd42-retrospective.md` |

## Does not cover

Pixel positions; drag in ui-smoke (DEF-2); macOS and Windows installers; Cyberpunk RED import; companions as
attached records; roles and secrets; Obsidian's rendering of pins; any PCGen field the parity templates do not
export (the ledger names the fields compared per build, so the uncompared set is derivable: the PCGen export's
field list minus the compared list).
