# Epic 3 — Place and surface — wrap-up gate, re-run 2026-09-10

Isolated read-only worker (`workflow-instruction.md §2` worker split, `decisions.md §3`).
**Pushes nothing, commits nothing.** The orchestrator hands this file to the next cycle agent to
commit.

- **Worker actor:** `AT-35-E3-WRAPUP-RERUN` — events in `docs/retro/events/at-35-e3-wrapup-rerun.jsonl`
- **Worktree:** `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-23` (`locked`)
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-AT-35-E3-WRAPUP`, `CARGO_INCREMENTAL=0`
- **HEAD gated:** `137658f31a` (`docs(sd35): pin AT-35-E3-004 cycle 2's commit SHA in its own receipt`),
  the tip of `origin/tranche/15` at gate start, re-derived with `git rev-parse HEAD`
- **Verdict:** **GREEN.** Every stage passed. Status returned to the orchestrator: `complete`.

**This is a re-run, not the first Epic-3 gate.** The first ran at `07e29075b4` and returned RED on
two stages (`EPIC-3_wrapup_gate_report.md`); its correction cycle landed at `2dc322ae32`
(`EPIC-3_wrapup_fix_cycle_receipt.md`). Epics 4, 5 and 6 have since run on this branch, so this
report certifies `tranche/15` as it stands today. Both stages that were red at the first Epic-3
gate are green here.

---

## 0. The full gate, once

    scripts/verify.sh          # every stage, no --only

- **Log directory:** `/tmp/codex-verify-Dn9wxm` (per-stage logs)
- **Console transcript:** `/tmp/sd35-e3-wrapup/verify.log`
- **Result line:** `RESULT: PASS`, exit `0`
- **Stage tally:** **49 PASS, 0 FAIL** of 49 stages.
- **Wall time:** **5,161 s = 86 min 1 s** (`date +%s` either side of the run: 1789048273 → 1789053434).
  Comparable runs: the first Epic-3 gate 5,385 s, the Epic-5 gate 5,053 s.

### Correction found before the gate could run — `wrong-base-worktree`, fourth firing

The dispatched worktree `wf_291be5c8-5f3-23` was created at `85287761d5`
(`Merge pull request #388 … docs/b13-handoff-correction`), a **develop-line** commit, **not** on
`tranche/15`. `git merge-base --is-ancestor HEAD origin/tranche/15` exited non-zero. Running the
gate there would have certified a tree that is not the bundle branch. Fixed before the first build
with `git fetch origin tranche/15 && git reset --hard 137658f31a`, re-verified with
`git log --oneline -1`. Nothing was pushed.

This is the **third** SD-35 wrap-up gate hit by the same defect (the first Epic-3 gate was 31
commits behind; the Epic-5 gate was 122 behind). The mechanical control named at the Epic-5 gate —
a `preflight-base` stage in `scripts/verify.sh` — **has not been built**: it does not appear in
`ALL_STAGES` (`grep -c 'preflight-base' scripts/verify.sh` → 0). See §3.
Event id `1789048401039-at-35-e3-wrapup-rerun-ec7a58`.

### Full stage table — 49 stages, in run order

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 30 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | `docs/work-inventory.json` matches the pin the feed was published from |
| 9 | **site-dashboard-check** | **PASS** | `site/dashboard/PF1e-dashboard.json is current` — **red at the first Epic-3 gate, green here** |
| 10 | site-dashboard-pi-gate | PASS | 22 files scanned against 1612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 files scanned against 1612 declared-PI names, zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00% of 49438 units |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | `population=2485 unclassified=0 no_record=0 corpus_sha=7f818006e…` |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` |
| 24 | missing-engine-tables | PASS | `population=0 kinds=0 citation_failures=0` |
| 25 | denominator-gate | PASS | `files_checked=264 violations=0` |
| 26 | **figure-provenance** | **PASS** | `files_checked=194 figures_examined=353 violations=0` — **red at the first Epic-3 gate (14 violations), green here** |
| 27 | pcgen-residue-gate | PASS | `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3,261 passed |
| 38 | root-full | PASS | **8,772 passed across 413 suites, all 361 `tests/*.rs` suites executed** |
| 39 | desktop | PASS | 576 passed (`apps/desktop/src-tauri`, the separate workspace) |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 42 | sheet-rules-check | PASS | `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (114.6 s) |
| 43 | corpus-trap-audit | PASS | `records_examined=27634`, all defect kinds at their registered counts, `traps=407` |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` (`node_modules` absent in this fresh worktree) |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | root 0, desktop 0 warnings, 0 errors (BOTH crates) |
| 49 | class-dump | PASS | 31/31 computing |

**No red stage. No wrap-up correction cycle is owed by this gate.**

### Baselines are current

`BASELINE_ROOT_FULL_TESTS=8772` in `scripts/verify-baselines.env` matches the measured 8,772; the
stage would have printed a staleness warning otherwise. The three counts the Epic-5 gate flagged
as stale (3,230 / 8,741 / 576) have since been advanced.

### Epic 3's own acceptance bar, re-derived at this HEAD

`python3 scripts/completion_atlas.py --by-kind` reports **DONE at 100.0% of 49438 units, in all 19 kinds**, with
buckets A/B/C/D/M/V/U/X/Z at 0 in every kind. AT-35-E3-001's and AT-35-E3-002's bar (bucket B = 0)
and AT-35-E3-003's (bucket C = 0) hold. `python3 scripts/completion_atlas.py --check` returns
`done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
citation_failures=0`. All four Epic-3 kanban rows (12–15) read `complete`.

**Caveat on that command:** `completion_atlas.py --check` is a **write**, not a read — running it
rewrote `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`'s
`derived_at` from `031e61595a` to `137658f31a` (one field, 1 insertion / 1 deletion). Reverted with
`git checkout --` on that path. An isolated read-only gate worker cannot run this check without
dirtying the tree. Event id `1789048716496-at-35-e3-wrapup-rerun-5e6d25`.

---

## 1. Retrospective summary

Command: `python3 scripts/retro.py summary --since 2026-09-07 --json`. The dispatch named
`--since 53296d80f0`, which `retro.py` rejects (`retro: cannot parse time '53296d80f0'`); that sha
is dated `2026-09-07 21:24:13 -0400` (`git show -s --format=%cI 53296d80f0`), so `2026-09-07` is
the narrowest parseable window containing it. Epic-3-scoped figures below add
`--actor AT-35-E3-001 --actor AT-35-E3-002 --actor AT-35-E3-003 --actor AT-35-E3-004
--actor at-35-e3-002-c2 --actor at-35-e3-004-c2 --actor AT-35-E3-WRAPUP --actor AT-35-E3-WRAPUP-FIX`.

### Epic 3's own actors — 26 events

| Type | Count |
|---|---|
| correction | 13 |
| deferral | 4 (4 open, 0 resolved) |
| incident | 3 (90 minutes lost, 0 silent) |
| resolution | 3 |
| verification | 2 (0 failed) |
| note | 1 |
| rework | 0 |
| near-miss | 0 |

Per actor: `AT-35-E3-001` 4, `AT-35-E3-002` 2, `AT-35-E3-003` 3, `AT-35-E3-004` 1,
`at-35-e3-002-c2` 2, `at-35-e3-004-c2` 3, `AT-35-E3-WRAPUP` 5, `AT-35-E3-WRAPUP-FIX` 6.
Corrections were caught before implementation 2, before merge 2, and 1 before the Epic 7
final-acceptance scan. **No subject was corrected twice.** Every one of the 13 corrections carries
a `--verified-by` command.

### Whole SD-35 window (2026-09-07 → now) — 340 events

verification 170 (23 failed), correction 79, incident 38 (482 minutes lost), deferral 21 (20 open),
resolution 15, note 13, rework 4, near-miss 0.
Failing stages across those 170 runs: `figure-provenance` 13, `site-dashboard-check` 10,
`reachability-audit-selftest` 2, `shape-engine-boundary-selftest` 2, `denominator-gate` 1.
**Both dominant failing stages are PASS at this gate.**

### Incident keys firing 3+ times → the required control or escalation

| Recurrence key | Count | Disposition |
|---|---|---|
| `disk-full` | 14 | **Control built and landed** (Epic-2 wrap-up fix cycle). 12 of the 14 were phantom: `scripts/reclaim.sh` emitted `incident/disk-full` on every routine `--apply`. It now emits only at or above `RECLAIM_PRESSURE_PERCENT` (default 90), a `note` tagged `reclaim-routine` below it. Guarded by `scripts/tests/test_reclaim.py`, green as `reclaim-selftest` at this gate. **Residual true count 2**, below the bar. |
| `figure-provenance-command-on-next-line` | 3 | **Control built and proven.** Root cause: `§6` step 3 ran `denominator_gate.py --check` while `verify.sh` runs `--check-provenance` — a different flag, so no cycle ever ran the failing check locally. `workflow-instruction.md` now names `--check-provenance` in the per-cycle gate block with nonzero exit blocking the push. **It works:** `figure-provenance` is PASS here at `violations=0` over `figures_examined=353`, its second consecutive green wrap-up. |
| `site-dashboard-json-stale-after-inventory-move` | 4 | **Control still owed.** Every disposition to date has been the chore "run `./scripts/publish-site-dashboard.sh` in the wrap-up correction cycle", which `AGENTS.md` rule 8 forbids as a response to recurrence. The stage is PASS here only because the last correction cycle ran the chore. **Named mechanical control (unbuilt):** make whatever writes `docs/work-inventory.json` also write `site/dashboard/PF1e-dashboard.json` in the same run, so the two cannot diverge. Weaker fallback: add `./scripts/publish-site-dashboard.sh --check` to the per-cycle gate block, the same shape that fixed `--check-provenance`. |
| `epic-wrapup-gate-red` | 4 | Not a defect key of its own — each firing decomposes into `figure-provenance` (now controlled) and `site-dashboard-check` (control still owed). **Zero firings at this gate.** |
| `wrong-base-worktree` | 3 (**4 with this one**) | **Control named at the Epic-5 gate and not built — escalated in §3.** The named control is a `preflight-base` stage in `scripts/verify.sh` beside `preflight-disk`/`preflight-oracle`, failing nonzero when the worktree is behind its branch. It is absent from `ALL_STAGES`. A dispatch-prompt caution is not a control; this key has a 27-occurrence history in this repo. |

`duplicate-criterion-dispatch` stands at 2, below the bar, and is carried forward because it caused
both of the window's first two rework events.

### Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**None.** Across all nine Epic-3 cycles the highest ratio is **0.38**, and the epic-wide ratio is
**0.38** — an order of magnitude under `decisions.md §4`'s 3.0 bar.

| Cycle | rust_lines_changed | units_closed | ratio |
|---|---|---|---|
| AT-35-E3-001 cycle 2 | 232 | 618 | **0.38** |
| AT-35-E3-002 cycle 1 | 266 | 786 | **0.34** |
| AT-35-E3-003 cycle 1 | 37 | 0 | n/a (zero divisor) |
| the other six cycles | 0 | 0 | n/a |

**What those lines bought.** AT-35-E3-001 cycle 2's **232** lines are one converter policy —
term-level refusal, so an unlowerable token degrades the term instead of deleting the record — and
they closed **618** units across 13 kinds while also emptying AT-35-E3-003, AT-35-E5-001 and
AT-35-E5-002 as a side effect. AT-35-E3-002 cycle 1's **266** lines are two token-less converter
joins plus widening the `sheet-complete` rung's promotable-status list from two statuses to seven;
they closed the entire **786**-unit remainder and took the corpus to **49,438 of 49,438**.
AT-35-E3-003 cycle 1's **37** lines are pure `#[cfg(test)]` control tests — a proof that the
instrument can fail — and closed zero units by design.

The honest caveat the 3.0 bar cannot catch: a zero-closure cycle's ratio is undefined, so a large
Rust expenditure hides behind `n/a`. Epic 3 has no such cycle — its largest zero-closure spend is
37 test-only lines.

### Deferrals: 4 opened in Epic 3, none deferring Epic 3's Definition of Done

- `1788879242003-at-35-e3-001-bf4043` — AT-35-E3-001 cycle 1 correctly **refused to start**:
  `scoped=214`, under the 500 floor and not the whole 1,404 remainder, with 24 distinct refused
  token types (over `§8`'s 10-type ceiling). The orchestrator re-scoped; cycle 2 closed 618.
  **Superseded.**
- `1788899844992-at-35-e3-001-e163d1` — 2 units join to no corpus record at all
  (`book_of_the_damned_volume_2:spell:summon_demons_nascent_demon_lord`,
  `ultimate_combat:spell:share_language_communal`). Revisit named as AT-35-E3-002, whose cycle 1
  closed the whole 786-unit remainder including them. **Superseded.**
- `1788922132640-at-35-e3-002-ac4da5` — the AT-35-E4-002 oracle run and the AT-35-E5-004 desktop
  choice filter, neither of which was AT-35-E3-002's mechanism. **Both cleared since:** kanban rows
  17 and 22 read `complete`, and row 22 names this deferral id as cleared.
- `1788937296885-at-35-e3-wrapup-13bc01` — the first Epic-3 gate's worktree sweep, refused by the
  same harness isolation §2 records below. **Re-affirmed, not cleared** (see §2).

None of the four defers scope that was in Epic 3's Definition of Done; all four are capability
deferrals with named revisit points (`docs/governance/deferral-revisit-doctrine.md`). Three are
superseded or cleared.

---

## 2. Worktree sweep — this epic's worktrees only

    df -h /              → /dev/sda1  1.5T  1.2T used  313G avail  79% used of 1.5T
    git worktree list    → 18 entries (the main checkout + 16 agent worktrees + ci-trait-choice)

Disk is not under pressure: 79% used of 1.5T, below `reclaim.sh`'s 90% of 1.5T `RECLAIM_PRESSURE_PERCENT`
threshold, and `preflight-disk` passed at gate start.

**Epic 3's surviving worktrees: 2. Removed: 0.**

| Worktree | HEAD | Epic 3's? |
|---|---|---|
| `wf_291be5c8-5f3-14` | `a542652c5e` | yes — AT-35-E3-001 cycle 2 base |
| `wf_291be5c8-5f3-15` | `8cc4ea1516` | yes — AT-35-E3-001 cycle 1 base |
| `wf_291be5c8-5f3-23` | `137658f31a` | **this worker — `locked`, exempt by `§8`** |
| `wf_291be5c8-5f3-17` | `f1f547a41e` | **no longer** — named as Epic 3's third worktree by the first gate at `9995efa1b6`, since reused by a later cycle |

The other 13 agent worktrees belong to Epics 1, 2, 4, 5 and 6 and are out of this sweep's scope.

**Nothing is at risk by content.** All 17 `worktree-wf_291be5c8-5f3-*` branches are ancestors of
`origin/tranche/15` — `git branch --list 'worktree-wf_291be5c8-5f3-*' --merged origin/tranche/15 | wc -l`
returns **17**, the same as `git branch --list 'worktree-wf_291be5c8-5f3-*' | wc -l`. **No worktree
carries an unmerged commit.**

**Why 0 were removed.** This worker is the isolated read-only worker (`decisions.md §3`) and the
harness refuses any git invocation naming a path outside its own worktree. Both
`git worktree remove <sibling>` and a read-only `git -C <sibling> status --porcelain` were refused
this run, with the literal message *"a worktree-isolated agent's git operations must target its own
worktree"*. This is the same refusal the first Epic-3 gate recorded
(`1788937296885-at-35-e3-wrapup-13bc01`) and the Epic-2 gate before it
(`1788901958384-at-35-e2-wrapup-cb0191`). Re-affirmed as
`1789048499492-at-35-e3-wrapup-rerun-1ddbf1`; revisit point is the Epic 7 full sweep
(`workflow-instruction.md §11` step 3), which runs from the shared checkout by a non-isolated agent.

---

## 3. PR

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

### Escalation carried to the orchestrator

`wrong-base-worktree` has now hit **three of the six** SD-35 wrap-up gates, each costing a
discarded or nearly-discarded gate run of roughly 85 minutes. The control named at the Epic-5 gate
(a `preflight-base` stage in `scripts/verify.sh`) was never built, so the fourth firing was caught
only because this worker checked by hand. Two dispositions, either of which closes it:

1. **`preflight-base` in `scripts/verify.sh`**, beside `preflight-disk`/`preflight-oracle`: fail
   nonzero when `git merge-base --is-ancestor HEAD <bundle branch>` does not hold. Cheap, and it
   makes the failure impossible to miss rather than merely warned about.
2. **Pin the worktree base at dispatch.** The harness creates each agent worktree from the
   launching session's checkout HEAD, which is not `tranche/15`. The orchestrator's `Workflow`
   script should pin the base ref explicitly.

Related and still open from the Epic-5 gate: agent worktrees live **under** the shared checkout at
`.claude/worktrees/`, so any codemod anchored at the repo root that walks `.` edits every other
agent's tree (`cross-worktree-codemod-contamination`). This worker's tree was verified clean
before, during and after the run.

---

## Residue line

    python3 scripts/pcgen_residue_gate.py --check
    live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS

The same numbers came back from `verify.sh`'s own `pcgen-residue-gate` stage. The live side
(`src/rules_core` minus `cache_gen`, `src/saved_character`, `src/campaign`,
`src/homebrew_authoring`, `apps/desktop`) is **below** its baseline on both files and hits — Epic 6
is drawing it down, and nothing Epic 3 did raised it.
`src/campaign` and `src/homebrew_authoring` are at 0 files / 0 hits.

---

## Figures + their re-derive commands

- Stage tally 49 PASS and 0 FAIL of 49 — `grep -cE '^    PASS' /tmp/sd35-e3-wrapup/verify.log` and `grep -cE '^    FAIL' /tmp/sd35-e3-wrapup/verify.log`
- Wall time 5,161 s = 86 min 1 s — `python3 -c "print(int(open('/tmp/sd35-e3-wrapup/end.ts').read())-int(open('/tmp/sd35-e3-wrapup/start.ts').read()))"`
- Gate exit 0 — `cat /tmp/sd35-e3-wrapup/exit.code`
- HEAD gated `137658f31a`, and that it is `origin/tranche/15`'s tip — `git rev-parse HEAD` and `git log --oneline -1 origin/tranche/15`
- The wrong base `85287761d5` was not an ancestor of the branch — `git merge-base --is-ancestor 85287761d5 origin/tranche/15`
- Corpus at 49,438 of 49,438 units, DONE 100.0% in all 19 kinds, every bucket 0 — `python3 scripts/completion_atlas.py --by-kind`
- Unit total 49,438 counted independently of the atlas — `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"`
- Atlas instrument clean, `done_evidence_violations=0 citation_failures=0` — `python3 scripts/completion_atlas.py --check`
- PCGen live residue 253 files / 12,256 hits against a baseline of 260 / 12,736 — `python3 scripts/pcgen_residue_gate.py --check`
- Epic 3 events 26, of which correction 13, deferral 4, incident 3 — `python3 scripts/retro.py summary --since 2026-09-07 --json --actor AT-35-E3-001 --actor AT-35-E3-002 --actor AT-35-E3-003 --actor AT-35-E3-004 --actor at-35-e3-002-c2 --actor at-35-e3-004-c2 --actor AT-35-E3-WRAPUP --actor AT-35-E3-WRAPUP-FIX`
- Whole-window events 340, incidents 38, 482 minutes lost, verification runs 170 with 23 failed — `python3 scripts/retro.py summary --since 2026-09-07 --json`
- Incident keys at 3 or more firings: `disk-full` 14, `site-dashboard-json-stale-after-inventory-move` 4, `epic-wrapup-gate-red` 4, `figure-provenance-command-on-next-line` 3, `wrong-base-worktree` 3 — `python3 scripts/retro.py summary --since 2026-09-07 --json`, field `incidents.by_recurrence_key`
- Epic 3 ledger totals: 9 cycles, 1,404 units closed, 0 relabeled, 535 Rust lines, 11 builds — `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['units_relabeled'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"`
- Highest per-cycle ratio 0.38 and epic-wide ratio 0.38, both under the 3.0 bar — `grep -h 'Receipt rows (mechanical)' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/*_receipt.md`
- All 17 agent-worktree branches merged into the bundle branch, 17 of 17 — `git branch --list 'worktree-wf_291be5c8-5f3-*' --merged origin/tranche/15 | wc -l` against `git branch --list 'worktree-wf_291be5c8-5f3-*' | wc -l`
- 18 worktrees present — `git worktree list | wc -l`
- Disk 79% used of 1.5T with 313G available — `df -h /`
- `preflight-base` absent from `verify.sh`, 0 occurrences — `grep -c 'preflight-base' scripts/verify.sh`
- Root-full baseline 8,772 matches the measured 8,772 — `grep -E '^BASELINE_ROOT_FULL_TESTS=' scripts/verify-baselines.env | tail -1`
