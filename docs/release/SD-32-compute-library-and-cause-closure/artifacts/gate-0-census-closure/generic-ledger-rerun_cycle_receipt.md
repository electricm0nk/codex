# Cycle `generic-ledger-rerun` — measurement only, `decisions.md §17` item 3

- **Card ID:** none closed — measurement cycle per the dispatch brief; rows 11 and 15 left
  `in-progress` (row 15's own bar not met, see memo §3)
- **Commit SHA:** see push step below (this receipt lists the file set; SHA recorded once pushed)
- **Files touched:**
  - `scripts/generic_pass_state_rederive.py` (new) — committed re-derive script: re-runs
    `shape_ledger.py`, `shape_coverage_standing_gate.py`, `card15_reconcile.py`, and the T2a/T12
    corpus join live, in one command
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/generic-pass-state.md` (new) — the memo
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` — this entry appended
  - `docs/retro/events/generic-ledger-rerun.jsonl` (new, auto) — one harmless preflight-oracle
    verify.sh side-effect from oracle bootstrap on a fresh worktree
  - `docs/retro/events/sd31-transcribe.jsonl` — 4 harmless `verify.sh --only` preflight/gate
    side-effect appends, same shape prior receipts in this bundle note
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (script self-healed off an initial
  sd32-prefixed working filename, which tripped the audit's own bundle-tag pattern — same
  precedent as the `t2b-census` cycle's rename; renamed to `generic_pass_state_rederive.py` before
  committing, all in-doc references updated, and the two internal scratch-file names inside the
  script itself narrowed off the same prefix too)
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** dispatch brief items 1-6 (re-run shape ledger, run/cross-check
  `card15_reconcile.py`, report card 15's bar honestly, re-derive card 11's five shapes, produce a
  mechanism-sized replacement estimate, note Gate 3 without touching its budget)
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
  fresh worktree, oracle slot empty, bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest
  <repo-local pcgen slot>`, `scripts/verify.sh --only preflight-oracle` PASS
- **Status:** complete (this cycle's own measurement scope)
- **Notes:** two prior same-bundle cycles' self-reported figures (generic-enumeration `8e98424eb`,
  generic-spell-ingest `dcbcd803f`) were re-derived independently rather than trusted, per the
  dispatch brief's own instruction — every figure in the memo either matches an independent
  re-derivation exactly or is explicitly marked "not independently re-run" with the reason.
- **Discovery forwards:**
  1. T12 grew 2,453→2,515 (+62) from `inner_sea_magic`'s Gate-0 book onboarding (a prior,
     concurrent cycle on this same branch), not from anything this cycle touched — traced to the
     unit-id level, all 62 new, 0 removed.
  2. T2a residual correspondingly shifted 2,775→2,716 (the T12 overlap grew, not any T2a unit
     individually resolving).
  3. Card 15's own `remaining_undisposed: 0` arithmetic is honest but does not mean the acceptance
     bar is met — 24,117 units still carry no family; the memo states this explicitly (§3) rather
     than reading `remaining_undisposed: 0` as closure.
  4. Confirmed independently (own `cargo test` run, not re-quoted) that
     `ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status` is still red at this
     cycle's own tip, caused by `8e98424eb` per a concurrent lane's own receipt
     (`unred-branch_cycle-1_cycle_receipt.md`, pulled in by this cycle's rebase). Not fixed —
     `Kind`/`refine_kind` machinery is a concurrent lane's scope. Does not affect any figure in the
     memo (the data is correct; the test assertion is stale).
- **Next-cycle plan:** the memo's §7 table — 11 named mechanisms, largest being `Kind::Ability`
  (5,886 units) and the `is_internal_category` narrowing (2,574 units), both reusing classifiers
  that already exist and are proven. Dispatch by mechanism, not by book.
