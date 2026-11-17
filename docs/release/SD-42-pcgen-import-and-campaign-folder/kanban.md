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
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 46: 37 at authoring (2 + 8 + 12 + 4 + 6 + 5) + E1.8 (discovered by E1.2) + E2.12, E2.13, E4.6 (operator, 2026-11-17, decisions.md §26) + E4.7, E4.8, E4.9, E4.10, E4.11 (operator, 2026-11-17, decisions.md §27)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the lead 2026-11-16 from `origin/tranche-21` @ `1046daefc0` |
| C1 | 0.22.0 on 14 surfaces + notes pointer | haiku | complete | C0 | commit `589815b65f`; receipt `artifacts/cycle_0/C1_cycle_receipt.md`; Phase 1, first |
| E1.1 | Diagnosis run over the 8 files; H1–H7 proved or refuted | opus | complete | C1 | receipt `artifacts/epic_1/E1.1_cycle_receipt.md`; H1 H3 H4 H7 proved, H2 proved (a narrower: 1 slot conflict, not 3), H5 refuted, H6 not exercised; operator table local |
| E1.2 | One stack per line; `equipmentSets` on the wire; locations from the set tree; `QUANTITY:0` noted; no cap | opus | complete | E1.1 | commit: the one adding the E1.2 receipt; receipt `artifacts/epic_1/E1.2_cycle_receipt.md`; 6/6 `pcg_import_stacks`; discovered E1.8 |
| E1.3 | Custom names and every property; `CopyFacts` gains four fields | opus | complete | E1.2 | commit: the one adding the E1.3 receipt (code `0b494a0bfe`); receipt `artifacts/epic_1/E1.3_cycle_receipt.md`; 6/6 `pcg_import_custom`, 2/2 `copy_facts_` |
| E1.4 | Unmappable items → Create items at their location | opus | complete | E1.3 | commit: the one adding the E1.4 receipt (code `d7cacc4343`); receipt `artifacts/epic_1/E1.4_cycle_receipt.md`; 3/3 `pcg_import_unmapped` |
| E1.5 | The Notes block in `bio.notes` | opus | waiting | E1.4 | `decisions.md §7` |
| E1.6 | Starfinder import; the drone per Q-A | opus | waiting | E1.5 | `decisions.md §8` |
| E1.7 | Whole-character parity harness + gate; `explained.tsv` | opus | waiting | E1.6 | `decisions.md §9`; P-9 |
| E1.8 | Bundle items: PCGen `QUANTITY` counts bundles; pieces = × `base_quantity` | opus | waiting | E1.7 | discovered by E1.2; `epic-breakdown.md` E1.8 |
| E1.MC | E1 merge check + Epic report | opus | waiting | E1.7, E1.8 | adversarial; the import lane ends |
| E2.1 | Folder model (root) + `campaign_folder.rs` commands + known folders | opus | complete | C1 | code `039dd404a5`; receipt `artifacts/epic_2/E2.1_cycle_receipt.md`; root 5 + desktop 3 tests |
| E2.2 | Member #1; typed members; Create / Edit / Manager over the folder | opus | waiting | E2.1 | `decisions.md §13.1` |
| E2.3 | Drive removal; folder-backed Manager; hub modes carry a folder | opus | waiting | E2.2 | `decisions.md §13.2`; NR-6 |
| E2.4 | Migration screen + command + backup | opus | waiting | E2.3 | `decisions.md §14` |
| E2.5 | Help screen; `opener:default`; Obsidian link; live tree | opus | waiting | E2.4 | `decisions.md §13.3` |
| E2.6 | Party Resources tab | opus | waiting | E2.5, E3.1 | `decisions.md §15.1` |
| E2.7 | Adventure Log tab + frontmatter parser | opus | waiting | E2.6 | `decisions.md §15.2` |
| E2.8 | Maps tab with pins | opus | waiting | E2.7 | `decisions.md §15.3` |
| E2.9 | Wiki tab: folders, pages, links, backlinks, search | opus | waiting | E2.8 | `decisions.md §15.4` |
| E2.10 | `Characters/` backup model: master.json, counter, stand-in, move master, diff | opus | waiting | E2.9 | `decisions.md §16` |
| E2.11 | DM toolkit party from the folder | opus | waiting | E2.10 | `decisions.md §17` |
| E2.12 | "Main menu" is the first ☰ entry on every screen but the main one | opus | waiting | E2.11 | operator UI-1 2026-11-17; `decisions.md §26.1`; owns `ui/layout/AppMenu.{tsx,test.ts}` |
| E2.13 | The Manager's open-campaign button: full column width, the main screen's Campaign Manager style | opus | waiting | E2.12 | operator UI-2 2026-11-17; `decisions.md §26.2`; owns `ui/characterHub/LandingScreen.tsx` (export only) |
| E2.MC | E2 merge check + Epic report (≥ 10 screenshots, looked at) | opus | waiting | E2.13 | adversarial; the campaign lane ends |
| E3.1 | Editor core: packages, byte-stable round trip, live preview, reading view | opus | complete | C1 | lane `editor`; `decisions.md §18`; commit `75e9feca1d`; receipt `artifacts/epic_3/E3.1_cycle_receipt.md` |
| E3.2 | `[[` completion; link clicks | opus | complete | E3.1 | commit `931098912b`; receipt `artifacts/epic_3/E3.2_cycle_receipt.md` |
| E3.3 | Toolbar, auto-save, disk-change warning, drag-in images | opus | waiting | E3.2 | `save_attachment` fence exit rule in the row |
| E3.MC | E3 merge check + Epic report | opus | waiting | E3.3 | adversarial; the editor lane ends |
| E4.1 | Import writes `equipment_sets.json`, Create items, Notes, `game_system`; a new root every import | opus | waiting | E1.MC, E2.MC, E3.MC, SD-41 merged | Phase 2; first write to `character_hub.rs` (R25) |
| E4.2 | Copy overrides end to end (engine, DTOs, screen, Weapons tab, print) | opus | waiting | E4.1 | G-1 `NEW_KEYS`; G-2 |
| E4.3 | Mirror hooks: `after_write` with stand-in + counter; membership dialog over folders; import choice | opus | waiting | E4.2 | `decisions.md §16` |
| E4.7 | Weightless container contents: PCGen `CONTAINS:*N` / `N%` parsed; Bag of Holding and Handy Haversack contents add nothing to the carried total; nesting; capacity check still counts | opus | waiting | E4.3 | operator BAG-1 / HVS-1 2026-11-17; `decisions.md §27.1`; settled bundles regenerated |
| E4.8 | Slot kinds: a copy is offered and accepted only in a slot of its own kind (menu, drag, backend); the slot override lifts it; a saved wrong slot loads with a warning | opus | waiting | E4.7 | operator SLOT-1 2026-11-17; `decisions.md §27.2` |
| E4.9 | Customize a held copy: one button per equip row / container-table row opens rename (per copy, `CopyFacts.name`) + attach modifier (the existing flow, now per copy); the redundant list under the equip section and everything only it used deleted; print `Camping Gear (Sack)` | opus | waiting | E4.8 | operator REN-1 + CUS-1 2026-11-17; `decisions.md §27.3` |
| E4.10 | Gear detail panel: selecting a row shows the record (weight, cost, type, slot, weapon/armor stats, capacity, description, book) and the copy (name, size, modifiers, overrides, quantity, location); `—` for absent | opus | waiting | E4.9 | operator DET-1 2026-11-17; `decisions.md §27.4` |
| E4.11 | Stacks: identical copies at one place are one row with the count everywhere (screen tree, print); a container copy holding anything, or a renamed one, is its own row; a stack splits by Move to's "how many?" and the parts move on their own; parts rejoin when put back together | opus | waiting | E4.10 | operator STK-1 2026-11-17; `decisions.md §27.5` |
| E4.4 | ui-smoke rows: import + campaign (new + 8 rewritten) + E2.12/E2.13's two rows + E4.7–E4.11's six rows; inventory | opus | waiting | E4.11 | the only `spec.json` writer (R24); `decisions.md §26.4, §27.6` |
| E4.5 | Proof runs: 7 samples (CI shape) + the operator's file (local, counts) | opus | waiting | E4.4 | `decisions.md §10`; P-8 |
| E4.6 | Linux AppImage writes its own menu entry + icon on every start; re-points after a move or update; never for .deb/.rpm/Windows/macOS | opus | waiting | E4.5 | operator LNX-1 / Q27 2026-11-17; `decisions.md §26.3`; `desktop/update/self_install.rs` only |
| E4.MC | E4 merge check + Epic report + E5.G's screenshots | opus | waiting | E4.5, E4.6, E4.7, E4.8, E4.9, E4.10, E4.11 | adversarial |
| E5.G | OPERATOR GATE: import + Campaign Manager + editor, `usable: yes` ×3 | sonnet | waiting | E4.MC | the run ends here until the operator signs |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E5.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-22 → develop (final action) | sonnet | waiting | E5.3 | |
