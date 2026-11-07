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
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 24
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt | lead | complete | — | this commit; branch cut by the lead 2026-11-07 |
| C1 | 0.20.0 on 14 surfaces + notes pointer | haiku | complete | C0 | version commit `3ceeb63b66`; receipt `artifacts/cycle_1/C1_cycle_receipt.md` |
| E0.1 | PCGen reference render + block map + density figures | opus | complete | C0 | side lane `ref`; no cargo; artifacts/epic_0/E0.1_cycle_receipt.md; std 3 pp, simple 3 pp |
| E0.2 | Repair the 14 stale ui-smoke rows | opus | waiting | C1 | side lane `smoke` beside E1; lands before E4.2 (both write `spec.json`); operator answer "1: B", decisions.md §22 |
| E1.1 | Layout model: id, look, place, columns; dispatcher; grid; vocabulary fixture | opus | waiting | C1 | |
| E1.2 | Proof harness: print_proof + rasterize.sh | sonnet | waiting | E1.1 | |
| E1.MC | E1 merge check + Epic report | opus | waiting | E1.2, E0.1 | |
| E2.1 | Number-box looks: abilities, hitPoints, initiative, armorClass, attack, saves, speed; Combat split | opus | waiting | E1.MC | |
| E2.2 | Skills look (two narrow columns) | opus | waiting | E2.1 | |
| E2.3 | Weapons look + page 1 assembled on the grid | opus | waiting | E2.2 | |
| E2.MC | E2 merge check + Epic report | opus | waiting | E2.3 | |
| E2.G | OPERATOR GATE: Aldric page 1 printed, `usable: yes` | sonnet | waiting | E2.MC | the run ends here until the operator signs |
| E3.1 | Feats, class features, actions looks | opus | waiting | E2.G | |
| E3.2 | Defense, gear (worn / carried / left behind), money looks | opus | waiting | E3.1 | |
| E3.3 | Spells, spellbook, bio, math, custom, pets, character details looks | opus | waiting | E3.2 | |
| E3.MC | E3 merge check + Epic report | opus | waiting | E3.3 | |
| E4.1 | Presets as lists; PrintPreset; golden cases renamed | opus | waiting | E3.MC | |
| E4.2 | Print dialog offers presets; booleans removed; ui-smoke rows | opus | waiting | E4.1, E0.2 | serial edge E0.2 → E4.2 (`spec.json`) |
| E4.MC | E4 merge check + Epic report | opus | waiting | E4.2 | |
| E5.1 | Widest verify + clean build + packaged resources | sonnet | waiting | E4.MC | |
| E5.2 | OPERATOR GATE: the full set printed, `usable: yes` ×5 | sonnet | waiting | E5.1 | the run ends here until the operator signs |
| E5.3 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.2 | |
| E5.4 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.3 | |
| E5.5 | Graphify LAST → PR tranche-20 → develop (final action) | sonnet | waiting | E5.4 | |
