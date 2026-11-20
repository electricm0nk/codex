---
canonical: true
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
tranche: 22
date: 2026-11-20
status: final
---

# Campaign Codex 0.22 — PCGen Import Accuracy and the Campaign Folder

A PCGen `.pcg` file now imports the way PCGen holds it: one stack per place an item is held, with its
count, its custom name and its properties, and nothing dropped. A campaign is now a plain folder of
Markdown files that you can sync however you like and open in any editor; the Campaign Manager reads it
through four tabs and a built-in Markdown editor. The Linux `.deb` and AppImage start on Ubuntu 24.04
again (#409).

Figures are listed under "Figures" as `R-n`, each with the command that produced it. Run the commands
from the repository root. They were run on 2026-11-20 against `tranche-22`.

## Summary

SD-42 rewrote the PCGen import's gear, names and notes, added Starfinder import, and checked the whole
imported character against PCGen's own engine for 7 sample files (R-6). It replaced the campaign's
Drive-folder mirror and browser storage with a campaign folder: `campaign.json`, Party Resources, an
Adventure Log, Maps with pins, a Wiki, and a `Characters/` backup of every member's character. Old
campaigns are migrated once, with a backup. The gear screen gained weightless containers, slot kinds,
per-copy customizing, a detail panel and stacks. The Linux release now builds against glibc 2.35 and a
release gate refuses any shipped Linux binary that needs more (R-7). The bundle has 61 cards; 60 were
`complete` when this file was written, and the 61st, E5.4, is the PR itself (R-1). **The operator gate
was waived, not signed** (R-2): no person has used the import, the campaign folder or the editor yet,
and the first hands-on look is the alpha built after the merge.

## User-Visible Changes

- **Import: one stack per place (E1.2, E1.8, E4.1, E4.11).** Each `.pcg` gear line becomes one stack
  with its count at the location PCGen holds it: the file's equipment sets become Codex equipment sets,
  and nested containers stay nested. A line PCGen splits over two places is two stacks. Bundle items
  count bundles, so one bundle of 20 bullets is 20 pieces. A `QUANTITY:0` line imports nothing and is
  listed in Notes. Every import makes a new character; nothing
  is overwritten.
- **Import: custom names and properties (E1.3, E4.2).** A renamed item keeps its name on that copy, and
  its size, damage, weight and cost overrides and its other custom properties are kept on the copy,
  shown on the gear screen and the Weapons tab, and printed.
- **Import: nothing dropped (E1.4, E1.5).** An item Codex has no record for is imported as a Create item
  at its place. Everything the import could not map (unknown keys, classes, races, custom feats, eqmods
  kept as text) is listed in the character's Notes tab.
- **Starfinder import (E1.6, E4.14).** A Starfinder `.pcg` imports as a Starfinder character. A drone
  file is refused with a message to import the mechanic instead (`decisions.md §21`, Q-A).
- **Import totals match PCGen (E1.7, E4.13, E4.17, E4.18).** The imported character's totals were
  compared, row by row, against PCGen's own output for 7 sample characters (R-6). Every difference is
  fixed or written up with its reason in the parity ledgers (`scripts/oracle_harness/pcg_import_parity/`).
  Fixes reached every Pathfinder character, not only imported ones: armor check penalty on skills
  (masterwork and mithral lessen it), favored-class skill ranks and hit points, Skill Focus on its
  chosen skill, racial and class skill bonuses (Keen Senses, Sure-Footed, Trapfinding), Halfling Luck
  on saves, Small size on Stealth and Fly, a weapon's `+N` and wrong-size attack terms, and a Cleric's
  orison DC.
- **Ranged attack (E4.17, E4.20).** The sheet's Attack box and the printed Attack block show the ranged
  attack bonus beside the melee one.
- **Weapons by their own names (E4.15, E4.21).** Two copies of one weapon each get their own row with
  their own die and totals. A held weapon is printed under its custom name, or else the record's
  player-facing name (`Heavy Flail`, never `Heavy Flail (Base)`).
- **Gear screen (E4.7–E4.11, E4.16).**
  - A Bag of Holding's or Handy Haversack's contents add nothing to the carried weight; the bag's own
    capacity check still counts them.
  - An item can go only into a slot of its kind (menu, drag and the backend all refuse the rest); the
    slot override lifts this, and a save holding a wrong slot loads with a warning.
  - One Customize button per row renames that copy or attaches a modifier to it. Changing a copy's `+N`
    keeps its name, size, overrides and place.
  - Selecting a row shows a detail panel: the record (weight, cost, type, slot, weapon or armor stats,
    capacity, description, book) and the copy (name, size, modifiers, overrides, quantity, location).
  - Identical copies at one place are one row with a count, on screen and in the print. Move to asks
    "how many?" to split a stack; parts rejoin when put back together.
- **The campaign is a folder (E2.1–E2.3).** Create a campaign by choosing a folder, or open one you
  already have. The folder holds `campaign.json`, `Party Resources.md`, `Adventure Log/`, `Maps/`,
  `Wiki/` (with `NPCs/`, `Places/`, `Factions/`, `Items/`), `Characters/` and `Attachments/`. You sync it
  yourself (Syncthing, a cloud folder, git). The creator is member #1; members are names. The Google
  Drive settings tab and the Drive commands are gone.
- **Migration (E2.4).** A campaign from an earlier version (browser storage, or an old Drive-folder
  mirror) is moved into a new folder on first open. The old data is written first to
  `campaign-migration-backup-<date>.json` in the app data directory; nothing is deleted.
- **Four tabs (E2.6–E2.9).** Party Resources opens its file in the editor. The Adventure Log lists one
  file per session per author and makes a new entry. Maps shows an image with pins you place by click; a
  pin can name a Wiki page and opens it. The Wiki has folders, pages, `[[links]]`, backlinks and search.
- **Characters in the campaign (E2.10, E2.15, E4.3).** A character in a campaign is backed up into the
  folder's `Characters/` on each save, stamped with the device that wrote it and its save counter. Another device can open a
  backup read-only, import it as a stand-in, or import it as the moved master, and can compare its copy
  with the backup (shown, never merged). A backup written by another master is never overwritten. This
  device's name is editable in Settings.
- **Markdown editor (E3.1–E3.3).** A live-preview editor (CodeMirror) with a Reading view, a toolbar,
  `[[` completion, auto-save, a warning when the file changed on disk, and drag-in images. A file is
  saved byte-for-byte as you left it.
- **Help screen (E2.5).** Explains the folder layout, links to Obsidian, and shows the live tree of the
  open campaign.
- **Menus and screens (E2.12, E2.13, E2.14).** "Main menu" is the first ☰ entry on every screen but the
  main one. The Manager's open-campaign button is full width. The Print dialog and the DM console read
  the campaign folders.
- **Linux AppImage menu entry (E4.6).** Every AppImage start writes its own menu entry and icon naming
  the running file, so a moved or updated AppImage re-points them. `.deb`, `.rpm`, Windows and macOS are
  unchanged.
- **Linux on Ubuntu 24.04 (E4.12, #409).** 0.21.x was built on a host with glibc 2.43, so it could not
  start on Ubuntu 24.04 (glibc 2.39). The Linux release now builds in an `ubuntu:22.04` container: the
  glibc floor is 2.35, stated once in `tools/release/check_glibc_floor.py`, and the release gate fails on
  any shipped Linux binary above it (R-7). Fixes #409.

## Defects Fixed

- **#409: the Linux build needed glibc 2.43** (`decisions.md §28`). Fixed as above; a proof build of the
  `.deb` and the AppImage passed the gate on `tranche-22` (R-7).
- **Imported gear was wrong** (operator intake 2026-11-15; E1.1's diagnosis, `technical-design.md §1`).
  A named weapon imported as its base (a custom longsword as a plain Longsword); worn items lost their
  slot; one row per copy, capped at 99, so rations were listed many times; a `QUANTITY:0` line imported
  as one item; a Starfinder file was refused. All of these are fixed (E1.2–E1.6, E4.1, E4.2).
- **Alias weapons had no Strength on damage** (E4.MC). A `BASEITEM:` alias weapon (118 records, for
  example the heavy flail) now adds Strength to damage and uses its base's critical and wield.
- **Pathfinder records with two `TYPE:` tokens kept only one** (E4.18): 553 records now keep both.
- **Stored-XSS path in the Reading view** (E3.MC). A `javascript:` link split by a tab or prefixed by a
  control character passed the URL check; a shared campaign folder is untrusted input. Fixed.
- **The backup mirror's final rename** now retries once after 200 ms (E2.MC).
- **The Print dialog and DM console read legacy browser-storage campaigns** (E2.14); they read the
  campaign folders now.

## Operational Notes

- **Version.** The repo value is `0.22.0` on both desktop manifests (R-3). `docs/release/current-release.json`
  points to this file: `python3 tools/release/resolve_release_notes.py --repo-root .` prints its path.
- **New files on disk.** Per install: `campaign-folders.json` (the known campaign folders) in the app
  data directory. Per campaign folder: `campaign.json` (schema 2), `.codex/nonce`, and
  `Characters/<slug>-<id>/master.json` beside each backup. Old campaigns leave
  `campaign-migration-backup-<date>.json`, and an old `.config/<name>.json` is renamed `.json.bak`.
- **Commands.** 35 Tauri commands are new and 5 were removed (the `drive_*` four and
  `write_campaign_drive_artifacts`): 128 → 158 (R-4).
- **New npm packages** (E3.1, E2.5): the CodeMirror 6 and Lezer Markdown packages and `marked`, all MIT
  (E3.MC licence list, 11 of 11), and `@tauri-apps/plugin-opener` 2.5.3 (`MIT OR Apache-2.0`).
- **Release workflow.** The Linux publish job runs in `container: ubuntu:22.04`, its Rust cache key is new
  (`ubuntu-22.04-container`), and a `Gate Linux glibc floor` step runs on the staged `.deb` and AppImage
  before any upload. `linux-build-check.yml` repeats that build on any push or pull request that changes
  it, and publishes nothing.
- **Seeds and goldens.** The Pathfinder seed renders changed only where a card named the change (G-1):
  E4.17 added the ranged row and moved each seed's Dexterity skill totals by its worn chain shirt's armor
  check penalty, which is that card's correction. The Starfinder and Cyberpunk RED goldens were
  byte-identical at E4.MC (20 of 20 PDFs).

## Verification Evidence

- **Widest verify (E5.1, on `cfa9776b66`).** `scripts/verify.sh --full -j 8 --show-actuals` gave
  `RESULT: PASS` on every stage. The floors it recorded are in R-5.
- **Proof runs (E4.5).** The CI-shape import suites passed with the oracle at its pin: 7 of 7 samples
  compared. The operator's own file was imported locally only, and its receipt holds counts, never its
  contents: 89 gear lines, 90 stacks, 217 pieces conserved, 0 capped.
- **Parity (E4.13, E4.17).** Desktop parity 6 of 6 builds equal or explained; the E4.13 and E4.17 ledger
  rows went 9 → 0 and 49 → 0 (their receipts). The ledgers hold 20 and 16 explained rows (R-6).
- **Linux proof build (E4.12).** `linux-build-check.yml` run 44512053205 on `tranche-22` succeeded; its gate
  printed `codex-desktop max GLIBC_2.35 floor 2.35 OK` and `pcg_import max GLIBC_2.34 floor 2.35 OK` for
  both the `.deb` and the AppImage (R-7).
- **Final-acceptance scan (E5.2).** G-8 and G-9 clean; the operator's file never entered the repo (both
  `-S` greps → 0).
- **ui-smoke.** The spec grew from 203 rows to 223 (R-8); E4.4 ran its new and rewritten rows by name.
- **Operator gate (E5.G).** Waived by the operator on 2026-11-20 ("Skip e5.g"). The three `usable: yes`
  lines exist only as the lead's record of the waiver (R-2). E5.1's fresh-install check moved, by
  operator ruling, to the alpha built after the merge.

## Known Issues

- **Nobody has used it yet.** The import, the campaign folder and the editor were checked by tests and
  by agent screenshots only (E2.MC 19, E4.MC 22). The first human look is the post-merge alpha.
- **Deferred by ruling (`forward-scope-register.md`).**
  - No GM / player roles and no GM-only secrets in a campaign folder (FSR-C1, FSR-C2).
  - A stand-in's changes are merged back by hand; Codex shows the diff and never merges (FSR-C3).
  - Companions, familiars, mounts and drones are not attached to an imported character (FSR-C4).
  - No Cyberpunk RED import: there is no `.pcg` for it (FSR-C5).
  - No pin rendering inside Obsidian (FSR-C6).
  - You cannot remove a modifier attached to a held copy (FSR-C9).
  - Eqmod prices the converter does not evaluate stay unpriced, and a Create item's eqmods are text
    (DEF-1).
  - ui-smoke has no drag step; drag-in images are not covered (DEF-2).
  - Tables are aligned text while editing; the Reading view renders them (DEF-3).
  - Obsidian syntax beyond `[[links]]`, image embeds and callouts shows as plain Markdown (DEF-4).
  - No file watcher: campaign lists refresh on open and on Refresh (DEF-5).
- **Named and not fixed (epic reports in `progress.md`).** The Help screen ignores Escape. Opening a
  campaign from the sheet took 10–30 s on the test box (E2.MC; not re-measured). A second "Move to" menu opens without closing
  the first. A `TEMPLATESAPPLIED:[NAME:Human]` line imports as a custom feat named `[NAME:Human]`. The
  editor's task-box widget toggles text on a read-only (over 2 MB) file (the change is not saved).
- **Ubuntu 20.04 and other hosts below glibc 2.35 are not supported.** The published 0.21.x assets are
  not rebuilt; the fix arrives with the first 0.22 alpha.

## Does not cover

- macOS and Windows builds; the glibc gate covers Linux only.
- The operator's file is one file; other real `.pcg` files are covered only by the 7 PCGen samples.
- The built `pcg_import` sidecar spawned by a packaged app (the tests link the converter in-process).
- Real multi-device sync of a campaign folder through a sync tool; the backup and master checks are
  tested on one disk only.
- R-1 changes with every later card. It is pinned to the tree at E5.3's closing commit.

## Figures

- **R-1** Kanban rows, and rows `complete`, at E5.3's closing commit: **61 60**.
  `awk -F'|' '{ gsub(/\\\|/, "") } $2 ~ /^ (C|E)[0-9]/ {n++; if ($5 ~ /^ complete *$/) c++} END{print n, c}' docs/release/SD-42-pcgen-import-and-campaign-folder/kanban.md`.
  The one row that is not `complete` is E5.4: graphify and the PR.
- **R-2** `usable: yes` lines, all marked WAIVED: **3**.
  `grep -c '^- \(Import ([^)]*)\|Campaign Manager ([^)]*)\|Editor\) — usable: yes' docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_5/E5.G_cycle_receipt.md`.
- **R-3** Desktop manifest versions: **0.22.0,0.22.0**.
  `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,`.
- **R-4** Registered Tauri commands: **158**, from 128 at the merge base `d6890a6c73` (SD-41 merged).
  Implementation A: Python, `re.search(r'generate_handler!\[(.*?)\]', t, re.S)` over `git show <rev>:apps/desktop/src-tauri/src/main.rs`,
  `//` comments stripped, split on `,`; the set difference gives 35 added (34 in `campaign_folder.rs`
  and `describe_held_equipment` in `character_hub.rs`) and 5 removed (`drive_delete_campaign`, `drive_list_campaigns`,
  `drive_load_campaign`, `drive_save_campaign`, `write_campaign_drive_artifacts`).
  Implementation B: `awk '/generate_handler!\[/{f=1} f{print} f&&/\]\)/{exit}' apps/desktop/src-tauri/src/main.rs | sed 's#//.*##; s/.*generate_handler!\[//; s/\].*//' | tr ',' '\n' | awk 'NF' | awk 'END{print NR}'` → 158.
- **R-5** Test-count floors recorded by E5.1 (last assignment in `scripts/verify-baselines.env`): root lib
  3211, root full 6909 across 300 suites, ingest 1922 across 187 binaries, desktop 1577, frontend 216
  files; clippy 0 warnings in all three workspaces.
  `awk -F= '/^BASELINE_(ROOT|INGEST|DESKTOP|FRONTEND|CLIPPY)/{v[$1]=$2} END{for(k in v) print k"="v[k]}' scripts/verify-baselines.env | sort`.
- **R-6** PCGen sample exports and explained parity rows: **7** exports;
  `ls scripts/oracle_harness/pcg_import_parity/exports/ | awk 'END{print NR}'`. Explained rows, header
  excluded: **20** in `explained.tsv` and **16** in `explained_desktop.tsv`;
  `awk 'NR>1 && NF' scripts/oracle_harness/pcg_import_parity/explained.tsv | awk 'END{print NR}'` (and the same for `explained_desktop.tsv`).
- **R-7** The floor's one home: `grep -c 'GLIBC_FLOOR = "2.35"' tools/release/check_glibc_floor.py` → **1**;
  the workflows name the image, never the number: `grep -n '2\.35' .github/workflows/publish-tester-release.yml .github/workflows/linux-build-check.yml | awk 'END{print NR}'` → **0**.
  The proof run: `gh run list -w linux-build-check.yml -b tranche-22 -L 1 --json conclusion,url` →
  `success`, `https://github.com/electricm0nk/codex/actions/runs/44512053205`; its gate lines are in the
  E4.12 receipt (`gh run view 44512053205 --log | grep -E 'max GLIBC_|== Codex'`).
- **R-8** ui-smoke spec rows: **223**, from 203 on `origin/develop`.
  `python3 -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"`;
  the base: `git show origin/develop:apps/desktop/scripts/ui-smoke/spec.json | python3 -c "import json,sys;print(len(json.load(sys.stdin)['rows']))"`.

## Update Eligibility

Update eligibility is set at publish time by the release manifest (`tools/release/check_release_manifest.py`).
The repo keeps the version placeholder until then. This document does not decide eligibility. The first
tester publish of 0.22.x comes after the operator merges `tranche-22`; it is the first build that starts
on Ubuntu 24.04, and the AppImage writes its own menu entry on its first start.
