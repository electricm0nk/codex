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
| C | in progress | 3 of 4 (C0.0, C0.2, C1) | |
| E0 | waiting | 0 of 4 | |
| E1 | waiting | 0 of 5 | |
| E2 | waiting | 0 of 3 | |
| E3 | waiting | 0 of 6 | |
| E4 | waiting | 0 of 7 | |
| E5 | waiting | 0 of 5 | after E4.MC (C0.2) |
| E6 | waiting | 0 of 7 | |
| E7.1 | waiting | 0 of 1 | |
| E4a | waiting | 0 of 5 | serial after E7.1 (C0.2) |
| E7.2–E7.9 | waiting | 0 of 8 | |
| **Total** | | **3 of 55** | command below the table |

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
| C0.1 | 2026-10-02 | Haiku 4.5 | (see receipt) | partial | n/a (docs only) | `artifacts/cycle_0/C0.1_cycle_receipt.md` |
| E0.1 | 2026-10-02 | Sonnet 5.5 | 5df9eb36db | complete | unchanged (no engine/data change) | `artifacts/epic_0/E0.1_cycle_receipt.md` |

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

## Open blockers

None. Each entry here is a request for an operator ruling. It pauses only the named card and its
dependents, and it is **never** a closure path.

## DISCOVERED

None yet. A discovery becomes a new `kanban.md` card in the discovering cycle's commit.
