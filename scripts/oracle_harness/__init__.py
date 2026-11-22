"""SD-33 Epic 2 -- the oracle comparison harness (AT-33-E2-003/004).

Given a unit, answers `(ours, oracle, agree|disagree|unverifiable)`.
`unverifiable` is a first-class outcome -- never an error silently folded
into `agree` (`decisions.md` §7). See `compare.py` for the comparison logic
and `oracle_export.py` for parsing PCGen BatchExporter output
(`AT-33-E2-002`'s `KEY=VALUE`-per-line export shape) into the lookup table
`compare.run_comparison` needs.

`compare.run_comparison`'s output records are the exact `(unit_id, ours,
oracle, verdict)` shape `scripts/box_ledger.py::load_oracle_results` reads
(`AT-33-E1-002` condition 3) -- this package is what will eventually
populate
`docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/oracle-results.json`
for Epic 5's re-verification.
"""
