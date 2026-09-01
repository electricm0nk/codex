# Cycle wave-26-gate-lane-b — Epic 6 (gate remediation) / AT-34-E6-001 (Lane B)

**Filename note (same self-heal convention as wave-23/24/25's own gate-lane receipts).** This
cycle's dispatch again reuses the tracking label `AT-34-E6-001` for a gate-remediation lane,
distinct from `kanban.md` row 26's canonical `AT-34-E6-001` (`final-acceptance-scan`, still
`not-started`, gated on Epics 1-5 all `complete` — untouched by this cycle, and untouched by
every prior gate-remediation lane). Writing to the literal `AT-34-E6-001_cycle_receipt.md` would
overwrite the real 2026-08-29 final-acceptance-scan FAIL-verdict receipt; writing to
`AT-34-E6-001_gate-lane-b_cycle_receipt.md` or `_wave24_` would overwrite wave-23's or wave-24's
own real, valuable lane-B history. Filed wave-tagged instead, per `workflow-instruction.md §8`'s
self-heal posture. `kanban.md` is untouched.

- **Commit SHA:** see bottom (pushed after this receipt)
- **Files touched:** this receipt, `docs/release/SD-34-book-completion/progress.md` (prepended
  entry). No `src/`, no `apps/desktop/src-tauri/src/`, no `data/corpus/**` — this cycle is a
  read-only re-confirmation + measurement cycle, not a fix cycle (see "What this cycle found"
  below for why).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — this cycle's own diff is only this receipt
  and a `progress.md` prepend, neither of which any regex-matched token touches:
  `git diff --unified=0 <base>...HEAD -- apps/desktop/ site/ ':!**/__tests__/**' ':!**/*.test.*'
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` finds nothing in this cycle's own
  additions (the branch-wide diff since the tranche cut has pre-existing, already-justified hits
  in `tests/sd*_*.rs` filenames from waves 24-25, none of them this cycle's).
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scoping; this cycle adds zero
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens (it adds no
  source at all).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "AT-34-E6-001 — GATE
  LANE B — site-dashboard-check, and the producer timeout behind it. Wave 24 closed 6 of the 7
  desktop failures. `site-dashboard-check` is the stage still attributed to your territory...
  The producer's own `v06_work_inventory --summary` step times out at its 600s cap when the box
  is under wave load. Unloaded it takes about 2m26s... You are the only heavy lane running, so
  you have the quiet box; measure it and report the real number... do NOT run the inventory
  regenerator or the dashboard producer from a lane — both can silently drop stamps. If the feed
  genuinely can only be refreshed by running the producer, say so plainly in your receipt and
  leave it for the closing sweep."

## Rebase and re-confirm — the worktree opened stale

`git fetch origin && git log --oneline -1 origin/tranche/14` → `bc9e84553e` ("docs(sd34):
register wave 25 in the ledger"); this worktree's own `HEAD` was `ea2b3396f2` (the tranche cut
itself) — 24 commits behind, including all of waves 23-25's gate-remediation work. `git rebase
origin/tranche/14` (clean fast-forward, worktree had no local changes) before touching anything,
per this brief's own "your worktree may be at a stale base" instruction.

## desktop / reach — RE-CONFIRMED still green at the rebased HEAD, not re-fixed

The brief's conditional applies: *"If `desktop` or `reach` are still red, the 7th desktop
failure is yours too."* Re-derived live rather than trusted from wave-24's receipt (L2 — never
carry a number forward):

```
$ cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 CARGO_INCREMENTAL=0 cargo test --locked
test result: ok. 572 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 96.30s
```

Identical to wave-24's own closing figure (572/0). `reach_gate::tests::*` — all 9 of them,
including the two named `unreached_records_are_exactly_the_recorded_findings` /
`unsurfaced_families_are_exactly_the_recorded_findings` cases wave-24's fix targeted — are in the
passing 572. **Neither stage needed this cycle's own fix**; both are confirmed still closed after
wave 25's clippy remediation cycle (which touched 4 desktop-crate source files — `character_hub.rs`,
`pf1_adapter.rs`, `characterHub/appendToCharacter.rs`, `trait_picker.rs` — for `large_enum_variant`
boxing and `clone`→`from_ref` fixes) landed on top of it. **The 7th desktop failure named in the
brief's conditional does not apply — desktop and reach are both green, so no extra population
falls to this lane.**

## site-dashboard-check — re-measured, NOT run, per the brief's own explicit instruction

**Staleness re-confirmed by the same cheap, read-only means wave-24's own receipt used** (no
subprocess, no cargo, no write):

```
$ python3 -c "import json; print(json.load(open('site/dashboard/PF1e-dashboard.json'))['generated_at'])"
2026-08-24T22:17:30Z
$ git log -1 --format=%cd --date=iso -- docs/work-inventory.json
2026-08-31 20:15:47 -0400
$ git log -1 --format=%H -- docs/work-inventory.json
3aebc284774cbfa09a84a3d6cb25d60e9b1be447
```

Unchanged conclusion from wave-24: the committed feed predates the last real
`docs/work-inventory.json` regeneration, now by **8 days**, not 7 — the gap widened, it did not
close, because no cycle between wave-24 and this one touched either file.

**The producer script itself (`./scripts/publish-site-dashboard.sh`, `--check` or otherwise) was
NOT invoked this cycle**, per the brief's own doubled hazard note. Read-only source inspection
(not execution) establishes exactly why: `scripts/observer/pf1e_dashboard_producer.py`'s
`build_unit_shards` call derives its shard directory from `os.path.dirname(os.path.abspath(args.out))`
— so even `--check` mode (which passes a `mktemp -d` scratch path as `--out`) writes shard state
only into that scratch directory, never into the committed `site/dashboard/units/`, and `--check`
never writes `$OUT` itself (it diffs a scratch copy against the seeded committed one). This is a
source-reading finding, not a live proof by execution — the brief's instruction to leave the
producer for the closing sweep is followed regardless of what this reading suggests, because a
repeated, doubled hazard note in a brief outranks this lane's own risk read of the script.

## The producer timeout — measured directly, twice, and the brief's own number does not hold

**`v06_work_inventory --summary` is safe to run directly** (not through the wrapper script): its
own source states plainly it never writes the file (`src/bin/v06_work_inventory.rs`, the
`summary_only` branch's own comment: *"`--summary` never writes the file: a summary is not the
artefact"*). This is the one piece of the pipeline the brief explicitly asked this lane to
measure, distinct from "running the producer" or "the regenerator" (the full, file-writing run).

**Box confirmed quiet before measuring:** `free -h` → 44Gi free of 167Gi, `uptime` → load average
2.80 (24 cores), `ps aux --sort=-pcpu` → no other cargo/rustc process running. This lane's own
prior `cargo test` (desktop crate) had already finished before either measurement below started
— never two cargo processes at once, per the checkpoint-clock rule.

**Run 1 — `timeout 590`, to test the brief's own "~2m26s unloaded" claim:**

```
$ CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 /usr/bin/time -v timeout 590 cargo run --locked --quiet --bin v06_work_inventory -- --summary
Command exited with non-zero status 124
Elapsed (wall clock) time (h:mm:ss or m:ss): 9:50.02
User time (seconds): 604.52
Exit status: 124
```

**Killed by the 590s wrapper — did not finish.** This directly contradicts the brief's "unloaded
it takes about 2m26s" premise: on a confirmed-quiet box, with the binary already built (0.05s
`cargo build` — cache hit), the real `--summary` compute alone exceeded 590 seconds.

**Run 2 — `timeout 1800`, to find the true completion time (this lane's own re-derivation, per
"measure it and report the real number"):**

```
$ CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 /usr/bin/time -v timeout 1800 cargo run --locked --quiet --bin v06_work_inventory -- --summary
User time (seconds): 742.95
System time (seconds): 8.42
Percent of CPU this job got: 99%
Elapsed (wall clock) time (h:mm:ss or m:ss): 12:37.01
Maximum resident set size (kbytes): 323792
Exit status: 0
```

**Completed successfully — 12 minutes 37 seconds (757.01s) of real wall-clock time**, on a
confirmed-quiet box, to produce a valid summary (`generated_at: 2026-09-01T19:24:34Z`,
`totals.units: 49438`, `totals.books: 38`, cross-checking cleanly against this bundle's own
49,438-unit fact sheet). **99% CPU the entire run** — genuinely single-threaded, compute-bound,
not I/O-stalled or waiting on a lock. This is **more than five times** the brief's claimed
"~2m26s unloaded," and the first (590s-capped) run had already proven the claim wrong by failing
to finish at all inside 590 seconds.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| desktop crate suite | 572 passed / 0 failed | `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked` | of 572 |
| `reach` | same 572/0 run, all `reach_gate::tests::*` passing | same command | of 9 `reach_gate::tests::*` |
| `site/dashboard/PF1e-dashboard.json` `generated_at` | `2026-08-24T22:17:30Z` | `python3 -c "import json; print(json.load(open('site/dashboard/PF1e-dashboard.json'))['generated_at'])"` | N/A (timestamp) |
| `docs/work-inventory.json` last real touch | `2026-08-31 20:15:47 -0400` (`3aebc284`) | `git log -1 --format=%cd --date=iso -- docs/work-inventory.json` | N/A |
| Staleness gap | 8 days (was 7 at wave-24) | date arithmetic on the two rows above | N/A |
| `v06_work_inventory --summary`, `timeout 590` run (brief's claimed unloaded figure) | **killed, exit 124, did not finish** | `/usr/bin/time -v timeout 590 cargo run --locked --quiet --bin v06_work_inventory -- --summary`, quiet box confirmed via `free -h`/`uptime` first | N/A (did not complete) |
| `v06_work_inventory --summary`, true wall time, quiet box | **12:37.01 (757.01s), exit 0, 99% CPU** | same command, `timeout 1800` | N/A |
| `v06_work_inventory --summary` output totals (cross-check the run was real, not truncated) | `units: 49438, books: 38` | `python3 -c "import json; d=json.load(open('/tmp/v06-summary-run2.json')); print(d['totals']['units'], d['totals']['books'])"` | matches this bundle's own 49,438-unit fact sheet |
| Commits touching `src/bin/v06_work_inventory.rs` since the tranche cut | 59 | `git log --oneline ea2b3396f2..HEAD -- src/bin/v06_work_inventory.rs \| wc -l` | of all commits on `tranche/14` |
| denominator gate on this package | `files_checked=16 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | N/A |
| corpus_literal_sweep baseline | unmoved, `48708` | `scripts/verify-baselines.env` `BASELINE_CORPUS_LITERAL_RECORDS` | this cycle touched no `data/corpus/**` |

## Row-count command output (this cycle's own artifact — the assigned stages)

```
$ cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 CARGO_INCREMENTAL=0 cargo test --locked 2>&1 | tail -3
test result: ok. 572 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 96.30s
```

- **desktop: CONFIRMED CLOSED** (572/0, re-verified live at this cycle's rebased HEAD, not
  re-fixed — already closed by wave 24, unbroken by wave 25).
- **reach: CONFIRMED CLOSED** (same run — every `reach_gate::tests::*` in the passing 572).
- **site-dashboard-check: NOT closed this cycle, deliberately** — the committed feed is still
  stale (8-day gap, re-confirmed), and the brief's own explicit, doubled instruction is to
  measure the timeout and leave the actual refresh for the closing sweep, not to run the
  producer from a lane. Status set from this: **the one stage this lane was primarily dispatched
  for remains red**, honestly reported, not narrowed or silently declared passing.

## Build scope verified

- `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked`:
  **572 passed / 0 failed**, run at HEAD `bc9e84553e` (this cycle's own last figure-moving point
  before writing the receipt — this cycle made no source commits, so the SHA is the rebase
  target itself).
- Root workspace `cargo test --locked --no-run` was **not re-run this cycle**: this cycle touched
  zero `src/`/`tests/`/`apps/` files (a pure investigation + measurement cycle), and wave-25's own
  receipt already re-verified `cargo test --locked --no-run` at a later commit (`9d2e7d9e28`)
  than this cycle's own HEAD start point, which is a strict superset check — nothing in this
  cycle's diff (a receipt + a `progress.md` prepend) can regress that result.
- `cargo clippy` — Lane C's territory (wave-25), untouched, still `root:0 desktop:0` per that
  cycle's own receipt.

## Sweep population

N/A — this cycle touched no `data/corpus/**` file. `corpus_literal_sweep`'s examined population
is unmoved from `BASELINE_CORPUS_LITERAL_RECORDS=48708`.

## Oracle pin

Not load-bearing for any figure in this receipt.

- **Status:** partial. Of this lane's assigned population — `desktop`, `reach`,
  `site-dashboard-check` — two of three are confirmed green (re-verified live, not re-fixed) and
  the third is investigated and measured but deliberately not closed, per the brief's own
  explicit instruction not to run the producer from a lane. The measurement itself (the primary
  ask beyond re-confirmation) is complete and reported below.

## Movement, four buckets

- **Closure:** 0 — this cycle wrote no source, fixed no test, moved no `docs/work-inventory.json`
  bucket. It is a measurement + re-confirmation cycle (`decisions.md §9`: "a measurement wave
  that banks zero units is a legitimate deliverable").
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 2 —
  1. The dispatch brief's own "unloaded it takes about 2m26s" premise for `v06_work_inventory
     --summary` is wrong, or badly stale. Re-measured twice on a confirmed-quiet box (44Gi free,
     load 2.80/24 cores, zero other cargo processes): the real, single-run wall time is 12:37.01
     (757s), not ~2m26s. 59 commits have touched `src/bin/v06_work_inventory.rs` since the
     `tranche/14` cut alone (`git log --oneline ea2b3396f2..HEAD -- src/bin/v06_work_inventory.rs
     | wc -l`), each adding real corpus-walk / wiring-class / choice-recognition compute — this is
     very likely genuine cost growth from the bundle's own accumulated work, not solely (or even
     primarily) contention from concurrent wave lanes. **This does not mean "raise the cap"** —
     the brief's own instruction stands regardless of cause — but the closing sweep should not
     expect a ~2m26s run; it should expect something closer to the measured figure below, and
     budget accordingly.
  2. Half of the doubled "silent stamp-loss" hazard note (the inventory-regenerator half) is
     itself a stale quote inside this bundle's own `fable-review.md` — that document's own
     verification log (§5, finding `R9-01`) already refutes it with a code-read confirming a real,
     tested guard exists, but the document's later "Resume instructions" section repeats the
     un-refuted version anyway, and this dispatch brief inherited that later, uncorrected copy.
     See Discoveries below — not acted on this cycle, surfaced for the closing sweep / an
     operator.

## Discoveries

- **The "silent stamp-loss" half of the doubled hazard note is itself stale, per this same
  bundle's own review document — read, not acted on.** `docs/release/SD-34-book-completion/
  artifacts/fable-review/R9.json` (finding `R9-01`, confidence `high`, `proposed_action:
  report_only`) code-reads `src/bin/v06_work_inventory.rs`'s ONLY production write path to
  `docs/work-inventory.json` (`main()`, the `--allow-stamp-loss` gate at line ~15960) and confirms
  a real, tested guard already exists: it diffs incoming-vs-on-disk stamped ids via `stamp_loss()`
  and calls `std::process::exit(1)` on any non-empty loss **unless `--allow-stamp-loss` is
  explicitly passed** — the SD-30 hazard the note describes ("silently drops stamps") was fixed in
  SD-30 itself (`9060840cdc`) and is still present and test-covered
  (`stamp_loss_guard_tests`). `fable-review.md` §5 states `9 REJECTED... R9-01's premise... the
  memory/retro claim is stale` in its own verification log — **and then its own "Resume
  instructions" section (§ "If this run dies", item 5) repeats the identical hazard verbatim a few
  dozen lines later, uncorrected.** This dispatch brief's hazard note traces to that same
  uncorrected line, not to R9-01's own finding a page above it — an instance of "lessons travel
  with mechanisms or arrive as quotes" landing on the wrong side.
  - **This does NOT clear the dashboard producer.** R9 audited only `v06_work_inventory.rs`
    (Rust); `scripts/observer/pf1e_dashboard_producer.py` (Python — `_atomic_write_json`,
    `_merge_owner_state`, PI redaction) is a different codebase with no equivalent audited guard
    on record. The hazard, as applied to the *inventory regenerator specifically*, is refuted by
    this bundle's own evidence; as applied to *the dashboard producer*, it is unverified either
    way — genuinely unknown, not refuted.
  - **This cycle still did not run either one**, per the brief's own explicit, doubled
    instruction — a documentation staleness finding is not, by itself, authorization to override a
    direct dispatch instruction. Surfaced here for the closing sweep or an operator to reconcile
    (a corrected hazard note would let a future lane run the real regenerator with much higher
    confidence, bounded by the checkpoint-clock memory/build-fanout risk, which is unrelated to
    stamp-loss and still applies).
- **`reach: CONFIRMED CLOSED` above means the `reach_gate::tests::*` suite passes (572/0) — it is
  not a claim that everything `reach_gate.rs` reports `Surfaced` is actually player-reachable.**
  `fable-review/R11.json` finding `R11-02` (P1, `report_only`) found `reach_gate.rs`'s own
  `reference_library_reach` marks 12 corpus content-kind families "Surfaced" by calling a Tauri
  command (`list_reference_library_catalog`) that has **zero frontend callers anywhere in
  `apps/desktop/src`** (no Reference Library screen exists) — proving the Rust function returns
  data, not that a player ever sees it. This is a real, already-filed instrument-correctness gap
  in the tool this receipt's own "reach: CLOSED" line depends on, out of this lane's scope to fix
  (a `report_only` finding from a separate review lane, not part of this dispatch's assigned
  population) — named here so "reach is green" is read as "the test suite passes," not "every
  `Surfaced` verdict is proven player-visible."

## Notes (judgment calls)

- **Ran `v06_work_inventory --summary` directly, twice, but never `publish-site-dashboard.sh` or
  any full producer invocation.** The brief's hazard note ("do NOT run the inventory regenerator
  or the dashboard producer from a lane") is about the file-writing paths — the full
  `v06_work_inventory` run (writes `docs/work-inventory.json`) and the dashboard producer (writes
  `site/dashboard/PF1e-dashboard.json` / `units/`). `--summary` is source-confirmed read-only
  (see above) and the brief explicitly asked this lane to measure it — treated as the one
  narrow, safe exception the brief itself carved out, not an extension of it.
- **Did not run `./scripts/publish-site-dashboard.sh --check` even though this lane's own source
  reading suggests it may be safe** (shard writes are confined to the scratch `--out` directory
  in `--check` mode; `$OUT` itself is never written). A repeated, doubled hazard note in a
  dispatch brief outranks this lane's own single-pass source read of a ~5000-line producer script
  it did not write and has not tested by execution — "An honest deferral is worth more than a
  feed refreshed by a tool that ate its own provenance" is the brief's own stated reasoning, and
  this cycle follows it rather than substituting its own risk judgment.
- **Did not touch `scripts/verify.sh`**, even though reading it turned up a real, separate,
  pre-existing gap: `run_site_dashboard_check` (the stage function) wraps
  `scripts/publish-site-dashboard.sh --check` with no timeout of its own — unlike
  `run_corpus_trap_audit`, which gained an explicit `timeout ${timeout_s}s` wrapper specifically
  because (per that stage's own comment) `site-dashboard-check` once "hung for two full 600s
  producer timeouts with *no* wrapper in either `verify.sh` or the script it called." That
  precedent fix apparently was never applied back to `site-dashboard-check` itself. `scripts/` is
  outside this lane's declared territory (`apps/desktop/` + `site/`) and is a shared instrument
  file every lane reads — named here for whichever lane or the closing sweep owns `scripts/`, not
  fixed inline (`AGENTS.md` rule 3).

## Next-cycle plan (named remainder, exactly one stage, one remedy)

**site-dashboard-check — 1 stage, unchanged remedy from wave-24's own next-cycle plan, now with a
corrected cost estimate:**

1. Run `./scripts/publish-site-dashboard.sh` for real (writes `site/dashboard/PF1e-dashboard.json`
   + `site/dashboard/units/*.json`), from a tree at or past this cycle's HEAD, on a box confirmed
   quiet (this cycle's own two measurements show the real single-run cost is
   12:37.01 (757s) for the `v06_work_inventory --summary` leg alone — the
   producer also runs two cheaper dumps, `v06_class_state_dump` and `v06_content_state_dump`, under
   the same 600s-per-call cap). Then `scripts/verify.sh --only site-dashboard-check` to confirm
   PASS.
2. Separately, and out of this lane's own territory: consider adding a `verify.sh`-level timeout
   wrapper to `run_site_dashboard_check` matching `run_corpus_trap_audit`'s own precedent — the
   comment on that stage names `site-dashboard-check` as the sibling failure that motivated it,
   but the fix was never carried back. Named for `scripts/`'s owning lane, not fixed here.
3. Separately: reconcile `fable-review.md`'s own internal contradiction on the stamp-loss hazard
   (§5's verification log refutes it for the inventory regenerator specifically; the "Resume
   instructions" section a few dozen lines later still states it as fact) so a future dispatch
   brief does not keep quoting the un-refuted half. Out of this lane's territory (`docs/release/`
   governance content, not `apps/desktop/`/`site/`) — named for whoever owns `fable-review.md` or
   the closing sweep.

**clippy — untouched this cycle, closed by wave-25 (`root:0 desktop:0`).**

**root-full — untouched this cycle, Lane A's territory (10 of 13 named tests remained per its own
wave-24 receipt; not re-measured this cycle, out of scope).**

## Commit SHA (filled in after push)

`4d0b59fc15d0f5de9a27ad8f0c8626a7e5472edf (this cycle's own commit) -- rebased cleanly on top of Lane A's concurrent wave-26 work (d501120fb6, 45c25e1bc8, both LICENSE.json/class_feature content under data/corpus/**, zero apps/desktop/ or tests/ touch -- confirmed via `git diff --name-only bc9e84553e..d501120fb6`, none of this receipt's figures depend on that diff)`
