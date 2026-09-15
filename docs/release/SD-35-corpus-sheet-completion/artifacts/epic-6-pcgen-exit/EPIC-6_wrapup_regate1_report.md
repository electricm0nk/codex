# Epic 6 — PCGen exit — wrap-up gate report, **re-gate 1**

> ## SUPERSEDED — do not read this report's verdict as current
>
> Superseded by `EPIC-6_wrapup_gate_report_regate5.md`, the live one. Stamped by the Epic 6
> wrap-up correction cycle `AT-35-E6-WRAPUP-FIX2`, which found this report still unfolded in its
> author's worktree (`.claude/worktrees/wf_291be5c8-5f3-91`) and folded it here so the record is
> complete rather than dying with the worktree.
>
> It describes an earlier tree and an earlier red list. The current verdict at `4b69eb7aab` was
> 48 of 49 stages PASS with one red, `shape-engine-boundary-selftest`, which
> `AT-35-E6-WRAPUP-FIX2` fixed. Kept, not deleted: these are the record of how the red list was
> driven down.


Authored by the **isolated read-only worker** (`workflow-instruction.md §2` worker split,
`decisions.md §3`). This worker **pushed nothing and committed nothing**. Everything below was run
in its own git worktree at
`/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-91`, hard-reset to
`origin/tranche/15`, with its own `CARGO_TARGET_DIR=/home/ubuntu/workspace/cargo-sd35-e6-regate1`
and `CARGO_INCREMENTAL=0`.

This is the **re-run** of the Epic 6 wrap-up gate. The first gate (`EPIC-6_wrapup_gate_report.md`,
tree `77e8d3919a`) came back `FAIL` on 4 of 49 stages; `AT-35-E6-WRAPUP-FIX` cycle 1 (`952b313bbb`
+ `c8ce4f7f12`) repaired them. This report establishes whether the repair held.

- **Tree under test:** `origin/tranche/15` at **`24084e1782`**
  ("docs(sd35): stamp AT-35-E6-WRAPUP-FIX cycle 1's landed SHAs in its receipt, kanban and progress rows")
- **Gate run:** `scripts/verify.sh` — **every stage, no `--only`**
- **Wall time:** **8,496 s (2 h 21 m 36 s)**, 2026-09-14T03:34:32Z → 2026-09-14T05:56:08Z
  (`python3 -c "print(int(open('.../end.ts').read())-int(open('.../start.ts').read()))"` over the
  wrapper's own timestamps)
- **Gate log (this worker's):** `/home/ubuntu/workspace/sd35-regate-logs/verify-full.log`
- **Per-stage logs (verify.sh's own scratch dir):** `/tmp/codex-verify-G8CpUl/`
- **Result:** **`RESULT: PASS`**, `VERIFY_EXIT=0`. **49 of 49 stages passed, 0 failed.**

---

## 0. The full gate, once — stage table

Every row below is the literal `PASS`/`FAIL` line `scripts/verify.sh` printed. Re-derive with
`grep -E '^ +(PASS|FAIL) ' /home/ubuntu/workspace/sd35-regate-logs/verify-full.log`.

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
| 9 | **site-dashboard-check** | **PASS** | `site/dashboard/PF1e-dashboard.json` is current — **was RED in gate 1** |
| 10 | site-dashboard-pi-gate | PASS | 22 files scanned against 1,612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 files scanned against 1,612 declared-PI names, zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00%, i.e. 49,438 of the 49,438-unit inventory |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 corpus_sha=`7f818006e3…` |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 **not_held_by_engine=0** citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=**344** violations=0 (was 336 in gate 1) |
| 26 | **figure-provenance** | **PASS** | files_checked=**274** figures_examined=**614** **violations=0** — **was RED (violations=2) in gate 1** |
| 27 | pcgen-residue-gate | PASS | **live_files=0 live_hits=0 verdict=PASS** |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3,390 passed |
| 38 | root-full | PASS | 8,919 passed across 419 suites, **all 365 `tests/*.rs` suites executed** |
| 39 | **desktop** | **PASS** | **570 passed** against the corrected floor of 570 — **was RED in gate 1** (570 measured against a stale 576 floor) |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48,706 records examined of 51,523 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 42 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 (117.7 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27681; defects `wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249`; traps=407 — all at registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` (node_modules was absent in this fresh worktree) |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | **clippy** | **PASS** | **root:0 desktop:0** warnings, 0 errors — **was RED (desktop:1) in gate 1** |
| 49 | class-dump | PASS | 31/31 computing |

**Red stages: none.** All four of gate 1's reds are green at `24084e1782`. `AT-35-E6-WRAPUP-FIX`
cycle 1 is confirmed effective; no further wrap-up correction cycle is owed, and Epic 7's second
cycle is unblocked on this axis.

### 0a. This worker's own base was wrong before the gate ran

Before any measurement, this worktree was found to be cut from the **v0.8 / `tranche/14-ui`
lineage**, not `tranche/15`: **317 commits behind** the `tranche/15` tip and 47 ahead on unrelated
PR merges (`git rev-list --left-right --count origin/tranche/15...HEAD` → `317 47`;
`git merge-base --is-ancestor origin/tranche/15 HEAD` → NO). On that base
`scripts/pcgen_residue_gate.py` **does not exist**. Left uncorrected, the mandated residue check
would have reported "No such file" and every figure in this report would have described a tree
nobody is shipping.

Cleared by `git reset --hard FETCH_HEAD` onto `origin/tranche/15` @ `24084e1782`; tree clean
(`git status --porcelain` → 0 lines) before the gate started. Logged as incident
`1789365427951-at-35-e6-wrapup-regate-1-b600e3`, recurrence key `wrong-base-worktree`, marked
`silent` — **the 9th recurrence**. See §2 for the control this demands.

---

## 1. Retrospective read — `retro.py summary --since 2026-09-08 --json`

Command: `python3 scripts/retro.py summary --since 2026-09-08 --json`
Window: `2026-09-08T00:00:00+00:00` → open. Log: `docs/retro/events`, **0 invalid lines, 0 problems**.

### 1.1 Counts

| Metric | Value |
|---|---|
| Events, total | **576** |
| — correction | **199** |
| — deferral | **84** |
| — incident | **60** (578 minutes lost) |
| — verification | **181** |
| — note / resolution / rework | 31 / 16 / **5** |
| Deferrals open / resolved | **83 / 1** |
| Near-misses | 0 recorded, 0 escaped |
| verify.sh runs / failed | **181 / 22** — fail rate **12.15%** |
| Corrections attributable to Epic 6 actors | **112 of 199** (58.8%) |
| Epic 6 events, 11 actor ids | **214** |

`caught_before` for corrections: implementation 26, merge 14, release 5, brief 3, **"nothing
(already shipped)" 1**. The single already-shipped correction is the most expensive row in the
window and is the only one with no gate in front of it.

`by_failing_stage` across the 22 failed verify runs: **figure-provenance 14**,
site-dashboard-check 8, reachability-audit-selftest 2, shape-engine-boundary-selftest 2,
clippy 1, denominator-gate 1, desktop 1, pcgen-residue-gate 1. figure-provenance alone is 64% of
the red stages in this window.

### 1.2 Incident keys firing 3+ times — each needs a control or a named escalation

`§10 step 1` is binding here: 3+ firings is a missing mechanism, not bad luck (AGENTS.md rule 8).

| Key | Firings | Disposition |
|---|---|---|
| `wrong-base-worktree` | **8** in the window, **9 with this run's** | **ESCALATION, named.** The 8th firing already recorded that the prepared one-line fix — a `.worktrees/` line in `.gitignore` — is **outside the epic's granted file-touch set** (`workflow-instruction.md §3`), and that AGENTS.md rule 4 forbids taking it without an operator ruling. That is only half the defect: `.gitignore` addresses the *untracked-dir* symptom, not the *stale-base* one. The mechanical control that actually fits: **make base freshness a gate, not a habit** — every dispatched cycle and gate worker runs `git fetch origin <branch> && git merge-base --is-ancestor origin/<branch> HEAD` as its first command and **refuses to measure** if it returns non-zero. This is 2 lines and needs no new write scope beyond the dispatch template. **Asked of the operator: grant the `.gitignore` line and adopt the ancestry precondition in the dispatch template.** |
| `disk-full` | **6** | Partly controlled: `verify.sh` now has a `preflight-disk` stage (PASS this run, 53% of 1.5 T used, 688 G free) and emits `used_percent` automatically. The residual gap is that the preflight guards *verify.sh*, not the per-agent `CARGO_TARGET_DIR` allocations that actually fill the disk. **Control to adopt:** `.reclaim-claim` files already exist; the missing half is a reaper that deletes a claim dir whose claiming PID is gone. Named, not yet built. |
| `site-dashboard-json-stale-after-inventory-move` | **6** | **Controlled.** `site-dashboard-pin` (stage 8) and `site-dashboard-check` (stage 9) are both in the full gate and both green here. The mechanism exists; the 6 firings are pre-control. No further action. |
| `epic-wrapup-gate-red` | **5** | **Controlled by design, and this report is the evidence.** `§10 step 0`'s "red is fixed in a wrap-up correction cycle before the next epic's *second* cycle" is the mechanism; it ran once for Epic 6 and this re-gate closes the loop 49/49. Not a defect class — it is the protocol working. |
| `untracked-worktrees-dir-on-shared-checkout` | **5** | Same blocked one-line fix as `wrong-base-worktree` (the `.gitignore` entry). Folded into that escalation. |
| `figure-provenance-command-on-next-line` | **4** | **Control exists and is enforced** — `figure-provenance` is stage 26 of the gate and is the single most frequent red (14 of 22 failed runs), which is exactly what an effective gate looks like: it catches the thing before merge rather than after. The 4 firings are authoring-time friction, not escapes. **Improvement named, not built:** the checker reports the violating file and figure but not the accepted row shape; a `--explain` example in the failure text would remove most of the 4. |
| `retro-actor-lost-between-bash-calls` | **3** | **Mechanical control, adoptable today:** `RETRO_ACTOR` is exported per-`Bash`-call in this session precisely because the shell does not persist between calls. The durable fix is for `retro.py` to fall back to a `.retro-actor` file written once at cycle start when `$RETRO_ACTOR` is unset, instead of silently recording `actor_source: env`-less events. Named for the closure epic. |

Two further incident classes are worth naming even below the threshold, because both are
structural rather than accidental:

- `unfolded-gate-artifacts-die-with-the-worktree` (1, raised by `AT-35-E6-WRAPUP-FIX`): the §10
  step-2 sweep is **destructive with no fold-first control**, and 6 of 8 Epic 6 worktrees were
  holding gate reports and retro logs that had never been folded.
- **New this run:** `epic-wrapup-worktree-sweep-unreachable-from-isolated-worker`
  (`1789365436738-at-35-e6-wrapup-regate-1-99b612`) — see §2.

### 1.3 Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**None — and the reason is structural, not evasive.**

Re-derived over all **63** Epic 6 cycle receipts by parsing the mechanical row each one carries
(`closed=… relabeled=… rust_lines_changed=… ratio=… builds_recorded=… pcgen_live_files=…`):

- receipts carrying a mechanical row: **63 of 63**
- cycles with `closed > 0`: **0**
- distinct `ratio` values across all 63: **`{n/a}`** — every one of them, a division by zero
- total `rust_lines_changed` across Epic 6: **46,277**
- total `builds_recorded`: **85**
- `pcgen_live_files`: **254 → 0**

Re-derive:

```
cd docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit
python3 - <<'EOF'
import re,glob
rows=[]
for f in sorted(glob.glob('*_receipt.md')):
    m=re.search(r'closed=(\d+) relabeled=(\d+) rust_lines_changed=(\d+) ratio=(\S+) '
                r'builds_recorded=(\d+) pcgen_live_files=(\d+)', open(f).read())
    if m: rows.append((f,)+m.groups())
print(len(rows), sum(int(r[3]) for r in rows), sorted({r[4] for r in rows}))
EOF
```

**What the 46,277 lines bought.** `decisions.md §4`'s ratio test assumes a content epic, where
Rust lines are spent to close corpus units. Epic 6 closes **zero** units by design — it is the
PCGen exit, and its unit of progress is `pcgen_live_files`, not `units_closed`. Stating
"ratio = n/a" and stopping would be a carve-out, so the honest accounting is the substitute
denominator:

- **254 → 0 live PCGen-reading files** across `src/rules_core` (minus `cache_gen`),
  `src/saved_character`, `src/campaign`, `src/homebrew_authoring` and `apps/desktop`.
  At 46,277 Rust lines that is **≈182 lines per file evicted** — the cost of replacing a token
  read with a converted, engine-held value rather than deleting the read and leaving a hole.
- **The converter, parser, generators and oracle harness are intact** (`baseline_files=260`,
  `baseline_hits=12736` — unchanged from the WRAPUP-FIX cycle's start-and-end readings). The
  standing ruling is that these are KEPT for Starfinder; the residue gate's two-population design
  (`live_*` must be 0, `baseline_*` must *not* be driven to 0) is what makes "we removed PCGen"
  and "we deleted the converter" distinguishable.
- **Nothing on the sheet moved:** `sheet-rules-check` reads `records=49438 converted=49296
  refused=142`, `corpus-sweep` reports 0 findings over 413,314 compared tokens, and
  `shape-engine-boundary` reports `not_held_by_engine=0`. 46,277 lines of live-side rewrite with
  a byte-identical rendered result is the correct outcome for an exit epic.

The largest single cycles by Rust volume — `AT-35-E6-003-SWEEP` cycle 3 (4,020), `AT-35-E6-001`
cycle 4 (2,520), cycle 2 (2,173), `AT-35-E6-003` cycle 12 (1,978) — are all mass token-eviction
passes, and each carries its own `pcgen_live_files` reading in its receipt, which is the figure
that makes them auditable.

---

## 2. Worktree sweep — **NOT PERFORMED; ESCALATED**

`df -h /`: **765 G used of the 1.5 T `/dev/sda1`, 688 G available.** No disk pressure.

`git worktree list` (from this worktree):

| Worktree | HEAD | Note |
|---|---|---|
| `/home/ubuntu/workspace/repos/codex` | `24084e1782` | shared checkout, `[tranche/15]` |
| `.claude/worktrees/wf_291be5c8-5f3-2` | `4e321d2c6c` | 713 M |
| `…-3` | `986084c5a4` | 713 M |
| `…-4` | `942c8d3ae5` | 714 M |
| `…-5` | `3c43cf0531` | 714 M |
| `…-7` | `928272a444` | 606 M |
| `…-14` | `a542652c5e` | 859 M |
| `…-15` | `8cc4ea1516` | 858 M |
| `…-19` | `b78b076b2e` | 888 M |
| `…-21` | `07e29075b4` | 867 M |
| `…-24` | `cdcfc897ea` | 867 M |
| `…-30` | `00d0611e87` | 869 M |
| `…-89` | `77e8d3919a` | 911 M — **Epic 6's first wrap-up gate worker** |
| `…-91` | `24084e1782` | 766 M — **this worker, `locked`** |

**Why nothing was pruned.** The harness enforces worktree isolation on this worker: every git
invocation naming a path outside `…-91` is refused, including `git -C <sibling> status`,
`git -C <sibling> rev-list`, and `git worktree remove`. I therefore **cannot establish** whether
`…-89` (or any sibling) is dirty or carries unmerged commits, and the standing rule is explicit —
never remove a worktree carrying unmerged commits, and `AT-35-E6-WRAPUP-FIX` already recorded that
6 of 8 Epic 6 worktrees were holding **unfolded** gate reports and retro logs when it looked.
Pruning blind would destroy exactly that class of artifact. Refusing is the correct disposition,
not a shortfall.

**Escalation (incident `1789365436738-at-35-e6-wrapup-regate-1-99b612`, key
`epic-wrapup-worktree-sweep-unreachable-from-isolated-worker`):** `workflow-instruction.md §10
step 2` assigns the sweep to the one role structurally unable to perform it. **Asked of the
operator: move §10 step 2 off the isolated read-only worker and onto the LOCAL wrap-up correction
cycle**, which already runs on the shared checkout, already folds worktree artifacts, and can
therefore do the fold-before-remove that the destructive step requires. The isolated worker should
keep only the *reporting* half — `df -h` plus the `git worktree list` inventory above.

Carried to that cycle, in order: fold anything unfolded from `…-89` (Epic 6's first gate worker),
then remove `…-89`; `…-91` is `locked` and holds this report — remove it only after the
orchestrator has taken the file. The remaining 11 worktrees are **not Epic 6's** and are out of
this sweep's scope per `§10 step 2` ("this epic's worktrees only").

---

## 3. Pull request

**None.** `workflow-instruction.md §10 step 3`: no PR at an epic wrap-up.

---

## 4. PCGen residue gate — literal output

`python3 scripts/pcgen_residue_gate.py --check` (exit 0), run at `24084e1782`:

```
pattern raw_tokens files=0 hits=0
pattern raw_bonus_chains files=0 hits=0
pattern PcgenFormulaEvaluator files=0 hits=0
pattern render_pcgen_desc files=0 hits=0
pattern bonus_stack_reader files=0 hits=0
pattern pre_tokens files=0 hits=0
pattern BONUS: files=0 hits=0
pattern DEFINE: files=0 hits=0
pattern PRE[A-Z]+: files=0 hits=0
pattern SAB: files=0 hits=0
pattern DESC: files=0 hits=0
pattern %CHOICE files=0 hits=0
pattern %LIST files=0 hits=0
pattern TYPE= files=0 hits=0
pattern pcgen_import files=0 hits=0
root src/rules_core files=0 hits=0
root src/saved_character files=0 hits=0
root src/campaign files=0 hits=0
root src/homebrew_authoring files=0 hits=0
root apps/desktop files=0 hits=0
identifier_files=0 identifier_hits=0
live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS
```

**Residue line:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`

The live side is at zero and the baseline side is **unmoved** at 260 files / 12,736 hits — the
converter, parser, generators and oracle harness were kept, as the standing ruling requires.

---

## 5. Verdict

**`complete`.** 49 of 49 `scripts/verify.sh` stages passed at `origin/tranche/15` @ `24084e1782`
in 2 h 21 m 36 s. No red stage; no wrap-up correction cycle is owed for Epic 6.

Two items leave this gate as **named escalations requiring an operator ruling**, neither of which
blocks Epic 7:

1. the `.gitignore` `.worktrees/` line plus a **base-ancestry precondition in the dispatch
   template**, for `wrong-base-worktree` (now 9 firings) and
   `untracked-worktrees-dir-on-shared-checkout` (5);
2. reassigning `§10 step 2` (worktree sweep) from the isolated read-only worker, which the harness
   forbids from touching sibling worktrees, to the local wrap-up correction cycle.

This worker committed nothing and pushed nothing.

### To fold (exactly two untracked files, `git status --porcelain` in this worktree)

| File | Contents |
|---|---|
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate1_report.md` | this report |
| `docs/retro/events/at-35-e6-wrapup-regate-1.jsonl` | **3** events: `1789365367936-…-94a117` (verification, `mode: full`, `result: PASS`, `duration_seconds: 8495`, emitted by `verify.sh` itself), `1789365427951-…-b600e3` (incident, `wrong-base-worktree`), `1789365436738-…-99b612` (incident, `epic-wrapup-worktree-sweep-unreachable-from-isolated-worker`) |

Both are also copied, outside the worktree so they survive its cleanup, to
`/home/ubuntu/workspace/sd35-regate-logs/handoff/`.

This report was itself run through the two prose gates before hand-off, so folding it cannot turn
them red: `python3 scripts/denominator_gate.py --check` → `files_checked=345 violations=0`, and
`python3 scripts/denominator_gate.py --check-provenance` → `files_checked=275 figures_examined=614
violations=0`. Both counts are one file higher than the gate run's, which is this file.
