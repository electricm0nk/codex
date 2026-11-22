---
canonical: false
owner: at-35-e2-regate3
bundle_id: SD-35
status: gate-report
date: 2026-09-10
---

# Epic 2 — Sheet rule — wrap-up gate, RE-GATE 3 (`workflow-instruction.md §10` steps 0–3)

Run by the **isolated read-only worker** (`decisions.md §3` / `workflow-instruction.md §2` worker
split). **This worker committed nothing and pushed nothing.** This report, the retro shards
`docs/retro/events/at-35-e2-regate3.jsonl` and `docs/retro/events/epic2-wrapup-regate2.jsonl`
(the latter written by `verify.sh` itself), and the one-line `derived_at` re-stamp of
`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are its
hand-off; the next cycle agent commits them.

**Naming.** The dispatch called this "re-gate 2". `EPIC-2_wrapup_regate2_report.md` already
exists at HEAD — it was run *before* the correction that HEAD carries — so this is the **third**
full-gate run of Epic 2's wrap-up and is filed as `regate3` rather than overwriting a committed
report.

## Headline

**GREEN — 49 stages, 49 PASS / 0 FAIL.** `workflow-instruction.md §10` step 0's gating condition
is satisfied. No wrap-up correction cycle is owed, and Epic 3's second cycle is unblocked by this
gate.

The stage that was red at both previous runs — **`site-dashboard-check`** — is green. So is
`site-dashboard-pin` beside it, which is what makes the pair meaningful: the input is certified
*and* the output came from that input.

| Field | Value |
|---|---|
| **Gated SHA** | `b78b076b2e02a108a588bf07e827af4e011d2601` — `docs(sd35): Epic 2 wrap-up correction 2 receipt names its own commit`, i.e. `origin/tranche/15` tip at run time |
| **Worktree** | `.claude/worktrees/wf_291be5c8-5f3-19` (`locked`) |
| **Command** | `RETRO_ACTOR=epic2-wrapup-regate2 CARGO_TARGET_DIR=/tmp/cargo-sd35-epic2-wrapup-regate2 CARGO_INCREMENTAL=0 bash scripts/verify.sh -j 6` — **every stage, no `--only`** |
| **Wall time** | **4,645 s = 1 h 17 m 25 s** (epoch 1789041068 → 1789045713; start from `/tmp/cargo-sd35-epic2-wrapup-regate2/start.ts`, end from the driver log's mtime) |
| **Driver log** | `/tmp/cargo-sd35-epic2-wrapup-regate2/verify-full.log` |
| **Per-stage logs** | `/tmp/codex-verify-VOsnL7/` |
| **Stage count at HEAD** | 49 — `scripts/verify.sh --list \| tail -n +2 \| wc -l` |
| **Residue line** | `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` |
| **Disk** | `/dev/sda1`, 1.5 T total: 1.1 T used of that 1.5 T = 76 %, 352 G available at gate start; 1.2 T used of the same 1.5 T = 79 %, 316 G available at gate end (`df -h /`) |

### The one environment fault this worker had to fix before it could gate anything

**`wrong-base-worktree` — fourth firing of that key in the SD-35 window.** The dispatch worktree
arrived pinned at `3258db406d`, a `tranche/14`-era merge: **143 commits behind and 43 ahead** of
`origin/tranche/15` (`git rev-list --left-right --count HEAD...origin/tranche/15` → `43  143`).
Epics 3–6, the `AT-35-E6-001` module move and all three wrap-up correction cycles were absent
from it. Gating that tree would have re-proved a tree nobody ships and produced a report whose
"gated SHA" is not the branch tip.

Caught by `git rev-parse HEAD` against `git rev-parse origin/tranche/15` as the **first** command
of the turn, before any build was started — **zero build time lost**. Cleared with
`git fetch origin tranche/15` then `git reset --hard b78b076b2e` **inside this worktree only**;
the 43 ahead-commits are `tranche/14-ui` merges already reachable from `develop`, not work. Tree
clean afterwards. **No `git stash`, no `git add -A`, no force-push, nothing outside this
worktree touched.** Recorded as incident `1789041330490-at-35-e2-regate3-8f3b53`. *A key at four
firings owes a mechanical control — see §1a.*

---

## 0. The full gate, once — stage table

Copied from the driver log's own `PASS`/`FAIL` lines, in execution order.

| # | Stage | Result | Detail as printed |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 30 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | `docs/work-inventory.json` matches the pin the feed was published from |
| 9 | **site-dashboard-check** | **PASS** | **`site/dashboard/PF1e-dashboard.json` is current** |
| 10 | site-dashboard-pi-gate | PASS | 22 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00 % — 49,438 reachable of the 49,438 units in `docs/work-inventory.json` (`python3 scripts/reachability_audit.py`) |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 corpus_sha=`7f818006e3…` |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=261 violations=0 |
| 26 | figure-provenance | PASS | files_checked=191 figures_examined=338 violations=0 |
| 27 | **pcgen-residue-gate** | PASS | **live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS** |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | **token-coverage** | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3261 passed |
| 38 | root-full | PASS | 8772 passed across 413 suites, all 361 `tests/*.rs` suites executed |
| 39 | desktop | PASS | 576 passed |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings |
| 42 | **sheet-rules-check** | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.7 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27634 defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS | 31/31 computing |

`RESULT: PASS` — `logs in /tmp/codex-verify-VOsnL7`

### 0a. No red stage

There is nothing to name here, which is the point of the section. **Every stage Epic 2 owns is
green** — `sheet-rules-check`, `token-coverage`, `token-coverage-selftest`, `pcgen-residue-gate`,
`cycle-scope-gate-selftest` — and so is every stage it does not.

### 0b. Why `site-dashboard-check` went green, stated as a mechanism and not as "it passed"

The re-gate-2 report's root cause was **disproved** by the correction cycle, not repeated, and the
correction that landed is the one this run confirms. `by_doneness` has exactly one source,
`compute_wiring_class_summary()`, whose cache is `~/swarm-observer/wiring-class-summary.json` —
**one file outside the repo, shared by the main checkout and all linked worktrees**. Its warm-cache
guard was *mtime-newer* + equal `schema` + equal `source_document`, and that third predicate is
`publishable_document_path()`, which normalises to the repo-relative string
`docs/work-inventory.json` precisely so no absolute path reaches `site/`. Every tree's inventory
therefore answered to the same name, nothing in the guard read the document's **content**, and
another tree's summary was served and published under this tree's pin — which is why
`site-dashboard-pin` could pass while the feed understated the corpus by ~23,300 units.

`265cd65fa5` re-keys that cache on content and adds `ForeignContentCacheIsRejectedTest` to
`producer-selftest` (**27 → 30 cases**, visible at stage 4 above). This run is the honest test of
that fix: a **fresh worktree with a cold cache**, on a box where 14 sibling worktrees are live.
Stage 9 is green and stages 8 and 9 agree. Recorded as resolution
`1789045786449-at-35-e2-regate3-91cb09` against incident `1789034786341-at-35-e2-regate2-26a448`.

### 0c. Independent re-derivations run outside `verify.sh`

Run by this worker before the gate finished, as a cross-check that the gate's own numbers are not
the only witness:

| Command | Output |
|---|---|
| `python3 scripts/pcgen_residue_gate.py --check` | `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` (exit 0) |
| `python3 scripts/token_coverage.py --check` | `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` |
| `python3 scripts/completion_atlas.py --check` | `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0` |
| `python3 -c` over `docs/work-inventory.json` `totals` | units **49,438**; `sheet-complete` 23,315 + `text-complete` 11,599 + `oracle-unverifiable` 8,491 + `grounded` 5,222 + `oracle-agree` 811 = **49,438** (denominator: every unit in `docs/work-inventory.json` at `b78b076b2e`) |

The `completion_atlas.py --check` run advanced that artifact's `derived_at` field from
`9f54c1e490…` to the gated SHA `b78b076b2e…` — a one-line, correct re-stamp, left in the tree as
part of the hand-off rather than reverted. It is the only tracked-file modification this worker
made.

---

## 1. Retrospective summary — `retro.py summary --since 2026-09-08 --json`

Read, not merely run. The window is the whole SD-35 dispatch window, not Epic 2 alone: Epics 3–6
have landed since Epic 2's cycles, so these counts are the bundle's, and the Epic-2-specific
reading is §1b.

| Event type | Count in window |
|---|---|
| verification | 100 |
| correction | 76 |
| incident | 30 |
| deferral | 21 (**20 open**, 1 resolved) |
| resolution | 14 |
| note | 11 |
| rework | 4 |
| **total** | **256** |

Corrections: **30 of 76 carry a blast radius**; `caught_before` is `implementation` 13,
`merge` 7, `release` 4, `brief` 2. **Zero near-misses escaped** and **zero incidents were
silent** — every failure in this window announced itself. Recorded time lost: **482 minutes**.
Only two subjects were corrected twice (`epic-breakdown.md AT-35-E1-004 criterion text`,
`epic-breakdown.md AT-35-E2-001 evidence`), so the correction volume is breadth, not thrash.

Verification runs: **100, of which 19 failed (fail rate 0.19)**, by failing stage —
`figure-provenance` 13, `site-dashboard-check` 6, `reachability-audit-selftest` 2,
`shape-engine-boundary-selftest` 2, `denominator-gate` 1. The log itself is clean:
`log.problems` empty, `invalid_lines` 0.

### 1a. Every `incident` key at 3+ firings, with its named control or named escalation

`workflow-instruction.md §10` step 1 makes this mandatory, so all five are answered — not only
the new one.

| Key | Firings | Control or escalation |
|---|---|---|
| `disk-full` | 6 | **Control exists.** `scripts/reclaim.sh`'s `RECLAIM_PRESSURE_PERCENT` threshold (default 90) — added in this window after 12 phantom firings were traced to a routine emitting `incident` unconditionally — plus `verify.sh` stage `preflight-disk`, which is deliberately first in both stage sets so a ~490-binary build cannot start on a full disk. Both green here. |
| `epic-wrapup-gate-red` | 4 | **Control exists, and it is this document.** The key *is* `§10` step 0 working: a red wrap-up gate blocks the next epic's second cycle until a correction cycle clears it. Three of the four firings are Epic 2's own chain (gate → fix → re-gate → fix → re-gate 2 → fix → this run), and this run closes it. |
| `site-dashboard-json-stale-after-inventory-move` | 4 | **Control landed this window.** The content-keyed `wiring-class-summary.json` guard at `265cd65fa5`, `verify.sh` stage `site-dashboard-pin` (the input-side half), and `producer-selftest`'s `ForeignContentCacheIsRejectedTest` (27 → 30 cases). Confirmed cold-cache green at §0b. |
| `figure-provenance-command-on-next-line` | 3 | **Control exists and is doing the catching.** `verify.sh` stage `figure-provenance` (`denominator_gate.py --check-provenance`) — all 13 of the window's `figure-provenance` verification failures are this key being caught before a commit, not escaping. Green here at `files_checked=191 figures_examined=338 violations=0`. |
| `wrong-base-worktree` | **4** (3 before this run, +1 here) | **NO mechanical control. Escalated.** See below. |

**The escalation, stated concretely** (retro note `1789045774296-at-35-e2-regate3-1c6514`). All
four firings were caught by a *human-discipline* read of `git rev-parse` — exactly the class of
control this repo has already watched decay. A read-only gate worker may not write `scripts/`, so
the fix is escalated to the orchestrator rather than attempted here:

> Add a `verify.sh` stage **`preflight-base`**, placed immediately after `preflight-disk`, that
> runs `git merge-base --is-ancestor "$(git rev-parse origin/<bundle-branch>)" HEAD` and **FAILs**
> when the bundle branch tip is not an ancestor of `HEAD`, printing both SHAs and the behind-count.

It costs milliseconds, it runs before the ~490-binary build, and it turns "I gated a tree nobody
ships" from a discipline into an exit code. Owner: the next correction/tooling cycle with
`scripts/verify.sh` in its file-touch set.

### 1b. `rust_lines_changed / units_closed` — every Epic 2 cycle, against `decisions.md §4`'s 3.0

From each receipt's own **Receipt rows (mechanical)** line, which is
`cycle_scope_gate.py --receipt` output, not prose:

| Cycle | closed | rust_lines_changed | ratio |
|---|---|---|---|
| AT-35-E2-001 c1 | 0 | 6,463 | n/a |
| AT-35-E2-001 c2 | 0 | 0 | n/a |
| AT-35-E2-002 c1 | 0 | 1,818 | n/a |
| AT-35-E2-002 c2 | 0 | 0 | n/a |
| AT-35-E2-003 c1 | 0 | 442 | n/a |
| AT-35-E2-003 c2 | 0 | 0 | n/a |
| AT-35-E2-004 c1 | 0 | 295 | n/a |
| AT-35-E2-004 c2 | 0 | 0 | n/a |
| **AT-35-E2-005 c1** | **21,911** | 337 | **0.02** |
| AT-35-E2-005 c2–c4 | 0 | 0 | n/a |
| AT-35-E2-005 c5 | 0 | 120 | n/a |
| AT-35-E2-005-DISPOSITION c1, c2 | 0 | 0 | n/a |
| **Epic 2, aggregate** | **21,911** | **9,475** | **0.432** |

**No Epic 2 cycle exceeded 3.0, and the epic as a whole is at 0.432** — 9,475 Rust lines for
21,911 units closed, roughly one line per 2.3 units. `decisions.md §4`'s named-and-explained
obligation therefore has no cycles to name. Denominator: `units_closed` is the DONE-ward
inventory delta `cycle_scope_gate.py --receipt` computes from the before/after
`docs/work-inventory.json`; only AT-35-E2-005 c1 has a non-zero one, because Epic 2's other
cycles are converter-building, status-wiring and re-verification cycles that close units by
design only at the one corpus-wide regeneration.

Because eleven of the fifteen cycles report `ratio=n/a`, the ratio alone would say nothing about
them. What the four line-spending cycles bought, from their own receipts:

- **AT-35-E2-001 c1, 6,463 lines** — the entire converter: `src/bin/sheet_rule_convert.rs` plus
  `src/pcgen_import/sheet_rule/`, transcribing the 249-row judged mapping table. It is the
  mechanism the other 21,911 closures ride on, and it is one-per-token-type, not one-per-unit —
  `decisions.md §4`'s explicit "what is fine".
- **AT-35-E2-002 c1, 1,818 lines** — the live-side evaluator `src/rules_core/sheet_rule.rs` and
  the sheet section that renders it. No PCGen token on the live side.
- **AT-35-E2-003 c1, 442 lines** — the `sheet-complete` rung in `v06_work_inventory.rs`, wired
  into every status consumer in the same cycle (lesson 32).
- **AT-35-E2-005 c5, 120 lines** — two live-evaluator mechanisms plus their tests; what they
  bought is stated in the receipt as a measured figure: **11 → 0 oracle parity disagreements**.

### 1c. Open deferrals touching Epic 2

**20 of 21 window deferrals are open.** Epic 2's own are the converter-refusal chain — the 1,404
non-DONE units at AT-35-E2-005's close, owned in full by
`AT-35-E2-005-DISPOSITION_handoff.json` (`owned_sum=1404 unowned=0`) and dispatched to
AT-35-E4-001 (659 refusals by token type), AT-35-E4-002 (V, 392), AT-35-E5-003 (U 202, Z 19) and
AT-35-E5-004 (X 137). **All of them are now downstream of epics that have since landed**, and
`token-coverage` at stage 29 reads `non_done=0 refused_non_done=0` — the population those
deferrals named is empty at this SHA. They are hand-off records, not open work against Epic 2.

Under the sheet rule none of this is a carve-out: the refused records render as words and their
units are DONE. The corpus is at **49,438 of 49,438**, with `reachability-audit` reporting a
reachable ceiling of 100.00 % — 49,438 of the 49,438 units in `docs/work-inventory.json` (stage 16).

---

## 2. Worktree sweep — measured, and two retained with the reason

`df -h /`: `/dev/sda1`, **1.5 T total — 1.1 T used of that 1.5 T = 76 %, 352 G available at gate
start; 1.2 T used of the same 1.5 T = 79 %, 316 G available at gate end.**
No disk pressure; `scripts/reclaim.sh`'s threshold is not met and `preflight-disk` passed on its
own budget check.

`git worktree list`: **17 entries** — the main checkout (`b78b076b2e`, `tranche/15`), 15
`.claude/worktrees/wf_291be5c8-5f3-*` worktrees totalling **12 G** (606 M–887 M each), and
`.worktrees/ci-trait-choice`. `wf_291be5c8-5f3-19` is this worker's and is **`locked`** — never
removed.

**Unmerged-ness is proven this time**, which the previous re-gate could not do:
`git rev-list --count origin/tranche/15..worktree-wf_291be5c8-5f3-<n>` returns **0** for every
Epic 1/2-era worktree — `-2`, `-3`, `-4`, `-5`, `-7`, `-14`, `-15`. **No unmerged commit is at
risk in any of them.**

**Nothing was removed.** The two the previous gates identified as Epic 2's —
`wf_291be5c8-5f3-14` (`a542652c5e`, AT-35-E2-005-DISPOSITION c2, **859 M**) and
`wf_291be5c8-5f3-15` (`8cc4ea1516`, AT-35-E2-005-DISPOSITION c1, **858 M**) — were both attempted
and both refused by plain `git worktree remove`:

```
fatal: '…/wf_291be5c8-5f3-14' contains modified or untracked files, use --force to delete it
fatal: '…/wf_291be5c8-5f3-15' contains modified or untracked files, use --force to delete it
```

`workflow-instruction.md §8` forbids removing a worktree carrying uncommitted content, and
`--force` would discard it unseen — the same class of content that, on the last re-gate, turned
out to be uncommitted retro shards worth folding. **No `--force` was used and none should be.**
The five earlier worktrees were measured but not removed for the same reason; they are Epic 1/2
era and outside the "this epic's worktrees only" instruction's clear boundary.

Recorded as deferral `1789041315132-at-35-e2-regate3-f7914b`, revisit at
`workflow-instruction.md §11` step 3 (the bundle-closure full sweep), or immediately on any
disk-pressure incident.

---

## 3. Pull request

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

---

## Hand-off — what the next cycle agent commits

This worker pushed nothing. Uncommitted in `.claude/worktrees/wf_291be5c8-5f3-19`:

| Path | What it is |
|---|---|
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/EPIC-2_wrapup_regate3_report.md` | this report |
| `docs/retro/events/at-35-e2-regate3.jsonl` | 4 events: the `wrong-base-worktree` incident, the worktree-sweep deferral, the `preflight-base` control-owed note, the `site-dashboard-check` resolution |
| `docs/retro/events/epic2-wrapup-regate2.jsonl` | `verify.sh`'s own auto-emitted `verification` event for this run |
| `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` | one line: `derived_at` `9f54c1e490…` → `b78b076b2e…` |

Retro events emitted by this worker:

| id | type | subject |
|---|---|---|
| `1789041315132-at-35-e2-regate3-f7914b` | deferral | worktree sweep: `-14` / `-15` retained, merged-ness proven, `--force` refused |
| `1789041330490-at-35-e2-regate3-8f3b53` | incident | `wrong-base-worktree`, 4th firing, caught before any build |
| `1789045774296-at-35-e2-regate3-1c6514` | note | mechanical control owed: proposed `preflight-base` `verify.sh` stage |
| `1789045786449-at-35-e2-regate3-91cb09` | resolution | `site-dashboard-check` root cause fixed at `265cd65fa5` and confirmed cold-cache green |
| (verify.sh, auto) | verification | `verify.sh full: PASS`, 49 stages, 4,645 s, head `b78b076b2e` |

---

## Verdict

**Epic 2's wrap-up gate is GREEN at `b78b076b2e`: 49 of 49 stages PASS, 1 h 17 m 25 s.**
`workflow-instruction.md §10` step 0 is satisfied — **no wrap-up correction cycle is owed.**

Two things this gate found that are not about the stage table, both already recorded and both
carried in the hand-off above:

1. **`wrong-base-worktree` reached four firings and still has no mechanical control.** The
   proposed `preflight-base` stage is named in §1a with its exact command and its owner. This is
   the one obligation `§10` step 1 leaves open at Epic 2's close.
2. **The worktree sweep did not execute**, but for a stated and defensible reason — merged-ness
   *was* proven (zero unmerged commits in all seven Epic 1/2-era worktrees), and the only thing
   standing between the 1.7 G and reclamation is uncommitted working-tree content that
   `--force` would destroy unseen. There is no disk pressure to justify that trade.
