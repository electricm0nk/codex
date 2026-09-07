# Cycle salvage-2026-08-30 — bucket-v-widen infrastructure (partial landing, data absent)

**Provenance.** This cycle salvages a second isolated worktree's uncommitted diff (session
exited without committing; a `git diff`-based patch of 18,268 lines). It touched
`src/bin/v06_work_inventory.rs` (widen the bucket-V oracle-disposition ledger loader to merge
multiple files), `build_capability_register.py` (generic wording update, already live-computed),
and claimed a new corpus-wide oracle ledger at
`artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json` (6,590 rows) plus
a receipt and verifier script at that same path.

**What did not survive the salvage.** The ledger JSON itself, its remainder file, its
`build_bucket_v_widen_ledger.py` / `verify_bucket_v_widen.py` scripts, and its own cycle receipt
are **absent from the patch and absent from the repo** — none of them are tracked files, so a
plain `git diff` never captured them; they were lost when the session exited uncommitted. This is
the same failure shape recorded elsewhere in this bundle ("untracked corpus litter", "lessons
travel with mechanisms or arrive as quotes") applied to a whole data artifact rather than a fix.

**Consequence, verified live.** `load_bucket_v_oracle_dispositions` now reads two paths — the
original `core_rulebook`-only ledger (present, 2,712 rows, untouched) and the widen path
(`artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json`, **does not
exist**). `load_oracle_dispositions_from_path` returns an empty map for a missing file by
contract (same discipline as `load_sweep_verified`/`load_derived_fixture_verified` — a missing
ledger must never be misread as evidence), so **this cycle's regeneration moved zero units outside
`core_rulebook`'s already-applied 2,712** — confirmed by the whole-corpus diff below.

## What was landed

- `src/bin/v06_work_inventory.rs`: `load_bucket_v_oracle_dispositions` widened from a single
  hardcoded path to a small, book-disjoint list of ledger paths
  (`BUCKET_V_ORACLE_RESULTS_ALL_RELATIVE_PATHS`), via a new `load_oracle_dispositions_from_path`
  helper — 2 new tests (`merges_core_rulebook_and_widen_ledgers_from_disk`,
  `missing_widen_ledger_does_not_blind_the_core_rulebook_ledger`), both passing.
  `apply_bucket_v_oracle_disposition_stamps` needed zero changes (book-agnostic already). This is
  safe, generic, well-tested infrastructure that is a genuine no-op today and activates
  automatically — no further code change — the moment a real widened ledger lands at that path.
- `build_capability_register.py`'s `oracle_probe_surface_for_no_table_kinds` capability: wording
  made generic (re-check `population_by_kind` live rather than trusting a hardcoded three-kind
  prose list) — this field was already `population_source: "live"`, so no behavior changed. The
  salvaged patch's claim that this was "widened... by the bucket-V-widen lane" and its dangling
  citation to the (non-existent) `bucket_v_widen_cycle_receipt.md` were both removed — see below.

## What was deliberately dropped, and why

- **The widened oracle ledger's claimed population/movement numbers** (kanban.md row
  `bucket-v-widen`: "V corpus-wide 6,846 → 256"; progress.md's matching "complete" cycle entry;
  `capability-register.json`'s `population: 2062` / 29-book `books_unblocked` list) — **dropped
  entirely, not landed.** The ledger file that would justify them does not exist in this repo. Per
  `decisions.md §19`, only a verdict SD-33's harness **actually produced** may be reused, and I
  cannot verify a verdict I cannot read. Landing these numbers would have been exactly the
  counterfeit-closure shape `decisions.md §4`/`§9` exist to catch — a claimed movement with no
  artifact behind it.
- **The dangling citation** to `artifacts/bucket-v-widen/bucket_v_widen_cycle_receipt.md` in
  `build_capability_register.py`'s `cited_from` field — removed; citing a file that does not exist
  is worse than citing nothing.
- **`kanban.md` row 28 and its matching `progress.md` cycle entry**, as originally salvaged — both
  removed for the same reason. No new row is added in their place; this receipt is the record of
  what actually happened.

## Figures + their re-derive commands

| Figure | Value | Command |
|---|---|---|
| Widen ledger file present? | No — `artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json` does not exist | `find . -iname '*bucket-v-corpus-wide*'` (from repo root) |
| `core_rulebook` bucket V | `81`, unchanged | `python3 scripts/completion_atlas.py --book core_rulebook --check` |
| Corpus-wide bucket V | `6,846`, unchanged from pre-cycle baseline | `python3 scripts/completion_atlas.py --check` |
| Whole-corpus units with any status change attributable to this widening | `0` | before/after diff by unit id (see sibling `AT-34-E4-002_cycle_receipt_2.md`'s 131-unit diff — none carry `oracle-agree`/`oracle-unverifiable` as their **new** status outside `core_rulebook`) |
| `oracle_probe_surface_for_no_table_kinds` capability population | `130` (unchanged — `core_rulebook`-only, `ability`/`companion`/`template`), `books_unblocked: ["core_rulebook"]` | `python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_capability_register.py` |
| `verify_capability_register.py` | `PASS: 11 capabilities named, X-bucket reconciliation sums to live population (170), 0 flagged built_by_sd34=true` | same script |
| `cargo test --bin v06_work_inventory load_bucket_v_oracle_dispositions_tests` | 2 passed, 0 failed | `cargo test --bin v06_work_inventory load_bucket_v_oracle_dispositions_tests` |
| `apply_bucket_v_oracle_disposition_stamps_tests` (unchanged, re-run to confirm the widened loader did not disturb the invariant) | 5 passed, 0 failed, including `disagree_verdict_is_never_dispositioned_and_stays_outstanding` | `cargo test --bin v06_work_inventory apply_bucket_v_oracle_disposition_stamps_tests` |

## Sweep population / build scope

Shared with the sibling `AT-34-E4-002_cycle_receipt_2.md` — one three-pass regen and one
`cargo test --locked --no-run` covered both salvaged diffs in the same cycle (`workflow-instruction.md §6` step 3: run once, after the last figure-moving commit).

## Status: complete (infrastructure only — the disposition itself is not done)

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 0. **Reclassification:** 0. **Reachability:** 0. **Instrument-correction:** 0.
  A zero-movement cycle is a legitimate deliverable (`decisions.md §9`) — this cycle's honest
  result is "the pipe is built, nothing has flowed through it yet."

## Verification that `decisions.md §19`'s constraint holds in the merged code

- **A `disagree` is never dispositioned:** `apply_bucket_v_oracle_disposition_stamps_tests::disagree_verdict_is_never_dispositioned_and_stays_outstanding` passes against the widened loader unchanged.
- **Only verdicts SD-33 actually produced may be reused:** `load_oracle_dispositions_from_path`
  reads a ledger file verbatim (`unit_id` → `(verdict, reason)`) and never infers an entry for an
  id absent from it; a missing file contributes nothing (tested). Since the widen file is absent,
  **zero units outside `core_rulebook` are dispositioned by this cycle** — the strongest possible
  form of "never infer a verdict the ledger does not cover."

## Next-cycle plan

1. Build the actual corpus-wide bucket-V ledger (cross-reference every other book's bucket-V
   population against SD-33's own committed `AT-33-E5-003.combined-oracle-results.json`, plus
   AT-33-E1-003's probe-surface census for structurally table-less kinds), write it to
   `artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json`, and **commit
   it in the same cycle that builds it** — this cycle's whole finding is that an uncommitted data
   artifact is load-bearing and must not be left to a session that might exit uncommitted.
2. Re-run the three-pass regen and re-derive `completion-atlas.json` /
   `capability-register.json` / `forward-plan.json` once that ledger lands; report movement by
   book, by set, per `decisions.md §19`'s own disposition table shape.
