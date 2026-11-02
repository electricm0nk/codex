---
canonical: true
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
board: local-file ./kanban.md
status: planning-ready
---

# SD-39 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a card
edits that card's row in its own closing commit. A row moves to `complete` only together with a receipt
and its commit SHA.

**Status vocabulary:** `ready` · `waiting` · `in-progress` · `complete` · `partial` (blocks closure) ·
`blocked-escalated` (operator ruling needed; blocks closure).

**Column order is load-bearing:** E9.5's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the script dispatches a card only when every dependency is `complete`.
Row order is dispatch order. **Closure rule:** no PR while any row other than E9.11 is anything but `complete`.

**Same-ID check** (`epic-breakdown.md` ↔ this file):

```bash
cd docs/release/SD-39-ui-print-corrections
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(grep -oE '^\| (C|E)[0-9][0-9.A-Za-z]*' epic-breakdown.md | sed 's/^| //' | sort -u) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 53
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt | lead | complete | — | this commit; branch cut by the lead |
| C1 | 0.19.0 on 14 surfaces + notes pointer (branch already cut) | haiku | complete | C0 | commit 49d598c13a; receipt artifacts/cycle_1/C1_cycle_receipt.md; acceptance command corrected by the lead (decisions.md §25) |
| E1.0 | Slot table + slot_of; prove repeatable and tags | opus | complete | C1 | commit 5014fb84a9; receipt artifacts/epic_1/E1.0_cycle_receipt.md |
| E1.1 | Feat budget rows + refusal | opus | complete | E1.0 | commit 1de994845f + receipt commit; receipt artifacts/epic_1/E1.1_cycle_receipt.md |
| E1.2 | Duplicate feat guard | opus | complete | E1.1 | commit 073780a2bb + receipt commit; receipt artifacts/epic_1/E1.2_cycle_receipt.md |
| E1.3 | Hide-unmet checkbox | opus | complete | E1.2 | commit 42a1c821b3 + receipt commit; receipt artifacts/epic_1/E1.3_cycle_receipt.md |
| E1.5 | ui-smoke feat rows reach the picker | sonnet | complete | E1.3, E1.4 | commit ec9f0becf4 + receipt commit; receipt artifacts/epic_1/E1.5_cycle_receipt.md |
| E1.4 | Add Custom chip styling fix | sonnet | complete | C1 | side lane ui-a; commit 18ca8b657c; receipt artifacts/epic_1/E1.4_cycle_receipt.md |
| E1.MC | E1 merge check + Epic report | opus | complete | E1.3, E1.4, E1.5 | merged nothing; receipt artifacts/epic_1/E1.MC_cycle_receipt.md |
| E1.6 | Feat pools a class grants by offers; Hunter counted once | opus | ready | E1.MC | discovered by E1.MC, decisions.md §21; row-order next |
| E2.1 | Layout registry + Abilities/Details moves + title | opus | complete | E1.MC | commit 7d935657cb + receipt commit; receipt artifacts/epic_2/E2.1_cycle_receipt.md |
| E2.2 | Combat row 2x2 + 1x4 | opus | complete | E2.1 | commit f321378d43 + receipt commit; receipt artifacts/epic_2/E2.2_cycle_receipt.md |
| E2.3 | Show Progression drawer shell | opus | complete | E2.2 | commit c10cbc057c + receipt commit; receipt artifacts/epic_2/E2.3_cycle_receipt.md |
| E2.4 | ui-smoke rows + inventory | sonnet | waiting | E2.3 | |
| E2.MC | E2 merge check + Epic report | opus | waiting | E2.4 | |
| E3.1 | list_market + category fix + +N variants | opus | waiting | E2.MC | |
| E3.2 | Sell + +N purchase on existing purchase_equipment | opus | waiting | E3.1 | |
| E3.3 | Market dialog replaces Add buttons (PF) | opus | waiting | E3.2 | |
| E3.4 | Enhancement bonus → AC/attack/damage rows | opus | waiting | E3.3 | |
| E3.5 | SF + CPR Market shell | opus | waiting | E3.3 | side lane sf-cpr |
| E3.MC | E3 merge check + Epic report | opus | waiting | E3.4, E3.5 | |
| E4.1 | Slot check on set_equipment_active_state; wear if free | opus | waiting | E3.MC | |
| E4.2 | Prove carried adds weight only | opus | waiting | E4.1 | |
| E4.3 | Equipment panel | opus | waiting | E4.2 | |
| E4.4 | Custom item bonuses + slot | opus | waiting | E4.3 | |
| E4.MC | E4 merge check + Epic report | opus | waiting | E4.4 | |
| E5.1 | Ledger file, stamped on every add | opus | waiting | E4.MC | |
| E5.2 | Legacy ledger migration | opus | waiting | E5.1 | |
| E5.3 | Drawer content from list_progression_ledger | opus | waiting | E5.2 | |
| E5.MC | E5 merge check + Epic report | opus | waiting | E5.3 | |
| E6.1 | Overrides toggles + enforcement | opus | waiting | E5.MC | |
| E6.2 | Override tagging UI + DTO | opus | waiting | E6.1 | |
| E6.MC | E6 merge check + Epic report | opus | waiting | E6.2 | |
| E7.1 | .pcg reader + mapping + fixtures (ingest crate) | opus | waiting | E1.0 | side lane ingest |
| E7.2 | Sidecar + Import button + report | opus | waiting | E6.MC, E7.1 | |
| E7.MC | E7 merge check + Epic report | opus | waiting | E7.2 | |
| E8.1 | Print core: Typst, fonts, frame, PF layout, Save PDF cmd | opus | waiting | E7.MC | |
| E8.2 | PF blocks + three options | opus | waiting | E8.1 | |
| E8.3 | SF + CPR layout files | opus | waiting | E8.2 | |
| E8.4 | Settings paper + dialog + buttons + retire CSS print | opus | waiting | E8.3 | |
| E8.5 | Golden suite 24 PDFs | opus | waiting | E8.4 | |
| E8.MC | E8 merge check + Epic report | opus | waiting | E8.5 | |
| E9.1 | Packaged resources + clean build | opus | waiting | E8.MC | |
| E9.2 | Fresh-install acceptance | sonnet | waiting | E9.1 | |
| E9.3 | Widest verify once | sonnet | waiting | E9.2 | |
| E9.4 | Operator eyeball: three printed sheets | sonnet | waiting | E9.3 | operator signs |
| E9.5 | Final-acceptance scan | opus | waiting | E9.4 | |
| E9.6 | Retrospective written + cited | sonnet | waiting | E9.5 | |
| E9.7 | Worktree/branch sweep (sd39-* only) | haiku | waiting | E9.6 | |
| E9.8 | Release notes | sonnet | waiting | E9.7 | |
| E9.9 | Architecture truth-up + claims critic | sonnet + opus | waiting | E9.8 | |
| E9.10 | Graphify LAST | sonnet | waiting | E9.9 | |
| E9.11 | PR tranche-19 → develop (final action) | sonnet | waiting | E9.10 | |
