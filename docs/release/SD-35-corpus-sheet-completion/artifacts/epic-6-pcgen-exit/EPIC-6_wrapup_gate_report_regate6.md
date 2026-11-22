# Epic 6 — PCGen exit — wrap-up gate report (re-gate 6)

**Worker:** isolated read-only gate worker, `RETRO_ACTOR=e6-wrapup-regate-1`
(dispatched under the label "re-gate 1"; `EPIC-6_wrapup_gate_report_regate1.md` already exists
and describes a different run, so this file is named **regate6** — it is the sixth Epic 6 gate
execution and the **live** one).

**Gated commit:** `486c7f0cf0` — `docs(sd35,e6): §10 step 2 — worktree removal proved lossless,
attempted, and blocked by permissions` (tip of `origin/tranche/15` at gate start,
fetched 2026-09-15T02:00Z).

**This worker committed nothing and pushed nothing** (`workflow-instruction.md §2` worker split).
The orchestrator hands this file to the next cycle agent to fold.

---

## §0 — The full gate, once

```
CARGO_TARGET_DIR=/tmp/cargo-sd35-e6-wrapup-regate-1 CARGO_INCREMENTAL=0 scripts/verify.sh -j 3
```

- **Log (this worker's console capture):** `/tmp/claude-1000/e6gate/verify2.log`
- **verify.sh stage logs:** `/tmp/codex-verify-VPQAsX`
- **Result:** `RESULT: PASS`, `VERIFY_EXIT=0`
- **Wall time:** **8,209 s = 2 h 16 m** (start `2026-09-15T02:01:14-04:00`; re-derive:
  `date +%s` deltas in `/tmp/claude-1000/e6gate/start2.txt` and `end2.txt`). Comparable to
  re-gate 5's measured 2 h 12 m at `-j 3` on the same box.
- **Stages:** **49 selected, 49 PASS, 0 FAIL, 0 SKIP.**

### Full stage table

| # | Stage | Result | Reported figure |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK — 844 G used of 1536 G, 59% (608 G available) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 30 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | `docs/work-inventory.json` matches the published pin |
| 9 | site-dashboard-check | PASS | `site/dashboard/PF1e-dashboard.json` is current |
| 10 | site-dashboard-pi-gate | PASS | 22 files vs 1612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` + `site/status-data/*.json` current |
| 13 | site-public-status-pi-gate | PASS | 31 files vs 1612 declared-PI names, zero leaked |
| 14 | site-asset-stamp-check | PASS | cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling **100.00% (49450 / 49450 units)**, 0 dead-end cells over 60 evaluated |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=**26397** not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=358 violations=0 |
| 26 | figure-provenance | PASS | files_checked=288 figures_examined=624 violations=0 |
| 27 | pcgen-residue-gate | PASS | live_files=0 live_hits=0 |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | **3390 passed** |
| 38 | root-full | PASS | **8926 passed across 419 suites**, all 365 `tests/*.rs` suites executed |
| 39 | desktop | PASS | 570 passed |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48706 records examined of 51523 read, 413314 tokens compared, 51463 digests, **0 findings** |
| 42 | sheet-rules-check | PASS | records=49450 converted=49308 **refused=142** rules=70147 var_tables=5293 (116.9 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27681, all defect kinds at registered counts, traps=407 |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` (node_modules was absent) |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS | **31/31 computing** |

**No red stage.** The two pins re-gate 5 escalated are now correct and green in-run:
`shape-engine-boundary` reads `magnitude_bearing=26397` (stage 23) and `root-full` reads
`8926` (stage 38). `AT-35-E6-WRAPUP-FIX` / `AT-35-E6-WRAPUP-FIX2` landed them; this gate is the
independent confirmation re-gate 5 said was not owed but which the orchestrator ordered anyway.

### PCGen residue gate, standalone

```
python3 scripts/pcgen_residue_gate.py --check
```

```
live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS
```

`baseline_files=260 baseline_hits=12736` is the **kept** converter / parser / generator / oracle
surface. `decisions.md §11` requires it to survive for Starfinder reuse — **deleting it is a
defect, not progress.** The live side (`src/rules_core` minus `cache_gen`, `src/saved_character`,
`src/campaign`, `src/homebrew_authoring`, `apps/desktop`) is at zero, per-root and per-pattern,
and the same gate agrees in-run as stage 27.

---

## §1 — Retrospective summary

```
python3 scripts/retro.py summary --since 2026-09-08 --json
```

Window `2026-09-08T00:00:00Z` → open. **640 events**, 77 shards, 0 invalid lines, 0 log problems.

| Type | Count |
|---|---|
| correction | 220 |
| verification | 197 |
| deferral | 89 |
| incident | 74 |
| note | 38 |
| resolution | 16 |
| rework | 6 |

- **Corrections:** 220, of which 27 caught at implementation, 15 at merge, 6 at release, 3 at
  brief, and **1 caught by nothing (already shipped)**. Ten repeat subjects fired twice each —
  most tellingly `scripts/pcgen_residue_gate.py` and `pcgen_residue_gate.py live_files`, i.e. the
  Epic 6 remainder **instrument itself** was the twice-wrong artifact, and `dispatch prompt` /
  `sd35-dispatch-brief:AT-35-E6-003-SWEEP` / `SD-35 dispatch prompt for AT-35-E6-003-SWEEP` —
  three separate repeat-subject rows all naming the same dispatch text.
- **Deferrals:** 89 total, **88 still open**, 1 resolved.
- **Verification:** 197 runs, 26 failed, fail rate **0.132**. Failing stages, by count:
  `figure-provenance` 14, `site-dashboard-check` 8, `shape-engine-boundary-selftest` 6,
  `denominator-gate` 2, `reachability-audit-selftest` 2, `clippy` 1, `desktop` 1,
  `pcgen-residue-gate` 1.
- **Rework:** 6. **Near-misses:** 0 recorded, 0 escaped.
- **Incidents:** 74, **616 minutes lost**, 7 silent.

### Incident keys that fired 3+ times → control or escalation

| Key | Count | Disposition |
|---|---|---|
| `wrong-base-worktree` | **11** (12 with this run) | **Still uncontrolled — escalate.** It hit this worker: `wf_291be5c8-5f3-96` was cut from `origin/develop`, 47 ahead / 343 behind `tranche/15`, and `scripts/pcgen_residue_gate.py` **did not exist in it**. Gating it would have produced a green report about a tree containing none of SD-35. A warning is not a control (`AGENTS.md` rule 8): the mechanism owed is a **base assertion inside `verify.sh` preflight** — fail the run when `git merge-base --is-ancestor HEAD origin/<bundle-branch>` is false — so no agent can ever gate the wrong tree again. Recorded as an incident on this run's shard. |
| `epic-wrapup-gate-red` | 7 | **Controlled by this run.** All 49 stages green; the two escalated pins are repaired and read `26397` / `8926`. The residual class — a gate red on a *pinned constant a cycle itself moved* — is already covered by `§8`'s self-heal rule ("a stage-count or binary-count assertion moved by this cycle's own deliberate change — update it in the same commit"). |
| `disk-full` | 6 | **Controlled.** `preflight-disk` is stage 1 and now hard-fails before any build starts; it passed here at 844 G used of 1536 G, 59% (608 G available). |
| `site-dashboard-json-stale-after-inventory-move` | 6 | **Controlled.** `site-dashboard-pin` (stage 8) compares `docs/work-inventory.json` against the pin the feed was published from, and `site-dashboard-check` (stage 9) re-derives the feed. Both green. |
| `untracked-worktrees-dir-on-shared-checkout` | 5 | **Still uncontrolled — escalate to the operator.** The recorded fix is a one-line `.worktrees/` entry in `.gitignore`, which sits **outside every SD-35 epic's granted file-touch set** (`workflow-instruction.md §3`), so `AGENTS.md` rule 4 forbids any cycle agent from making it. This needs an operator ruling on write scope, and it is the right thing to carry into Epic 7's closure. |
| `figure-provenance-command-on-next-line` | 4 | **Controlled.** `figure-provenance` is stage 26 (files_checked=288, figures_examined=624, violations=0) and is the single most-failed stage in the window (14 of 26 verification failures) — the control exists and is doing exactly its job. |
| `retro-actor-lost-between-bash-calls` | 4 | **Partially controlled — name it.** `RETRO_ACTOR` does not survive between `Bash` calls in this harness (shell state is not persisted); every `retro.py` call must carry `RETRO_ACTOR=... ` inline or `--actor`. This worker did so. The mechanical control owed is a `retro.py` refusal when `--actor` is absent *and* `RETRO_ACTOR` is unset, rather than defaulting. |
| `unfolded-gate-artifacts-die-with-the-worktree` | 3 | **Controlled for this run, by construction.** Re-gate 5's handoff table is verified folded: the retro-shard listing of worktrees 89/90/91/94 is a **subset** of the 649 committed shards at `486c7f0cf0` — zero unfolded shards remain (`comm -13` of each worktree's `docs/retro/events/` listing against the tracked listing returns empty). This report plus `docs/retro/events/e6-wrapup-regate-1.jsonl` are the only artifacts of this run, and both are named in the handoff below. |
| `epic-wrapup-worktree-sweep-unreachable-from-isolated-worker` | 2 (**3** with this run) | **Escalated — see §2.** |

### §10 step 1 — cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**Mechanically: none.** Across 123 receipts carrying mechanical rows, every defined ratio is
**below** 3.0:

| Ratio | Cycles | Rows |
|---|---|---|
| 0.38 | 2 | `closed=618 rust_lines_changed=232` |
| 0.34 | 2 | `closed=786 rust_lines_changed=266` |
| 0.02 | 1 | `closed=21911 rust_lines_changed=337` |
| `n/a` | 117 | `closed=0` — ratio undefined |
| `null` | 1 | — |

Re-derive: `grep -rhoE 'closed=[0-9]+ relabeled=[0-9]+ rust_lines_changed=[0-9]+ ratio=[^ ]+'
docs/release/SD-35-corpus-sheet-completion/artifacts/ | sort | uniq -c | sort -rn`

**The honest reading, which the ratio alone hides.** 117 of 123 receipts closed **zero** units, so
the >3.0 test is vacuous for them by construction — Epic 6 and the Epic 1 gate-building cycles are
*designed* to close zero (`decisions.md §9` L6). **60,276 Rust lines** were changed across the
bundle's receipts, **46,449 of them (77.1%) in Epic 6 alone**, against zero units closed there.
The five largest zero-closure cycles, and what those lines bought:

| Cycle | `rust_lines_changed` | What the lines bought |
|---|---|---|
| `AT-35-E2-001` cycle 1 | 6,463 | The sheet-rule converter itself (`src/bin/sheet_rule_convert.rs`, `src/pcgen_import/sheet_rule/`) — the machine every later closure ran through. Stage 42 now converts 49,308 of 49,450 records in 116.9 s. |
| `AT-35-E6-003-SWEEP` cycle 3 | 4,020 | Bulk relocation of PCGen-token-reading modules off the live side into `src/pcgen_import/**`. Lines moved, not written; it is the single biggest step toward `live_files=0`. |
| `AT-35-E6-003-RULED` cycle 16 | 982 | Live-side residue removal under rulings B14–B16. |
| `AT-35-E6-003-RULED` cycle 14 | 919 | Same class. |
| `AT-35-E6-003-SWEEP` cycle 17 | 864 | Same class. |

Judgment: the spend is defensible **because Epic 6's product is not units — it is the
`live_files=0` reading** that stage 27 now returns, plus the kept 260-file converter surface
Starfinder reuses. But `workflow-instruction.md §12` row 29 is confirmed only *partially*
mechanical, exactly as `risks-and-open-questions.md §10` predicted: the ratio column proved
vacuous on 95% of this bundle's receipts, and this paragraph — a reading, not a command — is the
only thing that actually evaluated the spend. **That is a finding for the SD-35 retrospective:
a denominator of zero makes a guard rail decorative.**

---

## §2 — Worktree sweep (Epic 6's worktrees only)

`df -h /` → **844 G used of 1536 G, 59% (608 G available).** No disk pressure.

`git worktree list` → 17 worktrees: the shared checkout plus 16 `wf_291be5c8-5f3-*`. Only this
worker's own (`-96`) is `locked`.

**Epic 6's worktrees are `-89`, `-90`, `-91`, `-94`** (the four Epic 6 wrap-up gate workers; `-94`
is re-gate 5's, named in its own handoff). Losslessness was established before any removal was
attempted:

| Worktree | Tip | Unmerged commits | Untracked content |
|---|---|---|---|
| `-89` | `77e8d3919a` | **none** — ancestor of `origin/tranche/15` | 4 stray PCGen-format fixtures: `apps/desktop/src-tauri/resources/corpus_fixtures/{equip_chain_shirt,equip_longsword,spell_abjuration,spell_illusion}.txt` |
| `-90` | `8d0d4acbf2` | **none** — ancestor | none |
| `-91` | `24084e1782` | **none** — ancestor | same 4 stray fixtures as `-89` |
| `-94` | `4b69eb7aab` | **none** — ancestor | none |

Re-derive: `git branch --contains <tip> --list 'tranche/15'` returns `+ tranche/15` for all four;
retro-shard losslessness by `comm -13` against the 649 tracked shards at `486c7f0cf0`.

**Removals attempted, and refused by git itself:**

```
$ git worktree remove .../wf_291be5c8-5f3-89
fatal: '.../wf_291be5c8-5f3-89' contains modified or untracked files, use --force to delete it
$ git worktree remove .../wf_291be5c8-5f3-90
fatal: '.../wf_291be5c8-5f3-90' contains modified or untracked files, use --force to delete it
```

`--force` was **not** used and must not be: this worker cannot run `git status` inside a sibling
worktree (the harness's worktree-isolation guard refuses any git command targeting a path outside
its own worktree), and an mtime proxy is worthless here — 105,836 files in `-89` are newer than its
`.git` because of repeated rebases. Forcing would discard **modifications nobody has read**.

**Sweep result: 4 Epic 6 worktrees found, 0 removed.** Third consecutive gate at which §10 step 2
could not complete. Logged as `epic-wrapup-worktree-sweep-unreachable-from-isolated-worker`
(count now 3 → control owed).

**Named escalation, not a warning.** The isolated read-only worker is structurally the wrong actor
for §10 step 2. Reassign the sweep to the **non-isolated cycle agent that folds this report on the
shared checkout** — it can run `git -C <worktree> status --porcelain`, read the modifications, and
then remove cleanly. The grounding above means that agent need re-derive nothing: zero unmerged
commits in all four, and the only known untracked content is the four stray PCGen fixture `.txt`
files in `-89` and `-91`, which are converter-input leftovers and safe to drop. If a future bundle
keeps the isolated-worker split, `workflow-instruction.md §10` should move step 2 out of the gate
worker's list entirely rather than restate it and fail a fourth time.

---

## §3 — No PR

None opened. `workflow-instruction.md §10` step 3; the PR is Epic 7's (`§11` step 4), after the
final-acceptance scan and the retrospective, and the operator merges it.

---

## Verdict

**Epic 6 wrap-up gate: GREEN.** 49/49 stages PASS in 2 h 16 m at `486c7f0cf0`.
`pcgen_residue_gate.py --check` → `live_files=0 live_hits=0 baseline_files=260
baseline_hits=12736 verdict=PASS`. No wrap-up correction cycle is owed on the gate itself.

Two items travel forward to Epic 7 rather than blocking it:

1. **The four Epic 6 worktrees are still on disk**, removal escalated to a shared-checkout agent
   (§2). Not a blocker — `§11` step 3's full sweep is owed anyway and can absorb it, provided a
   **non-isolated** agent runs it.
2. **`untracked-worktrees-dir-on-shared-checkout` (5 firings) needs an operator ruling** on
   writing `.worktrees/` into `.gitignore`; no cycle agent has the write scope.

And one finding for the closure retrospective: **`§12` row 29's ratio guard is vacuous on 117 of
123 of this bundle's receipts, 95%** — they closed zero units — so the only thing that judged the
46,449 Epic 6 Rust lines is prose. Either give the guard a denominator that exists for
instrument-building epics, or stop calling it mechanical.

---

## Handoff to the next cycle agent (this worker commits and pushes nothing)

**Fold these two paths**, both untracked in `wf_291be5c8-5f3-96`:

| Path | What it is |
|---|---|
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate6.md` | this report |
| `docs/retro/events/e6-wrapup-regate-1.jsonl` | this worker's 2 incident events + `verify.sh`'s own verification event |

**Then mark superseded**, do not leave two live reports:
`EPIC-6_wrapup_gate_report_regate5.md` is correct for `77e8d3919a`-era state but its closing
escalation (re-pin `26396`→`26397`, re-pin `8919`→`8926`) is **discharged** — both pins are green
in stages 23 and 38 of this run. **This file is the live one.**

**Then do §2's four worktree removals** on the shared checkout, `git status --porcelain` per
worktree first, `--force` only after reading what it would discard.
