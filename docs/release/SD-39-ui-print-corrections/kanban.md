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
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 65
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
| E1.6 | Feat pools a class grants by offers; Hunter counted once | opus | complete | E1.MC | commit 2ce249f1fa + receipt commit; receipt artifacts/epic_1/E1.6_cycle_receipt.md |
| E2.1 | Layout registry + Abilities/Details moves + title | opus | complete | E1.MC | commit 7d935657cb + receipt commit; receipt artifacts/epic_2/E2.1_cycle_receipt.md |
| E2.2 | Combat row 2x2 + 1x4 | opus | complete | E2.1 | commit f321378d43 + receipt commit; receipt artifacts/epic_2/E2.2_cycle_receipt.md |
| E2.3 | Show Progression drawer shell | opus | complete | E2.2 | commit c10cbc057c + receipt commit; receipt artifacts/epic_2/E2.3_cycle_receipt.md |
| E2.4 | ui-smoke rows + inventory | sonnet | complete | E2.3 | commit 22688208ec + receipt commit; receipt artifacts/epic_2/E2.4_cycle_receipt.md; full-suite 64/133 green, 14 red, 52 blocked |
| E2.MC | E2 merge check + Epic report | opus | complete | E2.4 | merged nothing; receipt artifacts/epic_2/E2.MC_cycle_receipt.md |
| E3.1 | list_market + category fix + +N variants | opus | complete | E2.MC | commit 93c596cc1c + receipt commit; receipt artifacts/epic_3/E3.1_cycle_receipt.md; Armor cross-type 0 of 63 |
| E3.2 | Sell + +N purchase on existing purchase_equipment | opus | complete | E3.1 | commit 61aad6b9ab + receipt commit; receipt artifacts/epic_3/E3.2_cycle_receipt.md; discovered E3.6 |
| E3.3 | Market dialog replaces Add buttons (PF) | opus | complete | E3.2 | commit f905c625d2 + receipt commit; receipt artifacts/epic_3/E3.3_cycle_receipt.md; 8 market rows green |
| E3.4 | Enhancement bonus → AC/attack/damage rows | opus | complete | E3.3 | commit af1e860a9c + 1e81c4b92d + receipt commit; receipt artifacts/epic_3/E3.4_cycle_receipt.md; 210 + 49 offers reach the sheet |
| E3.5 | SF + CPR Market shell | opus | complete | E3.3 | side lane sf-cpr; commit 9efc712f1d + receipt commit; receipt artifacts/epic_3/E3.5_cycle_receipt.md; 15 market_shell tests, 10 market rows green; seeds NO_DIFF |
| E3.6 | Attach Modifier charges the +N it attaches | opus | complete | E3.3 | discovered by E3.2, decisions.md §21; commit c896b64374 + receipt commit; receipt artifacts/epic_3/E3.6_cycle_receipt.md; 18 market_attach_price tests |
| E3.MC | E3 merge check + Epic report | opus | complete | E3.4, E3.5, E3.6 | commit 6adc8d39c3 + receipt commit; receipt artifacts/epic_3/E3.MC_cycle_receipt.md; 4 refuted, all fixed here; no card discovered |
| E4.1 | Slot check on set_equipment_active_state; wear if free | opus | complete | E3.MC | commit 660246480a + receipt commits; receipt artifacts/epic_4/E4.1_cycle_receipt.md; desktop and root sweeps exit 0, 20 market rows green, seeds NO_DIFF |
| E4.2 | Prove carried adds weight only | opus | complete | E4.1 | commits 960de1b200, 3d91eab9a9 + receipt commit; receipt artifacts/epic_4/E4.2_cycle_receipt.md; discovered E4.5; root sweep exit 0, 20 market rows green, seeds NO_DIFF |
| E4.3 | Equipment panel | opus | complete | E4.2 | commit 2123327b7b + receipt commit; receipt artifacts/epic_4/E4.3_cycle_receipt.md; override tag moved to E6.2 (cycle, decisions.md §25); root and desktop sweeps clean, 24 rows green, seeds NO_DIFF |
| E4.4 | Custom item bonuses + slot | opus | complete | E4.3 | commit ab1300e371 + receipt commit; receipt artifacts/epic_4/E4.4_cycle_receipt.md; 11 custom_item_bonuses tests; desktop sweep clean, 26 rows green, seeds NO_DIFF |
| E4.5 | A worn item's stated numbers reach the totals | opus | complete | E4.4 | discovered by E4.2, decisions.md §21; commit efdf8ce4b2 + receipt commit; receipt artifacts/epic_4/E4.5_cycle_receipt.md; 7 desktop + 4 root worn_item_applies tests; both sweeps attributed, 2 test edits re-run; 26 ui-smoke rows green; seeds NO_DIFF; discovered E4.6 |
| E4.6 | A worn item's stated attack, initiative and speed reach the totals | opus | complete | E4.5 | discovered by E4.5, decisions.md §21; commit 5c21256395 + receipt commit; receipt artifacts/epic_4/E4.6_cycle_receipt.md; 6 desktop + 1 root worn_item_applies_attack tests; root 8599 passed, desktop 1048 passed, both 0 failed; seeds NO_DIFF; discovered E4.7 |
| E4.7 | A worn item's every-skill, CMB, CMD, Armor Class and speed-multiplier numbers reach the totals | opus | complete | E4.6 | discovered by E4.6, decisions.md §21; commit b28d044e20 + receipt commit; receipt artifacts/epic_4/E4.7_cycle_receipt.md; 6 desktop + 3 root worn_item_applies_every_skill tests; desktop 1053+1 passed, 0 failed; root 8601 passed, 1 failed in the sweep, that suite's reference read fixed and 3 passed on re-run; seeds NO_DIFF |
| E4.MC | E4 merge check + Epic report | opus | complete | E4.7 | commit 8aedb292b7 + receipt commit; receipt artifacts/epic_4/E4.MC_cycle_receipt.md; 4 refuted, all fixed here; root 8602 passed, desktop 1059 passed, both 0 failed; 26 ui-smoke rows green; seeds NO_DIFF; no card discovered |
| E5.1 | Ledger file, stamped on every add | opus | complete | E4.MC | commit 5b24205733 + receipt commit; receipt artifacts/epic_5/E5.1_cycle_receipt.md; 7 ledger_stamp tests; desktop 1066 passed, 0 failed; G-6 re-run for the fence exits; seeds NO_DIFF; no card discovered |
| E5.2 | Legacy ledger migration | opus | complete | E5.1 | commit 0d1d7aa995 + receipt commit; receipt artifacts/epic_5/E5.2_cycle_receipt.md; 14 ledger_migration tests; root sweep exit 0; desktop 1077 passed, 1 failed (CPR clock-seeded test, attributed, E5.4); G-3 and G-6 re-run; seeds NO_DIFF; discovered E5.4 |
| E5.3 | Drawer content from list_progression_ledger | opus | complete | E5.2 | commit ed89f74e53 + receipt commit; receipt artifacts/epic_5/E5.3_cycle_receipt.md; 11 progression_ledger tests; desktop 1089 passed, 0 failed; 184/184 frontend files; 6 ui-smoke rows green; seeds NO_DIFF; no card discovered |
| E5.4 | CPR therapy test stops failing on a high Extreme roll | sonnet | complete | E5.2 | discovered by E5.2, decisions.md §21; commit 3fe45c6f1c + receipt commit; receipt artifacts/epic_5/E5.4_cycle_receipt.md; test pinned to a 21-roll seed, RED Some(11), GREEN, 40 of 40 exact runs; figures corrected 19/74 to 21/76 |
| E5.MC | E5 merge check + Epic report | opus | complete | E5.3, E5.4 | commit 2521d50e0a + receipt commit; receipt artifacts/epic_5/E5.MC_cycle_receipt.md; two refuted and fixed (saved hit die lost by clone and export, ledger write not whole); desktop 1091 passed, 0 failed; mutations 15 of 15 red; seeds NO_DIFF; no card discovered |
| E6.1 | Overrides toggles + enforcement | opus | complete | E5.MC | commit 32ae8c1111 + receipt commit; receipt artifacts/epic_6/E6.1_cycle_receipt.md; 16 overrides_ tests + 9 override_; root 6751 passed, desktop 1116 passed, 0 failed each; 184/184 frontend files; G-3 and G-6 inside the desktop sweep; seeds NO_DIFF; no card discovered |
| E6.2 | Override tagging UI + DTO | opus | complete | E6.1 | commit 0c4d7ce308 + receipt commit; receipt artifacts/epic_6/E6.2_cycle_receipt.md; 13 admitted_tag_ tests, overrides_ still 16; desktop 1129 passed, 0 failed; 185/185 frontend files; 3 new ui-smoke rows green; seeds NO_DIFF; no card discovered |
| E6.MC | E6 merge check + Epic report | opus | complete | E6.2 | commit 356654dd52 + receipt commit; receipt artifacts/epic_6/E6.MC_cycle_receipt.md; refuted and fixed: a Market weapon read by no lookup, a race's and a deity's weapon refused, a Fighter with no Add Spell, a moved custom item untagged, a flag gone off refusing other items; root 6751 passed, desktop 1139 passed, 0 failed each; 185/185 frontend files; seeds NO_DIFF at the epic's width; no card discovered |
| E7.1 | .pcg reader + mapping + fixtures (ingest crate) | opus | complete | E1.0 | side lane ingest; receipt artifacts/epic_7/E7.1_cycle_receipt.md; the one remainder, ranks in Climb, Intimidate, Swim, closed by E7.3's commit f6ed94b1e7 |
| E7.2 | Sidecar + Import button + report | opus | complete | E6.MC | commit 7bf3f968ac + receipt commit; receipt artifacts/epic_7/E7.2_cycle_receipt.md; 8 pcg_sidecar tests through a built binary; desktop 1148+1 passed, 0 failed; 187/187 frontend files; 2 ui-smoke rows green; deb built, sidecar in usr/bin, 0 opens under the checkout; seeds NO_DIFF; no card discovered |
| E7.3 | Engine reads Climb, Intimidate, Swim at the real rank; importer writes it | opus | complete | E7.2 | commit f6ed94b1e7 + receipt commit; receipt artifacts/epic_7/E7.3_cycle_receipt.md; both twins read the rank, no rank reads untrained, Athletic and Persuasive +4 at 10; root selected_skill 28 passed, pcg_import_fixtures 11 passed, desktop 1149+1 passed, 0 failed; 187/187 frontend files; seeds NO_DIFF; no card discovered |
| E7.4 | No gear is required: a character computes with nothing worn or wielded | opus | complete | E7.3 | commit a4150fc4b1 + receipt commit; receipt artifacts/epic_7/E7.4_cycle_receipt.md; operator ruling R21; 9 no_required_gear tests, 14 loadouts through both twins; pcg_import_fixtures 11 passed; root 4 pins rewritten, their suites 22 passed; ingest 1857 and desktop 1158+1 passed, 0 failed; seeds NO_DIFF; no card discovered |
| E7.5 | Owned but not carried: Left behind on the Equipment panel | opus | complete | E7.4 | commit a44e177d18 + receipt commit; receipt artifacts/epic_7/E7.5_cycle_receipt.md; operator ruling R21, 2026-11-04 |
| E7.6 | A race's and a deity's weapon proficiency reach the attack row | opus | complete | E7.5 | commit ac9bb659ae + receipt commit; receipt artifacts/epic_7/E7.6_cycle_receipt.md; operator ruling R22, 2026-11-04 |
| E7.MC | E7 merge check + Epic report | opus | complete | E7.6 | commits c8ba870af8, e65cef12fd + receipt commit; receipt artifacts/epic_7/E7.MC_cycle_receipt.md; 6 defects fixed, 1 card discovered (E7.7), E7 commands green; root 6,771 passed, 0 failed, 27 ignored, 301 result lines, exit 0, at c8ba870af8, desktop 1,180 passed, 0 failed, 2 ignored, 2 result lines, exit 0, at e65cef12fd's tree (E7.6's 1,172 plus 5 and 3), ingest 1,857 passed, 1 failed, 43 ignored, 178 result lines, exit 101, at c8ba870af8. The one, `v06_wizard_pilot_case_verification::full_pipeline_runs_end_to_end_for_the_wizard_pilot_case`, failed on PCGen's gradle download during the internet outage (`Unable to process url: https://api.adoptium.net/...`); after it, `cargo test --locked -p codex-ingest -j 8 --test v06_wizard_pilot_case_verification` 3 passed, 0 failed; seeds: one text path on each PF seed, SF and CPR none |
| E7.7 | A weapon stating no damage dice has an attack row | opus | ready | E7.MC | discovered by E7.MC (`decisions.md §21`); `Net (Base)` is held and reads no attack row |
| E8.1 | Print core: Typst, fonts, frame, PF layout, Save PDF cmd | opus | complete | E7.MC | commits f8c29c3eeb, 784ae6fde5, dd4006a843 + receipt commit; receipt artifacts/epic_8/E8.1_cycle_receipt.md; typst and typst-pdf 0.15.1; Liberation Sans 2.1.5 (OFL) compiled in; save_sheet_pdf and print_sheet_pdf; print_smoke 6 passed; desktop 1,186 passed, 0 failed; sd24 audit 5 passed after a named lockfile exception; Aldric A4 and Letter byte-equal across runs; seeds NO_DIFF; discovered E8.6 (Hit Points and Speed have no single backend value) |
| E8.2 | PF blocks + three options | opus | complete | E8.1 | commit ed2000b229 + receipt commit; receipt artifacts/epic_8/E8.2_cycle_receipt.md; 15 block types, Spellbook, Show the math, Bio and campaign notes each on pages of their own; print_pf_blocks 8 passed, print_smoke 6 passed; desktop 1,194 passed, 0 failed; (no derivation recorded) Aldric 18 of 50, Elowen 17 of 57; seeds NO_DIFF; discovered E8.7 (skill totals have no single backend value) |
| E8.3 | SF + CPR layout files | opus | complete | E8.2 | commit 3fc88b76a2 + receipt commit; receipt artifacts/epic_8/E8.3_cycle_receipt.md; layout/sf1e.rs and layout/cpr.rs over the screens' boxes, Other Totals and Rules and Features; print loads through the envelope router; print_sf 4 passed, print_cpr 4 passed (4 SF seeds and 5 CPR fixtures, A4 and Letter, every engine row printed once); R14 control already present since SD-38 E4.8, incident 0 in SD-39; no card discovered |
| E8.4 | Settings paper + dialog + buttons + retire CSS print | opus | waiting | E8.3 | |
| E8.5 | Golden suite 24 PDFs | opus | waiting | E8.4 | |
| E8.6 | Hit Points and Speed: one backend value for screen and print | opus | waiting | E8.4 | discovered by E8.1 (`decisions.md §21`); the screen derives both in TypeScript, so the print cannot carry them under NR-2 |
| E8.7 | Skill totals: one backend value for screen and print | opus | waiting | E8.4 | discovered by E8.2 (`decisions.md §21`); the Skills panel sums each total in TypeScript and the engine states three, so the print prints ranks and `—` under NR-2 |
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
