# Cycle AT-33-E2-004 — Epic 2 Oracle harness / AT-33-E2-004

- **Commit SHA:** `84a5781c11`
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/oracle-comparison-fixtures.md` (new — carries the ruling, written in the same cycle as AT-33-E2-003)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated — records the ruling per this criterion's evidence requirement)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/AT-33-E2-004_cycle_receipt.md` (this file)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E2-004 — the Path A / Path B ruling is recorded and escalated
  >
  > Epic 2's closing receipt states Path A or Path B explicitly. **If Path B, the consequence for Epic 5's throughput is escalated to the operator as a decision point** (`decisions.md §5`) — never absorbed silently, never allowed to reduce the bundle to "coverage only" by drift.
  >
  > **Evidence:** the receipt, plus a `progress.md` entry naming the ruling and, if Path B, the escalation.

## Ruling: Path A

Stated explicitly, per the criterion's own evidence bar:

- `AT-33-E2-001`: the pinned PCGen builds headless on this box, by execution
  — 0 blockers among the three named risks.
- `AT-33-E2-002`: a hand-authored `.pcg` round-trips through
  `BatchExporter` via a hand-authored template, producing a real,
  independently-cross-checked (13 of 13 fields match) computed-value
  export.
- `AT-33-E2-003`: the comparison harness is built and proven — at the unit
  level (16/16 tests, all three verdicts) **and** at the live-tool level
  (`run.py` against the real committed export, feeding a real disagreement
  through `box_ledger.py`'s real fail-closed gate).

No fallback to Path B (source-reading, the `MaxCommand.java`-finding
method) was required at any point in this cycle.

## Consequence for Epic 5's throughput

**No degradation, no escalation filed.** `decisions.md §5`'s escalation
clause is conditioned on Path A *failing* ("If Path A fails, Epic 5's
throughput assumption changes and that is an operator decision point"). It
did not fail. Epic 5 (`AT-33-E5-001`/`002`, re-verifying the 8,330
fixture-/literal-blessed units against the oracle) can run the live-PCGen
export-and-compare path this cycle proved, rather than the slower,
per-shape Java-source-reading fallback `decisions.md §5` names for Path B.

**What this cycle does *not* claim**, stated explicitly so the ruling is
not mistaken for more than it proves: Path A being live is a mechanism
proof, not a throughput guarantee. Epic 5 still needs, per real unit under
re-verification, either (a) an authored `.pcg` exercising that unit plus a
template token covering it, or (b) a batch/bulk export strategy amortizing
one `.pcg`+template pair across many units of the same kind. This cycle
built and proved the mechanism on one hand-authored character; sizing and
executing that authoring effort across 8,330 units is `AT-33-E5-001`'s and
`AT-33-E5-002`'s own scope, not pre-decided here.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Named risks resolved without forcing Path B | 3 | of 3 named in `decisions.md §5` | `AT-33-E2-001_cycle_receipt.md` |
| Real oracle round-trips produced | 1 | of 1 attempted this cycle | `AT-33-E2-002_cycle_receipt.md` |
| Harness verdicts proven live against the real oracle export | 3 | of 3 (`agree`/`disagree`/`unverifiable`) | `AT-33-E2-003_cycle_receipt.md` |
| Path B fallback invocations this cycle | 0 | of 1 (Epic 2's own spike) | no Java-source-reading fallback file exists under `artifacts/epic-2-oracle-harness/` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this criterion is a ruling, not a unit-status change.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- `decisions.md §5`'s revisit condition ("Epic 2 declares Path A or Path B
  by its own closing receipt") is satisfied by this file plus
  `oracle-comparison-fixtures.md`'s "AT-33-E2-004" section, which carries
  the same ruling — kept in two places deliberately: this receipt is the
  per-criterion evidence `workflow-instruction.md §7`'s schema requires,
  `oracle-comparison-fixtures.md` is the artifact a reader following
  `README.md`'s "AT-33-E2-003 / AT-33-E2-004" pointer lands on first.

## Next-cycle plan

Epic 2 is complete (all four criteria, rows 5-8). Per `workflow-instruction.md §3`,
Epic 5 is gated on Epic 2 and can now be dispatched with the Path A ruling
in hand — no throughput-reduction decision to raise with the operator.
