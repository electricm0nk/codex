---
canonical: true
owner: god-emporer
bundle_id: SD-33
derived_at: 2dcf2aebebcb662ca7d280b145cd7cc67ebd469b
date: 2026-08-25
---

# THE-BOX — SD-33's living partition of the full inventory

`decisions.md` §1: this bundle builds **its own** box rather than citing SD-31's or
SD-32's. It partitions **all 49,438 inventory units** in `docs/work-inventory.json`
-- not the not-done subset, not a filtered view -- into named groups. Every unit
belongs to **exactly one** group. `scripts/box_ledger.py --check` proves that
structurally against the live file, every time it runs -- and, per
`AT-33-E1-002`, fails closed on four further conditions (oracle disagreement,
an `unverifiable` unit dispositioned `done`, and `derived_at` staleness) too:

```
$ python3 scripts/box_ledger.py --check
INFO: no oracle-results at .../epic-2-oracle-harness/oracle-results.json -- oracle disagreement check is wired but has nothing to examine yet (Epic 2 not landed)
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
```

## Population, confirmed by execution, not by memory

`AT-33-E1-001`'s scope note asked this cycle to confirm 49,438 before trusting it.
Two independent commands agree:

```
$ jq '.units | length' docs/work-inventory.json
49438
$ jq '.totals.units' docs/work-inventory.json
49438
```

`box_ledger.py` uses the first form (`len(units)`, a live count) as `population`,
never the `totals.units` field on trust -- see `load_inventory()`'s docstring.

## How the partition is built

`docs/work-inventory.json`'s `status` field is already a clean, exhaustive,
non-overlapping 9-way split of the full population -- confirmed by execution
(`jq -r '.units[].status' docs/work-inventory.json | sort -u` returns exactly the
9 values below; `jq '[.units[]|select(.status==null)]|length'` returns `0`; every
unit `id` is unique). Each group below matches on that field. The **disposition**
column is deliberately not a copy of the status name: it states what the status
value actually means for computed-value trust, per `decisions.md` §7 -- in
particular, `grounded` / `literal-verified` / `fixture-verified` are all
**engine- or artifact-verified, not oracle-verified**. None of them has yet been
checked against PCGen (Epic 2 builds that harness; Epic 5 re-examines the
fixture- and literal-verified populations against it). Calling any of the three
`done` here would be exactly the over-claim `decisions.md` §7 names.

| Group (status) | Count | Disposition | Re-derive command |
|---|---:|---|---|
| `grounded` | 3,234 | engine-grounded, oracle-pending | `jq '[.units[]\|select(.status=="grounded")]\|length' docs/work-inventory.json` |
| `literal-verified` | 6,589 | corpus-literal-verified (byte-compared to the upstream LST literal), oracle-pending | `jq '[.units[]\|select(.status=="literal-verified")]\|length' docs/work-inventory.json` |
| `fixture-verified` | 1,741 | fixture-verified (matched a pinned, independently-derived fixture), oracle-pending | `jq '[.units[]\|select(.status=="fixture-verified")]\|length' docs/work-inventory.json` |
| `ingested-magnitude` | 1,543 | held — engine holds the real numeric record, no consumer delta observed yet | `jq '[.units[]\|select(.status=="ingested-magnitude")]\|length' docs/work-inventory.json` |
| `text-complete` | 5,099 | done — no magnitude token exists to compute; real prose is shown instead | `jq '[.units[]\|select(.status=="text-complete")]\|length' docs/work-inventory.json` |
| `deferred-with-reason` | 46 | blocked — a claim-blocking engine diagnostic names this unit | `jq '[.units[]\|select(.status=="deferred-with-reason")]\|length' docs/work-inventory.json` |
| `not-ingested` | 26,943 | not-reachable — book is ingested but the engine holds no record for this unit | `jq '[.units[]\|select(.status=="not-ingested")]\|length' docs/work-inventory.json` |
| `not-started` | 19 | not-reachable — book has no compiled rule set at all | `jq '[.units[]\|select(.status=="not-started")]\|length' docs/work-inventory.json` |
| `unknown` | 4,224 | **unverifiable** — could not be classified; each unit carries its own `reason` | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` |
| **Total** | **49,438** | | `jq '.units\|length' docs/work-inventory.json` |

The `unknown` group is the explicit `unverifiable` bucket `decisions.md` §7
requires: "could not be classified" is, definitionally, a magnitude that cannot
currently be checked. `AT-33-E4-001`/`002` own driving this group's population
toward zero (root-cause first, movement reported in the four buckets --
closure / reclassification / reachability / instrument-correction -- never as a
bare count). `AT-33-E1-003`'s probe-surface census will further mark, inside the
non-`unverifiable` groups, which kinds have no probe capable of checking them at
all; that finding does not move any unit into a different top-level group here,
it is layered on top by a later cycle.

## Machine-readable ledger

`scripts/box_ledger.py` parses the block below (and only this block --
identified by the `json ledger` fence marker) to compute `uncovered` / `overlap`
against the live `docs/work-inventory.json`, independent of the table above.
Both the table and this block are maintained together; a mismatch between a
group's stated `count` here and the live recomputation is printed as a warning
by `--check` (not gated to fail closed -- gating that would require knowing
the *correct* count, which is what this whole partition exists to establish).

`AT-33-E1-002` adds the `"unverifiable"` boolean field below: a group that sets
it `true` states, structurally, that its units have not been checked against
anything but our own artifacts (`decisions.md` §7). `box_ledger.py --check`
fails closed if any group is ever edited to carry both `"unverifiable": true`
and `"disposition": "done"` at once -- exactly the over-claim SD-32 made for
8,330 units. Only `unknown` is `true` today; every group `box_ledger.py`
partitions the corpus into is listed explicitly, `true` or `false`, so a
later cycle adding a group cannot leave the field unstated by omission.

```json ledger
{
  "groups": [
    {
      "id": "grounded",
      "count": 3234,
      "disposition": "engine-grounded, oracle-pending",
      "unverifiable": false,
      "match": {"status": "grounded"},
      "command": "jq '[.units[] | select(.status==\"grounded\")] | length' docs/work-inventory.json"
    },
    {
      "id": "literal-verified",
      "count": 6589,
      "disposition": "corpus-literal-verified, oracle-pending",
      "unverifiable": false,
      "match": {"status": "literal-verified"},
      "command": "jq '[.units[] | select(.status==\"literal-verified\")] | length' docs/work-inventory.json"
    },
    {
      "id": "fixture-verified",
      "count": 1741,
      "disposition": "fixture-verified, oracle-pending",
      "unverifiable": false,
      "match": {"status": "fixture-verified"},
      "command": "jq '[.units[] | select(.status==\"fixture-verified\")] | length' docs/work-inventory.json"
    },
    {
      "id": "ingested-magnitude",
      "count": 1543,
      "disposition": "held",
      "unverifiable": false,
      "match": {"status": "ingested-magnitude"},
      "command": "jq '[.units[] | select(.status==\"ingested-magnitude\")] | length' docs/work-inventory.json"
    },
    {
      "id": "text-complete",
      "count": 5099,
      "disposition": "done",
      "unverifiable": false,
      "match": {"status": "text-complete"},
      "command": "jq '[.units[] | select(.status==\"text-complete\")] | length' docs/work-inventory.json"
    },
    {
      "id": "deferred-with-reason",
      "count": 46,
      "disposition": "blocked",
      "unverifiable": false,
      "match": {"status": "deferred-with-reason"},
      "command": "jq '[.units[] | select(.status==\"deferred-with-reason\")] | length' docs/work-inventory.json"
    },
    {
      "id": "not-ingested",
      "count": 26943,
      "disposition": "not-reachable",
      "unverifiable": false,
      "match": {"status": "not-ingested"},
      "command": "jq '[.units[] | select(.status==\"not-ingested\")] | length' docs/work-inventory.json"
    },
    {
      "id": "not-started",
      "count": 19,
      "disposition": "not-reachable",
      "unverifiable": false,
      "match": {"status": "not-started"},
      "command": "jq '[.units[] | select(.status==\"not-started\")] | length' docs/work-inventory.json"
    },
    {
      "id": "unknown",
      "count": 4224,
      "disposition": "unverifiable",
      "unverifiable": true,
      "match": {"status": "unknown"},
      "command": "jq '[.units[] | select(.status==\"unknown\")] | length' docs/work-inventory.json"
    }
  ]
}
```

## Explicitly rejected

Inheriting SD-31's 46-group partition (`decisions.md` §1) -- it was cut for a
world where objects were not yet ingested, and its groups answer a question
SD-32 already closed.

## Next-cycle plan

`AT-33-E1-002` (this cycle) extended `scripts/box_ledger.py` (same file) with
the remaining three fail-closed conditions: oracle disagreement (wired,
currently a no-op pending Epic 2's `oracle-results.json`), an `unverifiable`
unit dispositioned `done` (the ledger's new `"unverifiable"` field, above),
and the `derived_at`-SHA staleness gate (reads this file's front matter).
`AT-33-E1-003`/`004` add the probe-surface census and the `verify.sh`
denominator-gate stage. `THE-BOX.md` is amended **append-only** by Epics 2-5
thereafter (`workflow-instruction.md` §3): a later cycle may add new groups (for
example, once units currently `unverifiable` are reclassified) but must not
remove or shrink an existing one without the sum continuing to equal 49,438,
and every new group must state `"unverifiable"` explicitly.
