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
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 37 at authoring (2 + 8 + 12 + 4 + 6 + 5)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the lead 2026-11-16 from `origin/tranche-21` @ `1046daefc0` |
| C1 | 0.22.0 on 14 surfaces + notes pointer | haiku | complete | C0 | commit `589815b65f`; receipt `artifacts/cycle_0/C1_cycle_receipt.md`; Phase 1, first |
| E1.1 | Diagnosis run over the 8 files; H1–H7 proved or refuted | opus | waiting | C1 | lane `import`; the operator's table stays local |
| E1.2 | One stack per line; `equipmentSets` on the wire; locations from the set tree; `QUANTITY:0` noted; no cap | opus | waiting | E1.1 | `decisions.md §5` |
| E1.3 | Custom names and every property; `CopyFacts` gains four fields | opus | waiting | E1.2 | `decisions.md §6` |
| E1.4 | Unmappable items → Create items at their location | opus | waiting | E1.3 | `decisions.md §7` |
| E1.5 | The Notes block in `bio.notes` | opus | waiting | E1.4 | `decisions.md §7` |
| E1.6 | Starfinder import; the drone per Q-A | opus | waiting | E1.5 | `decisions.md §8` |
| E1.7 | Whole-character parity harness + gate; `explained.tsv` | opus | waiting | E1.6 | `decisions.md §9`; P-9 |
| E1.MC | E1 merge check + Epic report | opus | waiting | E1.7 | adversarial; the import lane ends |
| E2.1 | Folder model (root) + `campaign_folder.rs` commands + known folders | opus | waiting | C1 | lane `campaign`; `decisions.md §11` |
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
| E2.MC | E2 merge check + Epic report (≥ 10 screenshots, looked at) | opus | waiting | E2.11 | adversarial; the campaign lane ends |
| E3.1 | Editor core: packages, byte-stable round trip, live preview, reading view | opus | complete | C1 | lane `editor`; `decisions.md §18`; commit `75e9feca1d`; receipt `artifacts/epic_3/E3.1_cycle_receipt.md` |
| E3.2 | `[[` completion; link clicks | opus | waiting | E3.1 | |
| E3.3 | Toolbar, auto-save, disk-change warning, drag-in images | opus | waiting | E3.2 | `save_attachment` fence exit rule in the row |
| E3.MC | E3 merge check + Epic report | opus | waiting | E3.3 | adversarial; the editor lane ends |
| E4.1 | Import writes `equipment_sets.json`, Create items, Notes, `game_system`; a new root every import | opus | waiting | E1.MC, E2.MC, E3.MC, SD-41 merged | Phase 2; first write to `character_hub.rs` (R25) |
| E4.2 | Copy overrides end to end (engine, DTOs, screen, Weapons tab, print) | opus | waiting | E4.1 | G-1 `NEW_KEYS`; G-2 |
| E4.3 | Mirror hooks: `after_write` with stand-in + counter; membership dialog over folders; import choice | opus | waiting | E4.2 | `decisions.md §16` |
| E4.4 | ui-smoke rows: import + campaign (new + 8 rewritten); inventory | opus | waiting | E4.3 | the only `spec.json` writer (R24) |
| E4.5 | Proof runs: 7 samples (CI shape) + the operator's file (local, counts) | opus | waiting | E4.4 | `decisions.md §10`; P-8 |
| E4.MC | E4 merge check + Epic report + E5.G's screenshots | opus | waiting | E4.5 | adversarial |
| E5.G | OPERATOR GATE: import + Campaign Manager + editor, `usable: yes` ×3 | sonnet | waiting | E4.MC | the run ends here until the operator signs |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E5.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-22 → develop (final action) | sonnet | waiting | E5.3 | |
