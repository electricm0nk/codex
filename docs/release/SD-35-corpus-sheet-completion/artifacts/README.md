---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Cycle Artifacts

One subdirectory per epic, each seeded with a `.gitkeep`. Every cycle writes its receipt to
`artifacts/<epic-dir>/<criterion-id>_cycle<N>_receipt.md`, using the schema in
`../workflow-instruction.md §7`. A criterion that takes several cycles numbers them.

| Directory | Epic | Expected contents |
|---|---|---|
| `epic-1-tax-cut/` | 1 — Tax cut | `build-time.json` (before/after cold `--no-run` wall time, commands, load), `citation-anchor-proofs.md`, the consolidation's `cargo test -- --list` diff, the residue gate's first-run transcript |
| `epic-2-sheet-rule/` | 2 — Sheet rule | **`token-mapping/`** (pre-launch, 2026-09-08: four lane tables, four judge verdict files, `SYNTHESIS.md`, **`mapping-table.v1.json`** — the table AT-35-E2-001 transcribes — and `blockers.md`), **`token-coverage.json`** (re-derived every cycle from here on), the per-kind conversion-outcome counts, AT-35-E2-005's first-pass receipt with its wall time, **`oracle-parity-epic2.json`** |
| `epic-3-place-and-surface/` | 3 — Place and surface | `rate-ledger.json` (one row per cycle), receipts |
| `epic-4-resolve-and-verify/` | 4 — Resolve and verify | `rate-ledger.json`, `oracle-run-receipt.md` (with `PCGEN_ORACLE_SHA`, per-unit cost on the first 50, every disagreement named), per-cycle oracle comparisons |
| `epic-5-residues/` | 5 — Residues | `table-proofs.md`, **`completion-manifest.json`** (one row per unit), `capability-register-closed.json` |
| `epic-6-pcgen-exit/` | 6 — PCGen exit | **`oracle-parity-before.json`**, **`oracle-parity-after.json`**, the relocation's byte-identical `gen_book_cache` transcript, the closure-mode gate transcript |
| `epic-7-closure/` | 7 — Closure epilogue | final-acceptance scan receipt, closure-readiness report |

## The four artifacts that ARE the bundle

**`epic-2-sheet-rule/token-coverage.json`** — per PCGen token type: units carrying it, units
converted, units refused because of it, the converter mapping row that handles it. Sums
checked. The remainder is always named by token type.

**`epic-5-residues/completion-manifest.json`** — every one of 49,438 units, its final status,
and the evidence pointer establishing it. AT-35-E7-001 re-evaluates an independently drawn
sample of at least 200 `sheet-complete` units from their `SheetRule` files.

**`epic-6-pcgen-exit/oracle-parity-{before,after}.json`** — PCGen's exported totals for the
fixture roster against ours, at the start and end of the exit. **This is the operator's
permitted use of PCGen: testing that the rewrite is solid.** `disagree=0`, or every
disagreement resolved by a named commit.

**`epic-1-tax-cut/build-time.json`** — the cold build wall time before and after the
test-suite consolidation. The operator's complaint was a number; the answer is a number.

## Receipt requirements

Every receipt carries, per `../workflow-instruction.md §7`:

- the **scope gate** row — the literal `cycle_scope_gate.py --min 500 ...` output line
- the **receipt rows** — the literal `cycle_scope_gate.py --receipt` output: closed, relabeled,
  rust_lines_changed, ratio, builds_recorded, **pcgen_live_files**
- the **four buckets** row — closure / relabel / reachability / instrument-correction
- the **refused tokens** row — `<type>=<count>, ...` or `none`
- the **figures** row — every number with its re-derive command **and its denominator**
- the **build scope verified** row, naming the **SHA it ran at**
- the **sweep population** row when corpus records changed
- **`PCGEN_ORACLE_SHA`** on any figure from the pinned corpus or any oracle comparison

`- **Status:** <x>` is a **bullet**, not a `## Status` heading.
