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
| Workflow run id(s) | run 1 `wf_a9b94476-6a6` (task `wh7ev5lcg`, 2026-10-13 ~21:50 UTC, `args {DONE:[C0]}`): stopped at C1 dependency gate (C0 row still `ready`; gate correct). Run 2 `wf_c2267c33-f48` (task `wdpp3gh2w`, ~22:05 UTC, same args) after d899e53dde dispatched nothing (a note in C1's Depends-on cell parsed as a dependency; fixed 3b4b946e77, generator now rejects non-id tokens). Run 3 `wf_f42286f7-e39` (task `wyc1bpncy`, ~22:12 UTC, same args): C1 blocked-escalated on the OLD SD-k text because `.worktrees/sd38-main` (made by run 1 at a53be2166a) was never fast-forwarded; the serial-lane prompt now fast-forwards the tree before any read (generator, this commit). Run 4 `wf_9e6f5c69-d87` (task `wsjycvzfg`, ~22:20 UTC, same args); script `artifacts/cycle_0/sd38-workflow.js` |
| Script path | `artifacts/cycle_0/sd38-workflow.js` (written at launch; it is also the prefix backup) |
| Pinned `tranche-18` SHA for wrong-base resets | the C1 bump commit (written by C1) |
| Oracle | none for CPR (`decisions.md §23`); PF/SF gates keep `scripts/pcgen-oracle-pin.env` |

## Summary

| Epic | Status | Cards complete | Notes |
|---|---|---|---|
| C | in progress | 1 of 2 | C0 complete (1102f34aab); C1 next (SD-k revised: no wait on #398) |
| E0 | in progress | 2 of 7 | E0.1 complete (7732b17462, ebbb643800); E0.2 complete (481de470b6) |
| E1 | waiting | 0 of 10 | |
| E2 | waiting | 0 of 11 | |
| E6 | waiting | 0 of 5 | runs in the E2 window |
| E3 | waiting | 0 of 8 | |
| E4 | waiting | 0 of 8 | |
| E5 | waiting | 0 of 6 | |
| E7 | waiting | 0 of 11 | |
| **Total** | | **4 of 68** (C0, C1, E0.1, E0.2; derived after E0.2) | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'` (unescape `\|`) |

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

## Epic reports

(Each merge-check card and E1.V appends a block: cards done, figures with commands, seed deltas, open
blockers.)

## Decisions taken on safe defaults

(Each entry: card, SD-x / D-n / A-n, the alternative not taken.)

## Open blockers

None.

## For the operator

- Review `README.md §6` (author defaults A-1…A-11 in particular).
- PR #398 must merge before C1 can cut `tranche-18`.
