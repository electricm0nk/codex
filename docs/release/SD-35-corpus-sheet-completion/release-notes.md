---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Release Notes — build 0.15.0

Written at closure by `AT-35-E7-003`, not before.

**Required content:**

- What shipped, led by **the corpus at `DONE=49438 of 49438`** under the sheet rule
  (`decisions.md §1`), then the converter, the live evaluator, and the on-screen section, then the
  process figures: units per cycle (min / median / max), builds per cycle, build time before
  and after AT-35-E1-003. Not process narrative.
- The `Words` share per kind at closure (`risks-and-open-questions.md §2`).
- **The PCGen exit:** `pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0`;
  the residue count per epic from the first-run baseline; oracle parity before and after
  (`decisions.md §11`).
- Every figure states its denominator in the same construct. `scripts/verify.sh --only
  denominator-gate` scans this file; a bare hundred-percent token is caught specifically.
- The PR number, also recorded in `receipts.md`.
- Version confirmation: `apps/desktop/package.json` and
  `apps/desktop/src-tauri/tauri.conf.json` both at `0.15.0`. **The tranche digit is not
  bumped at closure** — it moves only on a new `tranche/N` cut (`decisions.md §10`).

*(empty — closure epilogue writes here)*
