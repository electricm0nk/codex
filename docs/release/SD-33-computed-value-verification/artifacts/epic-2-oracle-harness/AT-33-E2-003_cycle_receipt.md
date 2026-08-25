# Cycle AT-33-E2-003 — Epic 2 Oracle harness / AT-33-E2-003

- **Commit SHA:** (recorded post-commit — see `progress.md`'s pointer entry for this cycle, added in the same commit)
- **Files touched:**
  - `scripts/oracle_harness/__init__.py` (new)
  - `scripts/oracle_harness/compare.py` (new)
  - `scripts/oracle_harness/oracle_export.py` (new)
  - `scripts/oracle_harness/run.py` (new)
  - `scripts/tests/test_oracle_harness.py` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.ours-sample.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.ours-sample-with-bug.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/AT-33-E2-003_cycle_receipt.md` (this file)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E2-003 — the comparison harness answers the per-unit question
  >
  > Given a unit, the harness returns `(ours, oracle, agree|disagree|unverifiable)`. `unverifiable` is a first-class return, never an error swallowed into `agree`.
  >
  > **Evidence:** a committed fixture set exercising all three outcomes, including at least one **known-disagreeing** case. A harness that has never returned `disagree` has not been shown to be capable of it.
  >
  > **Fixture discipline** (`stc-authoring`): a fixture's expected value is transcribed from bytes the harness's own read path does **not** touch. A fixture built from the file the harness reads is a mirror, not a check.

## What landed

`scripts/oracle_harness/` — a new package: `compare.py`
(`normalize_numeric`, `compare_unit`, `run_comparison`), `oracle_export.py`
(`parse_oracle_export`/`load_oracle_export` — the one read path for
PCGen's `KEY=VALUE` BatchExporter output), and `run.py` (a CLI producing
`{"results": [...]}` in the exact shape
`scripts/box_ledger.py::load_oracle_results` (`AT-33-E1-002` condition 3)
reads.

`compare_unit(unit_id, ours, oracle)` returns `{"unit_id", "ours",
"oracle", "verdict"}` where `verdict` is `"agree"`, `"disagree"`, or
`"unverifiable"`. `oracle=None` or a blank oracle value returns
`unverifiable` as a **normal return value** — never raised as an exception,
never coerced to `"agree"` (`test_unverifiable_is_not_an_exception` proves
this directly).

**RED → GREEN:** `python3 -m unittest scripts.tests.test_oracle_harness`
failed with `ImportError: cannot import name 'compare' from
'oracle_harness'` before the package existed — the intended reason (nothing
to import yet). 16/16 green after implementation.

**Fixture discipline, held two ways:**

1. `CompareUnitTest`'s `oracle=...` literals were hand-transcribed, by
   reading the real committed `pf1_fighter_l1.computed.txt` bytes with the
   `Read` tool, into Python literals in the test source — the test never
   opens that file. `OracleExportParsingTest` (which *does* exercise
   `parse_oracle_export`, the harness's real read path) uses only an inline
   string literal, never the committed file. The two classes are proven
   independently so neither can mirror the other or the code under test.
2. **Live, tool-level second check** (beyond the unit tests): `run.py` was
   run for real against the real committed `pf1_fighter_l1.computed.txt`,
   and its output was fed to the real `scripts/box_ledger.py --check`
   (`AT-33-E1-002`'s own oracle-disagreement gate) — see
   `oracle-comparison-fixtures.md` for the full transcript. This is an
   end-to-end proof, not a unit-test-only claim.

**All three verdicts exercised, including a KNOWN-DISAGREEING case:**

- `agree`: `test_agree_numeric`/`test_agree_signed_numeric`/`test_agree_string`
  (unit tests); `fixtures/pf1_fighter_l1.ours-sample.json` → 4 of 5 units
  agree (live CLI run).
- `disagree`: `test_disagree_known_case` (unit test, `ours=2` vs the real
  transcribed oracle `+1` for BAB); `fixtures/pf1_fighter_l1.ours-sample-with-bug.json`
  → 1 of 5 units disagrees (live CLI run), **which then makes
  `box_ledger.py --check` exit 1** — the real fail-closed gate firing on a
  real disagreement record this cycle's own tool produced.
- `unverifiable`: `test_unverifiable_no_oracle_value`/`test_unverifiable_is_not_an_exception`
  (unit tests); both live CLI runs above also carry 1 of 5 units
  `unverifiable` (`SPELLDC.0`, a token the template never emits for a
  non-caster).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Unit test suite (new) | 16 passed, 0 failed | of `scripts/tests/test_oracle_harness.py`'s own 16 cases | `python3 -m unittest scripts.tests.test_oracle_harness -v` |
| Unit test suite (existing, re-run for regression) | 25 passed, 0 failed | of `scripts/tests/test_box_ledger.py`'s own 25 cases | `python3 -m unittest scripts.tests.test_box_ledger -v` |
| Combined suite | 41 passed, 0 failed | of both files' combined 41 cases | `python3 -m unittest scripts.tests.test_oracle_harness scripts.tests.test_box_ledger -v` |
| Live CLI run, agree-only `ours` file | agree=4, disagree=0, unverifiable=1 | of 5 units in `fixtures/pf1_fighter_l1.ours-sample.json` | `python3 scripts/oracle_harness/run.py --oracle-export docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt --ours docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.ours-sample.json --output docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo.json` |
| Live CLI run, known-disagreeing `ours` file | agree=3, disagree=1, unverifiable=1 | of 5 units in `fixtures/pf1_fighter_l1.ours-sample-with-bug.json` | same command with `ours-sample-with-bug.json` |
| `box_ledger.py --check` on the agree-only run | exit 0, `oracle_disagreement=0` | of the 5-unit run above | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo.json` |
| `box_ledger.py --check` on the known-disagreeing run | exit 1, `oracle_disagreement=1` | of the 5-unit run above | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle builds and proves the comparison instrument; it moves no inventory unit's status (the demo fixtures use synthetic `unit_id`s scoped to this cycle's `.pcg`, not real `docs/work-inventory.json` units).
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- `run_comparison`'s `ours` argument shape (`unit_id -> (oracle_export_key,
  ours_value)`) is deliberately simple — Epic 5's real invocation will need
  a richer mapping from `docs/work-inventory.json` unit ids to the correct
  oracle-export token per unit's `.pcg`/template; that mapping is Epic 5's
  own scope (per-unit fixture authoring), not something this cycle
  pre-builds, per `oracle-comparison-fixtures.md`'s AT-33-E2-004 note.
- `normalize_numeric` handles PCGen's signed-string export convention
  (`"+3"`, `"-2"`) as well as plain integers/floats, since every BAB/mod/save
  token in the real export uses that convention.

## Next-cycle plan

`AT-33-E2-004` (same cycle, same commit) records the Path A/B ruling.
