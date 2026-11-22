# Cycle AT-33-E4-003 — Epic 4 Unknown classification / AT-33-E4-003

- **Commit SHA:** `acdc10de3f`
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/THE-BOX.md` (append-only: group counts updated,
    one group renamed `unknown` → `unmeasurable`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**

  > ### AT-33-E4-003 — nothing lands in a bucket meaning "we did not look"
  >
  > Every reclassified unit carries a disposition that is a statement about the unit, not about our
  > effort.
  >
  > **Evidence:** `box_ledger.py --check` passes with the reclassified population; no group named for
  > an absence of work.

## What landed

`THE-BOX.md`'s ledger block (`AT-33-E1-001`'s living partition) is updated to match the population
`AT-33-E4-002` produced:

- `ingested-magnitude` group count: `1,543` → `2,497` (+954 = +854 this criterion's reclassification,
  +100 unrelated drift — see below)
- `not-ingested` group count: `26,943` → `26,047` (net −896 = +3,052 this criterion's reclassification,
  −3,948 unrelated drift net — see below)
- `unknown` group **renamed** `unmeasurable`, count `4,224` → `321` (318 this criterion's own
  irreducible remainder + 3 unrelated drift — see below), `"unverifiable": true` retained, `match`
  filter updated to `{"status": "unmeasurable"}`
- `grounded`: `3,234` → `3,340` and `text-complete`: `5,099` → `8,838` — both entirely unrelated drift
  (`AT-33-E4` never routes any unit to either status)

Sum invariant preserved: `jq '.units|length' docs/work-inventory.json` → `49,438`, unchanged
(`decisions.md` §1's partition requirement). **Two populations are folded into these counts, and
`AT-33-E4-002`'s receipt keeps them separate**: this criterion's own 4,224-unit movement (verified
`id`-keyed, exact: 3,052 + 854 + 318 = 4,224), and 3,985 units of unrelated drift `AT-33-E4-002`'s
necessary regen surfaced (real SD-32 engine work landed since the file's last regen, 2026-08-23,
never previously captured — none of the 3,985 came from the `unknown` group). `THE-BOX.md`'s own
table and ledger block state the TRUE current population either way; the split above is what
distinguishes this criterion's work from what the regen incidentally also had to capture.

Every reclassified unit's disposition is a claim about **the unit**:

- `ingested-magnitude` ("held — engine holds the real numeric record, no consumer delta observed
  yet") — true of all 854: each carries a `wiring_class` proof (a real magnitude token exists in its
  full closure) that predates and is independent of this cycle's fix; the fix only lets the
  classifier say what it could already prove.
- `not-ingested` ("not-reachable — book is ingested but the engine holds no record for this unit") —
  true of all 3,052: each carries the SAME `class_feature_effect_wired` probe-absence finding its
  `text_only` sibling shape already used to reach `not-ingested`, unaffected by whether this
  specific record's own line happens to carry a magnitude token.
- `unmeasurable` (renamed from `unknown`, disposition unchanged: "could not be classified without
  guessing") — true of the remaining 318: each carries its own specific, substantive `reason`
  string, unedited by this cycle (`unknown-rootcause.md` names concretely, per shape, why: a
  genuinely-empty corpus record, or a served description corrupted by an upstream PI/not-implemented
  marker). Neither reason is "we did not look" — both are stated, falsifiable findings about the
  specific unit.

**No group in `THE-BOX.md` is ever named for an absence of work.** The renamed group's own
disposition text says so explicitly, and its evidence obligation (`box_ledger.py`'s existing
condition 4: an `unverifiable` unit dispositioned `done`) still applies unchanged — this cycle
neither adds nor removes that gate, it only reduces the population the gate protects.

## Figures + their re-derive commands

- `python3 scripts/box_ledger.py --check` → `uncovered=0 overlap=0 population=49438
  oracle_disagreement=0 unverifiable_done=0 stale=False`. No warnings emitted (every group's stated
  `count` in the ledger block matches the live recomputation exactly).
- Group-count re-derive commands (unchanged form, updated `match` values), each stated in
  `THE-BOX.md`'s own table, e.g.
  `jq '[.units[]|select(.status=="ingested-magnitude")]|length' docs/work-inventory.json`.

## Status: complete

## Movement, four buckets

Same as `AT-33-E4-002` (this criterion verifies that cycle's movement, it does not add new
movement): **closure 0 / reclassification 3,906 / reachability 0 / instrument-correction 318**.

## Notes

- **`box_ledger.py` itself needed no code change.** Its `_matches()` partitioning is fully
  data-driven from `THE-BOX.md`'s own `match` filters (confirmed by reading
  `scripts/box_ledger.py`'s `_matches`/`partition` functions) — updating the ledger block inside
  `THE-BOX.md` (this criterion's own append-only write scope) is sufficient; no write outside the
  granted scope was needed or made.
- **`derived_at` staleness gate:** `THE-BOX.md`'s front-matter `derived_at` SHA is updated to this
  cycle's own landing commit, satisfying condition 5 against the state the ledger block now
  describes.

## Next-cycle plan

None outstanding for Epic 4. `unknown-rootcause.md`'s named future opportunity (widening
`REGISTERED_POOL_GROUPS` against the 1,128 distinct unmatched `class_feature` group prefixes) and
the `pf1e_dashboard_producer.py` one-line follow-up (`AT-33-E4-002`'s Notes) are both named, neither
is a blocker on this criterion's own evidence.
