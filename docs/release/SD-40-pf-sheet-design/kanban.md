---
canonical: true
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
board: local-file ./kanban.md
status: planning-ready
---

# SD-40 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a card
edits that card's row in its own closing commit. A row moves to `complete` only together with a receipt
and its commit SHA.

**Status vocabulary:** `ready` · `waiting` · `in-progress` · `complete` · `partial` (blocks closure) ·
`blocked-escalated` (operator ruling needed; blocks closure).

**Column order is load-bearing:** E5.3's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the script dispatches a card only when every dependency is `complete`.
Row order is dispatch order. **Closure rule:** no PR while any row other than E5.5 is anything but `complete`.
**Two operator gates:** E2.G and E5.2 stay `partial` until the operator's `usable: yes` lines exist.

**Same-ID check** (`epic-breakdown.md` ↔ this file):

```bash
cd docs/release/SD-40-pf-sheet-design
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(grep -oE '^\| (C|E)[0-9][0-9.A-Za-z]*' epic-breakdown.md | sed 's/^| //' | sort -u) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 40 (24 + E2.4 by E2.MC; + E2.5 at E2.G round 1; + 12 by the E2.G round-1 replan; + E3.6 by the operator's answers, decisions.md §23.7; + E2.15 by E2.7)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt | lead | complete | — | this commit; branch cut by the lead 2026-11-07 |
| C1 | 0.20.0 on 14 surfaces + notes pointer | haiku | complete | C0 | version commit `3ceeb63b66`; receipt `artifacts/cycle_1/C1_cycle_receipt.md` |
| E0.1 | PCGen reference render + block map + density figures | opus | complete | C0 | side lane `ref`; no cargo; artifacts/epic_0/E0.1_cycle_receipt.md; std 3 pp, simple 3 pp |
| E0.2 | Repair the 14 stale ui-smoke rows | opus | complete | C1 | side lane `smoke`; rows `02b9fa99da`; receipt `artifacts/epic_0/E0.2_cycle_receipt.md`; RED 0/14 → GREEN 14/14, spec 173 rows (none removed); lands before E4.2 (both write `spec.json`); decisions.md §22 |
| E1.1 | Layout model: id, look, place, columns; dispatcher; grid; vocabulary fixture | opus | complete | C1 | code `ad412a4a88`; receipt `artifacts/epic_1/E1.1_cycle_receipt.md`; SF/CPR 16/16 byte-identical; PF pages unchanged |
| E1.2 | Proof harness: print_proof + rasterize.sh | sonnet | complete | E1.1 | receipt `artifacts/epic_1/E1.2_cycle_receipt.md`; 16 PDFs, 130 PNGs |
| E1.MC | E1 merge check + Epic report | opus | complete | E1.2, E0.1 | tests `a537c27737`; receipt `artifacts/epic_1/E1.MC_cycle_receipt.md`; print_ 65 passed; G-1 empty; G-2 16/16 byte-identical; PF seeds 3/3 + `.pcg` imports 37 saved of 65 files, 296 renders, 0 diagnostics |
| E2.1 | Number-box looks: abilities, hitPoints, initiative, armorClass, attack, saves, speed; Combat split | opus | complete | E1.MC | code `9b3edac157`; receipt `artifacts/epic_2/E2.1_cycle_receipt.md`; print_pf_blocks 19, print_smoke 9, print_golden 24; G-1 empty; G-2 16/16 byte-identical; PF golden pages unchanged |
| E2.2 | Skills look (two narrow columns) | opus | complete | E2.1 | code `972b033a81`; receipt `artifacts/epic_2/E2.2_cycle_receipt.md`; print_pf_blocks 20, print_golden 24, print_smoke 9; G-1 empty; G-2 16/16 byte-identical; PF golden pages unchanged; Skills + Weapons row now prints whole on page 2 (page 1 is E2.3) |
| E2.3 | Weapons look + page 1 assembled on the grid | opus | complete | E2.2 | code `0d8465c13f` + `7d8983278b`; receipt `artifacts/epic_2/E2.3_cycle_receipt.md`; print_pf_blocks 22, print_golden 24, print_smoke 9; G-1 empty; G-2 16/16 byte-identical; PF golden pages A4 5/6/12/14 → 4/5/11/13, Letter unchanged; ten step-1 blocks on page 1 on Letter and A4 (3 seeds + a 3-weapon variant); `Place.under` stacked cells; fence exits layout/mod.rs, frame.typ, 4 looks, print_smoke.rs |
| E2.MC | E2 merge check + Epic report | opus | complete | E2.3 | test `f5e4c781b2`; receipt `artifacts/epic_2/E2.MC_cycle_receipt.md`; print_pf_blocks 23, print_golden 24, print_smoke 9, desktop 1250 + 2 passed; G-1 empty; G-2 16/16 byte-identical vs C1; page 1 whole (no run-over) + population probe (bought dagger + ring, no ranks, Elowen); discovered E2.4 |
| E2.5 | Page-1 header: portrait + Character Name + Campaign; no title, no identity block (rewritten, `decisions.md §23`) | opus | complete | E2.MC | code `f6bc0cd545`; receipt `artifacts/epic_2/E2.5_cycle_receipt.md`; print_pf_blocks 26 (23 + 3), print_golden 24, print_smoke 9, desktop 1253 + 2 passed; G-1 empty; G-2 16/16 byte-identical; PF A4 pages 4/5/11/13 → 5/6/12/14, Letter unchanged; Initiative + Hit Points share the top band for Letter room (E2.9 re-decides) |
| E2.6 | Engine: conditional notes for HP, initiative, saves, AC (`decisions.md §23`) | opus | complete | E2.MC | side lane `engine` (`.worktrees/sd40-engine`); no `print/**`; `3cfa76522c` + `abb7effa16` + `0519db3cd5`; receipt `artifacts/epic_2/E2.6_cycle_receipt.md`; root conditional_notes 6 passed, desktop 2 passed; G-1 SAME_EXCEPT_NAMED ×3 (`conditionalNotes`); Aldric holds Bravery (acceptance corrected) |
| E2.7 | Engine: AC parts as explanations rows; 10 + Σ parts = AC | opus | complete | E2.6 | side lane `engine`; code `a957b928e5` + `d945250ec1`; receipt `artifacts/epic_2/E2.7_cycle_receipt.md`; root armor_class_parts 5 passed, desktop 1 passed; `.pcg` probe 37 of 37 saved state parts, 37 sum; G-1 SAME_EXCEPT_NAMED ×3 (`defense.armor_class.*`); AC row re-pointed where the fixed path's sum differs (Aldric + ring 16 → 17); discovered E2.15 |
| E2.8 | Engine: remaining lb + known languages on the load response; the screen reads them | opus | waiting | E2.7 | side lane `engine`; operator Q1 "Known + one blank line" |
| E2.14 | Engine: per-weapon damage total + range | opus | waiting | E2.8 | side lane `engine`; operator Q5 "One total" |
| E2.15 | Engine: a custom item's AC bonus in the AC row and its part | opus | waiting | E2.14 | side lane `engine`; discovered by E2.7 (`character_custom.rs:427` moves the printed AC, not the row or parts) |
| E2.9 | Square block: HP + Initiative + Encumbrance + Conditions, with notes | opus | waiting | E2.5, E2.6, E2.8 | |
| E2.10 | Totals only: abilities, saves + notes, attack, speed, weapons | opus | waiting | E2.9, E2.14 | |
| E2.11 | Skills: one column, name + total, no wraps; not-added lines under the column | opus | waiting | E2.10 | |
| E2.12 | AC breakdown filled from the engine's part rows | opus | waiting | E2.11, E2.7, E2.15 | |
| E2.13 | Languages on page 1: comma list + exactly one write-in line | opus | waiting | E2.12, E2.8 | |
| E2.4 | Page 1 assembled + overflow rule (rewritten, `decisions.md §23.5`) | opus | waiting | E2.13 | was "4+ weapon lines"; parked `8f75f60b3e` on `sd40/e2.4-parked` is a starting point only; the lead deletes the branch when this is complete |
| E2.MC2 | E2 merge check, round 2 + Epic report | opus | waiting | E2.4 | |
| E2.G | OPERATOR GATE: Aldric page 1 printed, `usable: yes` (round 2) | sonnet | partial | E2.MC2 | PDFs + PNGs rendered to artifacts/epic_2/E2.G/; receipt holds the sign-off lines; round 1 (2026-11-11) recorded → `decisions.md §23`; round 2 re-renders after E2.MC2 and waits for the operator's `usable: yes` |
| E3.1 | Feats, class features, actions looks (section 2) | opus | waiting | E2.G | rewritten: no `feats-full` |
| E3.2 | Inventory pages: defense, gear (worn / carried / left behind), money (section 3) | opus | waiting | E3.1 | rewritten |
| E3.3 | Daily spells, spellbook, custom, pets looks (sections 2, 4) | opus | waiting | E3.2 | rewritten: bio, math, details moved to E3.4, E3.5 |
| E3.6 | Backstory field in the app (Bio UI + `bio.json`), ui-smoke row | opus | waiting | E3.3, E0.2 | operator answer Q2, `decisions.md §23.7`; before E3.4; writes `spec.json` after E0.2 |
| E3.4 | Bio pages: details, Backstory, campaign notes, ruled lines (section 5) | opus | waiting | E3.6 | |
| E3.5 | Show-the-math pages: every page-1 breakdown + derivations (section 6) | opus | waiting | E3.4 | |
| E3.MC | E3 merge check + Epic report | opus | waiting | E3.5 | |
| E4.1 | Sections as lists; `PrintSections`; golden 24 + 6 | opus | waiting | E3.MC | rewritten (was presets; `decisions.md §9` superseded by §23.4) |
| E4.2 | Print dialog: six checkboxes remembered per character; booleans removed; ui-smoke rows | opus | waiting | E4.1, E0.2 | rewritten; serial edge E0.2 → E4.2 (`spec.json`) |
| E4.MC | E4 merge check + Epic report | opus | waiting | E4.2 | |
| E5.1 | Widest verify + clean build + packaged resources | sonnet | waiting | E4.MC | |
| E5.2 | OPERATOR GATE: the full set printed, `usable: yes` ×5 | sonnet | waiting | E5.1 | the run ends here until the operator signs |
| E5.3 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.2 | |
| E5.4 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.3 | |
| E5.5 | Graphify LAST → PR tranche-20 → develop (final action) | sonnet | waiting | E5.4 | |
