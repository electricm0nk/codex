---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Release Notes — build 0.14.0

Written at closure by `AT-34-E6-003`, not before.

**Required content:**

- What shipped, led by **the Completion Atlas** — the count of remaining steps by bucket,
  what the Core Rulebook cost to finish, and what the 35 remaining books will cost. Then the
  defects found. Not process narrative.
- Every figure states its denominator in the same construct. `scripts/verify.sh --only
  denominator-gate` scans this file; a bare hundred-percent token is caught specifically.
- The PR number, also recorded in `receipts.md`.
- Version confirmation: `apps/desktop/package.json` and
  `apps/desktop/src-tauri/tauri.conf.json` both at `0.14.0`. **The tranche digit is not
  bumped at closure** — it moves only on a new `tranche/N` cut (`decisions.md §11`).

*(empty — closure epilogue writes here)*
