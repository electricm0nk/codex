---
canonical: true
bundle_id: SD-37
---

# SD-37 Progress

This is the live cycle log, and the surface the operator reviews on return. Every dispatched card
appends one row to the cycle log, **in its own closing commit**. Re-fetch this file before editing
it (`workflow-instruction.md §5`).

## Run handle

| Field | Value |
|---|---|
| Workflow run id | in `~/.claude/projects/-home-ubuntu-workspace-repos-codex/memory/sd37-launch-state.md` (launch decision below); E7.4 copies it here |
| Script path | `artifacts/cycle_0/sd37-workflow.js` (the launched script; it is also the prefix backup) |
| Prefix backup | `artifacts/cycle_0/sd37-workflow.js` — the `CORE` constant |
| Pinned `tranche/17` SHA for wrong-base resets | `9d03a769962c2d6fa118ef80f8cce18443a296cd` (C1 bump commit) |
| Oracle SHA | `7f818006e371188e5717fd18d74d18a420747fc6` |

## Summary

| Epic | Status | Cards complete | Notes |
|---|---|---|---|
| C | complete | 4 of 4 (C0.0, C0.1, C0.2, C1) | |
| E0 | complete | 4 of 4 | rows corrected from `kanban.md` by E1.MC |
| E1 | complete | 5 of 5 (E1.1–E1.4, E1.MC) | E1.MC receipt `artifacts/epic_1/E1.MC_cycle_receipt.md` |
| E2 | complete | 3 of 3 (E2.1, E2.2, E2.MC) | E2.MC receipt `artifacts/epic_2/E2.MC_cycle_receipt.md` |
| E3 | complete | 6 of 6 (E3.1–E3.5, E3.MC) | E3.MC receipt `artifacts/epic_3/E3.MC_cycle_receipt.md` (E3.MC corrected this row from 3 of 6; retro correction) |
| E4 | waiting | 0 of 7 | |
| E5 | waiting | 0 of 5 | after E4.MC (C0.2) |
| E6 | waiting | 0 of 7 | |
| E7.1 | waiting | 0 of 1 | |
| E4a | waiting | 0 of 5 | serial after E7.1 (C0.2) |
| E7.2–E7.9 | waiting | 0 of 8 | |
| **Total** | | **22 of 55** | command below the table |

Total complete, from this folder (C0.2: the authoring form, with `(C\|E)` escaped inside a table
cell, printed 0):

```bash
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'
```

---

## Cycle 0 — package authored; unattended mode acknowledged (2026-10-02)

**Operator on record.** The operator is away while this bundle runs. Directive of 2026-08-01
(verbatim): *"include instructions to all 3 that indicate they will be running in unnattended mode
since i will be out of town while this runs. They may not stop to ask questions - it might be days
before i notice."* This receipt is the third of the three mirrors. The other two are
`workflow-instruction.md`'s OPERATING METHOD callout and `decisions.md §12`.

- **Card:** C0.0. **Author:** orchestrating session's planning sub-agent, Sonnet 5.5 (see
  `decisions.md §11` planning disclosure; card C0.2 is the Opus review).
- **Tree:** `/home/ubuntu/workspace/worktrees/codex-sd37`, branch `tranche/17` @ `20bf84a3b2`
  (= `origin/develop`). `git status --porcelain` was empty before authoring.
- **Write scope honoured:** only `docs/release/SD-37-starfinder-1e/**`. No code, no version bump,
  no push. The main checkout `/home/ubuntu/workspace/repos/codex` (another session's tree) was read
  from only through git plumbing (`git worktree list`, `git branch -r`), never written.
- **Inputs read:** the three research reports (source inventory, engine-reuse map, predecessor
  lessons); `docs/release/template/template.md`; `docs/governance/workflow-instruction-template.md`;
  `.claude/skills/stc-authoring/SKILL.md`; the unattended-mode doctrine; `AGENTS.md`; the memory
  index and the notes named in the brief; and SD-35/SD-36 packages as shape siblings.
- **Figures re-derived:** CUI F-1 … F-20, each with two implementations
  (`content-unit-inventory.md §1`).
- **Status:** complete.

### Likely decision points and their safe defaults

These are the points that historically trigger questions, each with the default to take. Full
table: `decisions.md §12.1`.

| Decision point | Safe default |
|---|---|
| Book licence unclear (E0.2) | Exclude and log (SD-a) |
| SRD unreachable for hand values (E0.4) | `blocked-escalated` on E0.4 and its fixture dependants only; continue E1/E2/E3.1–E3.2/E4a |
| Unmapped overloaded PCGen field (E3.3) | Named refusal (SD-d) |
| PF render byte-diff (E1.4, E4a.4) | Fix it; never re-baseline (SD-i) |
| Seed build impossible in the oracle (E0.4) | Swap to another CRB option of the same class and log it (SD-g) |
| Any `decisions.md §12.3` stop condition (usage-limit error; token proxy; shown reading ≥ 85%) | Stop dispatching, write a resume receipt (SD-l) |
| No CI on `tranche/17` pushes | CI evidence = the PR's `pr-tests`, awaited by E7.9 (SD-n) |
| Rebase conflict in `kanban.md`/`progress.md` only | Keep both sides and continue; any other conflict → `blocked-escalated` (SD-o) |
| A fenced file is needed by the wrong lane | `declined` with `ownedBy` (R4) |
| Starship content met in records | Print its prose; no starship sheet (SD-f) |

### Pending retro events (to be emitted by C0.1; outside this package's write scope)

```bash
scripts/retro.py correction --subject 'SD-36 FS-3 / scope-draft / memory rules-tables-stay-rust-until-starfinder' \
  --claimed '181,797 lines in src/rules_core/rules_tables' --actual '180,883 lines (250 .rs files)' \
  --verified-by "find src/rules_core/rules_tables -name '*.rs' -print0 | xargs -0 cat | wc -l; Python os.walk agrees"
scripts/retro.py correction --subject 'SD-35 forward-scope-register C2.1 owner column' \
  --claimed 'owner SD-36' --actual 'SD-36 has no C2.1 row; adopted by SD-37 as E3' \
  --verified-by "grep -n 'C2.1' docs/release/SD-36-consolidation/forward-scope-register.md (no output)"
scripts/retro.py correction --subject 'SD-37 source-inventory research report' \
  --claimed 'system/gameModes/Starfinder has 12 .lst files' --actual '10 .lst + base.xml.ftl + bio/' \
  --verified-by "find \$PCGEN_REPO_DIR/system/gameModes/Starfinder -maxdepth 1 -name '*.lst' | wc -l"
```

Added by C0.2 (2026-10-02) — C0.1 emits these too, with `--actor sd37-c0.2` so the catch is
attributed to the review card (`retro.py` writes to `docs/retro/events/`, outside C0.2's scope):

```bash
scripts/retro.py correction --actor sd37-c0.2 --subject 'SD-37 package authoring (decisions.md §8, technical-design §4-5, CUI §3, risks R-1)' \
  --claimed 'HP|ALTHP is Hit Points; HP|CURRENTMAX (+HD) is Stamina' \
  --actual 'oracle routes CON*TL, Toughness and drone Energy Shield into ALTHP and RaceHP and +1 Hit Point into CURRENTMAX: ALTHP reads as Stamina, CURRENTMAX+HD as Hit Points (hypothesis for E3.3; SRD values to be sourced by E0.4)' \
  --verified-by "awk -F'\t' '!/^#/{for(i=1;i<=NF;i++) if(\$i ~ /HP\|(CURRENTMAX|ALTHP)\|/ && FILENAME !~ /scr_classes/) print FILENAME\": \"\$1\" :: \"\$i}' \$PCGEN_REPO_DIR/data/starfinder/paizo/core/*.lst" \
  --blast-radius 'decisions §8, technical-design §4-§5, CUI §3, risks R-1, epic-breakdown E3.3/E4.1, acceptance G-2'
scripts/retro.py correction --actor sd37-c0.2 --subject 'SD-37 package authoring (CUI F-6 / §2)' \
  --claimed '12,733 in-scope SF rows, 214 excluded' --actual '12,718 in scope, 229 excluded (paizo/core/_society/ is SFS guide content, 15 rows)' \
  --claimed-value 12733 --actual-value 12718 \
  --verified-by "awk '/^[^#[:space:]]/ && !/^(SOURCELONG|SOURCESHORT|SOURCEWEB|SOURCEDATE)/{r++} END{print r}' \$PCGEN_REPO_DIR/data/starfinder/paizo/core/_society/_abilities.lst -> 15; Python os.walk agrees"
scripts/retro.py correction --actor sd37-c0.2 --subject 'SD-37 package authoring (E7.3 closure scan, progress Total, E4.1/NR-2 grep)' \
  --claimed 'closure scan prints nothing only when every card is complete' \
  --actual 'the table-cell form (C\|E) is a literal-pipe regex: printed 0 rows over 54 open cards' \
  --verified-by "awk -F'|' '\$2 ~ /^ (C\\|E)[0-9]/ && \$5 !~ /complete/' kanban.md | wc -l -> 0 vs 54 with (C|E)"
scripts/retro.py correction --actor sd37-c0.2 --subject 'SD-37 package authoring (DEF-1 revisit command)' \
  --claimed 'any output means starship oracle data exists' \
  --actual 'test -d ... || grep printed nothing when the starship dir existed and nothing on stdout when PCGEN_REPO_DIR was unset' \
  --verified-by "unset PCGEN_REPO_DIR; test -d \"\$PCGEN_REPO_DIR/data/starfinder/paizo/core/starship\" || grep -lE '^ABILITY:starship' \"\$PCGEN_REPO_DIR\"/data/starfinder/paizo/core/*.pcc -> stdout empty, rc=2"
scripts/retro.py correction --actor sd37-c0.2 --subject 'SD-37 package authoring (CUI F-7 use in E4.1/E7.1/G-2)' \
  --claimed '11 SF classes' --actual '10 player classes + CLASS:Drone (TYPE:Monster)' \
  --verified-by "awk -F'\t' '/^CLASS:Drone\t/{for(i=1;i<=NF;i++) if(\$i~/TYPE:/) print \$i}' \$PCGEN_REPO_DIR/data/starfinder/paizo/core/scr_classes.lst -> TYPE:Monster"
```

The source inventory's own "Caveats" section notes that its counts were not cross-checked. The
corrected figures are in CUI §1, "Stale figures found during authoring".

---

## Cycle log

| Card | Date | Model | Commit | Status | Seed deltas (Aldric / Elowen / Soldier / Mystic / Technomancer / Envoy) | Receipt |
|---|---|---|---|---|---|---|
| C0.0 | 2026-10-02 | Sonnet 5.5 (planning) | this package's commit | complete | n/a (no code) | this section |
| C0.2 | 2026-10-02 | Opus 5.5 | the `docs(sd37,c0.2)` commit | complete | n/a (no code) | `artifacts/cycle_0/C0.2_cycle_receipt.md` |
| C1 | 2026-10-02 | Haiku 4.5 | 9d03a76996 | complete | n/a (no code) | `artifacts/cycle_0/C1_cycle_receipt.md` |
| C0.1 | 2026-10-02 | Haiku 4.5 | (see receipt) | complete | n/a (docs only) | `artifacts/cycle_0/C0.1_cycle_receipt.md` |
| E0.1 | 2026-10-02 | Sonnet 5.5 | 5df9eb36db | complete | unchanged (no engine/data change) | `artifacts/epic_0/E0.1_cycle_receipt.md` |
| E0.4 (transcriber) | 2026-10-02 | Opus 5.5 | the `docs(sd37,e0.4)` commit | in-progress (reviewer pending) | Aldric / Elowen unchanged (not rendered); 4 SF seeds defined: Soldier HP 25 SP 24 RP 4 EAC 16 KAC 19; Mystic 34/25/6/16/16; Technomancer 29/35/6/18/19; Envoy 20/21/5/14/15 | `artifacts/epic_0/E0.4_cycle_receipt.md` |
| E0.4 (reviewer) | 2026-10-02 | Opus 5.5 | the `docs(sd37,e0.4): independent review` commit | complete | Aldric / Elowen unchanged (not rendered); 4 SF seeds unchanged by review, confirmed: Soldier HP 25 SP 24 RP 4 EAC 16 KAC 19; Mystic 34/25/6/16/16; Technomancer 29/35/6/18/19; Envoy 20/21/5/14/15. 126/126 agree, 0 corrections | `artifacts/epic_0/E0.4_cycle_receipt.md` |
| E0.2 | 2026-10-02 | Opus 5.5 | 2f0bd9d793 | complete | Aldric / Elowen unchanged (PF screen byte-unchanged; not rendered); 4 SF seeds not rendered before E4 (no caller of the new items) | `artifacts/epic_0/E0.2_cycle_receipt.md` |
| E0.3 | 2026-10-02 | Sonnet 5.5 | the `docs(sd37,e0.3)` commit | complete | Aldric / Elowen unchanged (not rendered); 4 SF seeds unchanged (not rendered; no engine/data change) | `artifacts/epic_0/E0.3_cycle_receipt.md` |
| E1.1–E1.3 | 2026-10-02 | Opus 5.5 | 60ea507dd3 (+ 209664dce2 C1 `Cargo.lock` repair) | complete | Aldric unchanged (rendered, sha256 1d830682…a569 before = after); Elowen unchanged (rendered, sha256 8d1a711c…00f2 before = after); 4 SF seeds not reachable (no SF data) | `artifacts/epic_1/E1.{1,2,3}_cycle_receipt.md`; PF pre-change baseline for E1.4 in `artifacts/epic_1/pf_baseline/` |
| E1.4 | 2026-10-02 | Opus 5.5 | 390917267e | complete | Aldric unchanged (rendered both trees by the E1.4 harness, sha256 1d830682…a569 before = after); Elowen unchanged (8d1a711c…00f2 before = after); 4 SF seeds not reachable (no SF data) | `artifacts/epic_1/E1.4_cycle_receipt.md` |
| E1.MC | 2026-10-02 | Opus 5.5 | the `docs(sd37,e1.mc)` commit | complete | Aldric unchanged (rebuilt before tree 209664dce2 vs merged 520d746125, sha256 1d830682…a569 both; planted CRB drop → 165c38f3…f7bc, restored); Elowen unchanged (8d1a711c…00f2 both; planted → bffcf7dd…ae14, restored); 4 SF seeds not reachable (no SF data) | `artifacts/epic_1/E1.MC_cycle_receipt.md` |
| E2.1 | 2026-10-02 | Opus 5.5 | the `feat(sd37,e2.1)` commit | complete | Aldric unchanged (rendered by the E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not reachable (no SF data) | `artifacts/epic_2/E2.1_cycle_receipt.md` |
| E2.2 | 2026-10-02 | Sonnet 5.5 | the `feat(sd37,e2.2)` commit | complete | unchanged: the only non-test source change is 36 `#[cfg_attr(test, derive(schemars::JsonSchema))]` lines (not rendered; see receipt "Seed deltas"); 4 SF seeds not reachable | `artifacts/epic_2/E2.2_cycle_receipt.md` |
| E2.MC | 2026-10-02 | Opus 5.5 | the `docs(sd37,e2.mc)` commit | complete | Aldric unchanged (rendered on 3fb7c4a7a1, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not reachable (no SF data) | `artifacts/epic_2/E2.MC_cycle_receipt.md` |
| E3.1 | 2026-10-03 | Opus 5.5 | the `feat(sd37,e3.1)` commit | complete | Aldric unchanged (rendered by the E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not reachable (no SF data before E3.4) | `artifacts/epic_3/E3.1_cycle_receipt.md` |
| E3.2 | 2026-10-03 | Opus 5.5 | the `feat(sd37,e3.2)` commit | complete | Aldric unchanged (rendered by the E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not reachable (no SF data before E3.4) | `artifacts/epic_3/E3.2_cycle_receipt.md` |
| E3.3 | 2026-10-03 | Opus 5.5 | the `feat(sd37,e3.3)` commit | complete | Aldric unchanged (E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not rendered (no SF package before E3.4); mapping-evaluated = SRD = PCGen: Soldier HP 25 SP 24 RP 4 EAC 16 KAC 19 / Mystic 34 25 6 16 16 / Technomancer 29 35 6 18 19 / Envoy 20 21 5 14 15 | `artifacts/epic_3/E3.3_cycle_receipt.md` |
| E3.4 | 2026-10-03 | Opus 5.5 | the `feat(sd37,e3.4)` commit | complete | Aldric unchanged (E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not rendered (no SF adapter before E4.6); first SF package lines (`E3.4_seed_terms.py`, + HD/RaceHP/Con/base/Dex terms = E3.3 for 4 of 4): Soldier Hp 6×3 Stamina 7×3, Defiance Series EAC +5 KAC +8 → 25/24/16/19; Mystic Hp 5×5 Stamina 6×5, tempweave +4/+4 → 34/25/16/16; Technomancer Hp 4×5 Stamina 5×5, D-suit I +5/+6 → 29/35/18/19; Envoy Hp 5×3 Stamina 6×3, carbon skin +3/+4 → 20/21/14/15 | `artifacts/epic_3/E3.4_cycle_receipt.md` |
| E3.5 | 2026-10-03 | Opus 5.5 | d08635eed8 | complete | Aldric unchanged (E1.4 harness, sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not rendered (no SF adapter before E4.6); `E3.4_seed_terms.py` over the 8-book package identical to E3.4 (HP/SP/EAC/KAC Soldier 25/24/16/19, Mystic 34/25/16/16, Technomancer 29/35/18/19, Envoy 20/21/14/15): unchanged | `artifacts/epic_3/E3.5_cycle_receipt.md` |
| E3.MC | 2026-10-03 | Opus 5.5 | the `docs(sd37,e3.mc)` commit | complete | Aldric unchanged (sha256 1d830682…a569 = E1.4); Elowen unchanged (8d1a711c…00f2 = E1.4); 4 SF seeds not rendered (no SF adapter before E4.6); `E3.4_seed_terms.py` over the committed package unchanged (Soldier 25/24/16/19, Mystic 34/25/16/16, Technomancer 29/35/18/19, Envoy 20/21/14/15) | `artifacts/epic_3/E3.MC_cycle_receipt.md` |

## Decisions taken on safe defaults

| Date | Card | Default (SD-x) | Alternative not taken | Why |
|---|---|---|---|---|
| 2026-10-02 | C0.0 | `decisions.md §1`–`§19` (all operator-away defaults) | listed per decision under "Revisit if the operator disagrees" | operator away at authoring |
| 2026-10-02 | E0.1 | Scope call, no SD-x covers it: edit `scripts/tests/test_fetch_pcgen_oracle.sh` (not in the §3 E0 row) | ship the probe without a test, or return declined | it is the only test of `fetch-pcgen-oracle.sh` (`oracle-pin-selftest`), no card row owns it, and TDD requires the failing test |
| 2026-10-02 | C0.2 | Re-sequence: E4a serial after E7.1; E5 after E4.MC (`decisions.md §3`) | (1) keep E4a ∥ E2–E6 with per-file fences for all 252 importers plus `schemas/rules/`, `verify.sh`, `tauri.conf.json`; (2) land only E4a.2 at a quiet point and keep E4a.1 parallel | the overlap (`workflow-instruction.md §3` command) covers files of E1, E2, E3, E4 and E6; E4a.1 alone collides with E2.2/E3/E4 (`verify.sh`, `schemas/rules/`) and E6.1 (`tauri.conf.json`). Serial costs wall time, never correctness, and leaves SF complete before E4a starts |
| 2026-10-02 | C0.2 | Exclude `paizo/core/_society/` (SFS guide core mods, 15 rows) with SSRGG (SD-a) | ingest it as part of the Core Rulebook | it is SFS guide content; SSRGG's PCC reads "All Rights Reserved"; E0.2 may re-admit it with a licence row |
| 2026-10-02 | C0.2 | `.lst` burn-down target = 0 in remaining `rules_tables` and in bundled data; provenance, if kept, goes to an unbundled file (`decisions.md §19`) | keep citations inside shipped JSON | ships PCGen file names, against the 2026-09-15 "nothing of PCGen in live code" intent; the authoring criterion had no target at all |
| 2026-10-02 | C0.2 | SD-n: CI evidence from the PR's `pr-tests`; E7.9 → Sonnet and waits for it | add a `tranche/17` trigger to a workflow | outside every card's write scope; template-level change |
| 2026-10-02 | C0.2 | SF-registry licence test moved E0.2 → E3.1 | keep it in E0.2 and make E0.2 wait for E1.MC | E0.2's other work is independent of E1; only the test needs the registry |
| 2026-10-02 | C0.2 | E0.4 owns the full seed builds (`seed-builds.md`), with constraints (Con mod ≠ 0, etc.) | C0.2 writes the builds itself | build picks need the SRD (E0.4 fetches it); C0.2 must not assert Core Rulebook values from recall |
| 2026-10-02 | launch (orchestrator) | One `CARGO_TARGET_DIR` per source tree, shared by the serial cards on the main tree (`workflow-instruction.md §2.1`) | a fresh dir per card, deleted after each card | serial cards never build at the same time; a fresh dir forces a full workspace rebuild per card. The cross-tree hazard stays out: each lane tree has its own dir |
| 2026-10-02 | launch (orchestrator) | Parallel lanes make an explicit worktree from `origin/tranche/17`; the script does not use `isolation: 'worktree'` | the harness's worktree isolation (`workflow-instruction.md §2.4` item 3) | the harness cuts its tree from the session checkout (`/home/ubuntu/workspace/repos/codex`, on `tranche/16`), the `wrong-base-worktree` incident class; the base check now tests for `kanban.md`, which only `tranche/17` has |
| 2026-10-02 | launch (orchestrator) | C1 runs first, alone; C0.1 then runs beside E0/E1 in its own tree | C0.1 ∥ C1 | C0.1's tree needs `origin/tranche/17`, which exists only after C1 pushes |
| 2026-10-02 | launch (orchestrator) | Each prompt tells the agent to read `workflow-instruction.md §2.1`, §5–§8, §12 and its card rows from the files; the script carries a short binding core | paste §2.1 and §6 verbatim into every prompt (`§2.4` item 5) | a Workflow script cannot read files, and a pasted copy drifts from the file the agents read; the file is the one source |
| 2026-10-02 | launch (orchestrator) | The run id is recorded in `~/.claude/projects/-home-ubuntu-workspace-repos-codex/memory/sd37-launch-state.md`; E7.4 folds it into the Run handle table | write it here at launch (`§2.4` item 8) | after launch the main tree has one writer (the running card); an orchestrator write breaks the clean-tree rule |
| 2026-10-02 | E0.2 | SF PI terms in a separate `SF_PI_TERMS`, screened as PF ∪ SF by `classify_field_sf` | append them to `PI_BLACKLIST_TERMS` | appending moves the signed-off PF list, its Python twin (`tests/pi_blacklist_terms_rust_python_agree.rs`) and `pi-sweep-baseline.tsv` |
| 2026-10-02 | E0.2 | Species names (ysoki, kalo, nuar, …) and `Starfinder` alone are not PI terms | list them | species records are the race mechanic; `Starfinder` names every book. Operator may overrule (`ogl-pi-blacklist.md §7`) |
| 2026-10-02 | E0.2 | SD-a: `paizo/core/_society` and LPJ stay excluded; every SF row `operator_sign_off: false` | re-admit `_society` with a licence row | `_society/_.pcc` declares no licence at all (no `COPYRIGHT:`, no `OGL.txt`); LPJ licence unverified |
| 2026-10-02 | E0.4 | SD-g scope: gear budget for the 3rd/5th-level SF seeds = Core Rulebook Table 11-5 wealth per level (4,000 / 9,000 credits; https://www.aonsrd.com/Rules.aspx?ID=230) | 1,000 credits of 1st-level gear (https://www.aonsrd.com/Rules.aspx?ID=39) | the Core Rulebook has no separate higher-level starting-gear rule; 1st-level gear would not give the Soldier heavy armour with a binding max-Dex cap |
| 2026-10-02 | E0.4 | Seeds take Core Rulebook class features only, not the optional *Starfinder Enhanced* ones AoN mixes into the Technomancer/Envoy pages | Enhanced technomancer/envoy options | the pinned oracle encodes the Core Rulebook; no hand-value column differs between the two |
| 2026-10-02 | E0.3 | Unit = one declaration row per distinct (book, kind, `KEY:`-or-name) in 12 record kinds; `.MOD` of a declared base, internal plumbing, class level/continuation rows, same-key rows and engine-config files are named non-unit buckets (SD-b spirit: count records, not rows) | count every F-6 row (12,718) as a unit | rows include `.MOD` patches and per-level class rows; the inventory must sum rows = units + named buckets. Operator may narrow kinds (proficiency, equipment) later |
| 2026-10-02 | E1.3 | SF converter path refused by name (`closure::BOOKS_RELATIVE` registers no Starfinder subtree; `--system starfinder-1e` exits 2 naming the system) — SD-d analogue | register `starfinder/paizo` (+ `lpj_design/infinite_space`) as the SF subtree now | E3.1 owns the SF include structure and E0.2 the licence of each dir; a registered subtree would also feed SF rows through PF's inventory and `var_names.json` paths (E1.3 receipt, "Does not cover") |
| 2026-10-02 | E1.4 | Render-hash harness at `apps/desktop/src-tauri/src/pf_seed_render_hash.rs` (a `#[cfg(test)]` child module of `character_hub`, run with `cargo test --bins pf_seed_render_hash`); before tree = 209664dce2 (parent of the E1 implementation commit) | the proposed root `tests/pf_seed_render_hash.rs`; before tree = ab7e0b8139 | the desktop crate is bin-only and its own workspace, so no integration test can reach `character_hub`; ab7e0b8139's desktop `Cargo.lock` fails `--locked` (repaired in 209664dce2) |
| 2026-10-02 | E1.MC | Discovery (race_trait_picker bypasses the race registry) routed to E5.1's row notes + `## DISCOVERED`; stale Summary rows E0/E1/Total re-derived from `kanban.md` | a new 56th `kanban.md` card plus an edit to E7.3's `n != 55` scan | E7.3's closure scan pins exactly 55 rows; the owning work (SF races) is already E5.1's criterion |
| 2026-10-02 | E2.1 | Edited the 3 files the new variants break by exhaustive match (`feat_prereqs/converted_gate.rs`, `level_up_option_filter.rs`, `sheet_rule_catalog.rs`: one arm + one test each) | decline with `owned_by` | no card's §3 row holds them (package-doc grep: 0 hits; none imports `rules_tables::`), so there is no owner to name; the variants cannot compile without them |
| 2026-10-02 | E2.1 | `Expr::KeyAbilityMod` in a feat prerequisite reports "the character record carries no key ability" (unverified) | decide it like `AbilityMod` against a 0 | nothing fills `CharacterFacts::key_ability` until E4; a 0 would refuse or admit on a fabricated value |
| 2026-10-02 | E2.1 | Two discoveries routed as `## DISCOVERED` notes to the owning epics (E3 SF load path; E4 `ClassChassis` + key-ability prerequisite), following E1.MC's exception | a new `kanban.md` card each | E7.3's closure scan pins exactly 55 rows |
| 2026-10-02 | E2.1 | PF `STACKING_TYPES` const kept; `STARFINDER_STACKING_TYPES` added beside it, chosen by `stacking_types(GameSystem)` from the package's system | one system-keyed table replacing the const | no existing import moves; both lists are the oracle's `BONUSSTACKS` row (`gameModes/{Pathfinder,Starfinder}/miscinfo.lst:17`, identical) |
| 2026-10-02 | E2.2 | Edited root `Cargo.toml` + `Cargo.lock` (one `[dev-dependencies]` entry, `schemars = "0.8"`, +42 lock lines, all from the local registry cache, resolved `--offline`) | decline with `owned_by` | no card's §3 row holds the root `Cargo.toml`/`Cargo.lock` (package-doc grep for both names: only C1's "root Cargo.toml stays 0.1.0" and its 14-surface list), so there is no owner to name; a serde-generated schema cannot exist without a generator crate |
| 2026-10-02 | E2.2 | Generator is a test-only derive (`#[cfg_attr(test, derive(schemars::JsonSchema))]`, dev-dependency) with the generator inside `sheet_rule.rs` `schema_publish_tests`, `RULES_SCHEMA_OUT=<dir>` to write | a shipped `schemars` dependency, or a new bin | a normal dependency would link into the desktop build; a new bin/test file is in no card's row. E4a.1 extends the same module for `rules_tables.schema.json` |
| 2026-10-02 | E2.2 | Two schemas published (`sheet_rule.schema.json` for `SheetRule`, `var_table.schema.json` for `VarTable`) | one file for `SheetRule` only | the package files are of two root types; the criterion names only `sheet_rule.schema.json` and says `*.schema.json` in the card title |
| 2026-10-02 | E2.2 | `jsonschema` validator dev-dependency dropped; package-file agreement is checked by a top-level key test over every 97th rule file | validate every file with `jsonschema` | `jsonschema 0.57` adds 644 lock lines and needs a crate (`zerocopy-derive 0.8.58`) missing from the offline cache |
| 2026-10-03 | E3.1 | Registry test at `crates/codex-ingest/tests/sf_license_registry.rs` (the real path of the proposed root `tests/sf_license_registry.rs`) | root `tests/` | the registry is converter-side (`decisions.md §6`) and the root crate cannot depend on `codex-ingest`; registering it in `src/rules_core/game_system.rs` would write in E1's row |
| 2026-10-03 | E3.1 | SF books registered by `.pcc` in a new `pcgen_import::system_books::BOOK_PCCS` (+ `EXCLUDED_BOOK_PCCS`); `closure::BOOKS_RELATIVE` keeps no SF subtree, so `PinnedTree::load_for(Starfinder1e)` stays refused by name | register `starfinder/paizo` in `BOOKS_RELATIVE` | `load_for` reads every `.lst` under each child dir of the subtree, which would read SSRGG and the nested `core/_society` (both excluded, SD-a); E3.4 wires `load_for` to the resolved `BOOK_PCCS` list |
| 2026-10-03 | E3.2 | SF formula-system census written by a new `bin/sf_formula_census` to `artifacts/epic_3/formula-system/sf-formula-census.json` | emit it from `sheet_rule_convert --system starfinder-1e` | the SF converter path stays refused by name until E3.4 wires the SF package (E1.3/E3.1 rows); the census is tool-side and carries no record name |
| 2026-10-03 | E3.2 | `--sf-formula` self-test added as class `SfFormulaMode` in `scripts/tests/test_token_coverage.py` (not named in the E3 row) | a separate test file | the `token-coverage-selftest` stage of `verify.sh` runs only that file; a separate file would be a test no stage runs |
| 2026-10-03 | E3.2 | A `MODIFY*` token in a file outside the registered books is refused by name by the ledger ("not in a registered book …"); 0 such today | let the Rust reader open the excluded books to classify them | E3.1's control: the converter never reads an excluded book (SD-a) |
| 2026-10-03 | E3.3 | SF mapping table kept at the §8 proposed path `artifacts/epic_3/token-mapping/sf-mapping-table.v1.json` | `data/starfinder-1e/` | the PF table lives in its package artifacts too; the E5 row says the converter regenerates `data/starfinder-1e/**` |
| 2026-10-03 | E3.3 | Oracle observations from 4 new minimal seed `.pcg` files (`oracle-builds/make_seed_pcg.py`) + PCGen's own `sf_soldier.pcg`/`sf_mechanic.pcg` referenced in place | only PCGen's two test characters | they match no seed; §8 asks for a run of a named build, and the seeds are the builds the fixtures name |
| 2026-10-03 | E3.3 | Theme and level-5 stat picks folded into the `.pcg` base scores (the run prints the fixture scores; the test checks) | model every theme/boost pick in `.pcg` | no mapped field reads the pick, only the final score |
| 2026-10-03 | E3.4 | Converted-book scope is a new `system_books::CONVERTED_BOOKS` (SF = Core Rulebook only) read by the tree, the population and the corpus generator | convert every `BOOK_PCCS` book now | `decisions.md §4`: E3.4 tunes on one book; E3.5 widens the list in one batch |
| 2026-10-03 | E3.4 | SF corpus generated whole by `bin/sf_corpus` as identity + provenance + licence screen (no token array, no description); the converter reads each row from the pinned tree | copy PF corpus shape (`raw_tokens`, rendered description) | a second copy of the source format is residue the shipped side forbids; the row is already in the pinned tree |
| 2026-10-03 | E3.4 | A record whose name is product identity (64) is renamed to its coordinate's `codex_neutral_name` and still converts; a PI description (43) is withheld | exclude the record (SD-a) | licence is OGL 1.0a (E0.2); only the name is PI — the PF §24 rename precedent |
| 2026-10-03 | E3.4 | SF `var_names.json` at `data/starfinder-1e/var_names.json` | `scripts/oracle_harness/` | that is PF's map and E7.1's row; an SF `--write` must never overwrite it (E1.3 "Does not cover") |
| 2026-10-03 | E3.4 | New PI screens (fact values; language/weapon/proficiency names) gated to Starfinder | apply to both systems | PF scan of the same shapes finds 0 hits, but any PF change needs SD-i's byte-identical gate as a PF change; PF `--check` stays PASS 49,450 |
| 2026-10-03 | E3.5 | A Starfinder fact whose NAME is product identity (`FACT:SkyfireCenturion\|True`, 7 records) is withheld | rename the fact to a neutral id | no rule references the fact; mirrors E3.4's PI fact-value withhold (SD-a) |
| 2026-10-03 | E3.5 | `PREATT`/`PREHANDS`/`PREREACH`, body-plan and space print rows, glued-token split, row-name alias for PI-renamed records, PI prerequisite-name withhold: all gated to Starfinder | apply to both systems | PF has 16 `PREATT` rows; any PF move needs SD-i's byte-identical gate as a PF change; PF structural diff stays PASS 49,450 |
| 2026-10-03 | E3.5 | SF race speed (`MOVE:Walk,0` prints `Walk 0 ft.`; `BONUS:VAR\|Walk\|<n>` converts to no line; 9 `MODIFYOTHER … Speed\|SET` degrade by name) left to E5.1 | model SET-override speed in the converter now | race print is E5.1's row and the change moves all 77 races; not in E3.5's criterion |

## Open blockers

None. Each entry here is a request for an operator ruling. It pauses only the named card and its
dependents, and it is **never** a closure path.

## DISCOVERED

A discovery becomes a new `kanban.md` card in the discovering cycle's commit (exception taken below by E1.MC, logged under safe defaults).

- **2026-10-02, E1.MC → E5.1:** `race_trait_picker::race_corpus()` (`apps/desktop/src-tauri/src/race_trait_picker.rs:489-490`, again at `:762-763`) loads race books from the Pathfinder `RACE_CORPUS_BOOKS` const, not from `RACE_CORPUS_BOOK_REGISTRY.books(system)`. The seed sheets' racial traits come from this loader. Evidence: dropping `core_rulebook` from the registry wrapper alone leaves Aldric/Elowen byte-identical, and dropping it from the const flips both (`artifacts/epic_1/E1.MC_logs/mut{A,C}-render.log`). Not an E1 defect: E1.2 allows "wrapped" consts, and this file is in no E1 row. Starfinder race loading (E5.1) must route this loader through the registry. Routed to E5.1's row notes rather than added as a new card, because E7.3 pins 55 rows.
- **2026-10-02, E2.1 → E3.x (first card that loads an SF package):** `corpus_loader::load_sheet_rules` builds every package with `SheetRulePackage::new()` (= Pathfinder 1e), so `live_sheet_rules_for(Starfinder1e)` would hand the `Var` fold the PF stacking list. No value differs today (the oracle's SF and PF `BONUSSTACKS` rows are identical, `miscinfo.lst:17`), but the SF load path must build with `SheetRulePackage::for_system(GameSystem::Starfinder1e)`. `corpus_loader.rs` is in E1's §3 row (E1 complete), not E2's.
- **2026-10-02, E2.1 → E4.x:** `technical-design.md §3` also lists `ClassChassis::{hp_per_level, stamina_per_level, key_ability}` under E2. They are not in E2.1's criterion, and `ClassChassis` (`src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`) is read off converted rows that exist only after E3. When E4 fills `CharacterFacts::key_ability`, `feat_prereqs/converted_gate.rs`'s "the character record carries no key ability" arm for `Expr::KeyAbilityMod` must become decidable.
- **2026-10-03, E3.4 → E4.2:** SF skill bonuses to the display records (e.g. `BONUS:SKILL|Display ~ Perception`) resolve to the oracle's own display-skill records (`scr_skills.lst:75+`, `KEY:Display ~ Acrobatics`, ids `display_*`), not to the base skill (`perception`). Which skill a sheet line totals is E4.2's decision; the package states it as the oracle does.
- **2026-10-03, E3.4 → E3.5/E3.MC:** E3.2's formula-system census predicate (a tab field starting `MODIFY:`/`MODIFYOTHER:`) misses `PART:<n>|MODIFY:…` fields: 1,081 across all SF trees, 273 in core (all `MODIFY:Damage|SET|` + a dice literal such as `1d6`). E3.4 lowers the core's; E3.5 meets the rest through `convert_sf_formula_token`. Retro correction emitted (`docs/retro/events/sd37-e3-4.jsonl`).
- **2026-10-03, E3.5 → E5.1:** Every Starfinder race (77, core included) prints `Walk 0 ft.` from `MOVE:Walk,0`, and its real speed (`BONUS:VAR|Walk|30`, and in the wide books `MODIFYOTHER:PC.MOVEMENT|Walk|Speed|SET|30`) converts to no sheet line. Evidence: `data/starfinder-1e/sheet_rules/core/race/lashunta.json` prose `Walk 0 ft.`; `E3.5_logs/figures.log` 9 `movement_speed SET` degradations. Race speed print/total belongs to E5.1 (races print). Routed to E5.1's row notes, not a new card, because E7.3 pins 55 rows (the E1.MC precedent). Retro deferral emitted (`docs/retro/events/sd37-e3-5.jsonl`).
- **2026-10-03, E3.5 → E3.MC:** E3.4's PART gap is closed for the 8 books: the wide `_report.json` has no `PART`/`unmapped:PART` degradation key. Loading the 7 books moves 87 core rule files (all cross-book: `granted_by`/option-list/`closure_rows` growth, and 10 `!PREFACT` gates resolved to their COM declaring rules); E3.MC pins these as the SF package's first delta classes (`E3.5_logs/core_delta_classes.log`, `fact_gate_check.log`).
- **2026-10-03, E3.MC → E4.1:** Three of the five SF Hit Point / Stamina terms are in no converted record: `HD:1` per level (a class token, no package line), `RaceHP` (value `BONUS:VAR|RaceHP|<n>` on `CATEGORY:Internal` `<Race> Race Selection ~ Default`, sum `BONUS:HP|CURRENTMAX|RaceHP` on Internal `Default`; neither is an E0.3 unit; `grep -rln RaceHP data/starfinder-1e/sheet_rules` → none) and `CON*TL` (the `Constitution` stat definition). The package holds only the class coefficients (`core/class/soldier.json` Hp 6×L, Stamina 7×L) and Toughness. Planting M2/M3/M4 on the table the converter reads leaves `sheet_rule_convert --system starfinder-1e --check` green (M1 alone goes red; `artifacts/epic_3/E3.MC_logs/converter_mutations.log`), so M2–M4 are load-bearing only on E3.3's seed-fixture model. E4.1 must name its source for each term (E4 holds the converter lane, so it may lower the race-selection `RaceHP` into the package) and re-plant M2–M4 against its `sf_seed` test. Routed to E4.1's row notes (E7.3 pins 55 rows).
