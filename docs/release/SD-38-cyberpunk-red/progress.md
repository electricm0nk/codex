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
| Workflow run id(s) | run 1 `wf_a9b94476-6a6` (task `wh7ev5lcg`, 2026-10-13 ~21:50 UTC, `args {DONE:[C0]}`): stopped at C1 dependency gate (C0 row still `ready`; gate correct). Run 2 `wf_c2267c33-f48` (task `wdpp3gh2w`, ~22:05 UTC, same args) after d899e53dde dispatched nothing (a note in C1's Depends-on cell parsed as a dependency; fixed 3b4b946e77, generator now rejects non-id tokens). Run 3 `wf_f42286f7-e39` (task `wyc1bpncy`, ~22:12 UTC, same args): C1 blocked-escalated on the OLD SD-k text because `.worktrees/sd38-main` (made by run 1 at a53be2166a) was never fast-forwarded; the serial-lane prompt now fast-forwards the tree before any read (generator, this commit). Run 4 `wf_9e6f5c69-d87` (task `wsjycvzfg`, ~22:20 UTC, same args): C1, E0.1, E0.2 complete (706d8da680, 7732b17462, 481de470b6); stopped at E0.4 `declined` — (a) E0.2 and E0.4 both depended only on E0.1, so the scheduler dispatched both into the one serial tree (fixed: generator's serial-lane mutex, one `main` card in flight at a time, plus E0.4 now depends on E0.3); (b) A-11's envelope-field premise refuted (no serde derive, hand-rendered store, 39 literal sites, PF-shaped payload required) — A-11 revised to CPR's own store (`decisions.md §21`), E0.4 card restated. Run 5 `wf_a66e367b-137` (task `w9jd8iezb`, args `{DONE:[C0,C1,E0.1,E0.2]}`); script `artifacts/cycle_0/sd38-workflow.js` |
| Script path | `artifacts/cycle_0/sd38-workflow.js` (written at launch; it is also the prefix backup) |
| Pinned `tranche-18` SHA for wrong-base resets | the C1 bump commit (written by C1) |
| Oracle | none for CPR (`decisions.md §23`); PF/SF gates keep `scripts/pcgen-oracle-pin.env` |

## Summary

| Epic | Status | Cards complete | Notes |
|---|---|---|---|
| C | in progress | 1 of 2 | C0 complete (1102f34aab); C1 next (SD-k revised: no wait on #398) |
| E0 | complete | 7 of 7 | E0.1 complete (7732b17462, ebbb643800); E0.2 complete (481de470b6); E0.3 complete (4edfb5e3fb); E0.4 complete (734ed496a4, a371dbed22); E0.5 complete (2db271c281); E0.6 complete (666edd9f47); E0.MC complete (f953bca5ce) — Epic report below |
| E1 | complete | 10 of 10 | E1.0–E1.8 complete; E1.V complete (MANIFEST pinned 53 kinds / 930 / 0 null, 112-record image sample, Rust twin) — Epic report below |
| E2 | in progress | 3 of 11 | lanes A, B, C running; E2.1, E2.5, E2.9 complete (count: `awk -F'\|' '$2 ~ /^ E2\./ && $5 ~ /^ complete *$/' kanban.md`, unescape `\|`) |
| E6 | waiting | 0 of 5 | runs in the E2 window |
| E3 | waiting | 0 of 8 | |
| E4 | waiting | 0 of 8 | |
| E5 | waiting | 0 of 6 | |
| E7 | waiting | 0 of 11 | |
| **Total** | | **19 of 68** (C0, C1, E0.1–E0.6, E0.MC, E1.0–E1.8, E1.V; derived after E1.V) | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` (unescape `\|`) |

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

## Open blockers

None.

## For the operator

- Review `README.md §6` (author defaults A-1…A-11 in particular).
- PR #398 must merge before C1 can cut `tranche-18`.
| 2026-10-14 | E6.1 | sonnet | complete | 8e65a98089 (code); receipt and rows in the next commit | `artifacts/epic_6/E6.1_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | `cpr_catalog` 8/8: 53 MANIFEST kinds = expected counts; p157 deck 7 of 7, 8th refused; desktop 2 suites / 818 passed / 0 failed; frontend 165/165 |
| 2026-10-14 | E6.2 | sonnet | complete | 61aa6d42cc (code); receipt and rows in the next commit | `artifacts/epic_6/E6.2_cycle_receipt.md` | PF Aldric/Elowen/Tomas sha256 unchanged; SF 4 seeds' lines unchanged (vs E0.6 base records) | 45 tabs = 45 non-print_only of 53 MANIFEST kinds, 779 records; ui-smoke browse-cyberpunk-catalog 2/2 green; frontend 166/166; 106 ui-smoke rows |
| 2026-10-14 | E6.3 | sonnet | complete | 7d59a9a0b9 (code); receipt and rows in the next commit | `artifacts/epic_6/E6.3_cycle_receipt.md` | PF/SF not re-rendered: no src/crates/src-tauri change (unchanged by construction) | Deck Calculator tool tab; pickers 3/27/6; frontend 167/167; 106 ui-smoke rows (no new row; spec.json is E6.4's) |
