# Cycle AT-34-E5-001 — Epic 5 (Price the remaining 35 books) / AT-34-E5-001

- **Commit SHA:** `cf4a60a7dd21b4842d642f9f1321763b21efaee0`
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_forward_plan.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_forward_plan.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/forward-plan.json` (new, generated)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-001_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (re-run after commit, on the committed diff)
- **Wired-integration audit result:** OK_NO_TOKENS (re-run after commit, on the committed diff)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**Evidence:**
  `artifacts/epic-5-forward-plan/forward-plan.json` — per book, per bucket: unit count, the
  mechanism that clears it, and the projected cost using the **measured** rates from Epics 2, 3
  and 4. Every projection names the rate it used and the sample size behind it. **A projection
  built on a thin sample says so in its own row.** A confident number from a thin sample is a
  failure this program has hit repeatedly."

## Figures + their re-derive commands

- **35 non-vehicle books, 37 total inventory books minus `core_rulebook`/`ultimate_campaign`:**
  `python3 scripts/completion_atlas.py --by-book | wc -l` → 37 lines; 37 − 2 = 35.
- **29,364 non-DONE units across the 35 books** (denominator: the 35-book population, itself 42,472 of the corpus's 49,438) — `python3 scripts/completion_atlas.py --check` → `population=49438`; `python3 scripts/completion_atlas.py --by-book` summed per-book, minus
  `core_rulebook` and `ultimate_campaign` rows. Cross-checked two ways inside
  `build_forward_plan.py` (an assertion compares the sum of per-book `remaining_non_done`
  against the sum of `totals_by_bucket`; both scripts agree: 29,364, per the same `completion_atlas.py --by-book` command above).
- **Remaining by bucket, 35 books (denominator: 29,364 non-DONE units in the 35-book
  population):** A=449, B=11,299, C=3,981, D=2,668, M=3,977, V=6,747, U=171, X=53, Z=19.
  Re-derive: `python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_forward_plan.py`
  then read `forward-plan.json`'s `population.remaining_by_bucket`.
- **Three pricing tiers (denominator: 29,364 non-DONE units in the 35-book population), never
  blended into one number:**
  1. **11,919 units (A+B+U) priced to DONE** with a measured rate.
  2. **3,981 units (C) priced only to V, not DONE** — `core_rulebook`'s one bucket-C cycle
     moved all 42 of its units to bucket V, none reached DONE directly; the rate exists but for
     a different endpoint, so this tier is reported separately rather than folded into either
     the DONE-hours total or the unpriced count.
  3. **13,464 of 29,364 (45.9%) carry NO measured rate at all** (buckets D, M, V, X, Z — zero
     dedicated clearing cycles have run against either vehicle book for any of these five
     buckets; `step-cost-ledger.json`'s own `buckets_not_yet_cleared` sections for both
     `core_rulebook` and `ultimate_campaign` state this directly).
  - 11,919 + 3,981 + 13,464 = 29,364 (self-checked by an assertion inside
    `build_forward_plan.py`). Re-derive: same command, read `summary.priced_to_done_units`,
    `summary.priced_to_v_not_done_units`, `summary.unpriced_units`.
  - **Correction recorded this cycle:** the first draft of this script folded bucket C's 3,981
    units into "unpriced" (the correct figure, 13,464 of 29,364 (45.9%), was misreported as
    17,445 of 29,364 (59.4%)) because the aggregation loop checked for keys
    `projected_cost_hours`/`projected_cost_hours_range` only, missing bucket C's
    distinctly-named `projected_cost_hours_to_reach_V_not_DONE` key. Caught before commit by
    hand-checking A+B+C+U against the printed unpriced total; fixed by adding an explicit third
    tier. Logged via `scripts/retro.py correction` (see Notes).
- **Bucket-A table-build rate: 136.857 s/table, sample_size=7** (Epic 2's 7 new-this-bundle
  tables, `companion` excluded as pre-existing/0-cost): re-derive by reading
  `artifacts/epic-2-tables/table-build-rate.json`'s `tables[].est_wall_time_seconds`, mean of
  the 7 `new_this_bundle` entries. Marked ESTIMATE in its own source (pro-rated, not
  independently timed).
- **Bucket-B rate range: 1.667–5.8 units/hour reaching DONE, sample_size=2 measurements**
  (`core_rulebook`: 29 cycles/503 net units/235 DONE; `ultimate_campaign`: 1 cycle/5 units/3
  DONE): `artifacts/epic-3-core-rulebook/step-cost-ledger.json` and
  `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json`, `.buckets_cleared_so_far.B`.
  **Thin sample, flagged in every bucket-B row** — Epic 4's own ledger concludes the 3.5×
  divergence is a sample-size artifact, not book-shape, and a single blended figure
  misrepresents any one book given Core Rulebook's own 28× per-mechanism spread.
- **Bucket-C rate: 41.3 units/hour net-reclassified-to-V, 0.0 units/hour reaching DONE,
  sample_size=1 cycle:** `artifacts/epic-3-core-rulebook/step-cost-ledger.json`
  `.buckets_cleared_so_far.C` (42 units, 61.0 minutes, all moved to bucket V not DONE).
- **Bucket-U rate: 40.28 units/hour reaching DONE, sample_size=2 cycles (48 units, 71.5
  minutes), `core_rulebook` only:** same ledger, `.buckets_cleared_so_far.U`.
- **Buckets D, M, V, X, Z: no measured rate exists (sample_size=0 for each)** — verified by
  reading both step-cost-ledger.json files' `buckets_not_yet_cleared` sections; every row in
  `forward-plan.json` for these buckets carries `"rate": null` plus an explanatory note citing
  the source, never a silently-absent field.
- **Priced-to-DONE projected-hours range: 1,952.42–6,782.37 hours** for the 11,919 A+B+U units
  only, across all 35 books — **not a total-program estimate**: bucket C's 3,981 units add a
  further **96.39 hours to reach V (not DONE)**, and 45.9% of the population (13,464 units, the
  unpriced tier) has no rate at all, so no single blended total-hours figure is asserted (see
  `summary.note` in the artifact). Re-derive: same build command, read
  `summary.priced_to_done_projected_hours_range` and `summary.priced_to_v_not_done_hours`.

## Row-count command output

```
$ python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_forward_plan.py
PASS: 35 books, all bucket counts match live completion_atlas.py, every row carries a rate-or-note
```

## Build scope verified

- `cargo test --locked --no-run` — see structured return value for exit code and SHA (this
  epic's file-touch set is `artifacts/epic-5-forward-plan/` only, JSON/Python, no Rust touched;
  run for the workspace-wide gate per `decisions.md §10` regardless).
- `apps/desktop/src-tauri`: not touched by this cycle — not re-run; workspace `cargo test
  --locked --no-run` covers the required minimum for a cycle that touches zero Rust.

## Sweep population

N/A — this cycle adds no `data/corpus/**` records. `corpus_literal_sweep`'s examined-population
is unchanged by this cycle.

## Oracle pin

N/A — no figure in this receipt derives from the pinned PCGen corpus; all figures derive from
`docs/work-inventory.json` and this bundle's own Epic 2/3/4 measured-rate artifacts.

- **Status:** complete

## Movement, four buckets

- **Closure:** 0 units closed this cycle (Epic 5 is a pricing exercise, not a clearing cycle —
  its file-touch set is `artifacts/epic-5-forward-plan/`, read-only against the rest of the
  repo, per `workflow-instruction.md §3`).
- **Reclassification:** 0.
- **Reachability:** 0 — no reachability probe run this cycle.
- **Instrument-correction:** 0 — this is a measurement-wave deliverable that banks zero cleared
  units by design (`decisions.md §12` L6: "measurement waves that bank zero units are
  legitimate deliverables").

## Notes (judgment calls)

- **Epic 5 is nominally gated on Epics 3 and 4 reaching zero** (`epic-breakdown.md`'s Epic 5
  header: "Gated on: Epics 3 and 4"), and at the time of this cycle neither has reached zero
  (`kanban.md` rows 13–17, 20: `in-progress`/`partial`). This cycle was dispatched directly at
  AT-34-E5-001 regardless. Resolution: AT-34-E5-001's own evidence text asks for "the projected
  cost using the **measured** rates from Epics 2, 3 and 4" — it does not require those epics to
  have reached zero, only that real measured rates exist to price from. Epics 2, 3, and 4 have
  each produced at least one real measured-rate artifact already (`table-build-rate.json`, both
  `step-cost-ledger.json` files), so the pricing input this criterion needs is present. The
  honest consequence, stated plainly rather than smoothed over: because Epics 3/4 have not run
  dedicated clearing cycles for buckets D, M, V, X, or Z, **45.9% of the 35-book population (unpriced tier) has
  no measured rate to price from today**, and this plan says so per-bucket rather than
  inventing one. If Epics 3/4 later run dedicated D/M/V/X cycles, re-running
  `build_forward_plan.py` picks up the new rates automatically — the script reads the live
  step-cost-ledger files, not a copied figure.
- **`companion` (28 units, `bestiary` only) sits in bucket A** even though a `companion` table
  already exists (built pre-SD-34, per SD-29). Evidence string is
  `companion_content_has_no_engine_table` — the existing table does not cover this shape of
  `bestiary` companion record. Priced using the same generic table-build rate as `power` for
  lack of a more specific measurement, with the mismatch called out explicitly in that row's
  `note` (the analogy is imperfect: extending an existing table is not proven to cost the same
  as building a new one from scratch). This capability gap is AT-34-E5-002's scope to name
  formally in `capability-register.json`; this receipt flags it so that criterion's author does
  not have to re-discover it.
- **No blended single "hours to finish everything" figure is stated.** Given 13,464 of 29,364
  units (45.9%) are unpriced, another 3,981 of 29,364 (13.6%, bucket C) are priced only to a
  different endpoint (V, not DONE), and the remaining priced 11,919 of 29,364 (40.6%) already
  carries a 3.5× thin-sample range on its largest bucket (B), a single headline number would be
  exactly the "confident number from a thin sample" failure the criterion's own evidence text
  warns against. The artifact's `summary.note` states this explicitly.

## Next-cycle plan

- **AT-34-E5-002** (capability register) should formally register the `bestiary` `companion`
  table-shape gap found in this cycle's notes, alongside `power`.
- **AT-34-E5-003** (`power` table costing) can consume this cycle's bucket-A rate figure
  (136.857 s/table, n=7) directly rather than re-deriving it.
- **AT-34-E5-004** (cheapest-first ordering, single-bucket-book flagging) can build directly on
  `forward-plan.json`'s per-book `buckets` maps — a single-bucket book is any row whose
  `buckets` dict has exactly one key (e.g. `beginner_box`: `Z` only, 19 units).
- If Epic 3 or Epic 4 lands a dedicated D/M/V/X clearing cycle before Epic 5 closes,
  **re-run `build_forward_plan.py`** — its rates read the live ledgers, not a snapshot, so the
  13,464-of-29,364-unpriced (45.9%) and 3,981-of-29,364-priced-to-V-only (13.6%) figures will
  shrink automatically and should be re-quoted, never carried forward from this receipt
  (`decisions.md §12` L2).
