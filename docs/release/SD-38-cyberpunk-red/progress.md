---
canonical: true
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Progress

The live cycle log and the surface the operator reviews on return. Every dispatched card appends one
row to the cycle log **in its own closing commit**; every merge-check card (and E1.V) appends an
**Epic report**. Re-fetch this file before editing it (`workflow-instruction.md §5`).

## Run handle

| Field | Value |
|---|---|
| Workflow run id(s) | run 1 `wf_a9b94476-6a6` (task `wh7ev5lcg`, 2026-10-13 ~21:50 UTC, `args {DONE:[C0]}`): stopped at C1 dependency gate (C0 row still `ready`; gate correct). Run 2 `wf_c2267c33-f48` (task `wdpp3gh2w`, ~22:05 UTC, same args) after d899e53dde dispatched nothing (a note in C1's Depends-on cell parsed as a dependency; fixed 3b4b946e77, generator now rejects non-id tokens). Run 3 `wf_f42286f7-e39` (task `wyc1bpncy`, ~22:12 UTC, same args): C1 blocked-escalated on the OLD SD-k text because `.worktrees/sd38-main` (made by run 1 at a53be2166a) was never fast-forwarded; the serial-lane prompt now fast-forwards the tree before any read (generator, this commit). Run 4 `wf_9e6f5c69-d87` (task `wsjycvzfg`, ~22:20 UTC, same args): C1, E0.1, E0.2 complete (706d8da680, 7732b17462, 481de470b6); stopped at E0.4 `declined` — (a) E0.2 and E0.4 both depended only on E0.1, so the scheduler dispatched both into the one serial tree (fixed: generator's serial-lane mutex, one `main` card in flight at a time, plus E0.4 now depends on E0.3); (b) A-11's envelope-field premise refuted (no serde derive, hand-rendered store, 39 literal sites, PF-shaped payload required) — A-11 revised to CPR's own store (`decisions.md §21`), E0.4 card restated. Run 5 `wf_a66e367b-137` (task `w9jd8iezb`, args `{DONE:[C0,C1,E0.1,E0.2]}`, 2026-10-14 → 2026-10-15, 59 agents): E0.3 … E7.2 complete (54 cards, every MC green, head 43e9dcdb27); stopped at E7.3 `declined` because its re-dispatch depended on the discovered card E7.3a, which the script did not hold (a discovered card from a `partial` result never joins the scheduler; the generator's id regex also rejected a trailing letter — both fixed by regenerating from the board; E4.8, discovered by E4.MC, likewise pending). Run 6 `wf_2ff89547-b82` (task `wts874755`, args `{DONE:[…54 complete ids…]}`); script `artifacts/cycle_0/sd38-workflow.js` |
| Script path | `artifacts/cycle_0/sd38-workflow.js` (written at launch; it is also the prefix backup) |
| Pinned `tranche-18` SHA for wrong-base resets | the C1 bump commit (written by C1) |
| Oracle | none for CPR (`decisions.md §23`); PF/SF gates keep `scripts/pcgen-oracle-pin.env` |

## Summary

| Epic | Status | Cards complete | Notes |
|---|---|---|---|
| C | complete | 2 of 2 | C0 complete (1102f34aab); C1 complete (706d8da680) (kanban rows; corrected by E6.MC, the row read 1 of 2 after C1 landed) |
| E0 | complete | 7 of 7 | E0.1 complete (7732b17462, ebbb643800); E0.2 complete (481de470b6); E0.3 complete (4edfb5e3fb); E0.4 complete (734ed496a4, a371dbed22); E0.5 complete (2db271c281); E0.6 complete (666edd9f47); E0.MC complete (f953bca5ce) — Epic report below |
| E1 | complete | 10 of 10 | E1.0–E1.8 complete; E1.V complete (MANIFEST pinned 53 kinds / 930 / 0 null, 112-record image sample, Rust twin) — Epic report below |
| E2 | complete | 11 of 11 | E2.1–E2.10 complete; E2.MC complete (dabf4176f2) — Epic report below (count: `awk -F'\|' '$2 ~ /^ E2\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E6 | complete | 5 of 5 | E6.1–E6.4 complete; E6.MC complete (251eda57a1) — Epic report below (`awk -F'\|' '$2 ~ /^ E6\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E3 | complete | 8 of 8 | E3.1–E3.7 complete; E3.MC complete (4a5cb95486) — Epic report below (`awk -F'\|' '$2 ~ /^ E3\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E4 | in progress | 8 of 9 | E4.1–E4.7 complete; E4.MC complete — Epic report below; E4.8 (discovered by E4.MC) waiting (`awk -F'\|' '$2 ~ /^ E4\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E5 | complete | 6 of 6 | E5.1 complete (d1adf600d0, df7394c0a7, 0ec709c355); E5.2 complete (aa1d6b7500, 37c4d687e6); E5.3 complete (3b0592252d, 06c93e0e2a); E5.4 complete (224cdce6fc); E5.5 complete (cd9564ec44); E5.MC complete (481718c99a) — Epic report below (`awk -F'\|' '$2 ~ /^ E5\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E7 | waiting | 0 of 11 | |
| **Total** | | **57 of 69** (C0, C1, E0.1–E0.6, E0.MC, E1.0–E1.8, E1.V, E2.1–E2.10, E2.MC, E3.1–E3.7, E3.MC, E4.1–E4.7, E4.MC, E5.1–E5.5, E5.MC, E6.1–E6.4, E6.MC; 69 = 68 authored + E4.8 discovered by E4.MC; derived at E5.MC: awk and Python both 57 complete of 69 rows) | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` (unescape `\|`) |

---

## Cycle 0 — package authored; unattended mode acknowledged (2026-10-13)

**Operator on record.** Mode ruling of 2026-10-13 (from the planner's brief): travelling for a few
weeks; autonomous; report after each epic; implementation lanes as Workflow scripts. Standing directive
of 2026-08-01 (verbatim): *"include instructions to all 3 that indicate they will be running in
unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it
might be days before i notice."* This receipt is the third of the three mirrors; the other two are
`workflow-instruction.md`'s OPERATING METHOD callout (🟡 UNATTENDED MODE) and `decisions.md §16`.

- **Card:** C0. **Author:** an Opus 5.5 sub-agent of the lead session, transcribing the planner's brief
  (Fable 5.1) and seven inventories (`decisions.md §15` planning disclosure).
- **Write scope honoured:** a staging directory outside the git tree only. No code, no version bump, no
  git state change. The lead copies the package into `docs/release/SD-38-cyberpunk-red/`.
- **Inputs read:** the brief; the seven inventories; `docs/release/template/template.md`;
  `docs/governance/workflow-instruction-template.md`; every SD-37 package file;
  `.claude/skills/stc-authoring/SKILL.md`; `docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit";
  `docs/governance/blocker-closure-doctrine.md` (first 60 lines); `docs/retro/sd37-retrospective.md`;
  the repo anchors cited in `technical-design.md` and `decisions.md`.
- **Figures:** transcribed from the inventories with their commands (CUI §2); none re-derived except
  F-19 (459 page files) and the version-surface sets (`decisions.md §2`), which the author re-ran.
- **Measured corrections at authoring** (to be emitted by C1 as `retro.py correction`, because this
  package's write scope excludes `docs/retro/`):
  ```bash
  scripts/retro.py correction --actor sd38-c0 --subject 'SD-38 design brief (version-surface grep)' \
    --claimed "grep -rn '0\.17\.' --include=*.json --include=*.toml --include=*.ts --include=*.yml apps .github tools docs/architecture lists the bump set" \
    --actual 'it misses apps/desktop/src-tauri/Cargo.lock (no *.lock include) and the brief variant with *.py catches tools/release/test_resolve_release_notes.py, which needs no edit' \
    --verified-by "git show --name-only --format= 9d03a76996 | awk 'NF' | sort; grep -n '0\.17\.0' apps/desktop/src-tauri/Cargo.lock"
  scripts/retro.py correction --actor sd38-c0 --subject 'SD-38 inventories (IP cost table page)' \
    --claimed 'roles.md cites p411; creation.md cites p412 for the IP spend table' \
    --actual 'unsettled: E1.2 reads both pages and records which holds the table (CUI F-33, U-37)' \
    --verified-by 'grep -n "p41[12]" artifacts/cycle_0/inventory/roles.md artifacts/cycle_0/inventory/creation.md'
  ```
- **Placeholder gate** (`workflow-instruction.md §9`) run over the staging copy; output classified in
  the lead's report; every hit is a class (a) schema/command placeholder or a class (b) deferred value.
- **Status:** complete. Package committed 1102f34aab; script a53be2166a; run id 36d7c51beb; receipt `artifacts/cycle_0/C0_cycle_receipt.md`. First run `wf_a9b94476-6a6` stopped at C1's dependency gate (C0 was still `ready` on the board — the gate worked; R8 holds). Relaunched after this commit.

### Likely decision points and their safe defaults

| Decision point | Safe default |
|---|---|
| Page text and image disagree | The image wins; both recorded (SD-a) |
| A count differs from CUI | Two implementations; lane records it; E1.V applies it (SD-c) |
| A rule the book does not state | Named refusal; `blocked-escalated` if it blocks a DoD item (SD-d) |
| PF/SF render diff | Fix it; never re-baseline (SD-e) |
| PR #398 still open at C1 | Resume receipt; new run with `START=C1` (SD-k) |
| Rebase conflict | `kanban.md`: take upstream, re-apply only your own row, duplicate-ID check empty (SD-l); `progress.md` append-only sections and `cpr/mod.rs` anchors: keep both (SD-l, SD-p); else `blocked-escalated` |
| Fresh install impossible on the box | E7.3 `blocked-escalated` with the operator action (SD-m) |
| A usage-limit stop | Resume receipt; never downgrade (SD-i) |

---

## Cycle log

Row template (one row per card, appended in the card's closing commit):

| Date | Card | Model | Status | Commit | Receipt | Seed deltas | Notes |
|---|---|---|---|---|---|---|---|
| 2026-10-13 | C1 | haiku | complete | 706d8da680 (bump; receipt and rows in the next commit) | `artifacts/cycle_1/C1_cycle_receipt.md` | not rendered (no Rust touched; see receipt) | tranche-18 cut from #398 head; 0.18.0 on 14 surfaces + pointer; A-grep predicate corrected (Cargo.lock crate lines) |
| 2026-10-13 | E0.1 | opus | complete | 7732b17462 (code), ebbb643800 (clippy base fix); receipt and rows in the next commit | `artifacts/epic_0/E0.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (base 67c589c5f5 in its own worktree) | `sheet_rules_relative() -> Option`; converter refuses CPR by name; verify stage reads `Some(…)` arms; root 299 / desktop 2 / ingest 177 suites green; clippy was red on the base (fixed); G-1's SF test name corrected in receipt |
| 2026-10-13 | E0.2 | opus | complete | 481de470b6 (licence file + matrix section; receipt and rows in the next commit) | `artifacts/epic_0/E0.2_cycle_receipt.md` | unchanged by construction (no src/crates/src-tauri/frontend file touched) | `data/cyberpunk-red/corpus/LICENSE.json` (signed_off true, not OGL, records_processed null → MANIFEST.json); license-matrix "Cyberpunk RED (SD-38 E0.2)" section, DEVIATION row; pi-sweep + pcgen-residue-gate PASS; note for E1: PDF 458 pages, `$P/p459.txt` empty |
| 2026-10-13 | E0.3 | opus | complete | 4edfb5e3fb (config + test; receipt and rows in the next commit) | `artifacts/epic_0/E0.3_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.1 records) | `data/cyberpunk-red/corpus/` bundled at its own path (bundle.resources 14 → 15); packaged_resources asserts it via `GameSystem::CyberpunkRed.package_roots`; tauri-resources-tracked PASS; clippy 0/0; desktop full 806 passed |
| 2026-10-13 | E0.4 | opus | complete | 734ed496a4 (code), a371dbed22 (audit-token fix); receipt and rows in the next commit | `artifacts/epic_0/E0.4_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.3 records) | `src/rules_core/cpr/{mod,input,sheet,store}.rs` (A-10 anchors; A-11 revised store); `local_store` `owns_payload` + `peek_game_system`; resolver + router → `CyberpunkAdapter`; stub 0002 retired for CPR; clippy 0/0; sd24 5/5; root 299 suites 6572 passed; desktop 810 passed |
| 2026-10-13 | E0.5 | opus | complete | 2db271c281 (code + spec row + inventory; receipt and rows in the next commit) | `artifacts/epic_0/E0.5_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.4 records) | landing chip `cyberpunk-red` selectable; Load list routes `ruleSystemId: cyberpunk-red` → `CyberpunkAdapter`; New Character closed with its hint until E3.2; ui-smoke `select-cyberpunk-red` RED → green (6/6 with neighbours, real store untouched); npm typecheck + 164/164 test files; no `.rs` touched (no clippy/cargo sweep) |
| 2026-10-13 | E0.6 | opus | complete | 666edd9f47 (evidence; receipt and rows in the next commit) | `artifacts/epic_0/E0.6_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged C1 → E0 head; SF 4 seeds' lines unchanged (base 706d8da680 in its own worktree) | G-1 gate: PF hash diff empty, SF `diff -r` empty, root `--lib sf_seed` 34/34 both, PF converter 49450/49450 PASS + SF 8582/8582 PASS both; card's named SF test writes 0 files (E0.1 correction stands), lines from the writer test; no code, no clippy |
| 2026-10-14 | E0.MC | opus | complete | f953bca5ce (evidence; receipt and rows in the next commit) | `artifacts/epic_0/E0.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged vs C1; SF 4 seeds' lines unchanged vs C1 (head 5dc2d53e59 vs E0.6's base record) | E0.1–E0.6 commands re-run on origin/tranche-18: 25/25 as expected; mutations: Stub routing → E0.4 test red, CPR resource removed → packaged_resources red, PF sheet-rule string moved → tauri-resources-tracked FAIL + PF render red; all restored green; no code, no clippy; ui-smoke `.out/` not git-ignored (reported) |
| 2026-10-14 | E1.0 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.0_cycle_receipt.md` | `artifacts/epic_1/E1.0_cycle_receipt.md` | unchanged by construction (Python + JSON only) | MANIFEST 53 kinds / 800 expected / 3 null; sum check against CUI §3 (800, two implementations); selftest 10 plants |
| 2026-10-14 | E1.3 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.3_cycle_receipt.md` | `artifacts/epic_1/E1.3_cycle_receipt.md` | unchanged by construction (Python + JSON only) | skill 66 / difficulty_value 7 / skill_modifier 10; names, categories, STATs diff empty vs skills.tsv; counts by two implementations; p130 DV pairing and p131 modifiers image-checked |
| 2026-10-14 | E1.7 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.7_cycle_receipt.md` | `artifacts/epic_1/E1.7_cycle_receipt.md` | unchanged by construction (JSON only) | 7 kinds / 59 records; F-84 4+3+8+12 = 27; 18 defenses image-checked; D-2, A-7 noted |
| 2026-10-14 | E1.1 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.1_cycle_receipt.md` | `artifacts/epic_1/E1.1_cycle_receipt.md` | unchanged by construction (Python + JSON only) | role 10 / role_ability 10; 18 nested counts equal JSON vs page text; 30 Team STAT rows, 6 Backup bands, Believability equal page; U-28 (Loyalty −4 row shared), U-29, D-4, D-8 settled on the page image |
| 2026-10-14 | E1.5 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.5_cycle_receipt.md` | `artifacts/epic_1/E1.5_cycle_receipt.md` | unchanged by construction (Python + JSON only) | cyberware 96 / therapy 3 / cyberpsychosis_band 5 / humanity_trauma 4; F-63 + F-64 re-derived by 2-3 implementations; p231 image check (D-5) recorded |
| 2026-10-14 | E1.8 | sonnet | partial | see `git log -1 --format=%h -- artifacts/epic_1/E1.8_cycle_receipt.md` | `artifacts/epic_1/E1.8_cycle_receipt.md` | unchanged by construction (JSON and docs only) | mooks_grunts 10 / encounter 55 / beat_chart 64, two implementations each (CUI F-89..F-91); verbatim prose for encounter + beat_chart not transcribed (119 records), needs an operator ruling |
| 2026-10-14 | E1.2 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.2_cycle_receipt.md` | `artifacts/epic_1/E1.2_cycle_receipt.md` | unchanged by construction (Python + JSON only) | 223 records over 9 kinds; 10 role skill lists = 86 by 3 implementations; F-28 (186 answer cells; 146 roll rows) and F-29 (336) re-derived; IP cost table is on p412 (U-37); images read p055 p057 p059 p061 p067 p069 p073 p075 p087 p088 p099 p104 p118 p411 p412 |
| 2026-10-14 | E1.6 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.6_cycle_receipt.md` | `artifacts/epic_1/E1.6_cycle_receipt.md` | unchanged by construction (JSON, Python and Markdown only) | 10 combat kinds / 95 records; D-1 image check of p174 (8 rows, no mapping printed); U-31 resolved by image (Treatment cell, not Quick Fix, carries "Quick Fix removes Injury Effect permanently"); CUI F-71 says 11 N/A, page and image show 8 (retro correction, CUI not in this card's scope: fix in the closure docs pass); weapon.json was not on the tree, so the 11-types check ran against p172 and E1.V re-runs it |
| 2026-10-14 | E1.4 | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.4_cycle_receipt.md` | `artifacts/epic_1/E1.4_cycle_receipt.md` | unchanged by construction (JSON, Python and Markdown only) | 15 gear kinds / 213 records; CUI F-53/F-54 corrected by two implementations: starter table 48 rows (awk `[0-9]e?b ?\(`; the CUI predicate gives 47 because Road Flare prints `10b (Cheap)`), gear 54 = 53 Master Gear List + Radiation Suit (p102, p340; no Master row) (retro correction); MANIFEST `gear.expected` 53 → 54 and `nested.in_starter_table` 47 → 48 plus CUI §3/F-53/F-54 are E1.V's to apply (SD-c, one writer), so the card's literal `--kinds` command exits 1 on the real manifest with exactly `COUNT gear: 54 records, expected 53` and exits 0 with the correction applied in a scratch copy; D-3 (Air Pistol base_weapon Very Heavy Pistol, Medium Pistol kept), D-10 (Road Flare, High Fashion Jewelry, Luxury Penthouse), A-6 (5 of 8), A-8; E1.6 deferred cross-check now passes (11 ranged types each in one single-shot range_dv row) |
| 2026-10-14 | E1.8 (attempt 2) | sonnet | complete | see `git log -1 --format=%h -- artifacts/epic_1/E1.8_cycle_receipt.md` | `artifacts/epic_1/E1.8_cycle_receipt.md` | unchanged by construction (JSON and docs only) | verbatim `text` on encounter 55 + beat_chart 64 = 119 records (two implementations: `awk '/"text"/{n++}END{print n}'` over both files and Python per-record count); mooks_grunts stays 10 complete; supersedes the attempt-1 partial row |
| 2026-10-14 | E1.V | opus | complete | b6dd7b8fa4 (data, MANIFEST, CUI, twin), f2b97084f7 (clippy fix); receipt and rows in the next commit | `artifacts/epic_1/E1.V_cycle_receipt.md` | unchanged by construction (no src/crates/src-tauri/frontend file; `git diff --name-only dc28ad5a5e..HEAD -- src crates apps` empty) | `--all` exit 0; twin 11/11; sample 112 over 53 kinds → 108 agree / 4 fixed / 0 disagree; 22 strings + 2 texts fixed; CUI §3 gear + E1.8 rows edited (named outside-row file); clippy FAIL→PASS (collapsible_if in the new test); root 300 suites 6583 passed; 2 corrections + 1 resolution |
| 2026-10-14 | E2.5 | opus | complete | 7eca436018 (engine + fixture), 5377876ab6 (clippy fix); receipt and rows in the next commit | `artifacts/epic_2/E2.5_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr/cyberware.rs`: HL preset sum, D-11 slot refusals (8 containers, A-5 extra eyes/arms), SP 7/11 highest-wins, BODY Grafted +2 (≤10) / Sigma 12 / Beta 14 with prerequisites, Kerenzikov +2, D-5 cap; `cpr.cyberware.*` rows; F-66 parity fixture from inventory §4, p118 image-checked; D-6 grep in decisions §20.6 prints 1 (a transcription_note), retro correction |
| 2026-10-14 | E2.9 | opus | complete | b387df53af (engine + tests), 8b3cea7355 (clippy fix); receipt and rows in the next commit | `artifacts/epic_2/E2.9_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr::netrunning` 11/11; rows `cpr.netrunning.{net_actions,deck_slots_available,deck_slots_used}` + D-2 printed line; A-7 both extra slots, over-capacity refused; clippy PASS after one collapsible_if fix; sd24 5/5; root 300/6594/0, desktop 2/810/0 |
| 2026-10-14 | E2.1 | opus | complete | 8dcf4cbb6d (engine + fixture); receipt and rows in the next commit | `artifacts/epic_2/E2.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 head records) | `cpr/stats.rs`: HP 10 + 5 x ceil((BODY+WILL)/2), Seriously Wounded ceil(HP/2), Death Save = BODY, Humanity = EMP x 10, current EMP max(0, floor(HUM/10)) (A-4), MOVE, LUCK pool; `cpr.derived.*` rows; p080 HP table fixture from the page (image-checked) 126/126 by two implementations; floor plant red (63 cells); clippy PASS; sd24 ok; root 300/6588/0, desktop 2/810/0 |
| 2026-10-14 | E6.1 | sonnet | complete | 8e65a98089 (code); receipt and rows in the next commit | `artifacts/epic_6/E6.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr_catalog` 8/8: 53 MANIFEST kinds = expected counts; p157 deck 7 of 7, 8th refused; desktop 2 suites / 818 passed / 0 failed; frontend 165/165 |
| 2026-10-14 | E6.2 | sonnet | complete | 61aa6d42cc (code); receipt and rows in the next commit | `artifacts/epic_6/E6.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | 45 tabs = 45 non-print_only of 53 MANIFEST kinds, 779 records; ui-smoke browse-cyberpunk-catalog 2/2 green; frontend 166/166; 106 ui-smoke rows |
| 2026-10-14 | E6.3 | sonnet | complete | 7d59a9a0b9 (code); receipt and rows in the next commit | `artifacts/epic_6/E6.3_cycle_receipt.md` | PF/SF not re-rendered: no src/crates/src-tauri change (unchanged by construction) | Deck Calculator tool tab; pickers 3/27/6; frontend 167/167; 106 ui-smoke rows (no new row; spec.json is E6.4's) |
| 2026-10-14 | E6.4 | sonnet | complete | e8063e2b4a (code); receipt and rows in the next commit | `artifacts/epic_6/E6.4_cycle_receipt.md` | PF/SF not re-rendered: no src/crates/src-tauri change (unchanged by construction) | GM Reference group: 8 print_only kinds, 151 records, every printed value on the page; ui-smoke browse-cyberpunk-gm-reference green (2/2 with catalog); frontend 168/168; 107 ui-smoke rows |
| 2026-10-14 | E6.MC | opus | complete | 251eda57a1 (evidence); receipt and rows in the next commit | `artifacts/epic_6/E6.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (head 2df632993b vs E0.6 base records) | E6.1–E6.4 re-run 9/9 as expected (cpr_catalog 8/8, npm 168/168, ui-smoke 3/3, inventory drift 0); 4 plants red then restored (program −1 → count test; demon −1 → GM test; Standard slot +1 engine-only → corpus_mismatch; engine+corpus → (7,8)≠(7,7)); no code, no clippy; moved E6.1–E6.4 rows from after "For the operator" into this log |
| 2026-10-14 | E2.6 | opus | complete | 84f7b89d49 (engine + fixture); receipt and rows in the next commit | `artifacts/epic_2/E2.6_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr/roles.rs`: 26 `cpr.role.` row ids over 10 Roles (rank + 16 rank-derived fields), ranks outside 1–10 refused; NET Actions twin with `cpr.netrunning.net_actions` flagged for E2.10 |
| 2026-10-14 | E2.2 | opus | complete | 560bbd299a (engine + fixture), 5fb6e7c16b (clippy fix); receipt and rows in the next commit | `artifacts/epic_2/E2.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 head records) | `cpr/skills.rs`: Skill Base = STAT + level for 66 skills (`cpr.skill.<skill>[.<specialty>]`), creation spend for Streetrat / Edgerunner / Complete Package (86, (x2) x 2, Basic >= 2, cap 6, Edgerunner floor 2, Cultural Origin language 4 free), 10 of 10 Role lists = 86; F-25 column caveat cleared by the p087 image + p088 text; 1 rework (clippy); after the §5 rebase onto E2.6/E6.x, `cargo test --locked -j 8 --lib cpr:: -- --test-threads=8` → 53 passed, 0 failed |
| 2026-10-14 | E2.7 | opus | complete | 40d3104bb5 (engine); receipt and rows in the next commit | `artifacts/epic_2/E2.7_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr/improvement.rs`: `cpr.improvement.{typical_skill,difficult_skill,role_ability_rank}.cost` and `cpr.improvement.new_role.cost` rows; `CprRoleHistory` (Roles in order taken) for multiclassing; Skill → table mapping is E2.2's `x2` flag, read by E5.2 |
| 2026-10-14 | E2.3 | opus | complete | aa084de37a (engine + tests), e9d3f45d05 (clippy fix); receipt and rows in the next commit | `artifacts/epic_2/E2.3_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 head records) | `cpr/armor.rs`: SP per location = highest source (worn armour and other SP sources such as cyberware), worst Armor Penalty once to REF/DEX/MOVE (Minimum 0), Bodyweight Suit SP 11 on both locations (one only), shield refused as worn; `cpr.armor.*` 6 rows; 2 plants red; 1 rework (clippy); after the §5 rebase onto E2.7, `cargo test --locked -j 8 --lib cpr:: -- --test-threads=8` → 67 passed, 0 failed |
| 2026-10-14 | E2.8 | opus | complete | 2c61942c2d (engine); receipt and rows in the next commit | `artifacts/epic_2/E2.8_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr/reputation.rs`: `cpr.reputation.level` (0–10, Who Knows About You text from the corpus) and `cpr.reputation.facedown` (COOL ± Rep, printed "<n> + 1d10"); `CprReputation::record` keeps the defining event unless the new Level is higher; 1 rework (SF lines out dir missing); lane B done: `sd38-b` tree and its `CARGO_TARGET_DIR` removed after the push; after the §5 rebase onto E2.3, `cargo test --locked -j 8 --lib cpr:: -- --test-threads=8` → 74 passed, 0 failed |
| 2026-10-14 | E2.4 | opus | complete | a180fb2f43 (engine + tests), f1ef20be3a (clippy fix), cda6994c46 (printed page citations); receipt and rows in the next commit | `artifacts/epic_2/E2.4_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 head records) | `cpr/critical.rs`: `cpr.wound.{lightly,seriously,mortally}_wounded` (+ `_dv` 10/13/15), `cpr.wound.death_save`, `cpr.wound.base_death_save_penalty`; HP 40 → 40/20/1; 11 of 22 injuries raise the Base DSP (3 implementations); root 300/6650/0, desktop 2/818/0; lane A ends, tree removed |
| 2026-10-14 | E3.1 | opus | complete | fabfa38ac0 (code); receipt and rows in the next commit | `artifacts/epic_3/E3.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `preview_cyberpunk_character` / `create_cyberpunk_character`: Streetrat row rolled or picked, Edgerunner 1d10 per STAT from its own column, Complete Package = corpus default 62, STATs 2–8; `CprRoller` (SplitMix64, u32 seed returned and replayed); rows = engine `cpr.stat.*` + `cpr.derived.*`; boundary `cprCreation.ts` |
| 2026-10-14 | E3.2 | opus | complete | 2c4a65ecb5 (code); receipt and rows in the next commit | `artifacts/epic_3/E3.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | New Character under Cyberpunk Red opens `CyberpunkCreateForm` (`form: 'cyberpunk'`): Role, Streetrat/Edgerunner/Complete Package, STAT + derived tiles from `preview_cyberpunk_character`; saves via `create_cyberpunk_character` with the preview's seed; `cyberpunkCreateLayout.ts` registry; ui-smoke row `create-cyberpunk-streetrat-stats` |
| 2026-10-14 | E2.10 | opus | complete | 7f20a4994c (engine, store, adapter, fixture, tests; rebased from 4cb1b866a2); receipt and rows in the next commit | `artifacts/epic_2/E2.10_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 head records) | `cpr/sheet.rs` `compose_sheet` over every engine module; `CprSheetChoices` + `cpr_sheet_choices.txt` (E0 fence exit: `input.rs`, `store.rs`); `CyberpunkAdapter::load_saved_character` returns the composed rows + printed lines; p121 labels-only branch, 29 fields + 66 skills have rows; root 300/6669/0, desktop 2/820/0; post-rebase cpr:: 95/95, desktop cpr_ 24/24 |
| 2026-10-14 | E2.MC | opus | complete | dabf4176f2 (evidence); receipt and rows in the next commit | `artifacts/epic_2/E2.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | E2.1–E2.10 re-run green on fb6582b96e (cpr:: 95/95, cpr_adapter 5/5); 7 plants red for the intended reason then restored; converters PASS; E2 Epic report |
| 2026-10-14 | E4.1 | opus | complete | 8e91816351 (code + fixtures); receipt and rows in the next commit | `artifacts/epic_4/E4.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `CyberpunkCharacterSheet` + `cyberpunkSheetLayout.ts` + `cyberpunkSheetModel.ts`; `CharacterHubPage` routes `cyberpunk-red` to it; every number one `cpr.` row (368 rows over 2 adapter fixtures; +1000 shift); no encumbrance box (D-12); `cpr_adapter.rs` test-only fence exit (fixture pin), cpr_adapter tests 6/6 |
| 2026-10-14 | E4.2 | opus | complete | 160cc5a2f2 (code + test); receipt and rows in the next commit | `artifacts/epic_4/E4.2_cycle_receipt.md` | PF/SF unchanged: diff is 4 CPR-only frontend files, no engine/adapter/shared file | Skills box: 66 skills under the 9 catalog categories (`list_cyberpunk_catalog('skill')`), Level + Base each one `cpr.` row, REF/DEX bases armour-penalised by the engine; 132 skill rows checked; ungrouped fallback while the catalog loads |
| 2026-10-14 | E3.3 | opus | complete | 55e63c7f21 (code + tests), da2b4bccfe (audit fix); receipt and rows in the next commit | `artifacts/epic_3/E3.3_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | Lifepath panel: 15 general + the Role's tables rolled (per-table seeded rollers) or picked, counts, per-column, ranges, D-9, branch skips; answers saved in `cpr_lifepath.txt` (E0 fence exit); Cultural Origin language → `Language (<name>)` at 4, engine rows `cpr.skill_level/skill.language.*`; root 6672/0, desktop 837/0, npm 170/170 |
| 2026-10-14 | E3.4 | opus | complete | 9070539fe2 (code + tests); receipt and rows in the next commit | `artifacts/epic_3/E3.4_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | Skills allocator: Streetrat fixed list, Edgerunner 2–6 over the Role's 20, Complete Package any of 66 from the Basic 13 at 2; engine-validated (refusals as problems), spent/left = `cpr.skills.creation_points_*` rows; full skill list saved; Cultural Origin tile reads its own row; desktop 841/0 + 1/0, npm 170/170 |
| 2026-10-14 | E4.3 | opus | complete | eda0a9f5e4 (engine + fixture + sheet + test), bb43d57f4e (clippy fix); receipt and rows in the next commit | `artifacts/epic_4/E4.3_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Weapons, range DV and armour blocks; engine rows for Hands, Skill Base, D-1 range DV, Autofire DV and range bands through the E2 fence exit; 13 weapons, 111 DV cells checked over 3 adapter fixtures |
| 2026-10-14 | E4.4 | opus | complete | f49905dce9 (engine + fixture + sheet + test; pre-rebase SHA); receipt and rows in the next commit | `artifacts/epic_4/E4.4_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Cyberware (piece HL rows + names, totals, slots used / total per container), Role Ability and Netrunner deck blocks; piece and deck-item names through the E2 fence exit; 4th adapter fixture (Decked Netrunner); 5 pieces, 14 containers, 6 Role rows, 8 deck lines checked |
| 2026-10-14 | E3.5 | opus | complete | 0b16bb047e (engine + desktop + frontend + tests), bd23f19245 (clippy fix); receipt and rows in the next commit | `artifacts/epic_3/E3.5_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | Role Ability at Rank 4 saved with the character; sub-choices listed and checked by the engine (`roles.rs` `choice_menu`/`choice_rows`, E2 fence exit) and saved in `cpr_role_choices.txt` (E0 fence exit, E3.3 precedent): Solo Combat Awareness, Tech Maker (D-7), Medtech Medicine + drugs (D-7), Nomad Motorpool, Exec Team Member (A-2); one valid + one refused per Role; root 6681/0, desktop 847/0, npm 170/170 |
| 2026-10-14 | E4.5 | opus | complete | 158c8a7e52 (engine + adapter + command + sheet + tests; pre-rebase SHA); receipt and rows in the next commit | `artifacts/epic_4/E4.5_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Reputation (Level, Facedown, resolved roll, events), IP current / total, Lifepath answers and a 22-entry critical-injuries checklist; ticking saves via `save_cyberpunk_criticals` and the Base Death Save Penalty row moves; E2 fence exit on `sheet.rs` (printed lines only) |
| 2026-10-14 | E3.6 | opus | complete | e218733fb0 (engine + store + desktop + frontend + tests), f5a37b232a (clippy fix); receipt and rows in the next commit | `artifacts/epic_3/E3.6_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Starting kit by method in the engine (`kit.rs`, E0 fence exit): fixed kit = `starting_kit` record with "or" picks + 500eb; Complete Package bought with 2,550eb + 800eb fashion-only, lost after save; money rows `cpr.kit.*` + `cpr.cash` (A-9); kit saved in `cpr_kit.txt` (E3.3 precedent), cash and weapons/armour/shield/ammo/deck as sheet choices; root 6694/0, desktop 852/0, npm 170/170 |
| 2026-10-14 | E4.6 | opus | complete | 37483e8015 (print module + adapter + mod line + sheet + model + tests + fixtures; pre-push SHA); receipt and rows in the next commit | `artifacts/epic_4/E4.6_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | `cpr_sheet_print.rs` prints the Role Ability text, cyberware effects, the D-6 Humanity Loss line, weapon specials and the D-2 brain-damage line with resolved figures from `cpr.` rows; the CPR sheet renders every unplaced line under Rules and features; 99 printed lines / 18 rule lines on 4 fixtures, each once; `grep -c '(-2 EMP)'` 0 |
| 2026-10-15 | E3.7 | opus | complete | 9d58fcf55b (engine + desktop + frontend + tests; pre-rebase 9418fb634c); receipt and rows in the next commit | `artifacts/epic_3/E3.7_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Cyberware at creation: the Role pack (kit `cyberware` lines, "or" picks as kit picks) or Complete Package pieces from the cash (Fashionware from the fashion budget), names resolved and options placed by the engine (`cyberware.rs` `record_name`/`place`), D-11 refusal `cpr_creation.cyberware`, Humanity/EMP/cap/slot rows from the composed sheet; 10 of 10 packs = F-66 over 18 combinations; fence exits `cprCreation.ts`, `kit.rs`, `cyberware.rs` (retro correction, fifth recurrence) |
| 2026-10-15 | E3.MC | opus | complete | 4a5cb95486 (ui-smoke rows + evidence); receipt and rows in the next commit | `artifacts/epic_3/E3.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Streetrat (Media row 4), Edgerunner (Netrunner) and Complete Package (Fixer, 62, Heavy Pistol + Neural Link) built through the live form, saved, loaded, sheet opened; every number on each loaded sheet = `compose_sheet` of the captured saved files (computed rows 114/115/110 all on screen, live digit lines 130/131/120 all from it, cells = row 212/214/208); refusal row for 63; 4 plants red; no `.rs` in diff; lane e3 ends, tree removed |
| 2026-10-15 | E4.7 | opus | complete | a42c8d0723 (ui-smoke rows + inventory + evidence); receipt and rows in the next commit | `artifacts/epic_4/E4.7_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Three ui-smoke rows (Streetrat Solo row 1, Edgerunner Netrunner, Complete Package Fixer with bought gear) open the loaded CPR sheet and assert the E4 blocks (46/27/31 expects, each re-derived by `E4.7/crosscheck.py` from book formulas + corpus); RED no such row ids → `--from select-cyberpunk-red` 14/14 green; plant range DV → red; inventory 115 rows; full 115-row suite red on 9 pre-SD-38 PF rows (13c387b7ef, afff7a621f), stopped at 40 rows, attributed |
| 2026-10-15 | E4.MC | opus | complete | this commit (docs only: evidence, receipt, board rows, discovered E4.8) | `artifacts/epic_4/E4.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | E4.1–E4.7 re-run green on 3a24b12915 (npm 171/171; cpr_adapter 8/8; cpr_sheet_print 4/4, 99 lines, `(-2 EMP)` 0; ui-smoke 4/4, real store untouched; inventory 115 rows R12 clean); 7 plants red then restored; finding: Other totals on 6 of 6 live sheets (11 raw-id rows), fixtures carry 0 → E4.8 discovered (lane main, dep E4.MC), card count 68 → 69; E4 Epic report |
| 2026-10-15 | E5.1 | opus | complete | d1adf600d0 (command + dialog + tests + ui-smoke row), df7394c0a7 (audit fix), 0ec709c355 (clippy fix); receipt and rows in the next commit | `artifacts/epic_5/E5.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | IP spend: `cpr_improve.rs` prices a plan (IP entered + one-step Skill Level / Role Rank purchases) through `improvement.rs` on the p. 411 tables, refuses skipped Levels and overspend with the engine's reason, writes nothing on refusal, saves through `CprCharacterStore` and answers with the reloaded sheet; the sheet's Improve button opens `CyberpunkImproveDialog`, every figure a `cpr.improve.*` row; `--bins cpr_improve` 7/7; ui-smoke `improve-cyberpunk-skill` green (Handgun 6→7 = 140, IP 60 / 200 after reload); npm 173/173; desktop 868/0 failed; clippy PASS; audits OK after 2 self-healed reworks |
| 2026-10-15 | E5.2 | opus | complete | aa1d6b7500 (role history + roleSwitch + print lines + dialog + tests), 37c4d687e6 (clippy fix); receipt and rows in the next commit | `artifacts/epic_5/E5.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Role switch (A-1): Rank 1 of a new Role for 60 IP only while the latest Role is Rank ≥ 4 (p. 143); `cpr_sheet_choices.txt` gains `role.earlier = <Role> <rank>` (old files parse unchanged); the sheet shows every held Role's rank rows and Role Ability line (E2 + E4 fence exits, gates re-run); `--bins cpr_improve` 11/11 (rank 3 refused, nothing written; rank 4 → Solo 4 + Nomad 1); npm 173/173; root 300 suites / desktop 872 passed, 0 failed; clippy PASS after 1 self-healed rework; audits OK |
| 2026-10-15 | E5.3 | opus | complete | 3b0592252d (dice reader + install loss + command + dialog + tests), 06c93e0e2a (sheet fixtures regenerated); receipt and rows in the next commit | `artifacts/epic_5/E5.3_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Cyberware install after creation (A-3, D-11): entered Humanity Loss or one rolled on the piece's dice with the pinned seed, refused outside the dice range; over-slot install refused, nothing written; Humanity/EMP rows move (Gills 12: 80 → 68, EMP 8 → 6); `--bins cpr_improve` 16/16; npm 173/173; root 300 suites 6704 / desktop 877 passed, 0 failed (first desktop sweep 876/1 on stale sheet fixtures, regenerated: 1 rework); clippy PASS; audits OK |
| 2026-10-15 | E5.4 | opus | complete | 224cdce6fc (therapy catalog + `humanity.regained` + sheet + `therapy` purchase + dialog + tests); receipt and rows in the next commit | `artifacts/epic_5/E5.4_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Therapy (D-5): Standard 2d6 / Extreme 4d6 regain entered or rolled with the pinned seed, refused outside the dice, stopped at maximum Humanity (68 + 20 → 78, regains 10; at the cap 0); Addiction printed, refused as a purchase; `--bins cpr_improve` 22/22; cap mutation red; npm 173/173; root 300 suites 6706 / desktop 883 passed, 0 failed; clippy PASS; audits OK |
| 2026-10-15 | E5.5 | opus | complete | cd9564ec44 (`reputation` purchase + preview rows + dialog section + tests); receipt and rows in the next commit | `artifacts/epic_5/E5.5_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | Reputation change (p. 193-194): Level 1-10 entered with deed/cowardice and the event, recorded by the engine's `record` (only a higher Level replaces; a higher cowardice event replaces a deed and the Facedown goes negative: COOL 4, cowardice 6 → −2); every event saved; `--bins cpr_improve` 27/27; 2 mutations red; npm 173/173; desktop 888 passed, 0 failed (no root file changed); clippy PASS; sd24 5/5; audits OK |
| 2026-10-15 | E5.MC | opus | complete | 481718c99a (rows + inventory + evidence); receipt and rows in the next commit | `artifacts/epic_5/E5.MC_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63) | E5.1–E5.5 re-run on 9646a423ce: `--bins cpr_improve` 27/27; 6 new ui-smoke rows (E5.2 switch + lock, E5.3 install + A-3 refusal, E5.4 D-5 cap + range refusal, E5.5 cowardice and lower deed, E5.1 skip refusal) + improve-cyberpunk-skill 7/7 green, real store untouched; plants reverted: IP ×15/10 → 8 red + app row red, switch at Rank 3 → 2 red, gate removed → 3 red + app row red; npm 173/173; inventory 122 rows R12 clean; E5 Epic report |
| 2026-10-15 | E7.1 | opus | complete | 49887659f2 (starter build enum + CPR starter requests + tests + ui-smoke rows + inventory + builds + cross-check); receipt and rows in the next commit | `artifacts/epic_7/E7.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63); CPR starters new (loaded.json sha256 in `E7.1/seeds.txt`) | Three CPR starters seeded on launch and by Restore starter characters through `cpr_creation::create_at_root` (`cpr_adapter::starter_requests`): Jax Moreno (Streetrat Solo row 3), Nyx Calder (Complete Package Netrunner, 62 STAT / 86 skill points, 2,090 of 2,550eb), Imani Okafor (Edgerunner Medtech, seed 2045); `--bins starter_seed` 23/23 (second Restore writes nothing, byte-identical); `crosscheck.py` re-derives every sheet number (231/213/223 rows); ui-smoke 8/8, real store untouched; desktop suite 891 + 1 passed, 0 FAILED |
| 2026-10-15 | E7.2 | opus | complete | 26c315e426 (packaged_resources.rs: CPR corpus + Nyx Calder sheet under the packaged root); receipt and rows in the next commit | `artifacts/epic_7/E7.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (42/72/51/63); CPR Jax/Nyx/Imani loaded.json sha256 unchanged vs `E7.1/seeds.txt` | Packaged CPR root: 53 MANIFEST kind files read + parsed, directory = manifest + LICENSE/MANIFEST, every sheet catalog loads, Nyx Calder composes 213 `cpr.` rows with E7.1's figures; two plants RED for the intended reason; tauri-resources-tracked PASS; clean clone builds (npm ci, npm run build, cargo build --locked, exit 0); desktop suite 891 + 1 passed, 0 FAILED |
| 2026-10-15 | E7.3 | sonnet | partial | this commit (docs only: receipt, board rows, discovered E7.3a) | `artifacts/epic_7/E7.3_cycle_receipt.md` | no code touched; PF/SF/CPR seeds not rendered (E7.4 re-derives) | .deb and AppImage built from 8186b6e9b8, installed on clean profiles, Check ok, notes load, Installed panel real, CPR seed Nyx Calder opens (HP 40, Humanity 56/70, EMP 5), Install enabled on a 0.16.0-stamped AppImage; Pending / Rollback panel reads `unknown` x2 on both, so "no unknown" is not met; discovered E7.3a (opus, no cargo, lane main), E7.3 re-runs after it |
| 2026-10-15 | E7.3a | opus | complete | ad9c5ed0e4 (adapter + model + panel + tests); receipt and rows in the next commit | `artifacts/epic_7/E7.3a_cycle_receipt.md` | not rendered: diff confined to `apps/desktop/src/update/**`, no sheet render path touched; PF/SF/CPR seeds unchanged by construction (E7.4 re-derives) | Fresh-install Update page: `no-pending-update` now renders Pending update state `idle` / Rollback state `none`; the "deterministic placeholders" footnote is gone; Last check prints no Install-disabled reason when Eligibility is `eligible` (adapter wrote the fallback `'ineligible'`); new whole-page test after an eligible Check asserts no `unknown` anywhere; npm 173/173 files, typecheck, ui-smoke:doc clean; does not cover `promoted` / `verification-failed`; E7.3 attempt 2 next |

## Epic reports

(Each merge-check card and E1.V appends a block: cards done, figures with commands, seed deltas, open
blockers.)

### Epic report — E0 seam (E0.MC, 2026-10-14, origin/tranche-18 5dc2d53e59)

- **Cards done:** 7 of 7 — E0.1, E0.2, E0.3, E0.4, E0.5, E0.6, E0.MC
  (`awk -F'|' '$2 ~ /^ E0\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 7).
- **What E0 delivered:** `GameSystem::CyberpunkRed` (`cyberpunk-red`, corpus `data/cyberpunk-red/corpus`, no sheet-rule
  root); converter refuses it by name; CPR licence file + licence-matrix DEVIATION row; the corpus bundled at its own
  path; `CyberpunkAdapter` + CPR's own store (save/list/load with 10 `cpr.stat.*` rows), Stub retired for the id; the
  landing chip `cyberpunk-red` selectable with its Load list, New Character closed until E3.2.
- **Figures (each from `artifacts/epic_0/E0.MC_cycle_receipt.md`, command there):** re-run 25 of 25 as expected
  (`awk '/^a/' E0.MC_evidence/results.txt`); mutations 3 of 3 red then restored; root `--lib game_system` 10 passed;
  `--lib saved_character` 11, `--lib cpr::store` 5, desktop `--bins rule_system_adapter` 4, `--bins cpr_adapter` 3,
  `packaged_resources` 1; npm 164/164 test files; ui-smoke 6/6 green, real store untouched; PF converter 49450/49450
  PASS, SF 8582/8582 PASS.
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…81e9b` / Tomas `2320c050…be7b` unchanged C1 → E0;
  SF-Soldier-3 51 / SF-Mystic-5 72 / SF-Technomancer-5 63 / SF-Envoy-3 42 lines unchanged. CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --json`):** 22 events — 3 corrections (C1 version grep, G-1 SF
  test name, A-11 envelope premise), 2 incidents (`clippy-red-on-base`, `shared-tree-two-writers`, 1 each, none
  recurring), 1 rework, 16 verification runs (4 failed: clippy 1, tauri-resources-tracked 3 of which 2 are E0.MC's planted
  mutations).
- **Open blockers:** none.
- **For the lead:** `apps/desktop/scripts/ui-smoke/.out/` (ui-smoke's default evidence dir, PNGs) is not git-ignored;
  every ui-smoke card leaves untracked PNGs (R-C, "never commit a PNG"). A one-line `.gitignore` entry is the control; it
  is outside every card's FILES YOU OWN. Next ui-smoke card: E3.2.

### Epic report — E1 transcription (E1.V, 2026-10-14, tranche-18 after f2b97084f7)

- **Cards done:** 10 of 10 — E1.0, E1.1–E1.8, E1.V
  (`awk -F'|' '$2 ~ /^ E1\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 10).
- **What E1 delivered:** `data/cyberpunk-red/corpus/` — 53 kind files, 930 records, every one with a `source_page`;
  `MANIFEST.json` pins every kind (0 null) and `scripts/cpr_manifest_check.py --all` + `tests/cpr_corpus_manifest.rs`
  (the Rust twin) both pass and both fail on the same 10 plants.
- **Figures (commands in `artifacts/epic_1/E1.V_cycle_receipt.md`):** MANIFEST 53 kinds / `expected` 930 / 0 null
  (python over MANIFEST; awk over CUI §3 → 930; twin asserts `(53, 0, 930)`); page-image sample 112 records over 53 kinds,
  seed 38, ceil 10% per kind (`E1.V_sample.py --check` → `kinds 53 sampled 112 OK`; independent python → `mismatch []`) →
  108 agree, 4 fixed, 0 disagree (`awk -F'\t' 'NR>1{c[$5]++} …' E1.V_verdicts.tsv`); 22 corpus strings + 2 `net_architecture`
  texts corrected from page images (`E1.V_fixes.py --check` → all applied); root sweep 300 suites, 6583 passed, 0 failed;
  PNG/PDF files in git 381 at C1 and at E1.V.
- **Corrections applied in E1:** gear 53/47 → 54/48 (E1.4, SD-c); CUI F-71 N/A 11 → 8 (E1.6); E1.V: `net_architecture[3]`
  paraphrase → printed text, 22 text-layer spacing strings.
- **Seed deltas:** unchanged by construction through E1 (no `src/`, `crates/`, `apps/` file changed in E1:
  `git diff --name-only 54f469ecfe..HEAD -- src crates apps` → empty); E0.MC's PF sha256 and SF line records stand. CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-14T03:00:00Z --json`, before E1.V's own events):** 1 deferral (E1.8, now
  resolved by E1.V's `resolution`, superseded by E1.8's attempt-2 rework), 1 rework, 0 incidents, 7 verification runs (3 failed);
  corrections in E1: E1.4 × 1, E1.6 × 1, E1.V × 2. No key fired more than once.
- **Open blockers:** none.
- **For the lead:** decisions.md §11 asks every record for verbatim `text`; `mooks_grunts` (10) and `net_architecture[1,2]`
  carry none (stat blocks and pure tables, no prose) — E6.4 prints them from their structured fields.

### Epic report — E6 catalogs, Netrunning tools, GM reference (E6.MC, 2026-10-14, origin/tranche-18 2df632993b)

- **Cards done:** 5 of 5 — E6.1, E6.2, E6.3, E6.4, E6.MC
  (`awk -F'|' '$2 ~ /^ E6\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 5).
- **What E6 delivered:** two Tauri commands (`list_cyberpunk_catalog`, `compute_cyberpunk_deck`, `apps/desktop/src-tauri/src/cpr_catalog.rs`)
  over the packaged corpus and the E2.9 engine; a landing → catalog flow with 45 player-facing tabs over 53 MANIFEST kinds
  (8 `print_only`); a Netrunning deck calculator whose every number comes back from `compute_cyberpunk_deck`; 8 read-only GM
  reference pages printing 151 `print_only` records.
- **Figures (commands in `artifacts/epic_6/E6.MC_cycle_receipt.md`):** re-run 9 of 9 as expected (`awk '/^b/' E6.MC_evidence/results.txt`);
  `--bins cpr_catalog` 8 passed; npm 168/168 test files; ui-smoke `browse-cyberpunk-catalog`, `browse-cyberpunk-gm-reference`,
  `landing-select-pathfinder` 3/3 green, real store 15379 entries sha256 unchanged; inventory 107 rows / 18 screens, drift 0; GM pages
  151 records (page test's own count; 8+2+4+3+5+10+55+64 = 151; E6.4 derived it from the MANIFEST sum). Plants: `program` −1 → E6.1 count
  test and E6.2 catalog test red; `demon` −1 → E6.4 GM test red; Standard deck slot +1 in the engine → `corpus_mismatch` refusal, and
  in engine + corpus together → p157 deck `(7, 8)` vs `(7, 7)` red; all restored green.
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…81e9b` / Tomas `2320c050…be7b` unchanged; SF-Soldier-3 51 / SF-Mystic-5 72 /
  SF-Technomancer-5 63 / SF-Envoy-3 42 lines and sha256 unchanged (head 2df632993b vs E0.6 base records). CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --actor sd38-e6.1 --actor sd38-e6.2 --actor sd38-e6.3 --actor sd38-e6.4 --json`):**
  1 event — 1 verification run (E6.1 clippy PASS); 0 corrections, 0 incidents, 0 deferrals, 0 rework. No key fired more than once.
  E6.MC emitted 1 correction (the Summary `C` row). Not logged by its card: E6.1's receipt records a test that was wrong on first GREEN (`extras_add_slots…` expected
  10 of 9) — a rework the retro log does not hold.
- **Open blockers:** none.
- **For the lead:** (1) the Netrunning deck screen has no ui-smoke row; its call into `compute_cyberpunk_deck` is proven by unit tests
  with an injected `compute` and E6.1's command tests, not through the running app — the next `spec.json` writer (E3.2) or E4.7 can
  add a `cyberpunk-deck-calculator` row; (2) the E6 lane appended its four cycle-log rows after `## For the operator`; E6.MC moved
  them into the cycle log; (3) the Summary `C` row still read `1 of 2` after C1 landed — corrected here; (4) `apps/desktop/scripts/ui-smoke/.out/`
  is still not git-ignored (E0.MC's finding); this card deleted its own PNGs before committing.

### Epic report — E2 engine (E2.MC, 2026-10-14, origin/tranche-18 fb6582b96e)

- **Cards done:** 11 of 11 — E2.1–E2.10, E2.MC
  (`awk -F'|' '$2 ~ /^ E2\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 11).
- **What E2 delivered:** `src/rules_core/cpr/{stats,skills,armor,critical,cyberware,roles,improvement,reputation,netrunning,sheet}.rs`:
  every CPR sheet number as a `cpr.` explanations row; `compose_sheet` feeds cyberware BODY/SP/Initiative/HL and the armour penalty into
  the derived STATs, skill bases, wounds, armour, EMP and the Netrunner deck; `CyberpunkAdapter` loads the composed sheet.
- **Figures (commands in `artifacts/epic_2/E2.MC_cycle_receipt.md`):** re-run 18 of 18 steps exit 0
  (`awk '/^b/ && /exit=0/' E2.MC_evidence/results.txt | awk 'END{print NR}'`); `--lib cpr::` 95 passed (module filters 83 =
  5+9+9+9+15+4+5+7+11+9); `--bins cpr_adapter` 5 passed; HP fixture 126 cells (two implementations). Plants, all restored green:
  HP `floor` → 3 tests red, 63 of 126 cells; drop D-11 → 5 refusal tests red; armour sum → 4 red (SP 22 vs 11); Media EMP from the
  "(−2 EMP)" string → 1 red (EMP 3 vs 4); netrunning +1 off → Cyberarm 3 red, Bodyweight Suit 3 red, NET Actions 2 red.
  Full sweeps not repeated: E2.10 ran them on the same code (root 300/6669/0, desktop 2/820/0).
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…81e9b` / Tomas `2320c050…be7b` unchanged; SF-Soldier-3 51 / SF-Mystic-5 72 /
  SF-Technomancer-5 63 / SF-Envoy-3 42 lines and sha256 unchanged (head fb6582b96e vs E0.6 base records); both converters `--check`
  PASS. CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --actor sd38-e2.1 … --actor sd38-e2.10 --json`):** 18 events — 2 corrections,
  7 rework, 9 verification (1 failing stage: clippy); 0 incidents, 0 deferrals; no repeat subject, no recurrence key. 5 of 7 rework
  events are clippy lints a targeted `cargo test` does not run; the §6 step 6 clippy gate (R11) caught each before push. E2.MC emitted none.
- **Open blockers:** none.
- **For the lead:** (1) the clippy-rework shape (5 of 7 E2 rework events) is held by R11, not prevented: a lane that ran
  `cargo clippy --tests` before its first commit would save one commit each time; (2) "netrunning +1 off" in E2.MC's criterion names no
  single +1 — all three readings were planted and each went red; (3) E3.MC and E4.1 now have E2.MC complete.

### Epic report — E3 create (E3.MC, 2026-10-15, origin/tranche-18 c32c03b16f + 4a5cb95486)

- **Cards done:** 8 of 8 — E3.1–E3.7, E3.MC
  (`awk -F'|' '$2 ~ /^ E3\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 8).
- **What E3 delivered:** New Character under Cyberpunk Red opens `CyberpunkCreateForm`: Role, Streetrat / Edgerunner / Complete
  Package STATs from one seedable roller (`cpr_creation.rs`), Lifepath (15 general + the Role's tables, Cultural language at 4),
  skills allocator per method, Role Ability Rank 4 with sub-choices, starting kit and cash (A-9), cyberware pack or bought pieces
  with Humanity/EMP preview and slot refusals (D-11); every figure on the form is an engine `cpr.` row; the character saves through
  the CPR store and loads through `CyberpunkAdapter`.
- **Figures (commands in `artifacts/epic_3/E3.MC_cycle_receipt.md`):** three characters built in the live app, one per method,
  saved, loaded and their sheets opened; against `compose_sheet` of the captured saved files: computed table rows on the live sheet
  114/114 (Streetrat Media), 115/115 (Edgerunner Netrunner), 110/110 (Complete Package Fixer); live lines with a digit all from it
  130/130, 131/131, 120/120 (Python re-count agrees); cells = their row 212/212, 214/214, 208/208; book-formula crosscheck 16/16,
  16/16, 17/17; ui-smoke 9/9 CPR rows green, real store untouched; `--bins cpr_creation` 35/35; npm 171/171; ui-smoke:doc 112 rows.
  Plants, all restored: CP 63 accepted → the refusal row red; pack HL +1 → all three sheet rows red; adapter HP twin → comparison
  A and B red; TSX SW twin → comparison C red.
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…1e9b` / Tomas `2320c050…be7b` unchanged; SF-Envoy-3 42 / SF-Mystic-5 72 /
  SF-Soldier-3 51 / SF-Technomancer-5 63 lines and sha256 unchanged (vs E0.6 base records). CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --actor sd38-e3.1 … --actor sd38-e3.7 --actor sd38-e3.mc --json`):** 17 events —
  6 corrections, 2 rework, 9 verification (2 failing stage runs: clippy); 0 incidents, 0 deferrals. Recurring subject: "FILES YOU OWN
  'same three files'" logged as a correction 4 times (E3.4, E3.5, E3.6, E3.7; E3.7's receipt counts E3.3 as the first recurrence) — every E3 card after E3.2 needed
  `cprCreation.ts` (the wire type) and from E3.5 an engine file; held by fence exits, not prevented. E3.MC emitted 1 rework
  (comparison A from set to multiset after its own plant).
- **Open blockers:** none.
- **For the lead:** (1) R-W: the "same three files" FILES YOU OWN shape recurred 4+ times; a later serial chain over one form needs its
  wire file (`apps/desktop/src/boundary/<x>.ts`) and the engine module in the cell, or a check that the cell is closed under the
  request type. (2) The loaded CPR sheet prints `cpr.cash` and `cpr.ammo.0` under their raw row ids in "Other totals" (no label in
  `cyberpunkSheetModel.ts`) — an E4 label gap for E4.MC/E4.7. (3) ui-smoke cannot address one skill's `+1` on the create form (the probe
  names the button by its text "+1"); E4.7/E5.1 rows that need a skill level will need distinct button text. (4) E4.7 and E5.1 now
  have E3.MC complete.

### Epic report — E4 sheet (E4.MC, 2026-10-15, origin/tranche-18 3a24b12915)

- **Cards done:** 8 of 9 — E4.1–E4.7, E4.MC (`awk -F'|' '$2 ~ /^ E4\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 8);
  E4.8 (discovered by this card) waiting. The epic is not closed until E4.8 is complete.
- **What E4 delivered:** a saved Cyberpunk RED character opens `CyberpunkCharacterSheet`, laid out from `cyberpunkSheetLayout.ts`: STATs
  (entered / current incl. EMP), the 66 skills in 9 categories with armour-penalised bases, HP / SW / Death Save / Humanity / MOVE / LUCK,
  wound track, armour SP and penalties, weapons (DMG printed, ROF, Hands, Concealable, Magazine, Skill Base) with each weapon's D-1 range
  DV row and Autofire row, cyberware (piece HL, totals, slots per container), Role Ability, Netrunner deck / Programs / Hardware / NET
  Actions, Reputation with Facedown, IP, Lifepath, the 22-entry Critical Injury checklist (tick saves through the adapter), and the rule
  lines `cpr_sheet_print.rs` prints under "Rules and features"; no encumbrance box (D-12); every number a `cpr.` row.
- **Figures (commands in `artifacts/epic_4/E4.MC_cycle_receipt.md`):** re-run on 3a24b12915: npm 171/171; model test 909 rows each = its
  row over 4 adapter fixtures, 264 skill rows / 9 groups, 13 weapons, 111 DV cells, 99 printed lines (awk and Python: 20 + 59 + 19 + 1);
  `--bins cpr_adapter` 8/8; `--bins cpr_sheet_print` 4/4, `(-2 EMP)` 0 × 4; ui-smoke `select-cyberpunk-red` + 3 `sheet-cyberpunk-*` rows
  4/4 green, real store 15379 entries `ddfc4461…` untouched; `ui-smoke:doc` 115 rows, R12 clean. Plants, all restored: 2 TSX hand-kept
  numbers → the row-mapping test red; 4 missing range rows (3 in the model, 1 in `sheet.rs`) → model test or adapter fixture pin red; the
  engine plant with fixtures regenerated → the D-1 assertion red.
- **Finding → E4.8:** every character saved through the real form loads with an Other totals box of raw ids (6 of 6 live sheets, 11 rows:
  `cpr.cash`, `cpr.ammo.<i>`; `artifacts/epic_4/E4.MC/other-totals-finding.txt`), while the 4 fixtures carry 0 such rows, so the test's
  "Other totals is empty" holds only on fixtures (retro `fixture-narrower-than-live`, 2 firings). E4.8 labels Cash / Ammunition (by name,
  an engine printed line) / Rent, adds a fixture carrying them, adds a ui-smoke Critical Injury tick, and carries the R-W control for
  `sf-seed-lines-out-dir-missing` (3 firings: E4.1, E4.5, E4.MC).
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…1e9b` / Tomas `2320c050…be7b` unchanged; SF-Envoy-3 42 / SF-Mystic-5 72 /
  SF-Soldier-3 51 / SF-Technomancer-5 63 lines and sha256 unchanged (vs E0.6 base records). CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --actor sd38-e4.1 … --actor sd38-e4.7 --actor sd38-e4.mc --json`):** 17 events —
  4 corrections, 5 incidents, 3 rework, 5 verification (0 failing runs); 0 deferrals. Incident keys: `fixture-narrower-than-live` 2
  (E4.7, E4.MC) → E4.8; `sf-seed-lines-out-dir-missing` 2 as a key (E4.5, E4.MC), 3 occurrences counting the untagged E4.1 and E4.MC
  rework events for the same cause → R-W control in E4.8; `stale-ui-smoke-rows` 1 (E4.7, Pathfinder rows).
  Corrections were each a different subject (G-1 SF command, E4.4 Cargo? column, a test premise, `alt_fire_modes` "None").
- **Open blockers:** none.
- **For the lead:** (1) the 68 → 69 change touched the E7.5 scan in `epic-breakdown.md` and `workflow-instruction.md §11`, `kanban.md`'s
  row check and `README.md`; `scope-draft.md:77` still says 68 (the authored scope, left as written). (2) E4.7's finding (a), the full
  ui-smoke suite red on pre-SD-38 Pathfinder rows, is not E4's and still has no card; it gates E7.4. (3) E4.8 and E5.1 both become ready
  now and both write `CyberpunkCharacterSheet.tsx` and `spec.json`: the main-lane mutex serialises them.

### Epic report — E5 improve (E5.MC, 2026-10-15, origin/tranche-18 9646a423ce)

- **Cards done:** 6 of 6 — E5.1–E5.5, E5.MC (`awk -F'|' '$2 ~ /^ E5\./ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'` → 6).
- **What E5 delivered:** the CPR sheet's Improve button opens `CyberpunkImproveDialog`, priced and saved by `cpr_improve.rs`
  (`preview_cyberpunk_improvement` / `apply_cyberpunk_improvement`): IP entered and spent on one-step Skill Levels and Role Ranks at the
  p. 411 prices (skips and overspend refused); a Role switch at the A-1 price only while the latest Role is Rank ≥ 4, both Roles' rows on the
  sheet; cyberware installed after creation with an entered or seeded-rolled Humanity Loss inside its dice (A-3), slots enforced (D-11),
  Humanity / EMP / Maximum Humanity recomputed; therapy (Standard 2d6 / Extreme 4d6) stopped at the D-5 maximum, Addiction printed; a
  Reputation event (Level 1-10, deed or cowardice) recorded with every event kept and the defining one marked, Facedown negative for
  cowardice. Every figure the dialog shows is a `cpr.improve.*` or sheet row; the sheet re-reads from the store.
- **Figures (commands in `artifacts/epic_5/E5.MC_cycle_receipt.md`):** re-run on 9646a423ce: `--bins cpr_improve` 27/27; ui-smoke 7 E5 rows
  7/7 green through the running app (6 added by E5.MC; spec 116 → 122 rows, python and awk agree), real store 15379 entries `ddfc4461…`
  untouched; npm 173/173; `ui-smoke:doc` 122 rows, R12 clean. Plants, all reverted (`src_clean exit=0`): IP price × 15/10 → 8 of 27 red and
  improve-cyberpunk-skill red; switch at Rank 3 → 2 red; switch gate removed → 3 red and improve-cyberpunk-role-switch red.
- **Seed deltas:** PF Aldric `7b2f2f26…4b42` / Elowen `f0230bf0…1e9b` / Tomas `2320c050…be7b` unchanged; SF-Envoy-3 42 / SF-Mystic-5 72 /
  SF-Soldier-3 51 / SF-Technomancer-5 63 lines and sha256 unchanged (vs E0.6 base records). CPR seeds: none yet (E7.1).
- **Retro (`scripts/retro.py summary --since 2026-10-13 --actor sd38-e5.1 … --actor sd38-e5.5 --actor sd38-e5.mc --json`):** 8 events —
  1 correction (therapy page p. 231 → printed p. 230, E5.4), 1 incident (`tdd-red-not-observed`, E5.3), 6 rework (E5.1 × 2: placeholder
  token, clippy `type_complexity`; E5.2: the same clippy lint; E5.3: stale sheet fixtures pushed before the desktop sweep; E5.MC × 2: a ui-smoke expect and a row note); 0 deferrals. No
  incident key fired more than once. The clippy `type_complexity` cause recurred (E5.1, E5.2) as rework, not as a keyed incident; both
  were caught by the R11 clippy step before push.
- **Open blockers:** none.
- **For the lead:** (1) E4.8 (discovered by E4.MC, dep E4.MC complete) is still `waiting`; every one of E5.MC's 7 live sheets still shows
  the OTHER TOTALS raw ids (`cpr.cash`, `cpr.ammo.0`, `cpr.ammo.1`, `artifacts/epic_5/E5.MC/explore/live-sheet-*.txt`), so the E4 epic is
  not closed and E7.5 will stop on it. E4.8 and E7.1 both write `spec.json` (main-lane mutex). (2) E4.7's finding (a), the full ui-smoke
  suite red on pre-SD-38 Pathfinder rows, still has no card and gates E7.4.

## Decisions taken on safe defaults

(Each entry: card, SD-x / D-n / A-n, the alternative not taken.)

- E0.4, `technical-design.md §2` "Adapter": `CyberpunkAdapter`'s Pathfinder-typed methods answer with no
  Pathfinder numbers — `chassis_resolve` zeros + claim-blocking `cpr_adapter.not_a_pathfinder_chassis`,
  `level_up` empty plan, `recompute` and `append_to_character` `success: false` with a named reason.
  Alternative not taken: zero-valued `success: true` answers.
- E1.V, SD-c + CUI §3 sum check: E1.V edited `content-unit-inventory.md` §3 (`gear` row; the E1.8 row split into
  `mooks_grunts` / `encounter` / `beat_chart` rows) and the F-53 / F-54 / F-71 / §2.8 text, outside its FILES YOU OWN,
  because the manifest check's sum test compares MANIFEST with §3 kind by kind (E1.4's receipt names the edit as E1.V's).
  Alternative not taken: leave §3 for a docs card and push MANIFEST with `--all` at exit 1.
- E2.5, D-11 / A-5 readings in `cyberware.rs`: (1) Linear Frame "Installation requires BODY 6/8" is checked against the BODY STAT before
  cyberware; alternative not taken: BODY after Grafted Muscle and Bone Lace (which would admit Sigma from BODY 4). (2) The Neural Link pool is
  5 slots per Neural Link installed; alternative not taken: refuse a second Neural Link (the book states no limit). (3) A body has 2 eyes, 2 arms
  and 2 legs before MultiOptic Mount (up to 5 more eyes) and Artificial Shoulder Mount (2 more arms); alternative not taken: no foundation count
  limit. (4) An explicitly bought Standard Hand / Foot is a piece (HL 2, counts against the D-5 cap); the free one that comes with a Cyberarm /
  Cyberleg is not an install; alternative not taken: never a piece. (5) Subdermal Grip uses a Neural Link slot only, never a Cyberarm slot;
  alternative not taken: both.
- E2.9, A-7 + p350 (`$P/p351.txt:39-40`, "Hardware installed in the Bodyweight Suit … can only take up 1 Option Slot"): the
  suit's extra slot holds one 1-slot Hardware only; a Program or a 2-slot Hardware there is refused as over capacity.
  Alternative not taken: a 2-slot Hardware placed in the suit costs 1 slot.
- E2.6, D-7 / A-2 in `roles.rs`: Maker specialty ranks = 2 × Maker Rank, counting Rank 1 as a rank gained (Rank 4 → 8;
  a Tech bought at Rank 1 by multiclassing gains two). Alternative not taken: 2 × (Rank − 1).
- E2.2, skills (no D-n/A-n names them): (1) a creation spend under 86 is accepted and shown as
  `cpr.skills.creation_points_remaining`; alternative not taken: refuse an under-spend. (2) The Cultural Origin
  language starts at its 4 free levels, levels bought above 4 cost as usual, below 4 is refused; alternative not
  taken: refuse buying it. (3) The (x2) flag comes from the Master Skill List (p. 90), not a Role-list line (p087
  prints Netrunner's and Medtech's (x2) skills unmarked); alternative not taken: the line's own mark.
- E2.7, multiclassing (A-1 names only the price): "the current Role" that must be at Rank ≥ 4 is the **latest** Role
  taken (p. 143 "until your latest Role is at least Rank 4"), so raising an earlier Role never unlocks a switch, and a Role
  already held cannot be bought again. Alternative not taken: any held Role at Rank ≥ 4 unlocks a switch.
- E2.3, armour (no D-n/A-n names it): (1) the Armor Penalty floor "Minimum 0" (p. 350 header) applies to REF and DEX as
  well as MOVE; alternative not taken: floor MOVE only. (2) Several pieces may be worn in one location (p. 184 "All your
  worn armor in a location"); highest SP counts, every piece's penalty competes for the worst; alternative not taken: one
  piece per location. (3) The Bulletproof Shield (no SP, held) is refused as worn armour; alternative not taken: ignore it.
  (4) A Bodyweight Suit named at either location covers both; alternative not taken: require both.
- E2.8, reputation (no D-n/A-n names it): (1) a cowardice event replaces the defining Reputation under the same
  rule as a deed, only when its Level is higher (p. 193 "a new deed's Reputation will only replace the old one if the
  Level is higher"; "Reputation received from a negative event … can replace" a positive one), and a later higher deed
  replaces a cowardice one; alternative not taken: a cowardice event always replaces. (2) Only a defining event for
  cowardice makes the Facedown term negative (p. 194 footnote); the engine's two kinds are Deed and Cowardice, and a
  desertion or betrayal the GM rules "just as bad" (p. 193) is recorded as Cowardice; alternative not taken: a third
  "uncool" kind with its own rule. (3) An equal Level keeps the earlier event; alternative not taken: the newer wins.
- E2.4, wounds + criticals (U-24/U-25/U-26 name the print/compute split; no D-n/A-n): (1) each threshold row's value is the HP the
  state begins **below** (Lightly = total HP, Seriously = ceil(HP/2), Mortally = 1, p. 186 "Less than …"); alternative not taken: the
  highest HP inside the state (39/19/0). (2) A Critical Injury the character already has is refused (p. 187 "until you get a Critical
  Injury that the target isn't currently suffering"); alternative not taken: count it twice. (3) The Stabilization DVs are rows of
  their own (`cpr.wound.*_dv`), since they land on the sheet as numbers; alternative not taken: text inside the threshold row.
  (4) `cpr.wound.death_save` repeats E2.1's `cpr.derived.death_save` (both BODY); E2.10 picks the one the sheet renders.
- E3.1, creation STATs (DEF-2 puts the roller in scope; no D-n/A-n names these): (1) Complete Package spends exactly the
  corpus `complete_package_points` record marked `default` (62, p. 79 "normally 62"); alternative not taken: a `rank`
  parameter offering the GM-option totals 50/70/75/80. (2) The roller seed is a u32 so it crosses JSON to the frontend
  exactly; alternative not taken: u64 (above 2^53 a JS number loses it). (3) Create refuses an id already saved;
  alternative not taken: overwrite.
- E3.2, Create form (no D-n/A-n names these): (1) a Complete Package allocation starts from the STATs on screen when the method is first chosen;
  alternative not taken: every STAT at a fixed start value. (2) The `±1` STAT buttons clamp only at 0; the 2–8 range and the 62 total are the
  engine's refusals, shown as its problems with Create closed; alternative not taken: a desktop clamp (a hand-kept rules table, R2).
  (3) Streetrat rows are offered from the Role's `streetrat_template` records; alternative not taken: a fixed 1–10 list.
- E2.10, sheet assembly (A-7, SD-b, SD-d, SD-g; E0 fence exit per decisions.md §7): (1) the choices beyond the STATs live in their own struct and file (`CprSheetChoices`, `cpr_sheet_choices.txt`), so `CprCharacterInput { stats }` and `CprSavedCharacter` keep their shape and no lane-A file or E3.1 literal breaks; alternative not taken: new fields on `CprCharacterInput` (breaks `stats.rs` tests, fence closed). (2) Deck extras come from the composed armour and cyberware (A-7); alternative not taken: caller flags. (3) An Exotic weapon without its own ROF/magazine prints its text, no number is inherited from `base_weapon` (Hurricane drum 16 ≠ Shotgun 4); alternative not taken: inherit. (4) Current HP / current LUCK stay play-time write-ins (the "out of" maxima are rows). (5) EMP skills use the current EMP (p. 81). (6) `cpr.derived.death_save` and `cpr.wound.death_save` both stay, asserted equal; p121 maps the box to the `cpr.derived` row (closes E2.4's note).
- E4.1, sheet core (D-12 and R2 applied; no D-n/A-n names the three choices below): (1) rows no E4.1 box names render in an "Other totals" box under their row id (SF sheet precedent);
  alternative not taken: hide them until E4.2–E4.5 (an engine row dropped from the sheet). (2) Both `cpr.derived.{death_save,seriously_wounded}` (vitals) and
  `cpr.wound.{death_save,seriously_wounded}` (wound track) render, each once; alternative not taken: hide one of each pair (an exemption list in the row-once test).
  (3) The STAT block's Current column shows the engine's adjusted row (armour REF/DEX/MOVE, cyberware BODY, EMP after Humanity Loss); alternative not taken: entered STATs only.
- E4.2, skills block (R2 applied): (1) skill categories and linked STATs come from the skill catalog, loaded by the sheet through `list_cyberpunk_catalog('skill')`;
  alternatives not taken: a 66-entry TS table (R2) or a category on each engine row (Rust through E2's fence on a TS-only card). (2) While the catalog loads or if it fails,
  the skills render ungrouped (error shown); alternative not taken: hide them. (3) The `skills` box entry lands in `cyberpunkSheetLayout.ts` (E4 lane file per §3, created by
  E4.1, not in E4.2's FILES YOU OWN cell); alternative not taken: `declined ownedBy` (no later card owns it). (4) The (x2) marker is not printed; alternative not taken: print it.
- E3.3, Lifepath (D-9; E0 fence exit per decisions.md §7): (1) each Lifepath table rolls on its own seeded roller (request seed + table place), so a pick never re-rolls another table; alternative not taken: one roller stream (a pick would move every later table's dice). (2) Every table with a `branch` may be marked "not on my path", default rolled; alternative not taken: parsing the choose-box `branches` text into a path picker. (3) An unpicked Cultural Origin language is rolled from the region's list; alternative not taken: Create blocked until picked (breaks E3.2's ui-smoke row). (4) Answers live in their own file `cpr_lifepath.txt` (`CprLifepath`); alternative not taken: a `CprSheetChoices` field (breaks `sheet.rs`'s full-literal `kestrel()` test, an E2 file). (5) Sweet Revenge! is answered once; alternative not taken: once per enemy.
- E3.4, skills allocator (no D-n/A-n names these): (1) Edgerunner starts at the Role's printed levels (86 spent); alternative not taken: every skill at 2. (2) Complete Package starts at the 13 Basic skills at 2 (26 spent, p. 90); alternative not taken: an empty list. (3) Fewer than 86 points may be spent (the engine's rule is at most 86; points left are a row); alternative not taken: refuse unspent points. (4) An unnamed `(choose 1)` line blocks Create; alternative not taken: save the printed placeholder. (5) The Cultural Origin language may be raised above its free 4 (Edgerunner, Complete Package); alternative not taken: hold it at 4.
- E4.3, weapons + range DV + armour (D-1 applied; E2 fence exit per decisions.md §7): (1) the weapon rows (Hands, Skill Base, range DV, Autofire DV, range bands) are computed in `sheet.rs`; alternative not taken: `declined ownedBy` (no later card computes weapon rows). (2) The range bands are rows `cpr.range_band.<b>.max` heading one shared range table, one row per weapon; alternatives not taken: a band header per weapon, or band labels as TSX text. (3) Autofire DV rows for SMG / Heavy SMG / Assault Rifle; alternative not taken: single-shot only. (4) Melee Skill Base = Melee Weapon (p. 176); alternative not taken: none for melee. (5) An Exotic Weapon reads range DV and skill from its `base_weapon`; alternative not taken: none for exotics.
- E4.4, cyberware + Role Ability + deck (D-11, A-7 applied; E2 fence exit per decisions.md §7): (1) each installed piece's name and preset Humanity Loss, and the deck / Program / Hardware names, are engine lines and rows in `sheet.rs` (`cyberware.rs` keeps the installed list); alternatives not taken: `declined ownedBy` (no later card names pieces or deck items) or names read from the save (the load response carries none). (2) Role values labelled by their row field under the Role's name; alternative not taken: loading `role.json` for the ability's printed name. (3) The Brain damage rule line stays for E4.6 (rule text); alternative not taken: print it in the deck box. (4) No per-piece container; alternative not taken: a container column (no engine line carries it).
- E3.5, Role Ability sub-choices (D-7, A-2; E2 + E0 fence exits per decisions.md §7): (1) Maker and Medicine points must all be placed; alternative not taken: allow unplaced points. (2) Combat Awareness points, Motorpool, Team Member and drug picks may be left partly unmade (rows report what is left); alternative not taken: refuse an unmade pick. (3) A Maker specialty takes at most one rank per Maker Rank, all 2 × rank placed; alternative not taken: per-rank-up pair entry. (4) A vehicle picked twice for the pool is refused; alternative not taken: allow it. (5) The Exec's Team Member chart row is picked 1–6, not rolled; alternative not taken: roll it. (6) The sub-choices are saved in their own file `cpr_role_choices.txt`; alternative not taken: new `CprSheetChoices` fields (breaks the `sheet.rs` test literal the live E4 lane was editing).
- E4.5, Reputation / IP / Lifepath / criticals checklist (E2 fence exit per decisions.md §7): (1) event, Facedown-roll and recorded-injury lines are engine printed lines in `sheet.rs`; alternatives not taken: `declined ownedBy` or the TSX appending "+ 1d10". (2) Lifepath answers are adapter lines (own file, no number); alternative not taken: widen `compose_sheet`. (3) The checklist lists the `critical_injury` catalog the sheet loads; alternative not taken: engine lines for unrecorded injuries. (4) A checklist save writes the choices without a revision bump; alternative not taken: advance `revision_id`. (5) The sheet keeps the reloaded answer in its own state; the hub's cached detail is not patched (`CharacterHubPage.tsx` is E5.1's); alternative not taken: an `onDetailRefreshed` prop.
- E3.6, starting kit + cash (A-9; E0 fence exits per decisions.md §7): (1) an "or" line not picked takes its first printed option; alternative not taken: refuse until picked. (2) The shop is the ten per-item Night Market kinds the corpus prices (weapons, attachments, ammunition, armour, gear, fashion, street drugs, cyberdecks, cyberdeck hardware, programs); vehicles, services, lifestyle and housing are not sold, cyberware is E3.7's; alternative not taken: sell those too. (3) Fashion is paid from the 800eb fashion budget first, the rest from the cash; alternative not taken: refuse fashion above 800eb. (4) The p345 ammunition availability and tens rule are enforced; alternative not taken: any caliber. (5) The kit's weapons, worn armour, first shield, ammunition and first Cyberdeck (with programs/hardware) are written to the sheet choices, programs without a deck stay in the kit file; alternative not taken: kit file only. (6) The kit lives in its own file `cpr_kit.txt`; alternative not taken: new `CprSheetChoices` fields. (7) Bought armour names Body or Head; alternative not taken: one purchase covers both.
- E4.6, print path (D-2, D-6 applied; no fence exit): (1) rule lines carry one `kind` per block (`role_ability`, `cyberware`, `humanity`, `weapon_special`, `netrunning`) and render under "Rules and features"; box lines keep `cpr`; alternative not taken: one `cpr` kind for all. (2) The sheet's `rules` = every printed line no box placed; alternative not taken: a kind filter (a future unplaced line would vanish). (3) Weapon specials = alternate fire modes or an Exotic Weapon's text; alternative not taken: print every weapon's `text` (a re-print of its table row). (4) The D-2 line keeps E2.9's `BRAIN_DAMAGE_RULE` wording, re-kinded with its HP figure in `cpr_sheet_print.rs`; alternative not taken: move the constant out of `netrunning.rs` (an E2 fence exit for no wording change).
- E3.7, cyberware at creation (D-5, D-6, D-11, A-9 applied; E2 fence exit per decisions.md §7): (1) a Streetrat/Edgerunner cannot buy cyberware at creation (p117 "predetermined from the start"); alternative not taken: let the 500eb buy cyberware. (2) The engine places every option (first foundation with room, a paired option one per foundation, else alone in a free meat limb where allowed); alternative not taken: a player mount choice per purchase. (3) Pack names the listing spells differently resolve by folded spaces/case or the subcategory's bracketed word ("Tech Hair", "Sandevistan Speedware"); alternative not taken: edit the `starting_kit` record. (4) Pack lines stay in `kit.lines` (section `cyberware`) and are saved in `cpr_kit.txt`; alternative not taken: a separate cyberware request and file. (5) The preview's STAT/derived rows come from the composed sheet (after the loss); alternative not taken: pre-cyberware rows beside a separate loss.
- E3.MC, merge check (no D-n/A-n names these): (1) Roles with no creation sub-choice (Media, Netrunner, Fixer) so each row's steps are only its method's; alternative not taken: Solo/Tech/Medtech/Nomad/Exec with sub-choice clicks. (2) The Edgerunner row expects only roll-independent figures (HL 14, Interface Rank 4) and leaves the rolled STATs to the compute_sheet comparison; alternative not taken: a fixed seed (no seed control in the form; a test-only control would be a shipped twin). (3) A fourth row, `create-cyberpunk-complete-package-refuses-63`, carries the "62 → 63 accepted" plant; alternative not taken: that plant at `--bins cpr_creation` only. (4) The Complete Package row adds Handgun at 1 and raises no skill (the skill `+1` buttons' probe name is "+1" for every skill); alternative not taken: edit `CyberpunkCreateForm.tsx` (not this card's file).
- E4.7, ui-smoke rows (D-1, D-2, D-6 applied; no fence exit): (1) three self-contained rows with their own characters; alternative not taken: `setup` reuse of the E3.MC rows (couples rows, re-asserts the same characters). (2) Roles Solo / Netrunner / Fixer (Combat Awareness, two armour locations and three weapon types; the deck block; bought gear with one armour location); alternative not taken: the E3.MC Roles. (3) The Edgerunner row expects only roll-independent figures (no Humanity / Maximum Humanity: the Netrunner EMP column is 4, 5, 6); alternative not taken: a seeded roll (no seed control in the form). (4) The pre-SD-38 Pathfinder rows found red were not edited (outside the criterion); alternative not taken: rewriting ≈50 PF rows here.
- E4.MC, merge check (discovery rule of `epic-breakdown.md` E7.5 block): the Other-totals label gap (E3.MC lead note (2), E4.7 operator findings (b) and (c)) and the R-W control for `sf-seed-lines-out-dir-missing` (3 firings) become one discovered card E4.8 (opus, cargo, lane `main`, depends on E4.MC), and the 68 in the closure scans is raised to 69 in the same commit; alternative not taken: fix them inside E4.MC (its FILES YOU OWN is `artifacts/epic_4/E4.MC_*`; the fix spans `sheet.rs`, the adapter fixtures, the model and `spec.json`). E4.8 is lane `main` so the main-lane mutex keeps it apart from E5.1 (both write `CyberpunkCharacterSheet.tsx` and `spec.json`); E5.1's own Depends-on is unchanged, so the two run in either order.
- E5.1, IP spend (paper-sheet rule applied; no D-n/A-n names these): (1) the IP entered in the dialog are added to `ip.earned` (the GM's award as entered, p. 410); alternative not taken: an award field outside the dialog. (2) A plan may hold several one-step purchases of one skill (4→5, 5→6), each priced at its own next Level; one purchase that skips is refused; alternative not taken: one purchase per skill per spend. (3) The spend writes `cpr_sheet_choices.txt` only and keeps the revision id (E4.5 checklist precedent); alternative not taken: bump `revision_id`. (4) The dialog opens from the CPR sheet's Improve button and `CharacterHubPage.tsx` is unchanged; alternative not taken: a hub mode for improvement.
- E5.2, Role switch (A-1; E2 fence exit for `input.rs`/`sheet.rs`/`improvement.rs`, E4 fence exit for `cpr_sheet_print.rs`, decisions.md §7): (1) the Role history is saved as `role` (the latest) plus `role.earlier = <Role> <rank>` lines in the order taken; alternative not taken: replace `role`/`role.rank` with a list key. (2) a Rank purchase raises the latest Role; alternative not taken: a Role picker per Rank purchase (raising an earlier Role is not offered by the dialog). (3) the preview's switch options read the saved character; a plan that reaches Rank 4 then switches is priced by the plan; alternative not taken: options recomputed after the plan. (4) a Cyberdeck block stays while any held Role is Netrunner (p. 143 features retained); alternative not taken: latest Role only.
- E5.3, cyberware install with Humanity Loss (A-3, D-11; E2 fence exit for `cyberware.rs`/`input.rs`/`sheet.rs`, E4 fence exit for `cpr_adapter.rs`/`cpr_sheet_print.rs` and the generated sheet fixtures, decisions.md §7): (1) the loss is saved as a `| humanity loss <n>` suffix on the `cyberware = <name> [@ <mount>]` line (old files parse unchanged); alternative not taken: a separate key. (2) the mount is chosen automatically by the engine's `place()` (E3.7 placement); alternative not taken: a mount picker. (3) the seed is pinned on the first preview so the apply keeps the rolled dice; alternative not taken: a fresh roll on apply.
- E5.4, therapy (D-5; E2 fence exit for `cyberware.rs`/`input.rs`/`sheet.rs`, decisions.md §7): (1) the regain is saved as one total `humanity.regained = <n>` line (old files parse unchanged); alternative not taken: a per-session therapy log. (2) Addiction therapy is printed in the dialog and refused as a purchase (it moves no sheet number); alternative not taken: a recorded sheet line. (3) a blank regain rolls with the plan's pinned seed; alternative not taken: entry only. (4) the sheet refuses a saved regain larger than the cyberware's total loss; alternative not taken: clamp silently. (5) new text cites the printed p. 230; existing p. 231 (file page) citations left unchanged to avoid fixture churn; alternative not taken: re-word them.
- E5.5, Reputation change (no D-n/A-n names it; paper-sheet rule applied; no fence exit, no `src/` change): (1) every awarded event is saved, a lower one too (p. 193: people may know of less impressive deeds), and the sheet marks the defining one; alternative not taken: drop an event that does not replace. (2) the change costs no IP and the GM's Level is entered, not computed. (3) the event's kind is deed or cowardice and the Level is always 1-10; alternative not taken: a signed Level entry. (4) a blank or multi-line description is refused (one saved line per event); alternative not taken: save a default description.
- E5.MC, merge check (A-1, A-3, D-5, D-11 applied; no fence exit, no `.rs` change): (1) the card's "cost × 10 → 15" planted as every IP price × 15 / 10 in `improvement.rs` `cost_to_reach`; alternative not taken: edit `ip_cost.json` (the corpus loader refuses a table that disagrees with its per-level rate, so the app would fail to load rather than misprice). (2) "switch at rank 3 accepted" planted twice: the gate one Rank down (cargo; no form path builds a Role at Rank 3) and the gate removed (cargo + the app row); alternative not taken: a test-only Rank 3 character in the app. (3) one self-contained character per row (E4.7 precedent); alternative not taken: `setup` reuse of improve-cyberpunk-skill. (4) E4.8's OTHER TOTALS gap, seen again on all 7 live sheets, is not re-carded (E4.8 holds it); alternative not taken: a second discovered card.
- E7.1, CPR starters (`decisions.md §14`; D-6, D-7, D-11, A-9 applied by the engine; E0 fence exit for the `select-cyberpunk-red` row in `spec.json`, decisions.md §7): (1) the CPR starters are seeded on launch too, each under its own marker (`.default_character_seeded_4/5/6`), like the PF starters; alternative not taken: Restore-only CPR starters (a fresh install would show an empty Cyberpunk list). (2) Names and the choices CUI §6 leaves open: Jax Moreno, Solo template row 3, Combat Awareness Initiative Reaction 2 + Threat Detection 2; Nyx Calder, the 62-point allocation, eight bought skills, a Standard Cyberdeck with six programs, Neural Link + Interface Plugs, Heavy Pistol, Light Armorjack; Imani Okafor, seed 2045, Shoulder Arms 6 -> 4 / First Aid 2 -> 4, Science (Biology), Surgery 2 / Pharmaceuticals 1 (Speedheal) / Cryosystem 1, Assault Rifle picks; Lifepath seeds 2020/2077/2045; alternative not taken: E4.7's row-1 Solo (duplicates an existing smoke character). (3) `select-cyberpunk-red` now expects the three starters and forbids "No characters yet"; alternative not taken: leave the row red.

## Open blockers

None.

## For the operator

- **E4.7 findings (2026-10-15):** (a) the full `npm run ui-smoke` is red on `tranche-18` independent of SD-38: 9 of the first 40 rows red (Pathfinder create / equipment-catalog rows) because the PF create form gained a Levels column (13c387b7ef) and the equipment catalog chips were renamed (afff7a621f), both from tranche-17-ui before the cut; ≈50 PF rows stall on it (`artifacts/epic_4/E4.7/full-suite-attribution.txt`). Needs an owner before E7.4/E7.11 (a stale-row rewrite in `spec.json`). (b) the live CPR sheet shows raw ids `cpr.cash`, `cpr.ammo.<i>` as labels under OTHER TOTALS for every created character (E4.7 `green/live-sheet-*.txt`); the E4.5 fixtures have none. (c) the critical-injury checkboxes have no accessible name, so no ui-smoke row can tick one. (b) and (c) are for E4.MC to rule on. **E4.MC ruling (2026-10-15):** (b) and (c) are one discovered card, E4.8 (lane `main`, after E4.MC); (a) is not E4's and still needs an owner before E7.4.
- Review `README.md §6` (author defaults A-1…A-11 in particular).
- PR #398 must merge before C1 can cut `tranche-18`.
