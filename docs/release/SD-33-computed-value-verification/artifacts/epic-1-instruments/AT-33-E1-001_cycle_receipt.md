# Cycle AT-33-E1-001 — Epic 1 Instruments / AT-33-E1-001

- **Commit SHA:** `08bfa4931d` (the commit that lands this receipt and the files below, on `tranche/13`)
- **Files touched:**
  - `scripts/box_ledger.py` (new)
  - `scripts/tests/test_box_ledger.py` (new)
  - `docs/release/SD-33-computed-value-verification/THE-BOX.md` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/AT-33-E1-001_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E1-001 — `THE-BOX.md` exists as a living partition of the full inventory
  >
  > The document partitions **all 49,438 inventory units** — not the not-done subset — into named groups, each carrying a count, a disposition, and a re-derive command. `uncovered == 0` and `overlap == 0`.
  >
  > **Evidence:** `python3 scripts/box_ledger.py --check` exits 0 and prints `uncovered=0 overlap=0 population=49438`. A committed `THE-BOX.md` whose group counts sum to the stated whole.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Inventory population (live count of `units` array) | 49,438 | whole inventory | `jq '.units \| length' docs/work-inventory.json` |
| Inventory population (stated `totals.units` field, cross-checked, not trusted on its own) | 49,438 | whole inventory | `jq '.totals.units' docs/work-inventory.json` |
| Distinct `status` values across all units | 9 | whole inventory | `jq -r '.units[].status' docs/work-inventory.json \| sort -u \| wc -l` |
| Units with null/missing `status` | 0 | whole inventory | `jq '[.units[] \| select(.status == null)] \| length' docs/work-inventory.json` |
| Duplicate unit `id`s | 0 | whole inventory | `jq -r '.units[].id' docs/work-inventory.json \| sort \| uniq -d \| wc -l` |
| `grounded` group count | 3,234 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="grounded")] \| length' docs/work-inventory.json` |
| `literal-verified` group count | 6,589 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="literal-verified")] \| length' docs/work-inventory.json` |
| `fixture-verified` group count | 1,741 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="fixture-verified")] \| length' docs/work-inventory.json` |
| `ingested-magnitude` group count | 1,543 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="ingested-magnitude")] \| length' docs/work-inventory.json` |
| `text-complete` group count | 5,099 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="text-complete")] \| length' docs/work-inventory.json` |
| `deferred-with-reason` group count | 46 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="deferred-with-reason")] \| length' docs/work-inventory.json` |
| `not-ingested` group count | 26,943 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="not-ingested")] \| length' docs/work-inventory.json` |
| `not-started` group count | 19 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="not-started")] \| length' docs/work-inventory.json` |
| `unknown` (= `unverifiable` bucket) group count | 4,224 | whole inventory (49,438) | `jq '[.units[] \| select(.status=="unknown")] \| length' docs/work-inventory.json` |
| Sum of all 9 groups | 49,438 | matches population exactly | `jq '.units \| length' docs/work-inventory.json` (equal by construction; also proven live by `box_ledger.py --check`'s `overlap=0`/`uncovered=0`) |
| `box_ledger.py --check` against committed `THE-BOX.md` | `uncovered=0 overlap=0 population=49438` | full 49,438-unit inventory | `python3 scripts/box_ledger.py --check` |
| Unit test suite | 9 passed, 0 failed, 0 skipped | `scripts/tests/test_box_ledger.py`'s own case count | `python3 -m unittest scripts.tests.test_box_ledger -v` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle builds the instrument; it does not move any unit's status.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

No unit's status changed. This cycle's deliverable is the partition mechanism itself (`decisions.md` §4: a lesson without a mechanism is a quote — `THE-BOX.md` + `box_ledger.py` is the mechanism for "sum the piles, always").

## Notes

- **Population confirmed, not assumed.** Per the scope note, 49,438 was confirmed by two independent live commands (`jq '.units|length'` and `jq '.totals.units'`) before being trusted — both agree, so no correction was needed. `box_ledger.py.load_inventory()` computes population from `len(units)` at runtime, never by reading `totals.units` on trust, so this stays true on every future run even if the two ever diverge.
- **Partition design choice.** `docs/work-inventory.json`'s `status` field is already an exhaustive, non-overlapping 9-way partition of the full population (verified: 9 distinct non-null values, 0 duplicate `id`s, group counts sum exactly to 49,438). `THE-BOX.md`'s 9 groups map 1:1 onto those 9 status values. The **disposition** column is not a restatement of the status name — per `decisions.md` §7, `grounded`/`literal-verified`/`fixture-verified` are labelled "…, oracle-pending" rather than `done`, because none of the three has been checked against PCGen yet (Epic 2 builds that harness; Epic 5 owns re-examining the fixture- and literal-verified populations against it). Labelling them `done` here would be exactly the over-claim `decisions.md` §7 exists to prevent.
- **The explicit `unverifiable` bucket** (`decisions.md` §7) is the `unknown` group (4,224 units, "could not be classified", each carrying its own `reason`). `AT-33-E4-001`/`002` own moving this population, reported in four buckets, never as a bare count.
- **`box_ledger.py` scope, deliberately narrow.** This cycle implements only `AT-33-E1-001`'s bar: uncovered/overlap/population computed live against the real inventory and the real committed `THE-BOX.md`. It does **not** implement `AT-33-E1-002`'s other four fail-closed conditions (oracle disagreement, an `unverifiable` unit dispositioned `done`, the `derived_at` staleness gate) — those are a different criterion in the same file, left for the next cycle rather than stubbed here. `THE-BOX.md`'s front matter already carries `derived_at: <HEAD SHA at authoring time>` so that gate has something real to read when it lands.
- **Machine-readable ledger format.** `THE-BOX.md` carries one fenced ` ```json ledger ` block (group id / count / disposition / match / command) that `box_ledger.py` parses directly, rather than a second, separate classification-table file (contrast `coverage_ledger.py`'s external JSON table) — this keeps the "living partition" claim literal: the file a human reads and the file the tool checks are the same file.
- **RED→GREEN preserved twice:** (1) TDD RED — `python3 -m unittest scripts/tests/test_box_ledger.py` failed with `ModuleNotFoundError: No module named 'box_ledger'` before the script existed, for the intended reason; GREEN after implementation, 9/9 passing including the live-corpus acceptance case. (2) Mutation proof on top of the passing suite — a copy of the committed `THE-BOX.md` with the `unknown` group deleted was fed to `--check` and correctly failed closed with `uncovered=4224 overlap=0 population=49438`, exit 1, naming a sample of the uncovered unit ids; the committed file itself was never touched (confirmed via `git status --porcelain` before and after).
- Left `docs/retro/events/sd31-transcribe.jsonl`'s pre-existing dirty state untouched — it belongs to another concurrent lane per `workflow-instruction.md` §1 item 5's launch-gate note, and per §5's "one writer per tree" rule this cycle did not write to it.
- Also present but untouched: `docs/release/SD-33-computed-value-verification/artifacts/sd-33-dispatch.workflow.js` (untracked, not this cycle's file, not referenced by this criterion).

## Next-cycle plan

`AT-33-E1-002` extends `scripts/box_ledger.py` in place with the remaining four fail-closed conditions named in `decisions.md` §1 (oracle disagreement — no oracle exists yet, so this condition is a no-op until Epic 2 lands but must still be wired so it activates automatically once it does; an `unverifiable` unit dispositioned `done` — a real cross-check against `THE-BOX.md`'s own `disposition` field; and the `derived_at`-SHA-is-an-ancestor-of-HEAD staleness gate, reading the front-matter field this cycle added), each proven by a dedicated RED→GREEN mutation test per the criterion's evidence bar ("five RED→GREEN mutation proofs, one per condition"). `AT-33-E1-003` then enumerates the probe surface by execution and `AT-33-E1-004` wires the denominator gate into `scripts/verify.sh`.
