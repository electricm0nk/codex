---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready — the intake and rulings Q2–Q25 are the operator's (artifacts/cycle_0/operator-brief-2026-11-16.md); the author's defaults are marked and name their revisit (decisions.md §21)
---

# SD-42 Scope Draft — PCGen Import Accuracy and the Campaign Folder

## 1. The request (operator, 2026-11-15 and 2026-11-16; `artifacts/cycle_0/operator-brief-2026-11-16.md`)

After installing alpha v0.20.152 the operator imported a PCGen character (Swashbuckler 10 / Gunslinger 1,
Human) and found the gear wrong: *"Weapons came from the default character (longsword), not the imported
file. Armor: some came, not all."* Stacks were not collapsed: *"Rations, trail listed many times instead of
once with a quantity."* Item 3 of the intake (per-container tables) shipped in SD-41 E3.11 and is **not** in
this bundle.

The rulings of 2026-11-16 (Q2–Q9) make the import a **whole-character** job for **Pathfinder and
Starfinder**: custom PCGen items keep their custom name and **every** custom property (not a base-item
fallback, not text only); the proof set is the operator's file plus seven PCGen sample files at the pinned
oracle; anything the import cannot map is still imported and listed in the character's Notes; stacks are one
row with a count, splittable, for every stack; a fixed import makes a new character; the operator's file
stays on this machine only.

The operator then said *"Campaign Manager will be the focus for the remaining items"* (Q11–Q25): the creator
is always member #1 (no roles yet); a campaign is a plain **folder** of Markdown files synced by whatever the
table already uses (Google Drive, Dropbox, Syncthing, a LAN share) — *"first and foremost free and open"*,
Obsidian-compatible; the Google Drive API path is removed and the folder is the only way; old campaigns
migrate on first open; Party Resources, Adventure Log, Maps with pins and a Wiki with backlinks and search
are built out; characters are backed up into a shared `Characters/` folder with the two-master checks; the
in-app editor is a live-preview CodeMirror 6 editor that never rewrites the user's Markdown, with the
toolbar, auto-save, disk-change warning and drag-in images.

## 2. Definition of Done

SD-42 is done when every row below holds on `tranche-22` and the operator has signed E5.G.

### A. PCGen import (rulings Q2–Q9)

1. **One stack, one row.** Every `EQUIPNAME` line of a `.pcg` becomes exactly one held copy with a piece
   count (`QUANTITY`), at the location its `EQUIPSET` entry names (a body slot, Equipped, Carried, Not
   Carried, or inside the container copy that holds it, nested to any depth). No stack is expanded into
   copies; no `MAX_COPIES` cap. (Q6; `decisions.md §5`.)
2. **Custom names and every custom property.** A `CUSTOMIZATION` item keeps its `NAME`; each `DATA`
   property (`EQMOD` materials, masterwork, enhancement and special abilities; `SPROP`; `_DAMAGE`
   overrides; `COST`/`WT` overrides; `KEY`; `SIZE`; `COSTMOD`) lands on the copy in the field Codex
   computes from, or — when Codex has no computing field for it — on the copy as a printed property **and**
   in the character's Notes. The gear screen, the Weapons tab and the print show the custom name. (Q2, Q5;
   `decisions.md §6`.)
3. **Nothing dropped.** An item whose base Codex does not hold is imported as a Create item (name, weight,
   price, container-ness from the file) at its location, and listed in Notes with the reason; a line of any
   other kind Codex cannot map is listed in Notes as well. (Q5; `decisions.md §7`.)
4. **Whole-character parity.** For each of the 8 proof files, Codex's computed character (classes, race,
   ability scores, HP, saves, AC, attack bonuses, skill totals, feats, spells, gear and money) is compared
   field by field with the real PCGen engine's output at the pinned oracle; every difference is either fixed
   in this bundle or an `explained.tsv` row naming the rule and the decision. (Q4, Q3; `decisions.md §9`.)
5. **Starfinder.** A `GAMEMODE:Starfinder` file imports against the Starfinder engine (SD-37) as an SF
   character, with the same whole-character check; the three SF samples are in the proof set. (Q7;
   `decisions.md §8`.)
6. **A new character every time; the test file never ships.** Import never touches an existing character;
   the operator's `.pcg` is read only from its local path, by tests that skip with a loud reason when it is
   absent; CI proves the 7 samples; the local run is required closure evidence. (Q8, Q9; `decisions.md §10`.)

### B. The Campaign Manager (rulings Q11–Q25)

7. **Member #1.** A new campaign's members list holds its creator first; the Manager never reads "0 members"
   for a campaign that exists. Members are a typed list of names; no roles. (CM-1, Q11; `decisions.md §13`.)
8. **A campaign is a folder.** `campaign.json` + Markdown files in a folder the user picks; the folder is the
   only truth (no localStorage model, no `.config/<name>.json` write-through); the Google Drive API path
   (`campaign_drive.rs`, the Drive settings panel, the Drive access gate) is **removed**; the Manager lists
   the folders it knows and opens or creates one. (Q12; `decisions.md §11, §12`.)
9. **Help screen.** A Help button on the Manager explains folder sharing with Google Drive as the worked
   example, says any sync tool works, cites Syncthing and Dropbox, shows the folder structure Codex created,
   says the files can be edited in Obsidian or any Markdown editor with a link to https://obsidian.md, and
   says plainly that everything in the folder is shared (secrets go in the GM's own folder). (Q12, Q13,
   Q19; `decisions.md §13.3`.)
10. **Migration.** On first open with old campaigns present (localStorage `codex.campaigns`, or an old Drive
    folder's `.config/<name>.json`), Codex asks for a folder per campaign, writes each entry as its own `.md`,
    and keeps a backup of the old data. (Q13; `decisions.md §14`.)
11. **Party Resources.** `Party Resources.md` is created with a starter table and stays fully free-text
    editable in Codex or any editor; nothing parses it. (Q14; `decisions.md §15.1`.)
12. **Adventure Log.** One file per session per author under `Adventure Log/`; "+" creates a file with
    default frontmatter (`campaign`, `author`, `posted`, `session` as a date or a range, optional
    `in_game_date` free text); entries sort by `session`; hand-edited or missing frontmatter never breaks the
    list. (Q15, Q16; `decisions.md §15.2`.)
13. **Maps with pins.** `Maps/<name>.png|jpg` with `Maps/<name>.md` notes; click to drop a pin; each pin
    links to a wiki page; pins live in the notes file's frontmatter. (Q17; `decisions.md §15.3`.)
14. **Wiki.** Free folders under `Wiki/` with starters `NPCs`, `Places`, `Factions`, `Items` (add, rename,
    delete); "+" makes a page with `title`, `type`, `tags` frontmatter; `[[links]]` are clickable; backlinks
    ("pages that link here"); search by name and text. (Q18; `decisions.md §15.4`.)
15. **Characters/ backup with the two-master checks.** Every character save also writes the character's
    folder into `<campaign>/Characters/<slug>-<id>/`; the backup records the master device (per-install id
    and readable name) and a save counter; importing a backup on another device offers "Play as stand-in"
    (default; never writes the backup) or "Move the master here"; before writing, Codex checks the counter
    matches what it last wrote, else it does not overwrite, warns who and when, and the local save still
    succeeds; stand-in differences are shown as a file-level diff, never merged. (Q20–Q22;
    `decisions.md §16`.)
16. **The DM toolkit reads the party from the folder.** The Encounter Builder adds party members from a
    campaign folder's `Characters/` backups, rated by the engine from the backup. (Q20; `decisions.md §17`.)
17. **The editor.** CodeMirror 6 (MIT) with Markdown live preview in Obsidian's manner; opening and saving a
    file without an edit leaves its bytes identical; `[[` suggests wiki page names; toolbar (bold, italic,
    heading, list, link, table); auto-save; a warning when the file changed on disk during the edit; dragged-in
    images saved to `Attachments/` and linked. (Q24, Q25; `decisions.md §18`.)
18. **ui-smoke rows** for the import report, the folder Manager, each tab, the help screen and the Characters/
    flow; the 8 old campaign rows rewritten; the inventory regenerated. (`decisions.md §12.1` SD-h.)
19. **Operator gate (E5.G).** The operator imports the test file on this machine and looks at the gear,
    weapons, armor, skills and feats; opens a campaign folder, every tab and the editor; writes `usable: yes`
    per line. (`decisions.md §12`.)
20. **Closure.** All cards `complete` → final-acceptance scan → retrospective written and cited → sweep →
    release notes → architecture truth-up + claims critic → **graphify LAST** → **PR last**
    (`decisions.md §23`). The lead merges.

## 3. Not in the Definition of Done (planned deferrals, `forward-scope-register.md §4`)

GM/Player roles and secrets (Q11, Q19: the operator designs roles first); two-way merge of a stand-in's
changes (Q22: diff shown, merge by hand); invites or any network delivery (Q12: invites happen outside
Codex); rendering pins in Obsidian (Q17: accepted); the SF1e drone file as a character when the SF engine
holds no Drone class (`decisions.md §21` Q-A); Cyberpunk RED import (no `.pcg` exists for it); PCGen
companions, familiars and mounts attached to the imported character (the Camel in the operator's file is
imported as the gear line it is); eqmod price formulas the converter does not evaluate (SD-41 DEF-5 stands).

## 4. Epic order and the SD-41 partition

C → **three Phase-1 lanes in parallel** on files no open SD-41 card owns (`decisions.md §4`): **E1 import
converter** (`crates/codex-ingest/src/pcg_import/**`, root crate), **E2 campaign folder model + Manager**
(`src/campaign/**`, `src/saved_character/campaign_mirror.rs`, `apps/desktop/src-tauri/src/campaign_folder.rs`
*(new)*, `apps/desktop/src/campaign/**`, `main.rs` registration lines), **E3 editor** (`apps/desktop/src/editor/**`
*(new)*, `package.json`) → *SD-41 merges; `tranche-22` catches up to `develop`* → **E4 Phase 2** (every write to
`character_hub.rs`: the import writes `equipment_sets.json` and the Notes; copy properties on the DTOs, the
screen and the print; the mirror hooks for `Characters/`; the import and campaign smoke rows) → **E5 gate and
closure**.

## 5. Mode

Unattended after launch except at E5.G. A taste question inside scope takes the safe default in
`decisions.md §12.1` and is logged; nothing else waits. A question only the operator can answer is one of
`decisions.md §21`'s, each with its default already taken.

## 6. One bundle, not two (`decisions.md §25`)

The import lane and the campaign lane share no file (`content-unit-inventory.md` F-20), so the bundle's wall
time is the longer lane, not the sum. The author keeps SD-42 as one bundle; the split point, should the
operator prefer one, is named in `risks-and-open-questions.md §4`.
