# SD-27 — Architecture Receipts Ledger

> **Append-only.** Written by the E4.2 architecture-closure gates, not by hand.
>
> This is **not** the per-cycle receipt store — those live at
> `artifacts/epic_<n>/<cycle>_receipt.md` (see `artifacts/README.md`). This file is
> the append-only ledger that `scripts/architecture-truth-up.sh` and
> `scripts/graphify-update.sh` require and append to; both locate the repo root by
> walking up from this path, so it must stay inside the repo.

<!-- Entries are appended below this line by the gates. -->

- cycle_id: 2026-07-28T12:00:13Z
  row_or_kind: architecture:truth_up
  bundle: SD-27
  branch: b1bd87c97e0c4c67cd20f229c85a2c2ef013e8c7
  integration_target: develop
  branch_tip_before: b1bd87c9
  branch_tip_after: b1bd87c9
  diff_path_count: 6774
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope
