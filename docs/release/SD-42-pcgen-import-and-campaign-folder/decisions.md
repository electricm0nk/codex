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

# SD-42 Decisions

Each entry is **OPERATOR** (a ruling, quoted), **DEFAULT** (the author's call, revisited at `§21` or by the
operator before launch) or **STANDING** (a rule carried from earlier bundles). `§20` wins where this file
differs from any other file in the package.

## §1 — Package location

`docs/release/SD-42-pcgen-import-and-campaign-folder/` (DEFAULT: the house path every sibling uses; the chassis'
`docs/stc/` naming is still unadopted in this repo — `ls docs/stc 2>/dev/null | awk 'END{print NR}'` → 0).

## §2 — Branch, board, version

`tranche-22`, cut 2026-11-16 from `origin/tranche-21` @ `1046daefc0` (`git merge-base HEAD origin/tranche-21`) while
SD-41 still runs there (`artifacts/epic_0/census.json` `sd41_open_cards`: E3.12, E4.MC, E4.G, E5.1–E5.4 of 56).
Board: `./kanban.md`. Version `0.22.0`: C1 sets it in one commit on the 14 surfaces SD-41's C1 bumped
(`git show --name-only --format= c817ad502f | awk 'NF' | grep -v '^docs/' | awk 'END{print NR}'` → 14) and points
`docs/release/current-release.json` at this package's interim notes. The published build is `0.22.<run>`.

## §3 — Operator rulings on record (verbatim, `artifacts/cycle_0/operator-brief-2026-11-16.md`)

| Ruling | Text (operator, 2026-11-16 unless dated) | Carried by |
|---|---|---|
| Intake 1 (2026-11-15) | "PCGen import is inaccurate, gear worst. Weapons came from the default character (longsword), not the imported file. Armor: some came, not all." | §5, §6, §7; E1.1–E1.4, E4.1, E4.2 |
| Intake 2 (2026-11-15) | "Stacks not collapsed: 'Rations, trail' listed many times instead of once with a quantity (10, 50...)." | §5; E1.2, E4.1 |
| Intake 3 (2026-11-15) | container tables — **shipped in SD-41 E3.11; excluded** | — |
| Q2 | "custom PCGen items keep the custom NAME and EVERY custom property; Codex must support all of them (not base-item fallback, not text-only)." | §6; E1.3, E4.2 |
| Q3 | "import proof set = operator's Aonghus file + PCGen's own sample .pcg files at the pinned oracle (not all-found, not Aonghus-only)." | §9, §10; E1.1, E1.7, E4.5 |
| Q4 | "SD-42 checks the WHOLE imported character (classes, race, abilities, HP, skills, feats, spells, gear) against PCGen, INCLUDING customizations — not gear only." | §9; E1.7 |
| Q5 | "anything the import cannot map directly is still imported (kept, not dropped) AND listed in the character's NOTES section. SD-42 then closes the gaps for the 5-file proof set." | §7; E1.4, E1.5 |
| Q6 | "stacks = ONE row with a count + total weight, splittable (sell/drop/move part of a stack); applies to ALL stacks (Market buys too), not only imports; stored as one item with a quantity, not N items." | §5; E1.2, E4.1 (Market buys: SD-41 E2.5, re-verified by E4.1's acceptance) |
| Q7 | "SD-42 import covers Pathfinder AND Starfinder, same whole-character accuracy check. Proof set = Aonghus + pf_Cleric/pf_Paladin/pf_Rogue/pf_goldielocks + sf_mechanic/sf_mechanic_drone/sf_soldier (under $PCGEN_REPO_DIR/code/testsuite/PCGfiles)." | §8, §9; E1.6, E1.7 |
| Q8 | "fixed import makes a NEW character (old copy untouched). Aonghus is a TEST file only — must NOT ship as a default/starter character." | §10; E4.1, E4.5 |
| Q9 | "Aonghus .pcg stays ONLY on this machine (/home/ubuntu/sd42-input/), never committed (repo is public; holds player name, DM notes). Tests that need it must skip-with-a-loud-reason when absent; CI proves only the 7 PCGen samples. Local proof run is required evidence for closure." | §10; E4.5, E5.G, G-8 |
| CM-1 / Q11 | "campaign shows '0 members'; creator must always be member #1." / "creator saved as member #1 (no roles yet). GM/Player roles are a planned later feature — operator wants to design how roles work first. Do not build roles in SD-42." | §13.1; E2.2 |
| CM-2 | "build out the stubbed Party Resources, Adventure Log, Maps, Wiki. Campaign = plain FOLDER of files; sync is agnostic and handled OUTSIDE Codex (LAN, Google Drive, Dropbox, Syncthing...). 'First and foremost free and open.' Content = Markdown, Obsidian-compatible ([[wikilinks]], frontmatter, one file per entry); simple in-app editor (text + preview); Obsidian usable on the same folder." | §11, §15, §18 |
| Q12 | "REMOVE the Google Drive API path (campaign_drive.rs); folder is the only way. Members become a plain typed list; invites happen outside Codex. Add a ROBUST help/info screen explaining folder sharing: Google Drive as the worked example, explicitly any tool works, cite Syncthing and Dropbox." | §12, §13; E2.3, E2.5 |
| Q13 | "old campaigns MIGRATE on first open: ask for a folder, write each entry as its own .md, keep a backup of the old data file. Help screen also shows the created folder structure and says the files can be edited in Obsidian or any Markdown editor, with a link to https://obsidian.md." | §14, §13.3; E2.4, E2.5 |
| Q14 | "Party Resources: shared loot + gold. Codex creates `Party Resources.md` with a STARTER table (suggested columns) and it stays FULLY free-text editable (Codex editor or any md editor). No structured parsing / move-to-character required." | §15.1; E2.6 |
| Q15 | "Adventure Log: ONE FILE PER SESSION PER PERSON. '+' button makes a file with default frontmatter so it shows in the Codex screen, fully editable afterwards. Frontmatter: campaign, author, posting date, session date as a single date OR a range (must be flexible). Codex must tolerate hand-edited/missing frontmatter." | §15.2; E2.7 |
| Q16 | "`session` = REAL play date (single or range), used for sorting. Plus an OPTIONAL free-text in-game date field (shown, not sorted/parsed)." | §15.2; E2.7 |
| Q17 | "Maps: image (png/jpg in Maps/) + same-name notes .md + PINS NOW: click map to drop a pin, each pin links to a wiki page; pins stored in the notes file frontmatter (Obsidian won't render them — accepted)." | §15.3; E2.8 |
| Q18 | "Wiki: free folders with starter folders Wiki/NPCs, Places, Factions, Items (add/rename/delete allowed); '+' makes a page with small frontmatter (title, type, tags); [[links]] clickable; backlinks ('pages that link here'); search by name + text." | §15.4; E2.9 |
| Q19 | "NO secrets in Codex for now; everything in the shared folder is shared; help screen says so plainly; GM keeps secrets in their own non-shared folder. Secrets wait for the later roles design." | §13.3; E2.5 |
| Q20 | "characters go in a shared `Characters/` folder. Why: players cover for absent players / players who leave; GM must see party sheets to model encounters in the DM toolkit (DM toolkit must read party from the folder)." | §16, §17; E2.10, E2.11, E4.3 |
| Q21/Q22 | "local copy is the ONE master; every save writes BOTH local and `Characters/` backup in the campaign folder (auto, not manual). Others/GM read the backup; it can be re-imported. Two-master trap, BOTH checks: (1) backup records master device (per-install ID + readable name); importing on another device asks 'Play as stand-in' (default; never writes backup) or 'Move the master here'; (2) save counter in the backup — before writing, Codex checks the counter matches what it last wrote; mismatch → do NOT overwrite backup, warn who/when changed it, local save still succeeds. Reuse campaign `save_with_conflict_detection` (src/campaign/local_store.rs) pattern. Stand-in changes are merged back by hand (Codex may show a diff, no merge)." | §16; E2.10, E4.3 |
| Q24 | "in-app editor: LIVE PREVIEW like Obsidian (CodeMirror 6, MIT; same lib as Obsidian). Must not rewrite the user's Markdown on save (byte-stable round-trip). `[[` suggests wiki page names. App has NO markdown lib today (checked package.json 2026-11-16)." | §18; E3.1, E3.2 |
| Q25 | "editor extras: ALL — toolbar (bold, italic, heading, list, link, table); auto-save + warn if file changed on disk during edit; drag-in images saved to `Attachments/` and linked." | §18; E3.3 |
| UI-1 (2026-11-17, relayed by the lead while run wf_141acf96-f25 was live) | "the app menu's FIRST option is 'Main menu' on every screen that shows the menu." | §26.1; E2.12, E4.4 |
| UI-2 (2026-11-17) | "the button that opens a campaign must be much larger: full column width, same colour/style as the main screen's Campaign Manager button." | §26.2; E2.13, E4.4 |
| LNX-2 / #409 (2026-11-17, GitHub issue by an external reporter 2026-11-16; operator ruling relayed by the lead) | Issue title: "Requires glibc-2.43 on Ubuntu 24". Operator: "add an SD-42 card. post now that this has been assigned to relase v0.22.x." (the lead posted the public comment on #409 the same day) | §28; E4.12, E5.3, E5.4 |
| LNX-1 / Q27 (2026-11-17) | "no app icon after installing the AppImage on Arch." / "Codex itself writes its menu entry + icon (~/.local/share/applications, icon dir) on first start when running as an AppImage on Linux, and re-points the path after an update. No extra tool required." | §26.3; E4.6 |
| BAG-1 / HVS-1 (2026-11-17, bug) | "a bag of holding is treating it's contents as cummulative from a weight perspective instead of treating them as weightless" / "the same goes for handy haversack" | §27.1; E4.7, E4.4 |
| SLOT-1 (2026-11-17, bug) | "bug: you can put anything into any slot. I added a backpack to a shield slot." | §27.2; E4.8, E4.4 |
| REN-1 (2026-11-17, enhancement) | "enhancement: we need the ability to rename an object in inventory. ie: Sack #1 could be renamed to Camping Gear" | §27.3; E4.9, E4.4 |
| CUS-1 (2026-11-17, enhancement) | "the gear screen shows the equip view with move/drop/sell/discard. And then the same list again with attach modifier. On the equip section, add a customize button which allows custom naming or adding modifiers. Then you can drop the redunant view below the equip section with the modifier buttons." | §27.3; E4.9, E4.4 |
| STK-1 (2026-11-17, bug) | "i purchased four sacks. This was listed as Sack x 4, four distinct times in the equip section as carried. In the second view it just shows Sack, Sack, Sack, Sack. It's ok to list an empty container as Sack x 4. But do not list Sack x 4, 4 times - leading one to believe there are 16 sacks. If one of the sacks has something inside, you would show Sack, and Sack x 3. Stacked items also should be able to be split: Sack x 2, Sack x 2. Or Arrows x 50, Arrows x 50 instead of Arrows x 100. That way some arrows could be put in a quiver and others in a backpack." | §27.5; E4.11, E4.4 |
| DET-1 (2026-11-17, enhancement) | "enhancement: when selecting a peice of gear, we need to display all the information about that gear. Weight/Cost/Stats/Description/Slots/etc" | §27.4; E4.10, E4.4 |

Q10 and Q23 do not appear in the intake file (its numbering runs Q2–Q9, Q11–Q22, Q24, Q25). The author
treats them as never asked; the operator confirms at `§21` Q-L that nothing was lost.

## §4 — Phasing against SD-41 (DEFAULT; the partition is derived, not assumed)

SD-41 is live on `tranche-21` in `.worktrees/sd41-main` (never written by SD-42). Its open cards and the files
they own (`../SD-41-gear-and-equipment-sets/kanban.md` rows not `complete`; the file sets from its
`workflow-instruction.md §3` and the E3.12 row): **E3.12** — `apps/desktop/src-tauri/src/character_hub.rs`
(`list_custom_equipment_options`), `apps/desktop/src/characterHub/BuyCustomDialog.tsx`; **E4.MC, E4.G** —
`artifacts/` only; **E5.1** — `scripts/verify-baselines.env`; **E5.2** — `docs/retro/**`; **E5.3** —
`docs/architecture/**`, SD-41's `release-notes.md`; **E5.4** — the graph outputs, the PR.

- **Phase 1 (now, three lanes):** nothing under `apps/desktop/src-tauri/src/character_hub.rs`,
  `apps/desktop/src/characterHub/BuyCustomDialog.tsx`, `docs/architecture/**`, `scripts/verify-baselines.env`.
  The lanes: `import` (E1: `crates/codex-ingest/src/pcg_import/**`, `crates/codex-ingest/src/bin/pcg_import.rs`,
  `crates/codex-ingest/tests/**` *(new files)*, `src/saved_character/equipment_sets.rs` (new `CopyFacts` fields
  only), `src/rules_core/damage_total.rs` (a damage override read), `scripts/oracle_harness/**` *(new files)*,
  `docs/architecture/pcg-import.md` **excluded** until Phase 2); `campaign` (E2: `src/campaign/**`,
  `src/saved_character/campaign_mirror.rs`, `apps/desktop/src-tauri/src/campaign_folder.rs` *(new)*,
  `apps/desktop/src-tauri/src/campaign_drive.rs` *(deleted)*, `apps/desktop/src-tauri/src/main.rs` (registration lines),
  `apps/desktop/src-tauri/src/encounter_rating.rs`, `apps/desktop/src-tauri/capabilities/default.json`,
  `apps/desktop/src/campaign/**`, `apps/desktop/src/settings/{GoogleDrivePanel.tsx,googleDrive.ts,SettingsModal.tsx}`,
  `apps/desktop/src/boundary/{listDriveCampaigns,writeCampaignLocalFolderArtifacts}.ts` *(deleted)* and new
  boundaries, `apps/desktop/src/characterHub/{CharacterHubPage.tsx,campaignMembershipModel.ts,CampaignMembershipDialog.tsx}`,
  `apps/desktop/src/dmToolkit/**`); `editor` (E3: `apps/desktop/src/editor/**` *(new)*, `apps/desktop/package.json`,
  `package-lock.json`).
- **Phase 2 (after SD-41's PR merges to `develop` and the lead rebases `tranche-22` onto `develop`, SD-m):**
  E4 — `character_hub.rs` (the import writes `equipment_sets.json`, the copy-property DTOs, the mirror hooks),
  `CharacterSheet.tsx`, `GearScreen.tsx`, `gearScreenModel.ts`, `print/blocks/gear.rs`, `spec.json`,
  `docs/architecture/**`.
- `main.rs` is owned by no open SD-41 card, so E2's registration lines are Phase 1; a rebase conflict there is
  SD-k (take upstream, re-apply the added lines).
- The prefix (`workflow-instruction.md §2.1`) refuses `tranche-21` as the branch and `sd41-*` as the tree; the
  Phase-1 partition check is `git diff --name-only <base>..HEAD -- apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src/characterHub/BuyCustomDialog.tsx docs/architecture scripts/verify-baselines.env | awk 'END{print NR}'` → 0.

## §5 — Import: one stack is one row; the wire grows a `equipmentSets` block (OPERATOR Q6; DEFAULT on the shape)

**Today** (`crates/codex-ingest/src/pcg_import/map.rs`, 1,033 lines): `read_item` (`map.rs:500`) emits one
`EquipmentWire` row **per copy** (`map.rs:534-535`, `self.equipment.extend((0..worn).map(|_| row(WORN)))`),
capped at `MAX_COPIES = 99` (`map.rs:156`); `quantity()` (`map.rs:920`) rounds `QUANTITY:0.0` **up to 1**; the
only locations are worn / carried (`read_equipment_sets`, `map.rs:236-262`): a slot, a container and nesting
are not read. `import_pcg_json` (`character_hub.rs:7973`) writes no `equipment_sets.json`, so SD-41's Default
migration (`equipment_sets::default_from`, `equipment_sets.rs:267`) makes every copy its own stack of one
bundle — 84 rows of rations (the operator's file: `EQUIPNAME_max_quantity` 84, `census.json`).

**DEFAULT:** the wire (`wire.rs`) gains `equipmentSets: { active: "<set name>", copies: [ { key: {itemId,
bonus, ordinal}, location: <Location JSON as equipment_sets.rs serialises it>, quantity, name?, size?, properties?
} ] }` — one entry per `EQUIPNAME` line, `quantity` = the line's `QUANTITY` (a `0` line is not imported and is
listed in Notes, `§21` Q-I), `location` from the `EQUIPSET` tree of the `CALCEQUIPSET` set: a depth-3 entry
whose name is a body-slot word → `Slot` (PCGen's slot names mapped to `BodySlot` by one table in `map.rs`,
unknown → `Equipped`), `Carried` / `Not Carried` → those, deeper entries → `Inside { container }` of the
entry above them. `equipmentSelections` keeps **one** row per line (worn → `EquippedActive`, else
`SelectedInactive`), so the engine input stays a list of items and the piece count lives where SD-41 keeps it
(`CopyFacts.quantity`). The desktop side (E4.1) writes `equipment_sets.json` from the block through
`equipment_sets::save` before the first load, so `default_from` never runs on an import. Stack identity is per
line: two `EQUIPNAME` lines of the same base at different locations stay two stacks (`§21` Q-G).

A sample holds a `QUANTITY` of 100,050 (`sf_mechanic`, `census.json`): `quantity` is `u32` end to end; no cap.

## §6 — Custom items keep the name and every property (OPERATOR Q2)

**Today:** `read_item` looks the key up **base item first** (`map.rs:510`: `[base_item, DATA.KEY, name]`), so the
custom `NAME` ("Mourn") is never carried; `equipment_modifiers` (`map.rs:960`) keeps only `PLUS<n>W/A/S` and
reports every other eqmod as "not applied"; `SPROP`, `_DAMAGE|2d4`, `ALTEQMOD`, `COST`, `WT`, `SIZE` and
`COSTMOD` are dropped. The operator's file carries 36 `NAME`, 17 `SPROP`, 11 `EQMOD`, 13 `KEY`, 1 `ALTEQMOD`
fields across 38 customised lines (`census.json` `operator_file.DATA_field_keys`); the samples add `SIZE` (12)
and `COSTMOD` (2).

**DEFAULT — the property map** (E1.3; one table in `map.rs`, printed in `docs/architecture/pcg-import.md` at E4.MC):

| PCGen property | Codex field (computes from) | When Codex has no such field |
|---|---|---|
| `NAME=` | `CopyFacts.name` (SD-41 §24: the per-copy custom name; shown on the gear screen, Weapons tab, print) | — |
| `EQMOD=Special Ability ~ +N ~ Weapon/Armor/Shield` (`PLUSnW/A/S`) | `applied_modifiers` (as today) | — |
| `EQMOD=Material ~ <m>`, `Special Quality ~ Masterwork ~ …`, `Special Ability ~ <ability>` | the eqmod keys SD-41's Buy Custom takes (`list_custom_equipment_options`, `purchase_custom_equipment_at_root`, `character_hub.rs:4309`): the copy is written as a Buy Custom build (base + material + masterwork + abilities), priced and weighed by the same code | an eqmod the option list does not hold → `properties` text + Notes |
| `SIZE=` | `CopyFacts.size` (SD-41 E2.12) | — |
| `_DAMAGE|<die>` / `ALTEQMOD=_DAMAGE|<die>` | `CopyFacts.damage_override` *(new)*: the Weapons row prints this die instead of the record's (`damage_total.rs`, one read; a dice term stays literal — the paper-sheet rule, `§24`) | — |
| `COST:` on the line when it differs from the record's | `CopyFacts.cost_override_copper` *(new)*: Sell and the printed value use it | — |
| `WT:` when it differs from the record's | `CopyFacts.weight_override_lbs` *(new)*: encumbrance and the container sums use it (one read in `encumbrance.rs` / `containers.rs` beside `quantity`) | — |
| `SPROP=` | `CopyFacts.properties: Vec<String>` *(new)*: printed under the item on the gear screen and the Inventory page; never computed from | — |
| `COSTMOD=`, `KEY=` | `KEY` is a lookup candidate only; `COSTMOD` folds into `cost_override_copper` | — |
| anything else in `DATA` | `properties` text + a Notes line "`<item>`: `<field>` kept as text" | — |

`CopyFacts` gains the four fields with `#[serde(default, skip_serializing_if = "Option::is_none")]`, so every
existing `equipment_sets.json` and SD-41's byte-identical seed renders hold (G-1). The engine reads
`damage_override` in one place; the twin rule (`§24`) forbids a second.

## §7 — Nothing dropped: unmappable items become Create items; everything unmapped is in Notes (OPERATOR Q5)

**Today:** an item whose base the catalog lacks becomes a text-only `custom.equipment` record ("An item Codex
has no record of", `map.rs:603`), never held, never weighed. **DEFAULT:** such a line becomes a **Create item**
(SD-41 §25: `CustomRecordDto` with `weight_lbs`, `price_gp`, `container_capacity_lbs` from the line's `WT`,
`COST` and whether the `EQUIPSET` tree places anything inside it) held at its location with its quantity;
the Notes block names it with the reason ("no Codex record for `<base>`"). Every other unmapped line (an
unknown key, a class or race Codex lacks, a spell not kept, an eqmod kept as text) is a Notes line too. The
block is written by the sidecar into `bio.notes` (`BioWire` gains `notes`; `CharacterBioDto.notes` already
exists with `#[serde(default)]`, `character_hub.rs:7218`), under a heading `## Imported from PCGen
(<file>, <date>)`, appended after any existing notes text (an import is a fresh character, so there is none).
The import report panel keeps showing the same lines.

## §8 — Starfinder import (OPERATOR Q7; DEFAULT on the mechanism)

**Today:** `finish` (`map.rs:635`) warns and reads an SF file against Pathfinder records; `fixtures_tests.rs:386`
**asserts** that `sf_soldier.pcg` cannot compute. **DEFAULT:** `convert` branches on `GAMEMODE`: `Starfinder` →
an SF `Catalog` (classes, themes, races, feats, skills and equipment read from `data/starfinder-1e/**`, the
tables `sf_adapter.rs` already reads) and an SF `CharacterInput` (the same struct; `source_package_id` the SF
package id; `game_system` `GameSystem::Starfinder1e.id()` on the envelope, which `rule_system_adapter.rs:185`
already routes to the SF adapter). The desktop's `import_character_from_json` takes `gameSystem` from the
wire (default `pf1`). Proof: `sf_mechanic` and `sf_soldier` import and compute; the drone file per `§21` Q-A.
The `fixtures_tests.rs:380` test flips to "an SF class computes".

## §9 — Whole-character parity against the PCGen engine (OPERATOR Q3, Q4)

The real PCGen engine runs at the pinned oracle (`crates/codex-ingest/tests/common/mod.rs:18`
`pcgen_engine_runnable`; `scripts/pcgen-run-character.sh`; `scripts/oracle_harness/sheet-totals.txt.ftl` and
`sf_parity.txt.ftl` already export a character's totals). **DEFAULT (E1.7):** `scripts/oracle_harness/pcg_import_parity/`
*(new)*: `run.sh` exports, for each proof file, PCGen's totals (classes and levels, race, the six scores, HP,
saves, AC, melee and ranged attack, every skill total, feats, spells, every gear line with quantity and
location, money) with the existing templates; `compare.py` imports the same file through `pcg_import` and
the engine (`crates/codex-ingest/tests/pcg_import_parity.rs` *(new)*, the shape of `sf_oracle_parity.rs`) and
writes one row per field: equal, or a difference. A difference is **fixed** by a discovered card or
**explained** in `explained.tsv` (build, field, both values, the rule and the decision). A stale row fails
(the SD-37 rule). The seven samples' exports are committed under `scripts/oracle_harness/pcg_import_parity/`;
the operator's file's export is written under `/home/ubuntu/sd42-input/parity/` and never committed (`§10`).
Skips loudly when the oracle is absent or the engine not runnable (`pcgen_engine_runnable`).

## §10 — A new character every import; the test file is local only (OPERATOR Q8, Q9)

`import_pcg` already mints a fresh UUID (`character_hub.rs:8015-8022`); E4.1 keeps that and adds a test that
an import never touches another root. The operator's file is read from `$CODEX_SD42_PCG` (default
`/home/ubuntu/sd42-input/Aonghus Macleod.pcg`); every test or script that reads it prints
`SKIPPED: operator .pcg absent at <path> (decisions.md §10)` and passes when it is absent; CI never has it;
`.gitignore` is **not** changed (the file is outside the repo). No fixture, seed, screenshot or receipt in the
repo may carry a line of the file; receipts carry **counts** (`census.py` prints only counts). The local
proof run (E4.5) is closure evidence: its receipt names the command and the counts.

## §11 — A campaign is a folder (OPERATOR CM-2; DEFAULT on the layout)

```
<campaign folder>/
  campaign.json            name, ruleSetId, ruleSetLabel, description, members[{name, addedAt}], createdAt, updatedAt, schemaVersion 2
  Party Resources.md       Q14 starter table
  Adventure Log/           one .md per session per author (Q15)
  Maps/                    <name>.png|jpg + <name>.md (pins in frontmatter, Q17)
  Wiki/NPCs/ Wiki/Places/ Wiki/Factions/ Wiki/Items/   starters; free folders (Q18)
  Characters/              <slug>-<characterId>/ backups (Q20–Q22; was `characters/` in SD-41 §29 — renamed, §16)
  Attachments/             dragged-in images (Q25)
  .codex/nonce             the conflict-detection counter `CampaignStore::save_with_conflict_detection` keeps
```

`campaign.json` at the root replaces `.config/<name>.json` (`§21` Q-K); `CampaignSnapshot` (`src/campaign/mod.rs`)
loses `assets` (files are the assets) and `party_character_ids` (the party is `Characters/`), gains
`members[{name, addedAt}]` (`§13.1`) and `schema_version: 2`; `CampaignStore::save_with_conflict_detection`
(`local_store.rs:194`) is the write path for `campaign.json`. Files are read and written only through Rust
commands in `apps/desktop/src-tauri/src/campaign_folder.rs` *(new; the capability set stays `core:default`,
`dialog:default` plus `opener:default` for the help link — no fs plugin)*. **Known folders** live in the app's
data root as `campaign-folders.json` (a list of absolute paths with the last-opened time); the Manager lists
them, "Open folder…" adds one by the dialog, "New campaign…" creates one in a picked parent folder.

## §12 — Run protocol (UNATTENDED after launch)

The standing autonomous rules apply (memory: "operator away — autonomous mode", "run to 100% DoD without
asking"):

1. Clear a blocker first; if only an operator ruling can clear it, mark **only that card** `blocked-escalated`
   with the exact ruling under `progress.md ## Open blockers`; continue every independent card.
2. Never open the PR while any card is open, partial or blocked-escalated.
3. A choice inside scope with a safe default (§12.1): take it, log it, continue. No `AskUserQuestion`.
4. **Report after each epic** (the merge-check card writes the Epic report block in `progress.md`).
5. **One planned wait:** E5.G. The run proceeds past it only on the operator's `usable: yes` lines. Defects
   found there are discovered cards (§19), not blockers to argue with.
6. **Two phases** (§4): the Phase-1 run ends after E1.MC, E2.MC and E3.MC; the Phase-2 run starts with
   `START=E4.1` after the catch-up rebase.

### §12.1 Safe defaults (taste questions never stop a card)

| # | Decision point | Safe default |
|---|---|---|
| SD-a | A PCGen slot word with no `BodySlot` (PCGen's `Fingers` vs the app's two ring places, `Shoulders`…) | the one table in `map.rs` maps every PCGen location name PCGen's `system/gameModes/Pathfinder/equipmentslots.lst` lists (the card quotes the count); an unlisted word → `Equipped`, logged once per import in Notes |
| SD-b | A worn item whose slot the engine refuses (`slot_conflict`) | the import keeps the location the file says; the engine's existing diagnostic shows on the sheet; the parity row explains it (a PCGen file can hold what the rules refuse) |
| SD-c | Two `EQUIPNAME` lines of one base at one location | two stacks (the file's shape); never merged on import |
| SD-d | A `QUANTITY:0` line | not held; one Notes line ("`<item>`: 0 copies in the file") |
| SD-e | An eqmod Codex's option list does not hold | kept as a `properties` text and a Notes line; the item still imports with its other properties |
| SD-f | A set name collision, a folder name collision | refuse with a diagnostic; never auto-suffix |
| SD-g | The Markdown renderer for the reading view | `marked` 18.x (MIT) with raw HTML escaped; no sanitiser dependency (`§18`) |
| SD-h | ui-smoke rows for folder flows | every row uses a per-run folder under `{XDG_DATA_HOME}` (the harness substitutes it, SD-41 E3.3); the 8 old campaign rows are rewritten, none deleted |
| SD-i | Usage-limit stop | stop dispatching, write a resume receipt |
| SD-j | No CI on `tranche-22` pushes | CI evidence = the closure PR's `pr-tests`, awaited by E5.4 |
| SD-k | Rebase conflict | `kanban.md`: take upstream, re-apply only your own row, run the duplicate-ID check; `progress.md` append-only sections: keep both; `main.rs` registration: take upstream, re-add the lines; any other file: abort, `blocked-escalated` |
| SD-l | No display on this box | the `run-desktop` skill's screenshot path with an isolated `XDG_DATA_HOME` (R5); PNGs under `artifacts/epic_5/E5.G/` are what the operator looks at first; the operator's own app is the judge |
| SD-m | SD-41 merges to `develop` between the phases | the lead rebases `tranche-22` onto `develop` once, re-runs E1–E3's tests, logs it in `progress.md`; never mid-card |
| SD-n | Screenshot and PDF artefacts | PNG at the app's window size, ≤ 8 images per card, ≤ 2 MB per card directory (`du -sk`) |
| SD-o | A frontmatter the parser cannot read | the file is listed with its filename as the title and a "frontmatter not read" mark; never hidden, never rewritten |
| SD-p | A pin whose wiki page is missing | the pin shows the page name and "page not found — create it"; one click creates the page under `Wiki/` |
| SD-q | A `Characters/` backup whose `master.json` is missing (an SD-41 `characters/` copy) | read as a stand-in with no master; opening it offers "Move the master here"; a save from the original master rewrites it with a `master.json` |
| SD-r | The folder is on a sync client that renames the file mid-write (temp + rename collides) | the writer retries once after 200 ms, then reports the warning; the local save is never lost |
| SD-s | A long line or a > 2 MB Markdown file in the editor | opened read-only with a notice ("edit this file in another editor") |

### §12.2 Watchdog — one long Workflow run per phase

The run id and the script path go into `progress.md ## Run handle` the moment the run starts; a run with
parallel lanes is never resumed (R9); `START=<card>` restarts.

### §12.3 Quota gate

Before each dispatch the script checks the session's remaining budget the way SD-41 did; under 10 % it writes
a resume receipt and stops.

## §13 — Members, the Drive removal, the help screen (OPERATOR CM-1, Q11, Q12, Q13, Q19)

### §13.1 Member #1

**Today:** `CampaignManagerScreen.tsx:61` shows `campaign.members.length`, and `members` holds only the invited
emails (`campaignModel.ts:109`), so a campaign with no invitee reads "0 members". **DEFAULT:** `members:
[{ name, addedAt }]`; the Create screen's first member row is "Your name", pre-filled from the last value
(`localStorage` `codex.campaign.yourName`) and required; the creator is saved first; the Manager and the Edit
screen show the typed list (add, rename, remove any but the first); no roles, no emails, no invites.

### §13.2 The Drive removal

Deleted: `apps/desktop/src-tauri/src/campaign_drive.rs` (446 lines, 5 commands, 7 registration lines in `main.rs`),
`apps/desktop/src/settings/GoogleDrivePanel.tsx`, `googleDrive.ts`, `campaignManagerAccessGate.ts` (+ test),
`boundary/listDriveCampaigns.ts`, `boundary/writeCampaignLocalFolderArtifacts.ts`, the localStorage model in
`campaignModel.ts` (13 TS files name the Drive path today, `census.json` `ts_files_naming_drive`). The Settings
modal loses the Google Drive panel; the landing page's Campaign Manager button is always enabled and opens the
Manager (empty → "New campaign…", "Open folder…", "Help"). `FriendsSection` stays (it is not Drive).

### §13.3 The help screen

`CampaignHelpScreen.tsx` *(new)*, reached from the Manager and from every campaign screen's header: (1) what a
campaign folder is and that Codex never syncs anything; (2) the worked example — Google Drive for desktop:
create the folder inside the synced Drive folder, share it with the players from Drive, each player points
Codex at their own synced copy; (3) **any tool works** — Dropbox, Syncthing, a LAN share, a USB stick — with
one line each for Dropbox and Syncthing; (4) the folder structure, rendered **live** from the open campaign
(`list_campaign_tree` command) or from the template when none is open; (5) "Edit these files in Obsidian or any
Markdown editor" with a link to https://obsidian.md (tauri-plugin-opener, already a Cargo dependency; the
`opener:default` capability and the `@tauri-apps/plugin-opener` npm package are added); (6) **everything in
the folder is shared** — no secrets in Codex yet; the GM keeps secret notes in a folder that is not shared
(Q19); (7) what the `Characters/` folder is and how stand-in and master work (Q21/Q22) in the player's words.

## §14 — Migration on first open (OPERATOR Q13)

`CampaignMigrationScreen.tsx` *(new)* opens when the Manager finds `localStorage` `codex.campaigns` (the
pre-SD-42 model) or a known folder holding `.config/<name>.json` (SD-41's write-through). For each old
campaign: pick a folder (default: `<parent>/<campaign name>/`), then `migrate_campaign` writes `campaign.json`
(members: the old emails as names, the migrating user's name first), `Party Resources.md` (the old resources
joined, else the starter), one `Adventure Log/<date>-<title>.md` per old entry (frontmatter from the old
`updatedAt`), `Maps/<title>.md` per old map note (no image), `Wiki/<title>.md` per old page, then writes the
old data to `<app data>/campaign-migration-backup-<date>.json` (and leaves `.config/<name>.json` in place,
renamed `.config/<name>.json.bak`) before clearing `localStorage`. Nothing is deleted. Re-running finds nothing to
migrate.

## §15 — The four tabs (OPERATOR Q14–Q18)

### §15.1 Party Resources

`Party Resources.md` is created with the campaign (and by migration): a heading, one sentence, and a table
`| Item | Qty | Value (gp) | Held by | Notes |` with one example row, then a `## Party gold` table
`| Date | Change | Balance | Note |`. Edited in the editor (§18); nothing parses it; nothing moves to a
character (Q14).

### §15.2 Adventure Log

One file per session per author: `Adventure Log/<session start YYYY-MM-DD> <author> - <title>.md` (`§21` Q-D).
"+" writes:

```
---
campaign: <name>
author: <your name>
posted: <today ISO date>
session: <today ISO date>          # or a range: 2026-11-14/2026-11-16
in_game_date:                      # optional free text, shown, never parsed
title: Session <n>
---
```

The list sorts by `session` start (a range sorts by its first date), newest first; `in_game_date` is shown
beside it; a missing or unreadable frontmatter follows SD-o. The parser (Rust, `src/campaign/frontmatter.rs`
*(new)*) reads the YAML subset Obsidian writes (`key: value`, lists of scalars, quoted strings) and never
rewrites a file it did not change.

### §15.3 Maps with pins

`Maps/<name>.png|jpg` with `Maps/<name>.md`; the notes file's frontmatter holds
`pins: [{ x: 0.42, y: 0.17, label: "Ferry", page: "[[Riverside Ferry]]" }]` (`x`, `y` as fractions of the image
size, `§21` Q-E). The Maps tab lists images; a map view shows the image (bytes via a command → data URL, the
portrait's precedent `boundary/characterPortrait.ts`); a click drops a pin and asks for the wiki page (a
picker with "create"); a pin click opens the page; pins can be moved and deleted; the body below the image is
the notes text in the editor. "Add map…" copies a picked image into `Maps/` and creates the notes file.

### §15.4 Wiki

`Wiki/` with starters `NPCs/`, `Places/`, `Factions/`, `Items/`; folders can be added, renamed and deleted
(delete only when empty, else a confirmation listing the pages). "+" in a folder writes
`---\ntitle: <name>\ntype: <folder name singular>\ntags: []\n---\n# <name>\n`. `[[Page]]`, `[[Page|alias]]` and
`[[Page#heading]]` resolve by `title`, then by filename stem, across the whole folder (Obsidian's rule);
a click opens the page or offers to create it (SD-p). Backlinks: the page view lists "Pages that link here"
from an index the Rust side builds by scanning `*.md` for `[[…]]` (rebuilt on folder change; one command).
Search: one box, matches by title (first) and body text (second), case-insensitive, over every `.md` in the
campaign (not only `Wiki/`), results open the file.

## §16 — `Characters/` backup with the two-master checks (OPERATOR Q20, Q21, Q22)

SD-41 §29's mirror (`src/saved_character/campaign_mirror.rs`, 273 lines) copies the character folder to
`<campaign>/characters/<slug>-<id>/` after every writer (`after_write`, `character_hub.rs:6984`). SD-42 changes:

1. **Target** `Characters/` (capital; the existing `characters/` folder is renamed on the first write, SD-q).
2. **`master.json`** in the backup: `{ deviceId, deviceName, saveCounter, savedAt }`. `deviceId` is a UUID minted
   once per install into `<app data>/device.json` *(new)*; `deviceName` is the host name, editable in Settings.
3. **Check 1 (master device):** `import_campaign_character` *(new command)* reads a backup; if `master.json`
   names another device, the dialog offers **"Play as stand-in"** (default: the local copy is created with
   `stand_in: true` in its `campaigns.json` membership — `after_write` never writes a stand-in's backup) or
   **"Move the master here"** (the local copy becomes master: the next save writes `master.json` with this
   device and `saveCounter + 1`; the old device's next save hits check 2).
4. **Check 2 (save counter):** before writing a backup, the mirror reads the backup's `master.json`; if its
   `saveCounter` differs from the counter this install last wrote for that character and campaign (kept in the
   character's `campaigns.json` membership as `lastBackupCounter`), the backup is **not** written and a warning
   names `deviceName` and `savedAt` ("the campaign copy was changed by `<device>` on `<date>`; your local save is
   safe; re-import to see their version"); the local save has already succeeded. A match writes the backup and
   `saveCounter + 1`. The pattern is `save_with_conflict_detection`'s nonce (`local_store.rs:194`), applied per
   character folder.
5. **Diff, no merge:** the Characters tab shows, for a stand-in whose master backup moved on, the list of
   files that differ (name, size, which side is newer) and the envelope's changed top-level fields; nothing is
   merged (Q22).
6. **Reading backups:** `list_campaign_characters` returns, per `Characters/*/`, the envelope summary (label,
   classes and levels, race, master device, counter, savedAt); the Characters tab lists them and opens a
   read-only sheet view through the existing load path on the backup root (`CampaignSheet.tsx` already loads a
   detail by id; it gains a load by root).

## §17 — The DM toolkit reads the party from the folder (OPERATOR Q20)

`encounter_rating.rs` rates a `PartyMemberInput { character_id | level }` (`:67-87`), resolving a saved
character by id (`:312-313`). **DEFAULT:** a third shape `{ campaignCharacter: { campaignDir, folder } }` is
rated from the backup's envelope (`character_level` over `SavedCharacterStore::load(backup root)`); the
Encounter Builder gains "Add from campaign…" listing the known folders' `Characters/` entries
(`list_campaign_characters`). The typed-level and saved shapes stay.

## §18 — The editor (OPERATOR Q24, Q25; DEFAULT on the packages)

**Packages** (versions and licences read from the npm registry on 2026-11-16 with `npm view <pkg> version license`;
E3.1 pins the exact versions in `package-lock.json` and re-reads the licence field of each installed
`package.json`): `@codemirror/state` 6.7.6, `@codemirror/view` 6.43.15, `@codemirror/language` 6.13.1,
`@codemirror/commands` 6.11.1, `@codemirror/autocomplete` 6.20.4, `@codemirror/search` 6.7.2,
`@codemirror/lang-markdown` 6.5.2, `@lezer/markdown` 1.8.1, `@lezer/highlight` 1.2.5, `@lezer/common` 1.5.5 — all
**MIT**; `marked` 18.2.0 — **MIT** (the reading view; SD-g). Nothing else; no sanitiser (raw HTML tokens are
escaped by a `marked` renderer override), no React wrapper (the component mounts an `EditorView` in a `ref`).

**Byte-stable round trip:** the editor holds the file's bytes as read; save writes `state.doc.toString()`
re-encoded with the original line ending (`\n` or `\r\n`, detected on load) and the original presence or absence
of a trailing newline and BOM; a test opens each fixture, saves without an edit and asserts `sha256` equal.

**Live preview** (Obsidian's manner): a `ViewPlugin` over the Lezer Markdown tree decorates headings (size),
emphasis and strong (style, markers hidden off the cursor line), inline code, list bullets, task boxes,
`[[wikilinks]]` (styled, clickable, resolved by `§15.4`'s rule), images (`![](Attachments/x.png)` rendered
inline from a data URL), tables (monospace alignment, no widget) and fenced code; the line the cursor is on
shows its raw markup. A "Reading" toggle renders the whole file with `marked`.

**Completion:** typing `[[` opens an autocomplete of every page title and filename stem in the folder
(`list_wiki_pages`), inserting `[[Title]]`.

**Extras (Q25):** a toolbar (Bold, Italic, Heading, List, Link, Table) applying the usual Markdown edits to the
selection; auto-save 1 s after the last edit and on blur; before every auto-save the component asks
`stat_campaign_file` for the mtime it loaded and, on a difference, shows "Changed on disk" with **Reload** and
**Keep mine** (which writes); dragging an image file onto the editor copies it to `Attachments/<name>` (a
collision adds `-2`) and inserts `![](Attachments/<name>)` (`§21` Q-F).

## §19 — Discovered cards and fence exits

A defect found by a card outside its own criterion becomes a **discovered card** (`E<n>.<next>`) with a
`Depends on`, inserted before its epic's merge check, with the card count re-stated in G-8 and `kanban.md` in
the same commit (R18). A file a card needs outside its FILES YOU OWN is a **fence exit**: named in the receipt
with the line it changes, allowed only when no other open card (SD-42's or SD-41's) owns it, else `declined`
with `ownedBy:<card>` (R4).

## §20 — Pre-launch corrections

| Date | Where | Was | Is | Found by |
|---|---|---|---|---|
| — | — | — | — | (the lead fills this table as corrections land; `§20` wins over older text) |
| 2026-11-17 | §5, E1.2 | one `CopyWire` per `EQUIPNAME` line; samples `copies_emitted == lines - zero_lines` | one stack (one row, one copy) per **place** a line is in: PCGen splits a line over `EQUIPSET` entries (pf_Paladin Javelin 3 Carried + 2 on the horse; 29 lines → 31 stacks); the invariant is stacks ≥ held lines and pieces conserved | E1.2 (`pcg_import_stacks`) |
| 2026-11-17 | §5, E1.2 | row `activeState` `EquippedActive` iff a depth-3 worn entry, else `SelectedInactive` | the row's state is `equipment_sets::project` of the imported set (Not Carried → `Absent`), because the app's `placed_in` drops a location whose projection differs from the saved state | E1.2 |
| 2026-11-17 | §5, E1.2 | `quantity` = the line's `QUANTITY` | for a bundle record PCGen's `QUANTITY` counts bundles; pieces = `QUANTITY × base_quantity` — discovered card E1.8 | E1.2 |
| 2026-11-17 | E1.2 | fixture `tests/fixtures/sd42_stacks.pcg` | `tests/fixtures/pcg_stacks.pcg` (the §6 identifier audit refuses `sd42_`) | E1.2 |
| 2026-11-17 | E1.3 | tests on the synthetic fixture `pcg_stacks.pcg`; "a test re-reads SD-41's fixture `equipment_sets.json`" | E1.3's own fixture `tests/fixtures/pcg_custom.pcg` (a second Longsword line breaks `pcg_import_stacks`' one-Longsword assertion); no `equipment_sets.json` fixture is committed (`git ls-files \| grep equipment_sets.json` → 0), so `copy_facts_old_file_reads_byte_identical` loads and saves a literal in SD-41's written shape | E1.3 |
| 2026-11-17 | §6, E1.3 | `COSTMOD` folds into `cost_override_copper` | PCGen's line `COST` already includes `COSTMOD` (pf_Paladin: `Outfit (Explorer's)` `COST:0`, `COSTMOD=-10`); the override is the line's `COST` when it differs from the record's price; `eqmods` ride on the `equipmentSelections` row (`EquipmentWire.eqmods`), beside `applied_modifiers` | E1.3 |
| 2026-11-17 | `epic-breakdown.md` E2.1 desktop acceptance | `cargo test … campaign_folder_` | `cargo test … campaign_folder::` (the named tests are `campaign_folder::tests::known_folders_round_trip` etc.; `campaign_folder_` matched 0 of 1372) | E2.1 |
| 2026-11-17 | `epic-breakdown.md` E1.5 RED | `error[E0560]: … no field notes` | `error[E0609]: no field 'notes' on type 'BioWire'` — the test reads `bio.notes`, it never builds a `BioWire` literal | E1.5 |
| 2026-11-17 | `epic-breakdown.md` E2.2 | Edit saves via "`write_campaign_file` of `campaign.json` through the nonce path"; "a conflict shows the existing message"; FILES YOU OWN = the three screens + two models | `write_campaign_file` refuses `campaign.json` (E2.1); E2.2 adds `save_campaign_json` (`campaign_folder.rs`, `main.rs`, boundary `knownCampaigns.ts`/`types.ts`) over `save_campaign_json_with_conflict_detection`; no conflict message existed in the UI (`git grep -n conflictDir -- apps/desktop/src` → 0), so the Edit screen states the kept copy's path | E2.2 |
| 2026-11-17 | `epic-breakdown.md` E2.3 | "the 5 `campaign_drive` tests"; R21 RED "the grep → 13 files"; FILES YOU OWN as listed | 7 tests (`cargo test -- --list` on `1dc5be2861`: 7 `campaign_drive::tests::` lines); the grep → 70 lines in 16 files on `1dc5be2861`; the deletion reaches, by R15 closure, `ui/characterHub/LandingScreen.tsx` (the gate import + prop, 3 lines; E2.13 keeps its two `export` keywords), `ui/layout/AppMenu.test.ts` (the gate-hint assertion; E2.12 keeps the rest), `ui/{cyberpunkCatalog/cyberpunkCatalog,starfinderCatalog/starfinderCatalog,settings/PrintPanel}.test.ts` (the gate prop / tab key), `ui/settings/friends.ts` (comment), `ui/boundary/setCharacterCampaigns.ts` (comment), `ui/characterHub/PrintDialog.tsx` and `ui/dmToolkit/DmToolkitScreen.tsx` (one import each: the legacy reader moves to `ui/campaign/campaignMigrationSource.ts` *(new)*; reading the folders is discovered card E2.14), `desktop/tests/old_save_loads_unchanged.rs` (the `#[path]` include of `campaign_drive.rs`); the Settings tab `google-drive` becomes `friends` (`FriendsSection` stays reachable, `§13.2`) | E2.3 |
| 2026-11-17 | `epic-breakdown.md` E1.7, §9 | the sidecar's test compares Codex's "import + engine output" for every field of every file, Pathfinder included | the Pathfinder totals are assembled in the desktop crate (`character_hub.rs`, `pf1_adapter.rs`, over the desktop's corpus, `corpus_fixtures.rs` `add_what_a_character_can_hold`), which `codex-ingest` cannot link (SD-36 D1); E1.7 compares Starfinder through the root crate's `sf_*` engine and Pathfinder's import fields (classes, race, scores, feats, spells, gear, money), and reads each Pathfinder total `not-computed`, ledgered to discovered card E4.13 which compares them in the desktop | E1.7 |
| 2026-11-17 | `epic-breakdown.md` E2.4 | tests `migrate_localstorage_shape_…`, `migrate_config_json_…`, `migrate_writes_a_backup_…`, `migrate_is_idempotent` under the filter `campaign_migrate_`; output "a new campaign folder" at `<picked parent>/<name>/` | the filter matches a test's full path (`campaign::migrate::tests::<name>`), so the names carry the prefix: `campaign_migrate_…` (unprefixed they match 0); the target is `<parentDir>/<sanitised name>/` as "New campaign…" makes it and must be new or empty (SD-f) — a `.config/` folder is not upgraded in place (its old `maps/`, `wiki/` collide with `Maps/`, `Wiki/` on a case-insensitive disk); "Open folder…" on a `.config/` folder opens the migration screen | E2.4 |
| 2026-11-17 | `epic-breakdown.md` E2.6 FILES YOU OWN | `PartyResourcesTab.tsx`, `CampaignSheet.tsx`, `src/campaign/folder.rs` | plus, by R15 closure, the command behind "Create starter file" (`create_party_resources_starter`: `desktop/campaign_folder.rs`, 2 lines in `desktop/main.rs`, boundary `campaignFiles.ts`) so the starter text has one home in Rust; the pure stem/link logic in `ui/campaign/tabs/partyResourcesModel.ts` + test | E2.6 |
| 2026-11-17 | §5, E1.2 | pieces no `EQUIPSET` entry of the imported set places are Carried | they are Not Carried (Carried only in a file with no set): PCGen counts only the set's entries (its export lists 28 entries for pf_Paladin, whose three `Gauntlet` lines are in none — oracle run, E1.10); Q5 still holds, the line is kept | E1.10 (`stacks_pieces_no_entry_places_are_not_carried`) |
| 2026-11-17 | §8, E1.6 | a Starfinder class-feature selection line (`PRIMARY FIGHTING STYLE ~ Blitz`, `Opening Volley Choice` / `Choose Combat Feat`, a gear boost, a mechanic trick) is a custom feat | it is the pick the app's choices dialog makes: the option record the line's key names, under the first rule whose offer selects it (`offer_selects`); a pick the engine refuses is undone and stays a custom feat (sf_soldier: Rapid Response +4 initiative, Opening Volley held) | E1.10 (`sf_selection_lines_are_engine_picks`) |
| 2026-11-17 | `epic-breakdown.md` E2.7 FILES YOU OWN, `new_entry` signature | FILES YOU OWN = `src/campaign/{frontmatter,adventure_log}.rs`, `mod.rs`, `desktop/campaign_folder.rs`, `desktop/main.rs`, `AdventureLogTab.tsx`, `adventureLogModel.ts` + test, `boundary/campaignFolder/adventureLog.ts`; `new_entry(dir, campaign, author, title)` | plus, by R15 closure, `ui/campaign/CampaignSheet.tsx` (the tab is mounted only there; it rendered the read-only file list) and the image resolver lifted from `PartyResourcesTab.tsx` into `boundary/campaignFolder/campaignFiles.ts` (`campaignImageUrl`, now one copy for both tabs); `new_entry(dir, campaign, author, title, today)` — the date is the UI's local date (`localIsoDate`), as `migrate_campaign` takes `now` from the UI; `frontmatter_` → 7 tests (5 in `frontmatter.rs`, 2 `adventure_log_` tests whose names hold the word) | E2.7 |
| 2026-11-17 | `epic-breakdown.md` E1.MC partition | `git diff --name-only <C1 sha>..HEAD -- apps docs/architecture scripts/verify-baselines.env \| awk 'END{print NR}'` → 0 | the files of the import lane's own commits (`--grep='sd42,e1\.'`): the branch-wide diff is 81 on `8aca2c44a2` because the campaign and editor lanes write `apps/` and the develop catch-up merge brought `docs/architecture/**` — none of it the import lane's | E1.MC |
| 2026-11-17 | §7, E1.5 (OPERATOR Q5) | the Notes block lists 9 kinds; a feat, skill, ability, template or unreadable money line Codex lacks is kept as a custom feat with no Notes line | every line kept as a custom feat is a Notes line (`### Lines kept as custom feats`, a 10th kind), unless the line already wrote one (an unknown key, class, race); 81 lines over 6 samples were missing (`notes_block_names_every_custom_feat_of_the_samples`) | E1.MC |
| 2026-11-17 | §7, E1.4 (OPERATOR Q2, Q5) | a Create item carries the line's `WT`, `COST`, `NOTE`; its `CUSTOMIZATION` (eqmods, `SPROP`, …) is not read | every `CUSTOMIZATION` field but `KEY` and `NAME` is a stat on the Create item (text, never computed) and a Notes line (`Eqmods kept as text` / `Properties kept as text`); population: 1 of the operator's 4 Create items, 1 of `sf_mechanic`'s 3 | E1.MC (`unmapped_create_item_keeps_its_customization`) |
| 2026-11-17 | `epic-breakdown.md` E2.8 acceptance, FILES YOU OWN, desktop commands | `--lib maps_` → ≥ 4 passed, RED "filter matches 0"; FILES YOU OWN = `maps.rs`, `mod.rs`, `desktop/campaign_folder.rs`, `desktop/main.rs`, `MapsTab.tsx`, `mapsModel.ts` + test, `boundary/campaignFolder/maps.ts`; commands `list_maps`, `read_map_image`, `set_map_pins`, `add_map` | `--lib campaign::maps::tests::maps_` (bare `maps_` matched 3 unrelated tests on `e43d730e06`: `table_class_id_maps_monk`, `equipment_sets_default_from_maps_each_state`, `eqmod_option_key_maps_codes_and_long_keys`); plus, by R15 closure, `ui/campaign/CampaignSheet.tsx` (mounts the tab; a pin opens its page on the Wiki tab, which shows that page in the editor above its file list until E2.9) and one line of `src/campaign/frontmatter.rs` (`scalar` keeps a flow collection raw, so a ` #` inside a quoted pin label is text); no `read_map_image`: `read_campaign_file` + `campaignImageUrl` (E2.7) already return an image's bytes as a blob URL; the pins line is `pins: [{ x: …, y: …, label: "…", page: "[[…]]" }]` (labels/pages JSON-quoted), a `- { … }` block list also reads, block maps are refused (shown "pins not read", never rewritten) | E2.8 |

## §21 — Questions only the operator can settle (defaults taken; the run does not wait)

| # | Question | Default taken | Card |
|---|---|---|---|
| Q-A | `sf_mechanic_drone.pcg` is a companion file (class `Drone`, 0 `EQUIPNAME` lines, `census.json`). Import it as its own saved character, or refuse? | if the SF engine computes a Drone build (SD-37's E5.4 parity build has a drone), it imports as its own SF character named by the file; else the import returns `Blocked` naming "a drone file is a companion; import the mechanic", and the parity ledger explains it | E1.6 |
| Q-B | Where "Your name" (member #1) comes from | a required field on the Create screen, pre-filled from `localStorage` `codex.campaign.yourName` | E2.2 |
| Q-C | Where the list of known campaign folders lives | `<app data>/campaign-folders.json` (local to the install, never in a campaign folder) | E2.3 |
| Q-D | Adventure Log filename | `Adventure Log/<YYYY-MM-DD> <author> - <title>.md` | E2.7 |
| Q-E | Pin coordinates | fractions of the image's width and height (`0..1`), so a resized image keeps its pins | E2.8 |
| Q-F | Image link syntax for dragged-in images | `![](Attachments/<name>)` (Obsidian renders it; `![[…]]` is Obsidian-only) | E3.3 |
| Q-G | Two `EQUIPNAME` lines of one base at different locations | two stacks, as the file has them | E1.2 |
| Q-H | One operator gate or two | one, E5.G, covering import and the Campaign Manager; E1.MC/E2.MC/E3.MC/E4.MC leave screenshots | E5.G |
| Q-I | A `QUANTITY:0` line (the operator's file has 1: `census.json`) | not held; listed in Notes | E1.2 |
| Q-J | `Characters/` with a capital C while SD-41 wrote `characters/` | rename on first write; SD-q | E2.10 |
| Q-K | `campaign.json` at the folder root vs `.config/<name>.json` | `campaign.json` at the root (visible in any file browser; Obsidian ignores it); `.config/<name>.json` is read by the migration only | E2.1 |
| Q-L | Q10 and Q23 are missing from the intake | treated as never asked | — |
| Q-M | Companions in a `.pcg` (the Camel line, `LOADCOMPANIONS`) | gear lines import as gear; companion files are not followed | E1.4 |

## §22 — PCGen is a converter and a test oracle, never live code (STANDING, 2026-09-07; OPERATOR Q3, Q4)

The sidecar `pcg_import` reads a `.pcg` and writes JSON; the app never parses one (SD-39 R13). The PCGen
engine runs only in tests and scripts at the pinned oracle (`scripts/pcgen-oracle-pin.env`,
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`), resolved through `$PCGEN_REPO_DIR` /
`$PCGEN_CORPUS_ROOT`; no literal local path enters a doc or a script; nothing from the oracle enters `apps/`,
`src/`, `crates/`, `data/`, `resources/` or `scripts/` except the committed parity exports under
`scripts/oracle_harness/pcg_import_parity/` (plain text totals of PCGen's own sample characters, the SD-37
precedent) — `bash scripts/verify.sh --only pcgen-residue-gate` stays green.

## §23 — Closure order (STANDING, operator-pinned 2026-09-14; memory "graphify runs against the FINAL repo state")

Final-acceptance scan → retrospective written and cited → sweep → release notes → architecture truth-up +
claims critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`).

## §24 — Engine-owned numbers; the paper-sheet rule (STANDING)

No TS and no Typst sums a weight, a price or a damage die; a copy's `damage_override`, `weight_override_lbs` and
`cost_override_copper` are read by the engine's existing rows (`damage_total.rs`, `encumbrance.rs`,
`containers.rs`, the Market's price path) in one place each; the screen and the print render the rows. A dice
term stays literal; a property with no number is printed (`SPROP`).

## §25 — One bundle (DEFAULT; `scope-draft.md §6`)

The import lane (E1, E4.1–E4.2) and the campaign lane (E2, E3, E4.3) are file-disjoint (F-20) and run in
parallel in Phase 1, so the bundle's wall time is the longer lane. Splitting would cost a second C1, a second
closure epic and a second SD-41-style catch-up with no saving in agent-hours. The split point, if the operator
wants two bundles, is `risks-and-open-questions.md §4`.

## §26 — Three cards the operator added mid-run (OPERATOR, 2026-11-17; placement DEFAULT)

Added on 2026-11-17 while the Phase-1 run was live, by a planning session in its own worktree
(`.worktrees/sd42-cards`), without touching the lanes' trees. The operator's words are the §3 rows UI-1, UI-2,
LNX-1 / Q27 (relayed via the lead's memory note of the same day). Card count 37 + E1.8 → **41**
(`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`); every count constant in this package was
re-stated in the same commit (R18).

### §26.1 E2.12 — "Main menu" first (UI-1)

Every ☰ menu is drawn by one component, `apps/desktop/src/layout/AppMenu.tsx` (`withOptions` appends Options);
its users are exactly `App.tsx` (the fixed copy on screens without a bar), `campaign/CampaignSheet.tsx` and
`characterHub/CharacterSheet.tsx` (`grep -rl "layout/AppMenu" apps/desktop/src --include=*.tsx` → 3 on
`3547882dee`). The main screen is `CharacterHubPage`'s `mode === 'landing'`; every other mode reaches it
by a Close/Cancel/Back chain today. The entry is therefore composed **inside `AppMenu`** from a host claim
(`claimMain`, the existing `claimMenu` pattern) that the hub makes whenever `mode !== 'landing'`, so the
three users need no edit and the entry is absent on the main screen by construction.

**Placement:** lane `campaign` as **E2.12**, because that lane owns `ui/characterHub/CharacterHubPage.tsx` and
`ui/App.tsx`. `ui/layout/AppMenu.tsx` and `AppMenu.test.ts` are owned by no card: **granted to E2.12 here** and
added to the §2.1 fence list and `workflow-instruction.md §3`. **Depends on E2.11**, not E2.3: E2.3 is the last
card that writes `CharacterHubPage.tsx`/`App.tsx`, but E2.4 and E2.5 still write `CampaignManagerScreen.tsx`
and the lane runs in **one** worktree — a row that became ready beside E2.4 could be dispatched into the same
tree (two writers). Hanging it after E2.11 costs nothing in a serial lane and keeps it before E2.MC, whose
`Depends on` becomes E2.13.

### §26.2 E2.13 — the open-campaign button (UI-2)

The main screen's Campaign Manager button is `LandingScreen.tsx`'s `ActionBanner` with `CAMPAIGN_MANAGER_ART`
(`:158`, `:254`, both module-private). The Manager's open button is the bar's primary `Load`
(`CampaignManagerScreen.tsx:164`) as E2.3–E2.5 leave it. **Reuse, not a new style:** the two names are exported
and the Manager renders the same component at full column width. `LandingScreen.tsx` is owned by no card:
granted to E2.13 for the two `export` keywords only. **Depends on E2.12** (same tree; E2.5 was the last writer
of `CampaignManagerScreen.tsx`).

### §26.3 E4.6 — the Linux AppImage's menu entry and icon (LNX-1 / Q27)

`apps/desktop/src-tauri/src/update/self_install.rs` (develop `183659ec70` + `b3a239b589`, in `tranche-22`
since the 2026-11-17 catch-up merge) already writes the XDG entry and the hicolor icon — but only when running
from `~/Codex/Codex.AppImage` or after an accepted install. A download answered "Just run this time" (or
started before the dialog is answered) gets neither, which is the one way this box can explain the operator's
Arch report; E4.6 makes the integration unconditional for every AppImage start, naming the running `$APPIMAGE`
(the path the updater replaces in place and `seed.rs` re-records after a move), so a moved or updated file
re-points its own `Exec`/`TryExec` on its next start. **Placement:** Phase 2, lane `main`, **E4.6** (the next
free id), **depends on E4.5**: no E4 card touches `update/`, but Phase 2 is one serial tree and E4.MC's
`Depends on` becomes `E4.5, E4.6`. The file is not SD-41-owned; nothing in Phase 1 needs it, and the card is
small enough that waiting for the merged base costs one cycle at most.

### §26.4 ui-smoke rows: folded into E4.4 (R24)

E4.4 is `waiting` on `origin/tranche-22` (`awk -F'|' '$2 ~ /^ E4\.4 / {print $5}' kanban.md` → ` waiting `),
so its row gains `main-menu-first` and `campaign-open-button`; **E4.4 stays the only `spec.json` writer**. No
new card writes `spec.json`. E4.6 has no ui-smoke row (Rust-only; the app under ui-smoke is not an AppImage).

### §26.5 What this placement does not change

The live Workflow script's §2.1 prefix was written at launch and predates the fence additions above; the
card rows' FILES YOU OWN are the grant (`§19`: a file no other open card owns). The lead dispatches the three
cards on their rows' Tier (`opus`) when their `Depends on` are `complete`; no script edit is required. The
SD-41 partition no longer binds (PR #410 merged 2026-11-17), but E4.1's `SD-41 merged` token is still the
lead's to clear by hand (R25).

## §27 — Five gear cards the operator added mid-run (OPERATOR, 2026-11-17; placement DEFAULT)

Added on 2026-11-17, later the same day as §26, by a planning session in its own worktree
(`.worktrees/sd42-cards2`, detached from `origin/tranche-22`), without touching the lanes' trees. The operator's
words are the §3 rows BAG-1 / HVS-1, SLOT-1, REN-1 + CUS-1, DET-1, STK-1 (relayed by the lead the same day; REN-1 and
CUS-1 arrived separately and are one card, E4.9, because CUS-1's "custom naming" is REN-1's rename). Card count
41 → **46** (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`); every count constant in this
package was re-stated in the same commit (R18). All five touch `character_hub.rs`, `GearScreen.tsx` and
`gearScreenModel.ts`, which E4.2 owns in Phase 2, so all five are Phase 2, lane `main`, and run **serially
after E4.3** (the last writer of `character_hub.rs` among E4.1–E4.3): E4.3 → E4.7 → E4.8 → E4.9 → E4.10 → E4.11 → E4.4.
Every repo figure below was derived on `tranche-22` @ `c58b8c47ec`; the PCGen figures at the pin
`7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`), resolved via `$PCGEN_REPO_DIR`.

### §27.1 E4.7 — weightless container contents (BAG-1 / HVS-1)

Operator: "a bag of holding is treating it's contents as cummulative from a weight perspective instead of treating
them as weightless" and "the same goes for handy haversack". **Verified mechanism.** PCGen's `CONTAINS:` token
(`code/src/java/plugin/lsttokens/equipment/ContainsToken.java:55-117` at the pin) reads its first `|` field as
`[*|N%]<UNLIM|pounds>`: a leading `*` sets `CONTAINER_CONSTANT_WEIGHT` ("the contents weigh nothing"), a leading
`N%` sets `CONTAINER_REDUCE_WEIGHT` (the contents weigh N % less); both at once is a parse failure. The PF data
carries the `*` flag on **61** distinct records (`cd $PCGEN_REPO_DIR/data/pathfinder && grep -rhE 'CONTAINS:\*'
--include=*.lst . | awk -F'\t' '{print $1}' | sort -u | awk 'END{print NR}'`): Bag of Holding (Minor, Type I–IV,
`*50`/`*250`/`*500`/`*1000`/`*1500`; the Looter's Satchel copy), **Handy Haversack `*120`**, Portable Hole
`*UNLIM`, Efficient Quiver `*UNLIM|Arrow=60|…`, the Bags of Concealment and Devouring, the bandoliers, Glove of
Storing, the three Carpets of Flying, and the mounts and pack animals (`*600|Any` on a Heavy Horse: a ridden
animal's load is not the rider's). The `%` form is on **1** record (Versatile Vest, `40%26|Any`). Codex's
converter (`crates/codex-ingest/src/pcgen_import/ir_converter.rs:700`, `container_capacity_of`) tests
`starts_with("UNLIM")` and then `f64::parse` on the first field, so `*250` and `*UNLIM` both fall to
`ContainerCapacity::Unstated`: the `*` is dropped **and the bag's own capacity is lost** — the settled bundle
(`data/corpus/core_rulebook/_settled/equipment.json`) holds `container: true` and no `container_capacity` for
either item, so no over-capacity warning fires for a bag today. SD-41 recorded this as deferred (`DEF-1`, its
decisions.md Q4: "weight-reduction kinds … contents weigh in full"). `rules_core::encumbrance::compute_encumbrance_sized`
(`src/rules_core/encumbrance.rs:424`) sums every non-absent copy's weight with no knowledge of the set tree;
`containers::container_loads` (`src/rules_core/containers.rs:40`) walks the tree for the capacity check and must
keep doing so.

**The rule (paper-sheet rule, §24):** the carried total is a number on the sheet, so it is computed right: the
contents of a `*` container — and everything nested inside them — add nothing to the carried total; the container's
own weight counts; the capacity check still counts the contents against the container's own capacity. The `%` form
is carried by the same number (`contents_weight_pct`: `*` → 0, `N%` → 100 − N, default 100) because it costs one
field and leaves no carve-out. **Placement:** E4.7, Phase 2, `main`, after E4.3; the first of the four. The record
field means the 30 settled bundles are regenerated (`cargo run --locked --bin gen_settled_corpus`, `--check` clean)
in the card's commit. **Does not change:** the Weight column's per-copy words (what the thing weighs), the capacity
warning, Starfinder bulk (no PF-style flag in SF data), a Create item's contents (no flag in `custom.json`; stated).

### §27.2 E4.8 — slot kinds (SLOT-1)

Operator: "bug: you can put anything into any slot. I added a backpack to a shield slot." **Verified mechanism.**
`moveTargets` (`apps/desktop/src/characterHub/gearScreenModel.ts:260`) offers every slot section that has a free
place, testing only `placesOf` and `slotOverride`, never the row's item; `dropTarget` (`:339`) reuses it. The
backend `move_copies_at_root` (`apps/desktop/src-tauri/src/character_hub.rs:6303`) runs `wear_check_in` only when
the copy is not already worn (`putting_on`, `:6337`), and that check (`item_slot_conflict` → `body_slots::slot_conflict`,
`src/rules_core/body_slots.rs:350`) answers only "is the **target** slot taken" — so the write boundary also accepts a
backpack in Shield, and a worn item moved slot → slot is never checked at all. The item's own kind is already on
the wire (`HeldEquipmentCopyDto.slot`, `:5555`; `PlacedEquipmentCopyDto.slot`, `ui/boundary/listHeldEquipment.ts:65`),
derived by `body_slot_of_item` (`apps/desktop/src-tauri/src/equipment_catalog.rs:262`) from the same slot table the
import parses (`crates/codex-ingest/src/pcgen_import/equipment_types.rs:118`, `EQSLOT … CONTAINS:<type>,…=<n>`).
**The rule**, in one engine place (`body_slots::slot_kinds_allowed`): own slot only; the `wield_*` family
interchangeable (as `slot_conflict` already treats it); a `none` copy takes no slot; the Custom tab's "Ignore body
slot limits" lifts it at both ends and is recorded as a flag, as a full slot is today. The backend refuses with a
readable reason (`equipment.wrong_slot`: "Backpack cannot go in the Shield slot: it is worn in no slot").
**Existing saves (DEFAULT, the lead's recommendation):** a copy already saved in a wrong slot **stays where it is
and the row carries a warning**; nothing is moved on load (a load that rewrites a player's set is a silent data
change; §16's two-master rule exists because silent rewrites are the trap). The operator may choose "move it to
Carried on load" instead; the card then changes one branch. **Placement:** E4.8 after E4.7 (same tree and files).

### §27.3 E4.9 — Customize a held copy: rename + modifiers; the redundant list goes (REN-1 + CUS-1)

Operator: "enhancement: we need the ability to rename an object in inventory. ie: Sack #1 could be renamed to
Camping Gear" and, later: "the gear screen shows the equip view with move/drop/sell/discard. And then the same list
again with attach modifier. On the equip section, add a customize button which allows custom naming or adding
modifiers. Then you can drop the redunant view below the equip section with the modifier buttons."
**Verified.** Rename: SD-41 E2.5 shipped the backend — `rename_equipment_copy` (`character_hub.rs:6767`, `_at_root`
`:5533`, registered `main.rs:85`, tested `:22544`) writes `CopyFacts.name` (`equipment_sets.rs:86`; blank clears)
and `ui/boundary/renameEquipmentCopy.ts` wraps it — but no component imports that boundary (`grep -rln
renameEquipmentCopy apps/desktop/src` → the boundary and one unrelated test); the gear screen's "Rename"
(`GearScreen.tsx:229`) renames the equipment **set**; E1.3 fills the same field for imported custom names. The
redundant view: `GearTab` (`CharacterSheet.tsx:1584`) renders `GearScreen` and then its own `items.map(...)` over
`corpusDerived.resolvedEquipment` (`:1647-1700`) — name, id, "✓ grounded", weight · cost, an **Attach Modifier**
button per row and the applied modifiers as a list. That button is the only entry to the modifier flow
(`handleAttachModifier` `:3280` → `pendingModifierAttachment` `:2851` → the Equipmods `ItemPickerModal`, config
`'modifier'` at `:318` → `handleModifierPicked` `:3291` → `attachEquipmentModifier`), and the flow targets a
**selection by `itemId`**, not a copy — with two Longswords the pick lands on the first. No remove-modifier action
exists (`git grep -n 'detach\|removeEquipmentModifier\|remove_equipment_modifier' apps/desktop/src` → 0), so the
modifier actions reachable today are exactly *attach* and *see the applied list*.

**The card (one, not two):** one **Customize** button on every equip row and every container-table title opens one
dialog with the name (rename, per copy) and the modifiers (the applied list + Attach modifier through the **same**
picker and boundary, now carrying the copy so it lands on that copy); then the second list and everything that only
served it is **deleted** — the `items.map` block, the `onAttachModifier` prop, the `ResolvedEquipmentDto`-typed
pending state and handler — with no empty handler or unused symbol left (no-stub doctrine; the receipt lists every
deleted symbol with a `git grep` → 0). **Defaults (the operator may change each):** rename is **per copy**
(`CopyFacts` is keyed by copy; "Sack #1" is one copy's title in a container table, and renaming it makes it its own
row, leaving "Sack" for the other); the name shows everywhere `held.name` is already read (rows, container titles,
"Into …" targets) and the printed Inventory reads **"Camping Gear (Sack)"** so the base item stays visible (the DTO
gains `baseName`); blank restores the base name; weight, capacity, slot and price never change with the name;
removing a modifier stays out (it never existed; `forward-scope-register.md` FSR-C9). **Placement:** E4.9 after
E4.8; E4.10 (the detail panel, which also edits `GearScreen.tsx` and `CharacterSheet.tsx`) is chained after it, so
the two never hold the same file at once.

### §27.4 E4.10 — the gear detail panel (DET-1)

Operator: "enhancement: when selecting a peice of gear, we need to display all the information about that gear.
Weight/Cost/Stats/Description/Slots/etc". **Verified:** no command describes a held copy today. The pieces exist:
`EquipmentCatalogEntryDto` (`equipment_catalog.rs:113`: key, category, name, cost, weight, book, types,
description, slot), `WeaponDamageDto` (`character_hub.rs:1118`), the settled record's `stat_effect`
(AC bonus, max Dex, ACP, spell failure), `is_shield`, `container`, and after E4.7 `container_capacity` and
`contents_weight_pct`; the catalog screen's `hasDescription` (`EquipmentCatalogScreen.tsx:65`) is the one honest
description test (null = "states none"). **Defaults (the operator may change each):** selecting a gear row or a
container-table row opens a panel under the tables with two blocks — the record (weight, cost, type, slot,
weapon stats, armor/shield stats, container capacity and the weightless words, description, source book) and
this copy (custom name beside the base name, size, material and modifiers, the E4.2 overrides, quantity,
location); every value is the engine's settled record or the copy's facts through one new `describe_held_equipment`
command that **reuses** the existing DTOs; an unstated value is `null` on the wire and `—` on the screen, never 0;
the panel does no arithmetic (NR-2; AGENTS.md's twin rule). **Placement:** E4.10 after E4.9 (it shows E4.7's flag
and E4.9's names).

### §27.5 E4.11 — stacks: one row per stack, split by moving (STK-1)

Operator: "i purchased four sacks. This was listed as Sack x 4, four distinct times in the equip section as carried.
In the second view it just shows Sack, Sack, Sack, Sack. It's ok to list an empty container as Sack x 4. But do not
list Sack x 4, 4 times - leading one to believe there are 16 sacks. If one of the sacks has something inside, you
would show Sack, and Sack x 3. Stacked items also should be able to be split: Sack x 2, Sack x 2. Or Arrows x 50,
Arrows x 50 instead of Arrows x 100. That way some arrows could be put in a quiver and others in a backpack."
**Verified mechanism.** The backend builder already folds the four bought lines into one row (`quantity 4`, four
`copies`; `character_hub.rs:5614`). The screen undoes it: `nodesOf` (`gearScreenModel.ts:207`) emits one tree node
**per copy** of a container stack so that each sack can hold things, and every node renders the **stack's** label
("Sack ×4", `GearScreen.tsx:370`) — four nodes each reading ×4. The second view is `GearTab`'s per-selection list,
deleted by E4.9 (§27.3); E4.11 lands the rule in what survives (the tree, the container tables, the printed
Inventory, which today prints "Sack #1 … #4"). Splitting already exists as a move: `move_equipment_copy` takes a
piece count (SD-41 §30, Q8 "stacks split to any count"), the screen asks "how many?" on any stack, and a part moved
back beside an identical stack rejoins through the same grouping. **Binding rules (the lead, 2026-11-17):** (1)
identical copies at one place are one row with the count, in every surviving view and on print; (2) a container
copy holding anything is its own row ("Sack" + "Sack ×3"), as is a renamed copy; (3) a stack splits into parts
that move on their own; (4) parts put back together display merged. **DEFAULT (the lead's recommendation, the
operator may change it):** a deliberate same-place split is **not kept** — a split is a step before a move, so there
is no Split button and no new saved state; "Sack ×2, Sack ×2" at one place reads "Sack ×4" until one part moves.
Weight and totals never change from regrouping (a test pins them). **Placement:** E4.11 after E4.10 (it edits
`gearScreenModel.ts`, `character_hub.rs` and `print/blocks/gear.rs`, which E4.9/E4.10 also hold).

### §27.6 ui-smoke rows: folded into E4.4 (R24); E4.4 moves after E4.11

E4.4 is `waiting` on `origin/tranche-22` (`awk -F'|' '$2 ~ /^ E4\.4 / {print $5}' kanban.md` → ` waiting `), so its
row gains `sheet-gear-bag-of-holding-weightless`, `sheet-gear-move-to-slot-kinds`, `sheet-gear-customize`, `sheet-gear-detail-panel`, `sheet-gear-sacks-one-row` and
`sheet-gear-arrows-split-quiver-backpack`, and **E4.4 stays the only `spec.json` writer**. Because those rows exercise E4.7–E4.11,
E4.4's `Depends on` becomes **E4.11** (it was E4.3): the Phase-2 chain is E4.1 → E4.2 → E4.3 → E4.7 → E4.8 → E4.9
→ E4.10 → E4.11 → E4.4 → E4.5 → E4.6 → E4.MC, and E4.MC depends on E4.5, E4.6, E4.7, E4.8, E4.9, E4.10, E4.11. E4.5 and E4.6 are
unchanged.

### §27.7 What this placement does not change

As §26.5: the live script's §2.1 prefix predates these rows; the card rows' FILES YOU OWN are the grant (§19). The
lead dispatches each card on its row's Tier (`opus`) when its `Depends on` is `complete`. Nothing in Phase 1 is
touched: the import lane's E1.2 already places imported copies where PCGen's `EQUIPSET` says, and a PCGen file
that put an item in a wrong slot loads under E4.8's warning like any other save.

## §28 — Issue #409: the Linux build needs glibc 2.43; the floor is 2.35, stated once and gated (OPERATOR, 2026-11-17; placement DEFAULT)

Added on 2026-11-17 by a planning session in its own worktree (`.worktrees/sd42-cards3`, detached from
`origin/tranche-22`), without touching the lanes' trees. The operator's words are the §3 row LNX-2 / #409. Card count
47 → **48** (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`); every count constant in this package was re-stated in the same commit (R18).

**Verified facts (2026-11-17, this box).** `gh release download alpha-v0.21.153-d6890a6c -R electricm0nk/codex -p
'Codex_0.21.153_amd64.deb'`, `dpkg-deb -x Codex_0.21.153_amd64.deb x`, then
`objdump -T x/usr/bin/codex-desktop | grep -oE 'GLIBC_[0-9.]+' | sort -Vu | tail -1` → **`GLIBC_2.43`** (the symbols above
2.35: `acosf`, `atan2f`, `log10f` @ 2.43; `pidfd_getpid`, `pidfd_spawnp` @ 2.39); the same on `x/usr/bin/pcg_import` →
`GLIBC_2.34`. The AppImage (`-p '*.AppImage'`, `./Codex_0.21.153_amd64.AppImage --appimage-extract`) holds the same two
binaries under `squashfs-root/usr/bin/` with the same maxima (`xdg-open` there is a script). Ubuntu 24.04 ships glibc
**2.39** (`curl -sL https://packages.ubuntu.com/noble/libc6 | grep -oE 'libc6 \([^)]*\)'` → `2.39-0ubuntu8.9`), so the app
cannot start there — the reporter's defect reproduces by inspection. Ubuntu 22.04 ships **2.35** (`…/jammy/libc6` →
`2.35-0ubuntu3.15`); `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev` and `patchelf` all exist in jammy
(`curl -sL https://packages.ubuntu.com/jammy/<pkg> | grep -oE '<title>[^<]*</title>'` names each).

**Cause.** `.github/workflows/publish-tester-release.yml` builds the Linux artifact set (`npx tauri build … --bundles
deb,appimage`, `:333`, after the sidecar at `:327`) in the job at `:238` on `runs-on: ubuntu-latest` (`:240`). The run that
published 0.21.153 (`gh run list -w publish-tester-release.yml -b develop` → run 153, id 44078215972; its Linux job
134045908897) logs `Runner Image … Image: ubuntu-26.04` (`gh api repos/electricm0nk/codex/actions/jobs/134045908897/logs
| grep -A3 'Runner Image'`): GitHub moved `ubuntu-latest` to Ubuntu 26.04 in November 2026 (`actions/runner-images#14748`),
and a Rust binary linked on 26.04 takes the newest symbol versions its host offers. The other jobs (`stamp`, `test`,
`finalize`, the macOS and Windows publish jobs) build no Linux ELF that ships; `pr-tests.yml` runs `cargo test` only; no
other workflow produces a shipped Linux artifact (`grep -lE 'tauri build|appimage|\.deb' .github/workflows/*.yml` → one file).
No other SD-42 card owns anything under `.github/workflows/` (`grep -rn '\.github' *.md` → only C1's `grep -rln` path list).

**The fix and the mechanism (AGENTS.md rule 8: a warning is not a control).** (a) The Linux publish job builds against
glibc 2.35. **Default: `container: ubuntu:22.04` on `runs-on: ubuntu-latest`**, not a runner pin, because the
`ubuntu-22.04` runner image is being retired — "Deprecation: September 17th, 2026 / Retirement: April 17th, 2027", with
brownouts on 2027-03-23, 03-30, 04-06, 04-13 (`gh api repos/actions/runner-images/issues/14254 --jq .body`); the image
is still offered today (`actions/runner-images` README, "Ubuntu 22.04 | x64 | `ubuntu-22.04`", and
`images/ubuntu/Ubuntu2204-Readme.md` → `Image Version: 20261111.360.2`). The container's glibc is the floor for as
long as the `ubuntu:22.04` Docker image exists (jammy standard support to 2027-04, ESM after), independent of the host
runner. **Fallback: `runs-on: ubuntu-22.04`** only if Tauri's AppImage step cannot run inside the container (its
`linuxdeploy` is itself an AppImage; without FUSE it needs `APPIMAGE_EXTRACT_AND_RUN=1` — the executor reads the bundler's
`appimage.sh` for it and sets it in the job `env:` if absent; this box could not check the bundler source); the
fallback is written up as a `§20` correction row naming 2027-04-17 and R-24 is re-ranked. (b) **The gate**:
`tools/release/check_glibc_floor.py` — `GLIBC_FLOOR = "2.35"` is the **one** place the floor lives (no `--floor` option;
the YAML names the image, never the number); inputs are ELF paths and/or `--deb <file>` / `--appimage <file>` (unpacked with
`dpkg-deb -x` / `--appimage-extract` into a temp dir, every ELF under `usr/bin/` scanned by `objdump -T`); one line per
binary `<name> max GLIBC_<v> floor 2.35 OK|ABOVE`; exit 1 on any `ABOVE`, exit 2 on a missing input or no ELF found (never
silent). The publish job runs it on the staged `.deb` and AppImage after `Stage release artifacts`; a red gate stops the
release before any upload. (c) **The self-test** `tools/release/test_check_glibc_floor.py` (pytest; `release-tooling-tests.yml`
already runs `pytest tools/release` on `tools/release/**` changes, and E5.1 runs it): the parser on a fixture holding
0.21.153's `objdump -T` symbol lines → `ABOVE`, exit 1; a fixture whose maximum is `GLIBC_2.34` → `OK`; `/bin/true` on the
test host → `OK` with its maximum printed; a missing path → exit 2. (d) **Proof before release**: a new
`.github/workflows/linux-build-check.yml` (`on: workflow_dispatch` + `pull_request` on `paths:
['.github/workflows/publish-tester-release.yml', '.github/workflows/linux-build-check.yml', 'tools/release/check_glibc_floor.py']`) repeats
the publish job's build steps in the same `container:` (checkout, apt, rust, node, `npm ci`, sidecar, `tauri build
--bundles deb,appimage`, the gate) and uploads nothing; a pytest asserts the two files name the same `image:` and the same
apt package list (so the proof build cannot drift from the shipping one). The executor runs it on `tranche-22`'s head
(`gh workflow run linux-build-check.yml --ref tranche-22`), waits for it, and puts the run URL and the gate's `OK` lines in
the receipt. `publish-tester-release.yml` itself gains no `workflow_dispatch`: it publishes an alpha from any ref it runs
on, and nothing publishes from `tranche-22`.

**Placement.** Phase 2, lane `main`, **E4.12** (the next free id), `opus`, `waiting`, **depends on E4.6** — the last card
in the Phase-2 serial chain before E4.MC — so the one-writer rule holds in `.worktrees/sd42-main` even though the card
touches no file any other card owns; E4.MC's `Depends on` gains E4.12 and quotes the check-workflow run URL. The card
runs no cargo locally (the builds run on GitHub Actions; the pytest is seconds), so it holds no cargo slot. E5.3's release
notes name the fix and the floor; E5.4's PR body carries `Fixes #409` so the issue closes on merge (the §3 row).

**What this does not change.** macOS and Windows builds; the `test`, `stamp` and `finalize` jobs and `pr-tests.yml`
(they ship nothing); users below glibc 2.35 (Ubuntu 20.04) — stated, not supported; the already-published 0.21.x
assets, which are not rebuilt (the reporter gets the next `0.22.<run>` from `develop`, as the lead's comment on #409
says); the Phase-1 lanes. As §26.5 / §27.7: the live script's §2.1 prefix predates this row; the card's FILES YOU OWN
is the grant (§19).
