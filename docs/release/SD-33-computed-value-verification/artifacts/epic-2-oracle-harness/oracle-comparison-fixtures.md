# AT-33-E2-003 / AT-33-E2-004 — comparison harness fixtures + Path A ruling

## AT-33-E2-003 — the harness answers `(ours, oracle, agree|disagree|unverifiable)`

**Code:** `scripts/oracle_harness/` (new package):

- `compare.py` — `normalize_numeric`, `compare_unit`, `run_comparison`.
- `oracle_export.py` — `parse_oracle_export`/`load_oracle_export`, the one
  read path for PCGen's `KEY=VALUE` BatchExporter output.
- `run.py` — CLI: `--oracle-export`, `--ours`, `--output` →
  `{"results": [...]}` in the exact shape
  `scripts/box_ledger.py::load_oracle_results` reads.

**Unit tests:** `scripts/tests/test_oracle_harness.py`, 16/16 green
(`python3 -m unittest scripts.tests.test_oracle_harness -v`). Exercises all
three verdicts, including `test_disagree_known_case` (a KNOWN-DISAGREEING
case) and two tests proving `unverifiable` is returned as data, never
raised.

**Fixture discipline** (`epic-breakdown.md` AT-33-E2-003, `stc-authoring`):
every `oracle=...` literal in `CompareUnitTest` was hand-transcribed, by
reading the real committed `pf1_fighter_l1.computed.txt` bytes with the
`Read` tool, into a Python literal in the test source. The test never opens
that file at run time, and `oracle_export.parse_oracle_export` (the
harness's actual file-reading code) is exercised only in the separate
`OracleExportParsingTest` class, against an *inline* string literal, never
against the committed file either. Neither test class can therefore mirror
the other, or mirror the code under test.

**Real end-to-end run against the real oracle export** (not just unit
tests — this is the harness's CLI, run against `AT-33-E2-002`'s real
committed BatchExporter output):

```bash
$ python3 scripts/oracle_harness/run.py \
    --oracle-export docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt \
    --ours docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.ours-sample.json \
    --output docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo.json
oracle_harness: 5 units compared -- agree=4 disagree=0 unverifiable=1 -> ...
```

`fixtures/pf1_fighter_l1.ours-sample.json` supplies HP/AC.TOTAL/BAB/VAR.CMB
(all matching the real export → `agree`) plus one unit
(`SPELLDC.0`, a token this template never emits — a level-1 Fighter has no
spell DC) → `unverifiable`.

**The known-disagreeing case, run through the real CLI and the real
`box_ledger.py` gate** (not just the unit test): `ours-sample-with-bug.json`
claims `BAB=2`; the real oracle value is `+1`:

```bash
$ python3 scripts/oracle_harness/run.py \
    --oracle-export docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt \
    --ours docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.ours-sample-with-bug.json \
    --output docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json
oracle_harness: 5 units compared -- agree=3 disagree=1 unverifiable=1 -> ...

$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.oracle-results-demo-DISAGREE.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: pf1_fighter_l1.BAB
$ echo "exit=$?"
exit=1
```

This proves the full chain live: harness → `disagree` verdict →
`box_ledger.py`'s `AT-33-E1-002` condition-3 gate fires and fails closed
(exit 1) on a real disagreement record produced by this cycle's own tool,
not a synthetic gate-only fixture.

The `agree`-only run against `box_ledger.py --check` (same command, the
non-buggy `ours` file) exits `0`:
`uncovered=0 overlap=0 population=49438 oracle_disagreement=0
unverifiable_done=0 stale=False`.

## AT-33-E2-004 — Path A / Path B ruling

**RULING: Path A.**

The pinned PCGen builds headless on this box (`AT-33-E2-001`) and a
hand-authored `.pcg` round-trips through `BatchExporter` producing real,
independently-cross-checked computed values (`AT-33-E2-002`). All three
named risks resolved in Path A's favor — none forced a fallback to Path B
(source-reading). The comparison harness (`AT-33-E2-003`) is built and
proven against real Path A output, including a real `disagree` case fed
through the real `box_ledger.py` gate.

**Consequence for Epic 5's throughput:** none of the kind negative — Path A
being live means Epic 5 (`AT-33-E5-001`/`002`, re-verifying the 8,330
fixture-/literal-blessed units against the oracle) can run the live-PCGen
path at full throughput rather than falling back to Path B's per-shape
manual Java-source reading, which `decisions.md §5` names as "slower and
per-shape." **No escalation is filed** — Path A being available is the
condition under which `decisions.md §5`'s revisit clause does *not* fire
("If Path A fails, ... that is an operator decision point"). Epic 5 will
still need, per-unit, an authored `.pcg` (or a batch of them) and a
BatchExporter template covering that unit's specific computed tokens — this
cycle proves the *mechanism* works, not that every unit already has an
authored fixture; that authoring cost is Epic 5's own scope, not a Path A/B
fork.
