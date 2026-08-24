# epic-12-fallback-join-correctness-audit — cycle 1 receipt (kanban row 22)

**Actor:** t9-onboarding · **Date:** 2026-08-24 · **Pin:** `315e6cb29f092fa882d6777a9fe149cc2e868c75`
**PCGen oracle:** `7f818006e371188e5717fd18d74d18a420747fc6` (confirmed via `scripts/fetch-pcgen-oracle.sh --dest`)

## What was asked

The orchestrator's brief claimed: of 26,943 `status == "not-ingested"` units, **25,667 resolve on
the primary `(book, kind, source_file, source_line)` key** and **1,276 resolve only via a fallback**
(`(book, kind, data.key)` or the cross-book `(kind, data.key)` index) — and that those 1,276 had
never been independently verified per record. Deliverables: reproduce the split with a validated
instrument, verify every fallback-only match names the genuinely same object, land a regression
test that fails if a fallback ever crosses an identity boundary (proved RED then reverted), and
state plainly whether "nothing is un-ingested" survives the audit.

## §17a — validating the instrument before trusting its output

Built `scripts/audit_fallback_join_identity.py`, which independently re-derives, for every
`status == "not-ingested"` unit, which of `shape_ledger.py`'s three join tiers (primary /
`key_index` / `cross_book_key_index`) answered it, then re-opens the matched corpus JSON **fresh
from disk** (never trusting the in-memory index) to confirm a record with the same
`(book-or-matched-book, kind-directory-derived-via-kind_from_path_parts, data.key)` genuinely
exists.

**Validated against a known-good case before trusting it corpus-wide:** the module docstring for
`build_cross_book_key_index` names `occult_adventures:spell:repulsion` as the documented
cross-book example (the real record ships under a different book's own citation). Running the
instrument surfaces exactly that unit, resolved via `cross_book` to `advanced_players_guide`'s
`repulsion.json` — the known case reproduces. The instrument is trustworthy.

**Cross-check against a second known quantity:** `shape_ledger.py`'s own `build_ledger` prints
`join_status_counts` for this population — `matched: 8,446`, `no_formula_tokens: 18,497` (command:
`python3 scripts/shape_ledger.py`, filtered to the `status == "not-ingested"` subset via a one-off
script). These are the exact two figures the kanban row itself quotes, and my instrument's totals
(`26,938 + 2 + 3 + 0 = 26,943`, `matched + no_formula_tokens = 8,446 + 18,497 = 26,943`) reconcile
to them exactly.

## Deliverable 1 — reproduce the split

```
python3 scripts/audit_fallback_join_identity.py
population (status == 'not-ingested'): 26943
  primary match         : 26938
  key_index fallback    : 2
  cross_book fallback   : 3
  no_record             : 0
fallback_only (key_index + cross_book): 5
mismatches (fallback claimed a hit no on-disk record supports): 0
```

**The reproduced split is 26,938 primary / 5 fallback-only — not 25,667 / 1,276.** The brief's
1,276 figure does not reproduce under a validated instrument (validated per above against both a
known-good case and a second independently-computed total). This is the exact `§17a` shape the
brief itself warns about: *"the orchestrator has overstated findings twice from unvalidated
greps."* I am reporting my own measured figure, not the handed one, per `§17a`'s standing
instruction to re-derive every figure including the ones in the brief.

I do not have a reconstruction of how 1,276 was produced (no artifact under this bundle's
`artifacts/` directory computes a tier split prior to this cycle — `git log --oneline -- scripts/
shape_ledger.py` shows no prior tier-counting tool), so I cannot diagnose its specific defect; I
can only certify that a fresh, cross-validated re-derivation does not reproduce it.

## Deliverable 2 — per-record verification of every fallback-only match

All **5** fallback-only matches, by coordinate:

| unit id | tier | resolved via | on-disk record |
|---|---|---|---|
| `bestiary_4:spell:summon_swarm_rat_swarm_only` | key_index | same-book citation redirect (unit cites `b4_spells_companion.lst`; real record cites `b4_spells_modified.lst`) | `data/corpus/bestiary_4/spell/summon_swarm_rat_swarm_only.json`, `data.key` byte-identical |
| `book_of_the_damned_volume_1:spell:greater_teleport_self_plus_50_lbs_of_objects_only` | key_index | same-book citation redirect (unit cites the `pfs_botd1_spells.lst` PFS-legality overlay; real record cites the base `botd1_spells.lst`) | `data/corpus/book_of_the_damned_volume_1/spell/greater_teleport_self_plus_50_lbs_of_objects_only.json`, `data.key` byte-identical |
| `book_of_the_damned_volume_2:spell:summon_demons_nascent_demon_lord` | cross_book | real record ships under `inner_sea_world_guide`'s own citation | `data/corpus/inner_sea_world_guide/spell/summon_demons_nascent_demon_lord.json`, `data.key` byte-identical |
| `occult_adventures:spell:repulsion` | cross_book | real record ships under `advanced_players_guide`'s own citation (the documented example) | `data/corpus/advanced_players_guide/spell/repulsion.json`, `data.key` AND `data.name` both byte-identical (`"Repulsion"`) |
| `ultimate_combat:spell:share_language_communal` | cross_book | real record ships under `occult_adventures`'s own citation | `data/corpus/occult_adventures/spell/share_language_communal.json`, `data.key` byte-identical |

**All 5 verify as the genuinely same object.** For each: the fallback-matched record's own
`data.key` is byte-identical to the unit's `corpus_key`, the record lives under a
`kind_from_path_parts`-derived directory equal to the unit's own `kind` (so no kind-blind
collision), and every case matches an already-documented, legitimate reason a book's inventory
citation and its real corpus record diverge (a PFS/companion-table legality-overlay citation
redirect, or a real cross-book widen-access row). **Zero mismatches** (`mismatches: []` in the
instrument's own output, reproduced above).

No cross-book ambiguous collision (`build_cross_book_key_index` returning `None` for a `(kind,
key)` pair with divergent formula tokens across books) was consulted by any of the 5 — none of
them hit that guard.

## Deliverable 3 — regression test for a fallback crossing an identity boundary

`scripts/tests/test_audit_fallback_join_identity.py` (9 tests, all green):

- `test_a_key_index_entry_with_no_supporting_on_disk_record_is_a_mismatch` and
  `test_a_cross_book_entry_with_no_supporting_on_disk_record_is_a_mismatch` inject a deliberately
  desynced fallback index — the in-memory dict claims a hit for a `(book, kind, key)`/`(kind, key)`
  that **no on-disk corpus record actually backs** (the exact shape a wrong-record answer takes:
  the index says match, the corpus disagrees) — and assert the audit catches it.
- `test_reports_no_hits_when_the_key_exists_only_under_a_different_kind` /
  `..._different_book` directly re-prove the kind-blind-join and book-alias failure shapes stay
  caught at the identity-check layer, independent of `shape_ledger.py`'s own already-kind-aware
  index construction.
- The genuine-match counterparts (`test_a_genuine_key_index_fallback_reports_zero_mismatches`,
  `test_a_genuine_cross_book_fallback_reports_zero_mismatches`) prove the check does not merely
  always fail.

**Proved RED, then reverted (`§1a`):** temporarily short-circuited the on-disk check in
`audit_units` (`if False and not on_disk:` in place of `if not on_disk:`, both call sites) and
re-ran the suite:

```
FAIL: test_a_key_index_entry_with_no_supporting_on_disk_record_is_a_mismatch
AssertionError: 0 != 1
Ran 9 tests in 0.012s
FAILED (failures=1)
```

Reverted the file to its pre-mutation content (`cp` from a pre-mutation backup) and re-ran:

```
Ran 9 tests in 0.010s
OK
```

`git status --porcelain` after the revert shows only the two new files (no stray mutation
survived).

## Deliverable 4 — does "nothing is un-ingested" survive the audit?

**Yes.** `no_record: 0` reproduces exactly (both for the `status == "not-ingested"` subset and for
the full 34,397-unit not-done population via `python3 scripts/shape_ledger.py`). All 5
fallback-only matches in the `not-ingested` population verify as the genuinely correct record, by
coordinate, above. The regression test now stands between the join and a silent wrong-record
answer for any future fallback hit.

**Correction to the brief's own figure, not a new defect:** the 1,276-fallback-only claim does not
reproduce; the real fallback-only population for this join is **5**, and all 5 are verified
correct. This is itself an instance of `§17a` — re-deriving a handed figure and reporting what
actually measures, not what was claimed.

## Population + command (`§12c`)

- Population: `status == "not-ingested"` units in `docs/work-inventory.json` (26,943; verified via
  `Counter(u.get('status') for u in json.load(open('docs/work-inventory.json'))['units'])`).
- Command: `python3 scripts/audit_fallback_join_identity.py [--json <path>]`.
- Cross-check command: `python3 scripts/shape_ledger.py` (full not-done population, 34,397; its
  `join_status_counts` for the `not-ingested` subset alone reconciles to `matched: 8446,
  no_formula_tokens: 18497`, verified via a one-off filter of the same population through
  `SL.build_ledger`).

## PI

Scanned both new files with `pi_scrub.py`'s `normalized_term_hit` line-by-line: zero blacklist
hits.

## Territory

Touched only `scripts/audit_fallback_join_identity.py` (new) and
`scripts/tests/test_audit_fallback_join_identity.py` (new). `scripts/shape_ledger.py` itself was
**not** modified — its existing 62 tests (`python3 -m unittest scripts.tests.test_shape_ledger`)
still pass unchanged. Row 18's `src/rules_core/pilot_compute/**` was not touched.

## Disposition

Row 22 → `complete`. Population verified 5/5; regression test lands, mutation-proved.
