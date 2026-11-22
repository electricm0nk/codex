# Cycle — Epic 6 Closure Epilogue / AT-34-E6-001 (GATE LANE B, second "wave 26" round) — desktop contradiction settled GREEN by fixing a real 4-record drift; site-dashboard-check re-confirmed genuinely un-closable from this lane

- **Commit SHA:** `3257813a4f09ca43cf4727490f2da82eeb2fe410`
- **Files touched:** `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 ea2b3396f2fde9223dde93522bd2288b463a21ee...HEAD -- apps/desktop/ site/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no hit)
- **Wired-integration audit result:** 4 hits, all pre-existing and false-positive, none in this cycle's own commit. `git diff --unified=0 ea2b3396f2fde9223dde93522bd2288b463a21ee...HEAD -- apps/desktop/ site/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` returns 4 lines, all introduced by wave-24's already-landed `AT-34-E3-001` commit (confirmed by `git show 3257813a4f --stat`, which touches only `corpus_ingest_diagnostic.rs`): 2× "placeholder" describing real, corpus-cited data rows ingested for shape-coverage completeness (not code stubs — `hardcoded placeholder to the real, corpus-cited CLASS_SKILL_LISTS table` is prose about a *replacement*, and "Human-ethnicity placeholder rows, ingested this cycle" names real ingested corpus rows), 1× "placeholder" in the same shape, 1× "todo" that is a path fragment inside a doc citation (`SD-31-corpus-closure-grind/todo/sweeps.md`), not a to-do marker. My own uncommitted-then-committed diff, checked in isolation (`git diff -- apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`), returns `OK_NO_TOKENS_UNCOMMITTED` on both greps.
- **Acceptance criterion (verbatim, dispatch brief):** "GATE LANE B — site-dashboard-check, and settle the desktop contradiction. **Two stages, and one of them may already be green.** Wave 25`s lane B reported the desktop crate at **572 passed / 0 failed** and called `desktop` CLOSED. The wave-28 closing sweep listed `desktop` as FAIL. Both cannot be right. **Run it yourself and settle it**, and if it is green, say so plainly... `site-dashboard-check` is the one nobody has closed... The standing hazard, unchanged: do NOT run the inventory regenerator or the dashboard producer from a lane — both can silently drop stamps... **Do not raise the cap to make it pass.** If the feed can only be refreshed by a tool this lane may not run, say exactly that and name who can. **Territory:** `apps/desktop/` and `site/`."

## Part 1 — `desktop`: settled. Neither prior report was wrong; the corpus moved between them. Now GREEN.

**RED, confirmed first, for the intended reason** (`cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-lane-b cargo test --locked`, at this cycle's pre-fix rebase HEAD `607b1b6c86`):

```
test result: FAILED. 571 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 94.82s
```

The one failure: `corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts` panicked at `src/corpus_ingest_diagnostic.rs:1508:13`:

```
assertion `left == right` failed: pathfinder_unchained: this diagnostic reports 127 records
from rules_tables plus 1144 known corpus-only records, but a live walk of
.../data/corpus/pathfinder_unchained accounts for 1267 real on-disk records.
  left: 1271
 right: 1267
```

**Both wave-25 (572/0 CLOSED) and the wave-28 sweep (FAIL) were right at their own point in
time — this is not a sweep misreport.** Wave-25's own receipt measured 572/0 at its own HEAD.
Between then and wave-28, lane A's `e5fd8dddb1` (2026-08-31, "fix PU equipmods dup-key
generator, delete 4 stale flat records") deleted 4 duplicate `equipment_modifier` records from
`data/corpus/pathfinder_unchained/`, moving the live on-disk walk from 1271 to 1267 without
anyone re-deriving `corpus_ingest_diagnostic.rs`'s own hardcoded `corpus_only_records` literal
for that book. Wave-27 lane C already diagnosed this exact cause and named it for lane B's own
territory (`apps/desktop/`) to fix — not a new finding, confirmed by re-running the suite myself
this cycle rather than trusting either prior report.

**Independent re-derivation of both sides of the equation, before touching the literal:**

- Live on-disk count: `find data/corpus/pathfinder_unchained -name '*.json' | grep -v
  LICENSE.json | grep -v '/_' | wc -l` → **1267** (matches the panic's own "right" value and
  `live_on_disk_record_count()`'s own directory walk — same `_`-prefixed-dir exclusion, same
  `LICENSE.json` exclusion).
- `reported` (compiled `rules_tables` sum for this book): unchanged at **127** — the panic
  message itself states it, live, at this cycle's own HEAD; no `rules_tables` module for
  `pathfinder_unchained` changed in `e5fd8dddb1` or any commit since wave-25.
- New `corpus_only_records` = `1267 - 127` = **1140** (was 1144, a real -4 matching the 4
  deleted duplicate records exactly).

**Fix:** updated the one literal (`1144u32` → `1140u32`) and appended a dated citation to the
existing comment chain (same convention every prior entry in that chain already uses), rather
than silently repinning — `decisions.md §17a`'s own instruction, quoted in the assertion's own
panic message, is "re-derive corpus_only_records fresh, never repin without proof."

**GREEN, re-confirmed twice** (once pre-commit on the dirty tree, once post-commit at
`3257813a4f`, both `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-lane-b cargo test --locked`):

```
test result: ok. 572 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 95.68s
test result: ok. 572 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 95.44s
```

**`desktop` is genuinely CLOSED at this cycle's HEAD (`3257813a4f`) — 572 passed / 0 failed,
verified live, twice.** Say it plainly per the brief's own instruction: it is green.

## Part 2 — `site-dashboard-check`: re-confirmed still genuinely un-closable from this lane, with the mechanical reason nailed down, not just restated

**Feed re-confirmed stale, read-only.** `site/dashboard/PF1e-dashboard.json`'s own
`generated_at`: `2026-08-24T22:17:30Z`. `docs/work-inventory.json`'s last real commit
(`git log -1 --format='%H %cI' -- docs/work-inventory.json`): `3aebc284774c...`
`2026-08-31T20:15:47-04:00`. Current time (`date -u`): `2026-09-02T00:30:08Z`. **Gap: ~8 days**
and unchanged in shape from wave-26's own "7→8 days" finding — nothing has refreshed the feed
since.

**Read the full mechanical chain, not just the symptom, to confirm this is a genuine cap-vs-
runtime defect and not a build-time or contention artefact:**

1. `scripts/verify.sh`'s `run_site_dashboard_check` (lines 600-621) runs
   `"$REPO_ROOT/scripts/publish-site-dashboard.sh" --check` with **no outer timeout wrapper of
   its own** — confirmed by reading the stage function directly; a comment at `verify.sh:1969`
   independently confirms this same gap ("a sibling stage, `site-dashboard-check`, hung for two
   full 600s producer timeouts with **no** wrapper in either `verify.sh` or the script it
   called").
2. `publish-site-dashboard.sh --check` (lines 42-71) invokes the **real** producer
   (`python3 "$PRODUCER" --out "$TMP"`, `$PRODUCER` = `scripts/observer/
   pf1e_dashboard_producer.py`) — even in `--check` mode. It only writes to a `mktemp -d`
   scratch dir (never the committed `$OUT` in place), so `--check` cannot itself corrupt the
   repo's copy, but it **does** run the real producer process, which the brief's hazard note
   forbids from this lane, and which prior lanes (wave-24, wave-26) already declined to run for
   the same reason.
3. Inside the producer, `load_work_inventory()` → `_load_cached_dump("v06_work_inventory", …,
   bin_args=["--summary"])` → `_run_state_dump()` runs `cargo run --quiet --bin
   v06_work_inventory -- --summary` under `subprocess.run(..., timeout=
   CLASS_STATE_BUILD_TIMEOUT_SECONDS)` (`pf1e_dashboard_producer.py:536-542`).
   `CLASS_STATE_BUILD_TIMEOUT_SECONDS = int(os.environ.get("PF1E_CLASS_STATE_TIMEOUT", "600"))`
   (line 122-124) — **one shared 600s cap for all three state-dump binaries**, not a
   work-inventory-specific one.
4. Wave-26's own receipt independently measured `v06_work_inventory --summary` (the read-only
   binary, run directly — explicitly the brief's own carved-out exception, not "the dashboard
   producer") at **757.01s wall time on a confirmed-quiet box** (44Gi free, load 2.80/24, zero
   other cargo processes) — **over the 600s cap by 157s, a fixed shortfall, not a load spike.**
   Not re-run this cycle (no new information a third measurement would add, and it costs ~13
   minutes of wall clock this lane cannot spend on a figure that hasn't changed — the code
   computing the cap and the code being timed are both byte-identical to wave-26's own
   inspection, confirmed via `git log --oneline -3 -- scripts/observer/pf1e_dashboard_producer.py
   scripts/publish-site-dashboard.sh`: the 3 most recent commits touching either file
   (`19d1c6fdcf`, `b2805a0b95`, `58b4f837cc`) all predate wave-25's own ledger entry
   (`bc9e84553e`) — nothing has touched either file since the original wave-23/24 "gate lane B"
   round, wave-26's measurement included).
5. **`_load_cached_dump()`'s own fallback (lines 561-597) is what makes this silent, not just
   slow:** on a `_run_state_dump()` timeout, it returns `None`, and the caller falls back to
   whatever stale cache is on disk (`cached`) rather than raising — "a STALE cache is
   deliberately preferred over `None`: a blank panel renders as 'not started'". This means a
   **real, write-mode** run of the producer would not error either — it would silently keep
   serving a week-plus-old work-inventory snapshot merged with fresh class/content-state data,
   which is exactly the "can silently drop stamps" shape the brief's hazard note names.

**Conclusion: this cannot be closed from this lane without doing one of the two things the
brief explicitly forbids** — running the dashboard producer/regenerator, or raising
`PF1E_CLASS_STATE_TIMEOUT`/`CLASS_STATE_BUILD_TIMEOUT_SECONDS` past 600s (even narrowly, only
for the work-inventory binary, which would still be "raising the cap to make it pass" in
substance). **Both are excluded by name in this cycle's own brief**, so I did neither, and did
not attempt any variant of running `publish-site-dashboard.sh` in any mode this cycle (matching
wave-24's own precedent for the identical reason).

**This is registered, inherited debt, not a gap this bundle's own scope silently created.**
`forward-scope-register.md`'s "Carried forward from SD-33" table already lists `site-dashboard-
check hang` by name, with the exact same root cause description ("`publish-site-dashboard.sh
--check` invokes `v06_work_inventory --summary` with no timeout wrapper... a 600s producer
timeout"), observed **three separate times during SD-33's own closure** before this bundle
started, and its own stated owner is **"A future SD-N, or whichever cycle next touches
`scripts/verify.sh`'s stage list"** — not this lane, whose territory is `apps/desktop/` and
`site/`, explicitly excluding `scripts/verify.sh`. Combined with this bundle's own three
independent hits this wave (wave-26, wave-27, wave-28) plus SD-33's three, this key has fired
**6+ times** — `decisions.md §12` L5 / `AGENTS.md` rule 8 both say a recurrence at that
frequency needs a mechanical control, not another restated warning; the mechanical control
(splitting the work-inventory timeout from the shared 600s cap, with its own outer wrapper on
the `site-dashboard-check` stage the way `corpus-trap-audit` already got one) is a
`scripts/verify.sh` change — outside this lane's granted territory, and exactly the register's
own named next-owner.

**Who can close it:** either (a) the operator, running `./scripts/publish-site-dashboard.sh`
directly and interactively, able to accept a run past 600s and to diff `work_inventory_panel`'s
unit totals before/after to confirm no stamp silently dropped, or (b) a future cycle explicitly
scoped to edit `scripts/verify.sh` and `scripts/observer/pf1e_dashboard_producer.py` (per
`forward-scope-register.md`'s own named owner) to give `site-dashboard-check` its own outer
timeout wrapper and give `v06_work_inventory`'s dump its own, wider, deliberately-reviewed
timeout separate from the two cheaper dumps' shared 600s — an architecture fix, not a
cap-bump-to-pass.

## Standing gates re-checked this cycle (my own territory's slice; not re-verifying lanes A/C's stages)

- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `files_checked=16 violations=0`.
- `cargo run --locked --bin corpus_literal_sweep` (root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-lane-b-root`) →
  `48706 records examined of 51476 read, ... 0 findings` / `CLEAN`. Matches
  `BASELINE_CORPUS_LITERAL_RECORDS=48706` exactly, unmoved — this cycle wrote no
  `data/corpus/**` records (`decisions.md §12` L8: delta 0, records added 0, consistent).

- **Build scope verified:**
  - `cargo test --locked --no-run` (whole workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-lane-b-root`): **exit 0**, run at HEAD `3257813a4f` (this cycle's own last figure-moving commit — nothing landed after it).
  - `apps/desktop/src-tauri` (separate cargo workspace): **exit 0**, `cargo test --locked --no-run` run explicitly (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-lane-b`), same HEAD; full `cargo test --locked` also run explicitly (not just `--no-run`): **572 passed / 0 failed**, same HEAD.
- **Sweep population:** `corpus_literal_sweep` 48706 → 48706 (unmoved; 0 `data/corpus/**` records added or removed this cycle — N/A delta, matches pinned baseline exactly).
- **Oracle pin:** N/A — no figure in this receipt is sourced from the pinned PCGen oracle corpus; the PU record counts are the repo's own `data/corpus/` and `apps/desktop/src-tauri` test output.
- **Status:** partial
- **Movement, four buckets:**
  - **Closure:** `desktop` (1 verify.sh stage) — genuinely closed this cycle, a real fix (not a re-pin without proof; the new literal was derived from a live re-walk, not asserted).
  - **Reclassification:** the wave-25-vs-wave-28 "contradiction" reclassified from an instrument disagreement into two correct-at-the-time reports separated by a real, dated, named intervening commit (`e5fd8dddb1`) — no instrument was ever wrong.
  - **Reachability:** none this cycle.
  - **Instrument-correction:** `corpus_ingest_diagnostic.rs`'s hardcoded `corpus_only_records` pin for `pathfinder_unchained`, `1144 → 1140`, re-derived fresh against a live walk per `decisions.md §17a`, not asserted.
- **Notes:** `site-dashboard-check` is **not** closed this cycle, deliberately, per the brief's own two named prohibitions (do not run the producer/regenerator; do not raise the cap) — both would have been the only two paths to green. This is not a judgment call under dispute; `forward-scope-register.md` already names this exact defect as inherited, registered SD-33 debt with a stated owner outside this lane's territory. `kanban.md` row 26 (`final-acceptance-scan`) is intentionally **not** touched this cycle — no board row tracks individual gate-remediation sub-waves, matching wave-24/25/26/27/28's own established precedent (see `progress.md`'s repeated "kanban.md not touched" notes).
- **Next-cycle plan:** the whole-gate re-measure (lane C's own "then sweep and report honestly" obligation this wave) should now find **2 red of 40**, not 3 — `root-full` (lane A's own territory this wave) and `site-dashboard-check` (this lane's, correctly not closable here). `desktop` should now be counted PASS in that re-measure; if it is not, that is a new regression to name, not a repeat of this cycle's already-settled finding. Whoever owns `scripts/verify.sh` next should read this receipt's Part 2 before touching `run_site_dashboard_check` or the producer's timeout constants.
