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

# SD-42 Forward Scope Register

Classes: **Committed** (in the DoD; a shortfall is a blocker), **Structurally implied** (falls out of
committed work), **Candidate** (not in the DoD; a revisit condition is *checked* at E5.2). Only the operator
moves scope between classes. The predecessor is SD-41 (`../SD-41-gear-and-equipment-sets/`), live at
authoring; its campaign write-through (§29), stacks (§28, §30) and Buy Custom (§24) are what SD-42 builds on.

## 1. Committed

| ID | Work | Source | SD-42 cards |
|---|---|---|---|
| COM-1 | One stack per `EQUIPNAME` line with its `QUANTITY`; location from the `EQUIPSET` tree (slot, Equipped, Carried, Not Carried, inside a container, nested) | Intake 2; Q6 | E1.2, E4.1 |
| COM-2 | Custom name and every custom property on the copy; computing fields where Codex has them | Q2 | E1.3, E4.2 |
| COM-3 | Unmappable items imported as Create items at their location; everything unmapped listed in Notes | Q5 | E1.4, E1.5, E4.1 |
| COM-4 | Whole-character parity against the PCGen engine for the 8 proof files; every difference fixed or explained | Q3, Q4 | E1.7 (+ discovered cards) |
| COM-5 | Starfinder `.pcg` imports as an SF character | Q7 | E1.6 |
| COM-6 | A new character every import; the operator's file local only; tests skip loudly; the local run is closure evidence | Q8, Q9 | E4.1, E4.5, E5.G |
| COM-7 | Member #1 is the creator; members a typed list; no roles | CM-1, Q11 | E2.2 |
| COM-8 | Campaign = folder; Drive API path removed; known folders; open/create | CM-2, Q12 | E2.1, E2.3 |
| COM-9 | The help screen (folder sharing, Drive example, any tool, Syncthing + Dropbox, folder structure, Obsidian link, "everything is shared") | Q12, Q13, Q19 | E2.5 |
| COM-10 | Migration of old campaigns with a backup | Q13 | E2.4 |
| COM-11 | Party Resources starter file, free text | Q14 | E2.6 |
| COM-12 | Adventure Log: one file per session per author; frontmatter; sort by `session`; `in_game_date` | Q15, Q16 | E2.7 |
| COM-13 | Maps: image + notes; pins in frontmatter; pin → wiki page | Q17 | E2.8 |
| COM-14 | Wiki: free folders with starters; "+" page; `[[links]]`; backlinks; search | Q18 | E2.9 |
| COM-15 | `Characters/` backup with master device + save counter; stand-in vs move master; diff shown | Q20, Q21, Q22 | E2.10, E4.3 |
| COM-16 | DM toolkit party from the folder | Q20 | E2.11 |
| COM-17 | CodeMirror 6 editor: live preview, byte-stable, `[[` completion | Q24 | E3.1, E3.2 |
| COM-18 | Editor extras: toolbar, auto-save, disk-change warning, drag-in images | Q25 | E3.3 |
| COM-19 | ui-smoke rows for import and every campaign flow; the 8 old rows rewritten | `decisions.md §12.1` SD-h | E4.4 |
| COM-20 | Operator gate; closure in the standing order | §12, §23 | E5.G, E5.1–E5.4 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | `CopyFacts` gains `damage_override`, `weight_override_lbs`, `cost_override_copper`, `properties`; `docs/architecture/persistence.md` names them (E5.3) | COM-2 |
| IMP-2 | The wire (`wire.rs`) gains `equipmentSets`, `gameSystem`, `bio.notes`; `docs/architecture/pcg-import.md` re-printed (E4.MC) | COM-1, COM-3, COM-5 |
| IMP-3 | `fixtures_tests.rs:380` ("a Starfinder soldier cannot compute") flips | COM-5 |
| IMP-4 | `CampaignSnapshot` schema 2: `members[{name, addedAt}]`, no `assets`, no `party_character_ids`; a schema-1 file is read by the migration only | COM-7, COM-8 |
| IMP-5 | 7 `drive_*` registrations leave `main.rs`; new `campaign_folder` commands enter; `capabilities/default.json` gains `opener:default`; `@tauri-apps/plugin-opener` enters `package.json` | COM-8, COM-9 |
| IMP-6 | `campaign_mirror.rs` target `Characters/`; `master.json`; `Membership` gains `stand_in`, `last_backup_counter`; `<app data>/device.json` | COM-15 |
| IMP-7 | `tranche-22` catches up to `develop` once SD-41's PR merges (SD-m) | `decisions.md §4` |
| IMP-8 | Architecture docs (`desktop-app.md`, `persistence.md`, `pcg-import.md`, `testing.md`, `glossary.md`) gain the folder model, the editor, the parity harness | E5.3 |
| IMP-9 | `tauri.conf.json` `bundle.resources` and `tests/packaged_resources.rs` cover any new runtime read (the SF catalog for the sidecar reads `data/starfinder-1e/**`, already packaged for SD-37 — the card asserts it) | COM-5 |

## 3. Candidates (not in the DoD)

| ID | Work | Source | Why not in SD-42's DoD | Revisit condition (checked at E5.2) |
|---|---|---|---|---|
| FSR-C1 | GM / Player roles; per-role visibility | Q11, Q19 | the operator designs roles first | the operator's roles design exists: `ls docs/release/SD-4[3-9]-*/scope-draft.md 2>/dev/null \| xargs -r grep -lis 'roles' \| awk 'END{print NR}'` ≥ 1 |
| FSR-C2 | Secrets (GM-only files) inside the campaign folder | Q19 | waits for roles | as FSR-C1 |
| FSR-C3 | Merging a stand-in's changes back into the master | Q22 | "merged back by hand (Codex may show a diff, no merge)" | the operator asks |
| FSR-C4 | Companions, familiars, mounts and drones attached to an imported character | `decisions.md §21` Q-A, Q-M | PCGen keeps them as separate files; Codex has no attachment model | the operator asks |
| FSR-C5 | Cyberpunk RED import | — | no `.pcg` exists for CPR | the operator asks |
| FSR-C6 | Rendering pins inside Obsidian (a plugin or a canvas file) | Q17 "accepted" | out of Codex | the operator asks |
| FSR-C7 | Reading a campaign's `Characters/` backup into the sheet with write-back (a second master) | Q21 | one master by ruling | the operator asks |
| FSR-C8 | A Market "merge identical stacks across locations" action | Q6 (Market buys already merge into the same stack: `character_hub.rs:4181`) | not requested | the operator asks |

## 4. Planned capability deferrals

The test (`docs/governance/deferral-revisit-doctrine.md`): was this in the DoD when scoped? No, by the
author's default or the operator's silence, named here.

| ID | Capability | Ruling | Revisit condition (a command, checked at E5.2) | Accepted cost |
|---|---|---|---|---|
| FSR-C9 | Remove an attached modifier from a held copy (the Customize dialog, E4.9, lists them and attaches; no remove action has ever existed: `git grep -n 'detach\|removeEquipmentModifier' apps/desktop/src` → 0) | operator REN-1 / CUS-1, `decisions.md §27.3` | the operator asks | sell or discard the copy and buy it plain |
| DEF-1 | Eqmod price formulas the converter does not evaluate (SD-41 DEF-5) | SD-41 §24, Q6 | `python3 -I -c "import json;print(len(json.load(open('data/price_overrides/price-unknown.json'))))"` — the operator asks | an imported copy with such an eqmod is priced by its line's `COST` (`cost_override_copper`) |
| DEF-2 | Drag in ui-smoke (map pins by click are covered; drag-in images are not) | the harness has no drag op (SD-41 DEF-2) | `python3 -I -c "import json;print('drag' in {s['op'] for r in json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'] for s in r['steps']})"` → `True` | drag-in is proven by a unit test of the drop handler and the operator's eye at E5.G |
| DEF-3 | Live preview of tables as rendered grids inside the editor | `decisions.md §18` (monospace alignment; the Reading view renders the grid) | the operator asks | tables read as aligned text while editing |
| DEF-4 | Obsidian-specific syntax beyond `[[links]]`, `![[embeds]]` of images and callouts | Q24 "Obsidian-compatible" | the operator asks | such syntax shows as plain Markdown in Codex; Obsidian renders it |
| DEF-5 | A watcher that reloads a campaign screen when another device changes a file | Q25's "warn if changed on disk during edit" covers the editor only | the operator asks | lists refresh on open and on a Refresh button |

If the operator says any of these was meant to be in the DoD, it becomes a blocker.

## 5. No deferrals for SD-42 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated`.

## 6. Excluded from this scope (the operator passed on them or the request did not name them)

Container tables (shipped, SD-41 E3.11); PCGen's code or data model in live code (`decisions.md §22`); any
network call, OAuth or invite delivery (Q12); roles and secrets (Q11, Q19); an engine change beyond the three
copy-override reads and the SF catalog (`decisions.md §24`); a new sheet tab.
