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
