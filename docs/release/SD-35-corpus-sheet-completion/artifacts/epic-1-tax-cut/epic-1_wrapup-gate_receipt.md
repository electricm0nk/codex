# Epic 1 wrap-up gate — Tax cut (`workflow-instruction.md §10` steps 0–3)

Written by the isolated read-only worker (`decisions.md §3` worker split). **This worker
pushed nothing and committed nothing**; the orchestrator hands this file to the next cycle
agent to commit. Retro actor `epic-1-wrapup-gate`.

- **Tree gated:** `928272a4444dd1cd9e5f177a3ba22ab6b3e36730` = `tranche/15` tip at dispatch
  (AT-35-E1-004's docs commit, "Epic 1 at 6 of 6"), checked out in worktree
  `.claude/worktrees/wf_291be5c8-5f3-7` on branch `worktree-wf_291be5c8-5f3-7` — the worktree
  was handed over at `fe5ae6cd4a` (the `develop` merge) and moved to the tranche tip with
  `git reset --hard 928272a444` before anything ran, so the gate measures Epic 1's work.
- **Epic window:** `53296d80f0` (2026-09-07T21:24:13-04:00, the commit under AT-35-E1-001's
  cycle start) → `928272a444`; 15 commits — `git log --oneline 53296d80f0^..928272a444 | wc -l`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (epic wrap-up gate — closes zero units by design, decisions.md §2)`.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — literal last line of `python3 scripts/pcgen_residue_gate.py --check` at `928272a444`, exit 0.
  Equal to AT-35-E1-005's first recording and to every Epic 1 receipt after it — not risen.

## Step 0 — the full gate, once

**Command:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-epic-1-wrapup-gate CARGO_INCREMENTAL=0
VERIFY_LOG_DIR=/tmp/codex-verify-sd35-epic1-wrapup /usr/bin/time -v scripts/verify.sh -j 6 --show-actuals`
— every stage, no `--only`, no `--quick`, from a cold (empty) target dir, in this worker's
worktree at `928272a444`. `-j 6` per `decisions.md §9` L11.

**Result: `RESULT: PASS`, exit 0, 45 of 45 stages PASS, 0 FAIL** (45 = `scripts/verify.sh --list
| tail -n +2 | wc -l` at this HEAD, as `decisions.md §3` predicts after AT-35-E1-001/002/005).

**Wall time: 1:22:04 (4,924 s)** — `/usr/bin/time -v` "Elapsed (wall clock)" in
`/tmp/codex-verify-sd35-epic1-wrapup/verify-time.txt`; started 2026-09-08T02:59:44Z at load
`0.93 3.47 4.56`, ended 04:21:48Z at load `5.49 7.09 6.94` (24 cores, 143 GB); max RSS
2,487,980 kB. **Against `decisions.md §3`'s ~100 min estimate, 82 min — 18 min under.** The
box was otherwise quiet (no sibling lane building); the 1-minute load never exceeded ~7.
`root-full` alone was ~56 min of the 82 (03:25Z → 04:21Z; 408 suites) — the "70-minute" figure
from the launch audit is now 56 on a quiet box after E1-003's fold, a single observation.

**Log path:** `/tmp/codex-verify-sd35-epic1-wrapup/` — `verify-full.log` (the stage table
and summary), `verify-time.txt`, `run-meta.txt` (HEAD, start/end, load, EXIT), and one
`<stage>.log` per stage. `/tmp` is where `verify.sh` puts its logs by default; nothing under
`docs/` was written by the run except the retro event named in the handoff.

**Stage table (45 stages, verbatim from `verify-full.log`; every row PASS):**

| Stage | Result | Measured |
|---|---|---|
| preflight-disk | PASS | disk budget OK |
| preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| oracle-pin-selftest | PASS | 11 passed, 0 failed |
| producer-selftest | PASS | 26 cases passed |
| pi-redaction-selftest | PASS | 49 cases passed |
| provenance-selftest | PASS | 32 cases passed |
| site-dashboard-selftest | PASS | 8 passed, 0 failed |
| site-dashboard-check | PASS | `site/dashboard/PF1e-dashboard.json` is current (14 min, single-threaded producer) |
| site-dashboard-pi-gate | PASS | 21 files scanned against 1,612 declared-PI names, zero leaked |
| build-public-status-selftest | PASS | 37 cases passed |
| site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` current |
| site-public-status-pi-gate | PASS | 31 files scanned against 1,612 declared-PI names, zero leaked |
| site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| reachability-audit-selftest | PASS | 11 cases passed |
| reachability-audit | PASS | reachable ceiling 48,893 of 49,438 units (98.90% of 49,438); 11 dead-end cells, all `ambiguous` wiring class (545 units of 49,438) |
| groundtruth-guard-selftest | PASS | 17 cases passed |
| supersession-gate-selftest | PASS | 16 cases passed |
| shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| shape-coverage-standing-gate | PASS | population=25408 unclassified=0 no_record=0 corpus_sha=7f818006… |
| cycle-scope-gate-selftest | PASS | 51 cases passed (AT-35-E1-001) |
| shape-engine-boundary-selftest | PASS | 15 cases passed (AT-35-E1-002) |
| shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True (AT-35-E1-002) |
| missing-engine-tables | PASS | population=449 kinds=2 citation_failures=0 (AT-35-E1-002) |
| denominator-gate | PASS | files_checked=215 violations=0 (SD-33 + SD-34 + SD-35 defaults, AT-35-E1-004) |
| figure-provenance | PASS | files_checked=145 figures_examined=140 violations=0 (SD-34 + SD-35 defaults, AT-35-E1-004) |
| pcgen-residue-gate | PASS | live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS (AT-35-E1-005) |
| pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| declared-pi-audit | PASS | clean |
| audit-selftest | PASS | 28 passed, 0 failed |
| reclaim-selftest | PASS | 13 passed, 0 failed |
| driver-selftest | PASS | 7 passed, 0 failed |
| corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| root-lib | PASS | 3186 passed (baseline 3186) |
| root-full | PASS | 8656 passed across 408 suites, all 360 `tests/*.rs` suites executed (baselines 8656 / 408 — AT-35-E1-003's floor) |
| desktop | PASS | 573 passed (baseline 573) |
| reach | PASS | 31 passed |
| corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings |
| corpus-trap-audit | PASS | records_examined=27634; defects wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249; traps=407 — all at registered counts |
| supersession-gate | PASS | 116 objects, all clean |
| frontend-install | PASS | `npm ci` (fresh worktree had no `node_modules`) |
| frontend-test | PASS | 100 of 100 files |
| frontend-typecheck | PASS | `tsc --noEmit` clean |
| clippy | PASS | root 0 warnings, desktop 0 warnings, 0 errors |
| class-dump | PASS | 31 of 31 classes computing |

**Baselines confirmed at HEAD (`--show-actuals`):** `BASELINE_ROOT_LIB_TESTS=3186`,
`BASELINE_ROOT_FULL_TESTS=8656`, `BASELINE_ROOT_TEST_BINARIES=408`, `BASELINE_DESKTOP_TESTS=573`,
`BASELINE_CORPUS_LITERAL_RECORDS=48706`, `BASELINE_FRONTEND_TEST_FILES=100`,
`BASELINE_CLIPPY_WARNINGS_ROOT=0`, `BASELINE_CLIPPY_WARNINGS_DESKTOP=0`, `BASELINE_COMPUTED_CLASSES=31`
— every measured actual equals `scripts/verify-baselines.env` at `928272a444`.

**Red stages: none.** No wrap-up correction cycle is needed before Epic 2's second cycle.

**Retro `verification` event (emitted by `verify.sh`, literal line, for the next agent to
append to the shared checkout's `docs/retro/events/epic-1-wrapup-gate.jsonl`):**

```json
{"actor": "epic-1-wrapup-gate", "actor_source": "env", "derived": true, "duration_seconds": 4924, "id": "1788841308344-epic-1-wrapup-gate-ad94e3", "log_dir": "/tmp/codex-verify-sd35-epic1-wrapup", "mode": "full", "origin": "derived", "repo": {"branch": "worktree-wf_291be5c8-5f3-7", "head": "928272a444", "worktree": "wf_291be5c8-5f3-7"}, "result": "PASS", "source": "verify.sh", "stages_passed": ["preflight-disk", "preflight-oracle", "oracle-pin-selftest", "producer-selftest", "pi-redaction-selftest", "provenance-selftest", "site-dashboard-selftest", "site-dashboard-check", "site-dashboard-pi-gate", "build-public-status-selftest", "site-public-status-check", "site-public-status-pi-gate", "site-asset-stamp-check", "reachability-audit-selftest", "reachability-audit", "groundtruth-guard-selftest", "supersession-gate-selftest", "shape-coverage-standing-gate-selftest", "shape-coverage-standing-gate", "cycle-scope-gate-selftest", "shape-engine-boundary-selftest", "shape-engine-boundary", "missing-engine-tables", "denominator-gate", "figure-provenance", "pcgen-residue-gate", "pi-sweep", "declared-pi-audit", "audit-selftest", "reclaim-selftest", "driver-selftest", "corpus-sweep-selftest", "corpus-trap-audit-selftest", "root-lib", "root-full", "desktop", "reach", "corpus-sweep", "corpus-trap-audit", "supersession-gate", "frontend-install", "frontend-test", "frontend-typecheck", "clippy", "class-dump"], "summary": "verify.sh full: PASS", "ts": "2026-09-08T04:21:48Z", "type": "verification"}
```

**Worktree state after the run:** `git status --porcelain` in the worker's worktree lists
exactly two untracked files — this receipt and the retro event shard above. No tracked file
moved: the dashboard and public-status producers found their outputs current.

## Step 1 — `scripts/retro.py summary` for the epic window

`python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00 --json` (the `--since`
flag takes a time, not a SHA; `53296d80f0`'s committer date is the epic start — `git show -s
--format=%cI 53296d80f0`). Generated 2026-09-08T03:00:20Z over 7 shards, 0 invalid lines.
Full JSON kept by the orchestrator; the counts below are read from it.

| Count | Value | Denominator |
|---|---:|---|
| events | 34 | all events in the window, 7 shards |
| `verification` | 14 | of 34 events; 14 runs, **0 failed**, fail rate 0.0 |
| `resolution` | 9 | of 34 |
| `correction` | 6 | of 34; 5 distinct subjects; 2 caught before implementation; 2 with blast radius |
| `incident` | 4 | of 34; 4 distinct recurrence keys, each fired once; `recurring: []`; time lost 2 min |
| `deferral` | 1 | of 34; **1 open, 0 resolved** |
| `rework` / `near_miss` | 0 / 0 | of 34 |
| commits joined | 15 | the epic's 15 commits, 1 author, 2.27 events per commit |

**Incidents — the 3+ rule (`§10` step 1).** No single recurrence key fired 3 or more times.
**But all 4 incidents are one condition under 4 names**: the denominator gate red on the four
pre-launch token-mapping artifacts (`artifacts/epic-2-sheet-rule/token-mapping/{SYNTHESIS,
formula,prereq,prose}.md`, 11 violations, committed pre-launch at `ba6b65e7b9`/`a232e27b03`),
found independently by AT-35-E1-001 (`denominator-gate-red-on-package-prose`), AT-35-E1-006
(`denominator-gate-red-on-untouched-artifacts`), AT-35-E1-003 (`pre-existing-red-gate`) and
AT-35-E1-005 (`package-gate-red-at-cycle-start`). By mechanism that is a 4-times recurrence,
and the key vocabulary hid it from `retro.py`'s own `recurring` clustering. Two dispositions,
both named:

1. **Mechanical control, already landed in this epic:** `scripts/denominator_gate.py`
   `DEFAULT_GLOBS` now includes `docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md`
   (AT-35-E1-004, `2bf452b038`; `grep -n 'SHEET_COMPLETION_BUNDLE_DIR, "artifacts"' scripts/denominator_gate.py`),
   so the `denominator-gate` `verify.sh` stage — run in this gate — fails closed on any
   figure-bearing artifact in this package, not only the root `.md`s the launch audit scanned.
   The violations themselves were cleared by AT-35-E1-002 (`815139fadd`; `violations=11 → 0`,
   no figure changed). Root cause: the pre-launch Fable lanes wrote artifacts under a
   `workflow-instruction.md §1` item 12 check that scanned only the package root
   (`files_checked=16`), so the gate that would have caught them never ran over their output.
2. **Named escalation to the orchestrator:** `retro.py`'s `incident` recurrence keys are
   free text; four lanes reporting one condition chose four keys, which defeats the 3+ rule
   this step exists to apply. Recommend the next gate-building cycle (an Epic 2 wrap-up fix
   cycle, or AT-35-E7-002) add a controlled recurrence-key list to `docs/retro/schema.json`
   and have `retro.py incident` reject an unlisted key, or at minimum have `summary` cluster
   by the `detected_by` command. Not built here — this worker writes nothing to the repo.

**Corrections — what was wrong.** 5 of 6 corrections are against this package's own prose
(`epic-breakdown.md` ×3, `content-unit-inventory.md §6`/`decisions.md §11` ×1,
`workflow-instruction.md §1` item 12 ×1); 1 of 6 is a lane re-deriving its own sizing figure
(AT-35-E1-003: 184 files / 74,932 lines / 2,219 tests). The one repeat subject is
`epic-breakdown.md AT-35-E1-004 criterion text` (×2: `DEFAULT_GLOBS` had already been widened
to SD-34 by AT-34-E1-006; SD-33's receipts cannot pass the provenance check, so that default
excludes SD-33 by a pinned test). This is AGENTS.md rule 9's shape exactly — briefs and package
docs drift, code does not — and every correction was caught by a lane running the cited
command before trusting the sentence. No control beyond the existing rule is proposed; the
rule worked.

**Deferrals — 1 open.** `1788832099943-at-35-e1-003-532b1c` (AT-35-E1-003): 244
`grounding_ref` / doc-comment citations in `src/` (`support_state_matrix.rs`,
`pilot_compute/mod.rs`, `class_feature_grant_consumer.rs`, `level_up/druid.rs`,
`pilot_compute_corpus.rs`) and 4 `docs/architecture/` pages still cite the pre-consolidation
paths `tests/sd18_<x>_widening.rs` / `tests/sd13_<x>_progression.rs` (now
`tests/sd18_widening/<x>.rs` / `tests/sd13_progression/<x>.rs`). The strings are citations no
code resolves; every matrix test's `contains("sd18_<x>_widening")` still holds because the
stem is unchanged. Revisit named: AT-35-E7-003's architecture-docs refresh, or any Epic 6 cycle
that touches those `src/` files. It was outside E1-003's file-touch set (`§3`), so it is a
planned deferral with an owner, not a blocker. **No `deferral` event was needed from this
worker** — the wrap-up scopes zero units.

**The ratio rule (`decisions.md §4`, `§10` step 1).** Every Epic 1 cycle closed **0 units by
design** (all six receipts: `closed=0 relabeled=0`), so `rust_lines_changed / units_closed` is
undefined (`ratio=n/a`) for all six — none "exceeded 3.0" because none has a value. The one
cycle with non-zero Rust lines is named anyway, as the rule intends:

| Cycle | `rust_lines_changed` | `units_closed` | What the lines bought |
|---|---:|---:|---|
| AT-35-E1-003 (`03072aea0c`) | 1,906 (+1,000 / −906 over 186 files; `git diff --numstat -M 53296d80f0..03072aea0c -- '*.rs'`) | 0 | 184 near-identical test files (74,932 lines) folded into 2 roster-driven binaries; 8,723 `--list` entries before = 8,723 after (`test-list-diff.txt`, `verdict=IDENTICAL`); test binaries built 544 → 362 (−182); paired cold `cargo test --no-run` 188.97 s → 145.89 s (−22.8%) and `Running` targets 590 → 408 (`build-time.json`). 278 of the 1,000 added lines are the two `main.rs` rosters; the rest are the three crate-root-relative rewrites at 3–4 lines per module. |
| AT-35-E1-001, E1-002, E1-004, E1-005, E1-006 | 0 | 0 | scripts, tests, docs only |

**Other actor in the window:** `sd31-transcribe` contributed 12 of the 34 events (all
`derived`-origin); it is not an Epic 1 lane and its events are not folded above.

## Step 2 — worktree sweep, this epic's worktrees only

`df -h /` at start: 750 G used of 1.5 T (703 G available of 1.5 T). `git worktree list` at start:
7 worktrees — the shared checkout on `tranche/15` at `928272a444`, five Epic 1 lane worktrees,
and this worker's (locked).

| Worktree | HEAD | Merged into `tranche/15`? | Action |
|---|---|---|---|
| `wf_291be5c8-5f3-1` (`worktree-wf_291be5c8-5f3-1`) | `d1b5738658` (AT-35-E1-003 docs) | yes — in `git log 53296d80f0^..928272a444` | **not removed** (see below) |
| `wf_291be5c8-5f3-2` (`worktree-wf_291be5c8-5f3-2`) | `f43116b870` (AT-35-E1-006 SHA fill-in) | yes — same range | not removed |
| `wf_291be5c8-5f3-3` (`worktree-wf_291be5c8-5f3-3`) | `b826695604` (AT-35-E1-001 docs) | yes — same range | not removed |
| `wf_291be5c8-5f3-4` (`worktree-wf_291be5c8-5f3-4`) | `0b4d995440` (AT-35-E1-002 docs) | yes — same range | not removed |
| `wf_291be5c8-5f3-5` (`worktree-wf_291be5c8-5f3-5`) | `8091645ed0` (AT-35-E1-005 SHA pin) | yes — same range | not removed |
| `wf_291be5c8-5f3-7` (this worker) | `928272a444` | n/a — `locked` | never removed by a worker |

5 of 5 lane worktrees are proven merged: each HEAD SHA is a commit in the epic's
`53296d80f0^..928272a444` range, so nothing on those branches is unmerged. **0 of 5 removed by
this worker**: the worker's sandbox refuses every git operation that targets a sibling worktree
(`git worktree remove`, `git -C <sibling> status`, `git merge-base`), so it can neither prune
them nor confirm they carry no uncommitted files. Handed to the orchestrator / next cycle agent,
to run from the shared checkout, one worktree at a time, **only after** `git -C <path> status
--porcelain` is empty:

```bash
for n in 1 2 3 4 5; do
  p=/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-$n
  git -C "$p" status --porcelain            # must print nothing
  git worktree remove "$p" && git branch -d "worktree-wf_291be5c8-5f3-$n"
done
git worktree prune
```

The five worktree branches are local only (never pushed); `git branch -d` refuses if anything
is unmerged, which is the second guard.

## Step 3 — no PR here

None opened.

## Handoff to the next cycle agent

- Commit this file at `artifacts/epic-1-tax-cut/epic-1_wrapup-gate_receipt.md`.
- The gate's `verification` retro event was emitted by `verify.sh` into this worker's
  worktree at `docs/retro/events/epic-1-wrapup-gate.jsonl` (uncommitted, worktree
  `wf_291be5c8-5f3-7`); its literal line is reproduced under Step 0 so it can be appended to
  the shared checkout's `docs/retro/events/` without reaching into this worktree.
- This worker's `CARGO_TARGET_DIR` (`/tmp/cargo-sd35-epic-1-wrapup-gate`, 30 G after the run,
  claim file pid `1499419`) was deleted by the worker at the end of its turn; `df -h /` moved
  from 750 G used to 810 G used of 1.5 T during the run (target dir + `node_modules`) and back
  after the deletion. If it is still present, remove it.
- **Status:** complete — 45 of 45 `verify.sh` stages PASS at `928272a444`; residue not risen; retro
  summary read and folded; worktree sweep enumerated (5 of 5 merged, 0 removed by this worker —
  handed over with commands); no PR.

---

## Addendum — second dispatch of this same gate (2026-09-08T15:3xZ)

The wrap-up gate was **dispatched a second time** into the same worktree
(`wf_291be5c8-5f3-7`, still `locked`, still at `928272a444`, `git status --porcelain` listing
only this receipt and the retro shard). Everything above was already on disk when the second
worker woke up. This section is that worker's findings; **it too pushed and committed nothing.**

### The full gate was NOT re-run, deliberately

`scripts/verify.sh` was **not** run a second time. The reasoning, stated so the orchestrator can
overrule it:

1. **Same tree, byte for byte.** The worktree HEAD is still `928272a444` and no tracked file has
   moved (`git rev-parse HEAD`; `git status --porcelain` → the two untracked files above). The
   recorded run gated exactly this tree.
2. **The standing rule forbids it.** "All changes, then ONE full verify; redo only if something
   broke." Nothing broke — nothing changed at all.
3. **The tree is superseded anyway.** `origin/tranche/15` has advanced to `4510517993`
   ("AT-35-E1-004 cycle 2 receipt … criterion at zero"), and Epic 2 and Epic 3 lanes are mid-flight
   (`/tmp/cargo-sd35-AT-35-E2-001`, `-E2-004`, `-E2-005`, `-E3-001`, and an
   `epic-2-wrapup-gate` target dir already exist). An 82-minute re-run at `928272a444` would
   re-answer a question about a tree the bundle has moved past, and would do it on a **contended
   box** — `uptime` at start `1.73 5.97 7.40` with six `rustc` processes live. Re-gating the
   *current* tip is Epic 2's wrap-up gate, not this one, and that gate is already provisioned.

**What this costs:** the second worker cannot independently re-witness the 45 PASS rows. It
verified what it cheaply could (below) and reproduced every figure that does not need a build.

### Independently re-verified at `928272a444` by the second worker

| Check | Command | Result |
|---|---|---|
| PCGen residue | `python3 scripts/pcgen_residue_gate.py --check` | `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`, exit 0 — **identical** to Step 0's row and to AT-35-E1-005's first recording. Not risen. |
| Retro window | `python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00 --json` | Reproduces Step 1 exactly: corrections 6 (2 with blast radius, 2 caught before implementation), incidents 4 (`time_lost_minutes` 2, `silent` 0, `recurring: []`), deferrals 1 open / 0 resolved, rework 0, near-misses 0, git-join 15 commits / 1 author, 0 invalid lines. Two figures moved, both explained: **events 34 → 35** and **verification runs 14 → 15**, the delta being this gate's own `verification` event, which had not yet been written when Step 1 was computed. Shards 7 → 8. |
| Incident 3+ rule | `incidents.by_recurrence_key` | Unchanged and still the four one-shot keys naming **one** condition: `denominator-gate-red-on-package-prose`, `denominator-gate-red-on-untouched-artifacts`, `package-gate-red-at-cycle-start`, `pre-existing-red-gate`. Step 1's two dispositions (the landed `DEFAULT_GLOBS` control; the escalation to constrain `retro.py`'s recurrence keys) stand as written. The escalation is **still unbuilt** and is re-raised here. |

### Two corrections against the receipt above

Both logged to `docs/retro/events/epic-1-wrapup-gate.jsonl` (uncommitted, this worktree):

1. **`1788881830549-epic-1-wrapup-gate-26730f` — the log path is dangling.** Step 0 cites
   `/tmp/codex-verify-sd35-epic1-wrapup/` (`verify-full.log`, `verify-time.txt`, `run-meta.txt`,
   per-stage logs). **That directory no longer exists** (`ls` → "No such file or directory"); it
   was reaped from `/tmp` between the two dispatches. The **verbatim 45-row stage table in Step 0
   and the `verification` event in the shard are now the only durable record of the run.** Do not
   cite the log path forward as a retrievable artifact. Anyone who needs live logs must re-run the
   gate — which, per the reasoning above, is better spent on the current tip.
2. **`1788881843136-epic-1-wrapup-gate-158cc8` — Step 2's worktree table is stale and its removal
   loop is no longer safe to paste.** All five Epic 1 lane worktrees have been **recycled by
   re-dispatched lanes** and have moved:

| Worktree | SHA in Step 2 | SHA now | Ancestor of `origin/tranche/15`? |
|---|---|---|---|
| `wf_291be5c8-5f3-1` | `d1b5738658` | `06872eff73` "AT-35-E1-003 re-dispatch — criterion already closed at `03072aea0c`, re-verified at HEAD" | yes |
| `wf_291be5c8-5f3-2` | `f43116b870` | `4e321d2c6c` | yes |
| `wf_291be5c8-5f3-3` | `b826695604` | `986084c5a4` | yes |
| `wf_291be5c8-5f3-4` | `0b4d995440` | `942c8d3ae5` | yes |
| `wf_291be5c8-5f3-5` | `8091645ed0` | `3c43cf0531` "AT-35-E1-005 re-dispatch — criterion already landed; re-verified at HEAD" | yes |

`git merge-base --is-ancestor <sha> FETCH_HEAD` exits 0 for all five against
`origin/tranche/15` = `4510517993`, so **nothing on them is unmerged** — the sweep's content
verdict is unchanged, 5 of 5 merged, **0 removed by this worker** (still read-only, and the
sandbox still refuses sibling-worktree git). But the *reason* they are removable has changed:
they are no longer finished Epic 1 lanes sitting idle, they are **general-purpose lane slots the
orchestrator is actively recycling**. The Step 2 loop therefore needs a second guard:

```bash
# BEFORE the loop: confirm the orchestrator has no lane assigned to the slot.
readlink /proc/*/cwd 2>/dev/null | grep worktrees | sort | uniq -c   # must not list the slot
git -C "$p" status --porcelain                                        # must print nothing
```

At the moment of writing, `readlink` over every live `cargo`/`rustc`/`node`/`python3` process
showed **only** `wf_291be5c8-5f3-7` (this worker) and `wf_291be5c8-5f3-15` (an Epic 2 lane, at
`8cc4ea1516`, **out of this epic's sweep scope**) occupied — slots 1–5 were idle. That is a
snapshot, not a guarantee; re-check at removal time.

### Disk

`df -h /` → **849 G used of 1.5 T, 604 G available, 59%**. The first worker's
`/tmp/cargo-sd35-epic-1-wrapup-gate` is **gone** (reaped with the log dir), so its handoff note
"if it is still present, remove it" is discharged. The second worker created **no** target dir
(it ran no cargo). Nothing to reclaim for this gate.

### Systemic note for the orchestrator

Three separate Epic 1 artifacts in this worktree set — `AT-35-E1-003`, `AT-35-E1-005`, and this
gate — were **dispatched a second time after already completing**, and each re-dispatch found its
predecessor's work intact and unpushed. The common mechanism is that **this worker class pushes
nothing by design** (`§2` worker split), so its output only reaches `tranche/15` when a *later*
cycle agent commits the hand-off — and for this gate that never happened: neither
`epic-1_wrapup-gate_receipt.md` nor `docs/retro/events/epic-1-wrapup-gate.jsonl` exists on
`origin/tranche/15` at `4510517993` (`git cat-file -e FETCH_HEAD:<path>` → "exists on disk, but
not in FETCH_HEAD" for both). **The hand-off step is the unreliable link, not the gate.** Worth a
mechanical control — e.g. the orchestrator committing the isolated worker's returned files as its
own step, rather than folding them into the next criterion's cycle where they can be dropped.

### Second-dispatch status

**complete.** No stage is red: the recorded gate is 45 of 45 PASS at `928272a444`, and every
figure re-derivable without a build was re-derived and matches. Residue `verdict=PASS`, not risen.
No PR. Nothing committed, nothing pushed.
