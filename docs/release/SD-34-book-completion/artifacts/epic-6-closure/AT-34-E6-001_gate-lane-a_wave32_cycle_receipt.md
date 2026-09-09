---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001
date: 2026-09-02
verdict: PASS
---

# Cycle Wave 32, Gate Lane A — Epic 6 Closure / AT-34-E6-001

**`AT-34-E6-001` is this cycle's tracking label, not the final-acceptance scan.** That scan's own
receipt is `AT-34-E6-001_cycle_receipt.md` and this cycle does not write it. This lane's assigned
population is the last two named `scripts/verify.sh` FAILs from wave 31's sweep (38 PASS / 2
FAIL of 40): `site-dashboard-check` and `denominator-gate` (`violations=3` of `files_checked=149`).

- **Base commit:** `4df2c3fa0a` (`origin/tranche/14` at the start of this cycle — confirmed via
  `git fetch origin && git log --oneline -1 origin/tranche/14`)
- **Fix commit:** `27ed4ab983` (original), rebased onto two concurrent lanes' pushes
  (`65c891e277`, `0eaba444bc` — scope confirmed below) to `e158f8af04`, pushed clean, no conflicts
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-6-closure/AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md`,
  `docs/release/SD-34-book-completion/progress.md`, `site/dashboard/PF1e-dashboard.json`,
  `site/dashboard/PF1e-dashboard.json.last-good`, `site/status-data.json`, this receipt.
- **Acceptance criterion:** verbatim, `epic-breakdown.md ### AT-34-E6-001`: *"Every criterion
  `AT-34-E1-001` … `AT-34-E5-004` is `complete`, and every `kanban.md` card is `complete` …"* — the
  bundle-level bar, which this lane does not claim. This lane's own bar, set by the dispatch brief,
  is 40/40 on `scripts/verify.sh`.

## FAIL 1 — `site-dashboard-check`

**Confirmed genuinely stale before touching anything:**

```
$ python3 -c "import json; print(json.load(open('site/dashboard/PF1e-dashboard.json'))['generated_at'])"
2026-08-24T22:17:30Z
$ git log -1 --format=%cd --date=iso -- docs/work-inventory.json
2026-08-31 20:15:47 -0400
```

The dashboard predates the last real touch of `docs/work-inventory.json` by 8 days. `--check`
(`./scripts/publish-site-dashboard.sh --check`) failed with exactly the message the brief quoted.

**Before regenerating, the committed copy was snapshotted and diffed against the fresh output**
(`AGENTS.md` "Generated artifacts mutated post-hoc" — this artifact has destroyed data before, so
the diff had to happen before the commit, not be assumed safe):

```
$ cp site/dashboard/PF1e-dashboard.json /tmp/pf1e-dashboard-before.json
$ cp -r site/dashboard/units /tmp/pf1e-dashboard-snapshot-before/units
$ python3 -c "... total rows across shards: ..."
total rows across shards: 49438
```

Ran the real regeneration: `./scripts/publish-site-dashboard.sh` (no `--check`), 38.2s wall.

**What actually changed, verified rather than assumed:**

- **Row/unit count: unchanged.** All 19 unit-kind shards under `site/dashboard/units/` carry the
  identical row count before and after — `49438` total both times, per-shard identical
  (`ability 4337`, `class 185`, `class_feature 18043`, … `trait 487`) — re-derive:
  `python3 -c "import json,glob,os; print(sum(len(json.load(open(f))['rows']) for f in glob.glob('site/dashboard/units/*.json')))"`
  → `49438`.
- **Same names, same fields, zero removed/added.** Spot-checked `PF1e-units-feat.json`: fields
  `['name','book','status','wiring_class','source_file','type_facet']` identical before/after;
  `len(names_before - names_after) == 0` and `len(names_after - names_before) == 0`.
- **No license/PI/`raw_tokens` loss.** This artifact does not carry raw corpus text at all — the
  17 hits for `license`/`raw_tokens` in the pre-regen file are (a) a benign `"license": "OGL"`
  string per `manifests.*.items[].license` entry (an identifier, not corpus prose) and (b) plain-
  text mentions inside `retrospective.corrections`/`retrospective.rework` (quoting past incidents
  by name, e.g. `"...LICENSE.json merge..."`), both unchanged in kind before and after. This is a
  status/metrics dashboard, not a corpus regeneration — the "destroyed license/PI + raw_tokens"
  hazard the brief warns about applies to `docs/work-inventory.json`/`completion-atlas.json`
  regeneration, a different generated artifact this cycle did not touch or regenerate.
- **What DID move, and why it is real, not loss:** `work_inventory.by_doneness_kind`,
  `mandate_headline`, `mechanically_confirmed_by_kind` etc. all shifted — these are pass-throughs
  of `docs/work-inventory.json`'s own `generated_at` field, `2026-08-23T21:13:46Z` (old cache) →
  `2026-09-01T00:01:15Z` (current file, unchanged this cycle). `work_inventory.status_vocabulary`
  lost the keys `not-ingested`/`unknown` and gained `oracle-unverifiable`/`oracle-agree`/
  `unmeasurable`/`engine-does-not-hold` — a vocabulary rename that landed in `docs/work-
  inventory.json` itself across the eight days between the two dashboards' source snapshots, not
  something this regeneration did. `mandate_headline.done: 15034 -> 23338` and denominator
  `45149 -> 49047` are both increases, consistent with eight days of the bundle's own closure work,
  not data loss in either direction.
- `site/status-data.json` changed 2 lines total (`generated_at` timestamp; the body is otherwise
  byte-identical) — re-derive: `git diff --stat site/status-data.json`.
- `site/dashboard/PF1e-dashboard.json.last-good` is the producer's own automatic fallback copy
  (`scripts/observer/pf1e_dashboard_producer.py:2787`), not something this script or cycle writes
  directly — it mirrors the main file 1:1 on every successful run.
- `site/dashboard/units/*.json` and `site/status-data/*.json` (30 per-book files) came out
  **byte-identical** to the committed copies — no diff in `git status --porcelain` for either path
  after the regeneration, confirming the row-count check above independently.

`./scripts/publish-site-dashboard.sh --check` now: `site/dashboard/PF1e-dashboard.json is
current` / `OK: status-data.json and status-data/*.json are up to date`, exit 0.

## FAIL 2 — `denominator-gate`, `violations=3` of `files_checked=149`

```
$ python3 scripts/denominator_gate.py --check
VIOLATION .../AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md:138: ... **99% CPU the entire run** ...
VIOLATION .../AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md:153: ... **12:37.01 (757.01s), exit 0, 99% CPU** ...
VIOLATION .../progress.md:127: ... (950s = 757s measured + ~25% margin) ...
files_checked=149
violations=3
```

Exactly the three the brief named (the `progress.md` line number had shifted from `:33` to `:127`
by prior prepends — the repo wins over the stale line reference; noted here per the brief's own
instruction to say so rather than following either silently).

**Fix 1 & 2 (receipt lines 138, 153):** both instances of bare `99% CPU` rewritten to `99% of 1
CPU core`, adding the `of <N>` denominator marker `DENOMINATOR_RE` requires, plus (line 138 only,
outside any Figures-section scope, so not required by `--check-provenance` but added anyway per
`AGENTS.md` rule 9) an inline re-derive command:
`` `/usr/bin/time -v timeout 1800 cargo run --locked --quiet --bin v06_work_inventory -- --summary 2>&1 | grep 'Percent of CPU'` ``
— the exact command whose transcript is already quoted verbatim in the fenced block above line
138, reproducing the `Percent of CPU this job got: 99%` line it cites.

**Fix 3 (`progress.md:127`):** rewritten from a bare `~25% margin` to `a margin of 25.5% of the
757s baseline` with a same-line re-derive command,
`` `python3 -c 'print(round((950-757)/757*100, 1))'` `` → `25.5`, confirmed:

```
$ python3 -c 'print(round((950-757)/757*100, 1))'
25.5
```

**Not satisfied with the words "same run"** — that phrase carries no digit and would not match
`DENOMINATOR_RE` (`\bof\b.{0,24}?[\d,]+` / `\bout of\b...` / `N/M` / `\bdenominator\b...`), so it
would not have cleared the gate; a real re-derive command was used instead. First attempt at this
fix put the denominator marker on a different physical line than the percent token (the gate is
line-scoped) — caught by re-running `--check` immediately after the edit rather than assuming it
passed, and corrected into one line before commit.

**`progress.md` is prepend-only as a rule.** This edit repairs an existing line's own provenance
in place — it does not add a new claim or change the figure's meaning (950s is still 757s + a
margin; the margin's precise value and its command are now stated) — a correction, not a rewrite,
per the brief's own explicit permission.

```
$ python3 scripts/denominator_gate.py --check
files_checked=149
violations=0
$ python3 scripts/denominator_gate.py --check-provenance
files_checked=79
figures_examined=126
violations=0
```

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| dashboard staleness gap before fix | 8 days | `git log -1 --format=%cd --date=iso -- docs/work-inventory.json` vs. the committed `generated_at` | N/A (date arithmetic) |
| unit rows, before regen | 49438 | `python3 -c "import json,glob; print(sum(len(json.load(open(f))['rows']) for f in glob.glob('/tmp/pf1e-dashboard-snapshot-before/units/*.json')))"` | of the bundle's own 49,438-unit fact sheet |
| unit rows, after regen | 49438 | `python3 -c "import json,glob; print(sum(len(json.load(open(f))['rows']) for f in glob.glob('site/dashboard/units/*.json')))"` | of the bundle's own 49,438-unit fact sheet |
| shard files compared | 19 | `ls /tmp/pf1e-dashboard-snapshot-before/units/*.json \| wc -l` | of 19 kind-shards + `index.json` = 20 files in the dir |
| `site/status-data.json` diff size | 2 lines changed | `git diff --stat site/status-data.json` | of the whole file |
| `PF1e-dashboard.json` diff size | 1287 insertions / 1550 deletions | `git diff --stat site/dashboard/PF1e-dashboard.json` | N/A (line-count diff, not a population) |
| denominator-gate, before fix | violations=3 of files_checked=149 | `python3 scripts/denominator_gate.py --check` at commit `4df2c3fa0a` | of files_checked=149 |
| denominator-gate, after fix (live `verify.sh` stage) | violations=0 of files_checked=151 | `scripts/verify.sh --only denominator-gate` at `e158f8af04` | of files_checked=151 (150 package files + this receipt itself, present on disk at scan time) |
| figure-provenance, after fix (live `verify.sh` stage) | violations=0 of figures_examined=128 | `scripts/verify.sh --only figure-provenance` at `e158f8af04` | of figures_examined=128, files_checked=81 |
| margin re-derive | 25.5 | `python3 -c 'print(round((950-757)/757*100, 1))'` | N/A (arithmetic, not a population count) |
| full sweep result | 40 PASS / 0 FAIL | `./scripts/verify.sh` (full mode, no `--quick`, no `--only`), run twice — see "Build scope verified" | of 40 stages in `ALL_STAGES` (`scripts/verify.sh:110`) |

## Row-count command output

Two full runs of `scripts/verify.sh` this cycle. **Run 1** (commit `27ed4ab983`, at 149-file
`files_checked`, before this receipt existed) hit a self-inflicted `figure-provenance` FAIL —
this receipt's own then-draft had an unsourced figure (a `same command against ...` cell whose
backtick span was a single bare path token, not a multi-word command
`_line_has_reachable_command` recognizes). Caught at the `figure-provenance` stage of that run,
fixed, and the run was killed (04:19–04:20, before the expensive `root-full` stage) rather than
let finish on stale content — re-run from a clean tree instead of reporting a self-caused FAIL as
this lane's own regression.

**Run 2** (clean, at the post-rebase commit `e158f8af04`) is the one this receipt reports:

```
$ ./scripts/verify.sh
...
SUMMARY
  passed:  40  preflight-disk preflight-oracle oracle-pin-selftest producer-selftest
  pi-redaction-selftest provenance-selftest site-dashboard-selftest site-dashboard-check
  site-dashboard-pi-gate build-public-status-selftest site-public-status-check
  site-public-status-pi-gate site-asset-stamp-check reachability-audit-selftest
  reachability-audit groundtruth-guard-selftest supersession-gate-selftest
  shape-coverage-standing-gate-selftest shape-coverage-standing-gate denominator-gate
  figure-provenance pi-sweep declared-pi-audit audit-selftest reclaim-selftest
  driver-selftest corpus-sweep-selftest corpus-trap-audit-selftest root-lib root-full
  desktop reach corpus-sweep corpus-trap-audit supersession-gate frontend-install
  frontend-test frontend-typecheck clippy class-dump

BASELINE NOTES (not failures — update deliberately):
  - BASELINE_ROOT_LIB_TESTS baseline is stale: 2336 recorded, 3028 measured.
  - BASELINE_ROOT_FULL_TESTS baseline is stale: 7469 recorded, 8372 measured.
  - BASELINE_ROOT_TEST_BINARIES baseline is stale: 569 recorded, 589 measured.
  - BASELINE_DESKTOP_TESTS baseline is stale: 515 recorded, 572 measured.

RESULT: PASS
```

**40 of 40. Zero `FAIL` lines** — re-derive: `grep -c '^    FAIL' <log>` → `0`; `grep -c
'^    PASS' <log>` → `40`. Full log at `/tmp/codex-verify-9RjHR1` (this run's own `logs:` line) —
also captured by `verify.sh`'s own retrospective emission, `docs/retro/events/sd31-
transcribe.jsonl`'s last line: `"head": "e158f8af04", "result": "PASS", "duration_seconds":
6692` (1h51m32s), `stages_passed` listing all 40 by name, `stages_failed` absent (only present on
a FAIL run). This is the authoritative, mechanically-emitted record of this exact run, not a
self-assessment (`decisions.md §4`).

**The four baseline-staleness notes are informational, not failures** — `RESULT: PASS` is the
line the stage-count gate reads; the four "stale baseline" lines are `verify.sh`'s own advisory
that `scripts/verify-baselines.env`'s recorded test counts are below the live corpus's (all four
moved **up**, consistent with wave 24–32 progress, not a regression) and were left un-repinned
this cycle — repinning a baseline is not in this lane's two named FAILs and was not touched, per
the "stay inside the granted write scope" rule.

## Build scope verified

Run at `e158f8af04` — the rebased commit both `verify.sh` runs above executed against (run 1's
build-affecting stages never reached completion; run 2 ran the whole gate at this exact SHA).

**Two concurrent lanes' pushes bracket this cycle's work, and both are confirmed build-scope-
empty, not assumed so:**

```
$ git diff --name-only 4df2c3fa0a 0eaba444bc -- src/ tests/ apps/ data/ Cargo.lock Cargo.toml
(no output)
$ git diff --name-only e158f8af04 cdce346131 -- src/ tests/ apps/ data/ Cargo.lock Cargo.toml
(no output)
```

The first (lanes B/C, pre-sweep) is why this cycle rebased before launching run 2 rather than
after — the sweep itself needed to run at a commit that would still be current once pushed. The
second (lane C again, landing *during* run 2's ~1h51m) is why the receipt still reports run 2's
result as valid at HEAD after a further rebase: neither touches `src/`, `tests/`, `apps/`,
`data/`, or the lockfiles, so no figure `root-full`/`desktop`/`reach`/`clippy`/`corpus-sweep`/
`corpus-trap-audit` depends on can have moved between the SHA the sweep ran at and the SHA this
receipt is committed against.

| Scope | Command | Result |
|---|---|---|
| `--no-run` | `cargo test --locked --no-run` | covered by the full sweep's `root-lib`/`root-full` build step (both PASS; `--no-run` is not run standalone this cycle — the full `cargo test` executions above are the wider, strictly more thorough check) |
| root workspace | `cargo test --locked --no-fail-fast` (via `verify.sh`'s `root-full` stage) | **PASS** — 8372 passed, 0 failed, all 543 `tests/*.rs` suites executed (589 result-bearing suites total) |
| desktop crate | `cd apps/desktop/src-tauri && cargo test --locked` (via `verify.sh`'s `desktop` stage) | **PASS** — 572 passed, 0 failed |
| `reach` | `reach_gate::tests::*` (via `verify.sh`'s `reach` stage) | **PASS** — 31 passed |
| clippy, both crates | `cargo clippy --locked --tests` (via `verify.sh`'s `clippy` stage) | **PASS** — root:0 desktop:0 warnings, 0 errors |

## Sweep population

**Full sweep, all 40 `ALL_STAGES`** (`scripts/verify.sh:110`) — this lane's whole assigned bar,
not a subset. **Diffed stage-by-stage against the wave-31 baseline (38 PASS / 2 FAIL of 40,**
`docs/release/SD-34-book-completion/artifacts/epic-6-closure/AT-34-E6-001_gate-lane-c_wave31_cycle_receipt.md`**):**

- **`site-dashboard-check`: FAIL → PASS.** This lane's fix.
- **`denominator-gate`: FAIL → PASS.** This lane's fix.
- **Every stage that was PASS at wave 31 is still PASS.** Re-derive: the wave-31 receipt's own
  38-name PASS list (`preflight-disk` … `class-dump`, minus the two FAILs) is a strict subset of
  this run's 40-name PASS list above — every one of those 38 names appears in this run's `passed:`
  line, and no name present there is absent here. **Zero regressions.**
- No stage moved from PASS to FAIL. No stage was skipped, added, or renamed between the two runs
  (`ALL_STAGES` is unchanged — same 40 names, same order, both receipts).

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus; `preflight-
  oracle` and `oracle-pin-selftest` both PASS unchanged as part of the sweep, confirming the pin
  itself was not touched or invalidated by this cycle.
- **Status:** complete. This lane's entire assigned population (the two named FAILs) is closed,
  proven by a full, live, 40/40 `scripts/verify.sh` run at the pushed commit.
- **Movement, four buckets:**
  - **closure:** 0 work-inventory units. This lane's work is instrument/artifact hygiene
    (a stale published projection, three unsourced prose figures), not corpus unit movement.
  - **reclassification:** 0.
  - **reachability:** 0.
  - **instrument-correction:** 2 verify.sh gate stages FAIL → PASS (`site-dashboard-check`,
    `denominator-gate`), closing this bundle's `scripts/verify.sh` gate from 38/40 (wave 31) to
    **40/40** — a correction to a previously-wrong reported state (a stale artifact silently
    served as current; three ungrounded prose figures), not new corpus work.
- **Notes:**
  - `docs/work-inventory.json` and `completion-atlas.json` were not read, written, or
    regenerated by this cycle.
  - `kanban.md` not touched, matching every prior gate-lane wave's own precedent — no board row
    tracks an individual gate-remediation wave.
  - Three concurrent lanes' commits landed on `origin/tranche/14` while this cycle was in
    flight — `65c891e277`/`0eaba444bc` (lanes C/B, before run 2 launched, rebased onto cleanly)
    and `cdce346131` (lane C again, during run 2's ~1h51m, rebased onto after). All three's whole
    touch set is `scripts/completion_atlas.py`, `docs/release/SD-34-book-completion/
    {kanban.md,progress.md}`, `docs/release/SD-34-book-completion/artifacts/{epic-1-atlas,bucket-
    d-mining}/**`, `docs/release/SD-34-book-completion/artifacts/sd-34-dispatch.workflow.js` — no
    `src/`, `tests/`, `apps/`, `data/`, `Cargo.lock`/`Cargo.toml` in any of them (both `git diff
    --name-only ... -- src/ tests/ apps/ data/ Cargo.lock Cargo.toml` commands above return
    empty), so the build-affecting scope of this cycle's sweep is unaffected by either rebase.
  - `docs/retro/events/sd31-transcribe.jsonl` gained one line from this cycle's own `verify.sh`
    run (the auto-emitted verification event, `AGENTS.md` "Retrospective Logging") — committed
    alongside this receipt as the append-only log the repo already tracks.
- **Next-cycle plan:** if 40/40 holds, the final-acceptance scan (`AT-34-E6-001_cycle_receipt.md`)
  can be re-attempted against whatever `kanban.md` state stands at that time — this lane's own
  fixes do not touch any Epic 1–5 criterion or kanban row.
