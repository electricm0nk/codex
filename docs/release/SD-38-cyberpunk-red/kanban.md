---
canonical: true
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
board: local-file ./kanban.md (Hermes board retired 2026-08-01)
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a
card edits that card's row in its own closing commit (`workflow-instruction.md §6` step 9). The
orchestrator never moves a row to `complete` from memory: a row moves only together with a receipt
and its commit SHA.

**Status vocabulary:** `ready` (dependencies met) · `waiting` (dependencies unmet) · `in-progress` ·
`complete` · `partial` (blocks closure) · `blocked-escalated` (only an operator ruling clears it; the
ruling needed is under `progress.md ## Open blockers`; blocks closure).

**Column order is load-bearing:** E7.5's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the Workflow script refuses to dispatch a card whose dependencies
are not all `complete` (rule R8). Row order is dispatch order.

**Closure rule:** no PR while any row other than E7.11 is anything but `complete`. DEF-1…DEF-5 are not
cards (`decisions.md §22`).

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package + unattended-mode receipts committed | lead session | complete | — | `artifacts/cycle_0/C0_cycle_receipt.md`; package 1102f34aab, script a53be2166a, run id 36d7c51beb |
| C1 | Cut `tranche-18` after PR #398; 0.18.0 on 14 surfaces + `current-release.json`, one commit; push | haiku | complete | C0 | `artifacts/cycle_1/C1_cycle_receipt.md`; bump commit 706d8da680 (15 files); SD-k revised: tranche-18 already cut from PR #398's head bb6107aee0; E7.11 re-checks develop |
| E0.1 | `GameSystem::CyberpunkRed`; `sheet_rules_relative` → `Option` | opus | complete | C1 | `artifacts/epic_0/E0.1_cycle_receipt.md`; code 7732b17462, clippy base fix ebbb643800; PF/SF seeds unchanged |
| E0.2 | LICENSE.json + licence-matrix section (DEVIATION) | opus | complete | E0.1 | `artifacts/epic_0/E0.2_cycle_receipt.md`; 481de470b6; pi-sweep + pcgen-residue-gate PASS with the dir present; PF/SF seeds unchanged (no code touched) |
| E0.3 | CPR corpus root bundled + `packaged_resources.rs` | opus | complete | E0.2 | `artifacts/epic_0/E0.3_cycle_receipt.md`; 4edfb5e3fb; bundle.resources 14 → 15, packaged_resources RED→GREEN, tauri-resources-tracked PASS, desktop 806 passed / 0 failed; PF/SF seeds unchanged |
| E0.4 | CPR store (A-11 revised) + `CyberpunkAdapter` routing; STAT rows | opus | complete | E0.3 | `artifacts/epic_0/E0.4_cycle_receipt.md`; 734ed496a4 + a371dbed22; resolver + envelope router → `CyberpunkAdapter`, RED on the stub's Would text; CPR save→list→load, 10 `cpr.stat.*` rows; `list_all` skips CPR (0 unreadable); root 299 suites / desktop 810 passed, 0 failed; PF/SF seeds unchanged |
| E0.5 | Landing `cyberpunk-red` selectable; Load list; New Character closed | opus | complete | E0.4 | `artifacts/epic_0/E0.5_cycle_receipt.md`; 2db271c281; chip `cyberpunk-red` available (selectable 2 → 3 of 6), Load list via `CyberpunkAdapter`, creation gate closed with its hint; ui-smoke `select-cyberpunk-red` RED on base (Aldric listed) → green, 6/6 with neighbours; spec 104 → 105 rows, inventory regenerated; npm test 164/164; PF/SF seeds unchanged |
| E0.6 | PF + SF byte-identical gate (base = C1 SHA) | opus | complete | E0.1, E0.2, E0.3, E0.4, E0.5 | `artifacts/epic_0/E0.6_cycle_receipt.md`; evidence 666edd9f47; base 706d8da680 (C1, own worktree) vs head 2ecbf5e9a8: PF Aldric/Elowen/Tomas sha256 diff empty; SF 4 seeds' lines diff -r empty; root `--lib sf_seed` 34 passed both; PF + SF converter `--check` PASS both with identical counts |
| E0.MC | E0 adversarial merge check | opus | complete | E0.6 | `artifacts/epic_0/E0.MC_cycle_receipt.md`; evidence f953bca5ce; on origin/tranche-18 5dc2d53e59: E0.1–E0.6 commands 25/25 as expected (PF 3 + SF 4 seeds equal C1 base); mutations 3/3 red for the intended reason and restored; E0 Epic report in `progress.md` |
| E1.0 | MANIFEST.json + `cpr_manifest_check.py` | sonnet | complete | E0.MC | `artifacts/epic_1/E1.0_cycle_receipt.md`; 53 kinds, expected sum 800 (= CUI §3 column sum, awk + Python), 3 null (E1.8); `--selftest` 10 plants; `--all` exits 1 with 53 MISSING + 3 NULL_COUNT (no kind files yet) |
| E1.1 | Roles lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.1_cycle_receipt.md`; role 10 / role_ability 10; checker exit 0; 18 nested counts equal by two implementations (`E1.1_nested_check.py`); image-checked p141, p149, p150, p152, p155 (U-28), p159–p160 (D-8), p164–p166 (U-29); D-4 29/29 kept |
| E1.2 | Creation lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.2_cycle_receipt.md`; stat 10 / streetrat_template 100 / role_skill_list 10 / complete_package_points 5 / lifepath_table 67 (15+52) / starting_kit 10 / ip_cost 3 / ip_award 8 / reputation_level 10; checker exit 0; 10 role lists = 86 (python, jq, sr.py); F-28 186 and F-29 336 re-derived; U-30, U-37 (p412), D-6, D-9 recorded |
| E1.3 | Skills lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.3_cycle_receipt.md`; skill 66 / difficulty_value 7 / skill_modifier 10; checker exit 0; skills.tsv diff empty; DV table and modifier table image-checked (p130, p131) |
| E1.4 | Gear lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.4_cycle_receipt.md`; 15 kinds / 213 records (8+29+13+8+10+3+9+54+10+5+33+4+11+14+2, python + awk); manifest check `--kinds` RED 15 MISSING → 14 of 15 kinds pass, `COUNT gear: 54 records, expected 53` is the SD-c correction (Radiation Suit: starter table p102 has 48 rows, CUI awk missed Road Flare `10b`; retro correction) that only E1.V may write to MANIFEST (gear 54, in_starter_table 48; exit 0 with it applied in a scratch copy); D-3, D-10 (3 records), A-6 (5), A-8 recorded; `E1.4_checks.py` ALL PASS (847 of 848 sentences verbatim); p096, p102, p339, p340, p351, p357, p379 image-checked |
| E1.5 | Cyberware lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.5_cycle_receipt.md`; cyberware 96 (F-63 8 categories, F-64 HL 12/22/25/26/11, each by 2-3 implementations), therapy 3, cyberpsychosis_band 5, humanity_trauma 4; manifest check `--kinds` exit 0 (RED exit 1, 4 MISSING); p231 image check recorded (Standard/Extreme rows carry the max-Humanity sentences, D-5 precondition met) |
| E1.6 | Combat lane (D-1 image check) | sonnet | complete | E1.0 | `artifacts/epic_1/E1.6_cycle_receipt.md`; 10 kinds / 95 records (18+10+8+9+6+22+4+9+7+2, awk + Python); p174 image: 8 rows, no weapon-type mapping, D-1 stands; F-70 18, F-74 9, F-79 7 each re-derived twice; CUI F-71 N/A 11 is 8 (retro correction); `E1.6_checks.py` ALL PASS |
| E1.7 | Netrunning lane | sonnet | complete | E1.0 | `artifacts/epic_1/E1.7_cycle_receipt.md`; 7 kinds, 59 records (9+4+27+3+3+8+5), `cpr_manifest_check.py --kinds <7>` RED exit 1 (7 MISSING) → GREEN exit 0; F-84 4/3/8/12 = 27 (awk + Python); 18 defenses image-checked p214-p217 |
| E1.8 | GM-tables lane (inventory first) | sonnet | complete | E1.0 | `artifacts/epic_1/E1.8_cycle_receipt.md`; mooks_grunts 10 / encounter 55 (18+20+17) / beat_chart 64 (10+13+29+3+9), each by two implementations (CUI F-89..F-91); attempt 2 added verbatim `text` to all 119 encounter + beat_chart records (55 + 64, two implementations); checker `--kinds` exit 0 |
| E1.V | Verify lane: 10% sample vs page image; MANIFEST pinned; Rust twin | opus | complete | E1.1, E1.2, E1.3, E1.4, E1.5, E1.6, E1.7, E1.8 | `artifacts/epic_1/E1.V_cycle_receipt.md`; MANIFEST 53 kinds / 930 / 0 null (python, awk over CUI §3, Rust twin); gear 54/48 (E1.4 SD-c) and mooks_grunts 10 / encounter 55 / beat_chart 64 pinned, CUI §3 rows matched; 112 sampled records (seed 38, ceil 10% per kind, two checks) → 108 agree, 4 fixed, 0 disagree; 22 corpus strings + 2 `net_architecture` texts fixed from page images (`E1.V_fixes.py`); `tests/cpr_corpus_manifest.rs` 11/11 (same 10 plants as E1.0); clippy PASS; root 300 suites 6583 passed 0 failed; PNG/PDF in git 381 = C1; E1 epic report written |
| E2.1 | Lane A: stats (HP table 126 cells) | opus | complete | E1.2, E1.V | `artifacts/epic_2/E2.1_cycle_receipt.md`; `cpr::stats` 5/5; p080 HP table 126/126 (Rust test + Python, 9 x 14 cells by awk on p080.txt); floor-for-ceil plant → 63 mismatches, red; EMP 44/39/50/46/−3 = 4/3/5/4/0 (A-4); rows `cpr.derived.{hit_points,seriously_wounded,death_save,humanity_max,emp_current,move,luck_pool}`; clippy PASS; sd24 ok; root 300/6588/0, desktop 2/810/0; PF/SF unchanged |
| E2.2 | Lane A: skills (10 lists = 86) | opus | waiting | E1.3, E2.1 |  |
| E2.3 | Lane A: armour | opus | waiting | E1.4, E2.2 |  |
| E2.4 | Lane A: wounds + criticals | opus | waiting | E1.6, E2.3 |  |
| E2.5 | Lane B: cyberware (D-5, D-6, D-11) | opus | complete | E1.5, E1.V | `artifacts/epic_2/E2.5_cycle_receipt.md`; 7eca436018 + 5377876ab6 (clippy); `cpr::cyberware` 15/15 (RED: 0 tests, then E0425 compile errors); 10 Role packages = 9/14/14/12/12/10/10/12/16/14 over 18 combinations (Rust + Python); D-11 refusal per container (Fashionware/Internal/External 8th, NL 6th, eye 4th, audio 4th, arm 5th, leg 4th); D-6 EMP 5 → 4 after Media/Lawman; D-5 EMP 6 + 2 pieces + 1 borgware → 52; mutations 5/5 red; root 300 suites 6598 passed 0 failed; PF/SF seeds unchanged |
| E2.6 | Lane B: roles (rank-derived values) | opus | waiting | E1.1, E2.5 |  |
| E2.7 | Lane B: improvement (IP, A-1) | opus | waiting | E1.2, E2.6 |  |
| E2.8 | Lane B: reputation | opus | waiting | E1.2, E2.7 |  |
| E2.9 | Lane C: netrunning (A-7, D-2) | opus | complete | E1.7, E1.1, E1.V | `artifacts/epic_2/E2.9_cycle_receipt.md`; b387df53af + 8b3cea7355 (clippy); `cpr::netrunning` 11/11 (RED: 0 tests, then 10 of 11 failing on 0-valued bodies); ranks 1–10 → 2,2,2,3,3,3,4,4,4,5 (p197 image-checked); p157 sample deck 7 of 7, 8th slot refused; Cyberarm +1, Bodyweight Suit +1 for 1-slot Hardware only, both on one deck 9 of 9 (A-7); D-2 printed line; rank plant red; root 300 suites 6594 passed 0 failed; desktop 2 suites 810 passed 0 failed; PF/SF seeds unchanged |
| E2.10 | Sheet assembly + adapter; p121 check | opus | waiting | E2.4, E2.8, E2.9, E1.V |  |
| E2.MC | E2 adversarial merge check | opus | waiting | E2.10 | writes the E2 epic report |
| E6.1 | Catalog + deck-calculator commands | sonnet | complete | E1.V, E2.9 | `artifacts/epic_6/E6.1_cycle_receipt.md`; `cpr_catalog` 8/8 on 53 MANIFEST kinds (RED: E0425, no `catalog_at`/`deck_at`); p157 sample deck 7 of 7, 8th slot refused; PF 3/3 + SF 4/4 unchanged |
| E6.2 | Catalog screens + landing links | sonnet | complete | E6.1 | `artifacts/epic_6/E6.2_cycle_receipt.md`; 45 tabs = 45 non-`print_only` of 53 MANIFEST kinds, 779 records rendered; ui-smoke `browse-cyberpunk-catalog` green (2/2); frontend 166/166; PF 3/3 + SF 4/4 unchanged |
| E6.3 | Netrunning tools screen | sonnet | complete | E6.2 | `artifacts/epic_6/E6.3_cycle_receipt.md`; Deck Calculator tool tab in the Netrunning group; 3 decks / 27 programs / 6 hardware pickers; every number the E6.1 command's row; frontend 167/167; PF/SF untouched (no src/crates/src-tauri change) |
| E6.4 | GM reference pages (print-only) | sonnet | complete | E6.3, E1.8 | `artifacts/epic_6/E6.4_cycle_receipt.md`; 8 print_only kinds / 151 records (MANIFEST sum and file lengths, two implementations) each on a GM Reference page; ui-smoke `browse-cyberpunk-gm-reference` 1/1 green; frontend 168/168; PF/SF untouched (no src/crates/src-tauri change) |
| E6.MC | E6 adversarial merge check | opus | waiting | E6.3, E6.4 | writes the E6 epic report |
| E3.1 | Creation commands (3 methods, seedable roller) | opus | waiting | E2.4, E6.1 |  |
| E3.2 | `CyberpunkCreateForm`; New Character opens it | opus | waiting | E3.1, E6.MC |  |
| E3.3 | Lifepath panel; Cultural language 4 | opus | waiting | E3.2 |  |
| E3.4 | Skills allocator | opus | waiting | E3.3 |  |
| E3.5 | Role Ability rank 4 + sub-choices (D-7, A-2) | opus | waiting | E3.4, E2.6 |  |
| E3.6 | Starting kit + cash (A-9) | opus | waiting | E3.5 |  |
| E3.7 | Cyberware pack + Humanity preview + slots | opus | waiting | E3.6, E2.5 |  |
| E3.MC | E3 adversarial merge check (3 methods through the form) | opus | waiting | E3.7, E2.MC | writes the E3 epic report |
| E4.1 | Sheet core + layout registry; no Encumbrance box | opus | waiting | E2.MC, E3.2 |  |
| E4.2 | Skills block | opus | waiting | E4.1 |  |
| E4.3 | Weapons + range DV + armour block | opus | waiting | E4.2 |  |
| E4.4 | Cyberware + Role Ability + deck block | opus | waiting | E4.3 |  |
| E4.5 | Reputation, IP, Lifepath, criticals checklist | opus | waiting | E4.4 |  |
| E4.6 | Print path `cpr_sheet_print.rs` | opus | waiting | E4.5 |  |
| E4.7 | ui-smoke rows + `ui-smoke:doc` | opus | waiting | E4.6, E3.MC |  |
| E4.MC | E4 adversarial merge check | opus | waiting | E4.7 | writes the E4 epic report |
| E5.1 | IP spend (skills, Role rank) | opus | waiting | E3.MC, E4.MC |  |
| E5.2 | Role switch (A-1) | opus | waiting | E5.1 |  |
| E5.3 | Cyberware install with Humanity Loss (A-3, D-11) | opus | waiting | E5.2 |  |
| E5.4 | Therapy (D-5 cap) | opus | waiting | E5.3 |  |
| E5.5 | Reputation change | opus | waiting | E5.4 |  |
| E5.MC | E5 adversarial merge check | opus | waiting | E5.5 | writes the E5 epic report |
| E7.1 | Three CPR starter seeds restorable | opus | waiting | E5.MC |  |
| E7.2 | Packaged resources + clean-checkout build | opus | waiting | E7.1 |  |
| E7.3 | Fresh-install acceptance (.deb, AppImage) | sonnet | waiting | E7.2 | SD-m if the box cannot install |
| E7.4 | Widest verify (both workspaces, npm, `verify.sh`, PF/SF byte-identical) | sonnet | waiting | E7.3 |  |
| E7.5 | Final-acceptance scan (stop if short) | opus | waiting | E7.4 |  |
| E7.6 | Retrospective written + cited | sonnet | waiting | E7.5 |  |
| E7.7 | Worktree/branch sweep | haiku | waiting | E7.6 |  |
| E7.8 | Release notes (marker removed; resolver passes) | sonnet | waiting | E7.7 |  |
| E7.9 | Architecture truth-up + claims critic | sonnet + opus | waiting | E7.8 |  |
| E7.10 | Graphify LAST | sonnet | waiting | E7.9 |  |
| E7.11 | PR `tranche-18 → develop` (final action) + `pr-tests` | sonnet | waiting | E7.10 | operator merges |

**Row check (68 cards).**

```bash
cd docs/release/SD-38-cyberpunk-red
awk -F'|' '$2 ~ /^ (C|E)[0-9]/{n++} END{print n}' kanban.md                     # → 68
python3 -c "import re;print(sum(1 for l in open('kanban.md') if re.match(r'^\| (C|E)[0-9]', l)))"   # → 68
diff <(awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort) \
     <(awk -F'|' '/^## Epic C/{s=1} s && /^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' epic-breakdown.md | sort) \
  && echo SAME_IDS
awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort | uniq -d   # → nothing (a rebase that kept both sides duplicates rows; SD-l)
```

Run by the auditor on 2026-10-13 over the staging copy: `68`, `68`, `SAME_IDS`, no duplicate, the
dependency check below printed nothing, and a depth-first search over `Depends on` found no cycle.

Every `Depends on` ID must be a row ID (or the named external precondition on C1):

```bash
awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2); id[$2]=1; d[$2]=$6} END{for(k in d){n=split(d[k],a,/, */); for(i=1;i<=n;i++){x=a[i]; gsub(/^ +| +$/,"",x); if(x!="—" && x !~ /PR #398/ && !(x in id)) print k" -> "x}}}' kanban.md
```

Pass = no output.
