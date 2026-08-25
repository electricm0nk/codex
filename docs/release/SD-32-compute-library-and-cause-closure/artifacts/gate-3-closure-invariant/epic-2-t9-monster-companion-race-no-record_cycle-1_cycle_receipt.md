# Cycle epic-2-t9-monster-companion-race-no-record — Card 11, T9 / Gate 3 — `no_record` reduction, `monster_ability`/`companion`/`monster`/`race`

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (recorded after push)
- **Files touched:** `scripts/shape_ledger.py`, `scripts/tests/test_shape_ledger.py`,
  `docs/work-inventory.json` (regenerated), `docs/retro/events/t9-onboarding.jsonl`.

## Scope and re-derived baseline

Dispatch brief scoped `monster_ability` (1,210), `companion` (773), `monster` (141), `race` (59)
against `decisions.md §20`'s `16300bde7` baseline. Re-derived fresh per `§17a` at this cycle's own
base (`d26996388`, `decisions.md §20`'s own commit) before doing anything:

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l_before.json
population (not-done units considered): 36028
no_record            20889  (58.0%)
```
Per-kind (script below): `monster_ability` 1210, `companion` 773, `monster` 141, `race` 59 — matches
the brief exactly.

## Finding 1 (real work): `data/corpus/bestiary/` vs `data/corpus/beastiary/` book-name split was never aliased in the ledger's own join

`docs/work-inventory.json` names the core Bestiary book `"bestiary"` (no trailing `a`) on every unit.
`data/corpus/`'s directory for the same book carries the historical `"beastiary"` spelling — already
documented as deliberate in two places: `scripts/transcribe_monster_tables.py`'s
`CROSS_TABLE_MONSTER_RECORDS = {"bestiary": "beastiary"}` and `src/bin/gen_book_cache.rs`'s
`corpus_book: "beastiary"` (with its own three-spellings comment in `companion_chassis.rs`). Every
downstream consumer that reads corpus JSON already knows this. `scripts/shape_ledger.py`'s
`build_corpus_index` did not: `main()` derives its `books` set from the *inventory's* `book` field
(`"bestiary"`) and walks `data/corpus/bestiary/` literally, which holds only 3 stray records — the
real 1,105 records for this book live one directory over, at `data/corpus/beastiary/`, and every
inventory unit whose book is `"bestiary"` reported `no_record` regardless of whether its record
existed.

**Fix**: `BOOK_CORPUS_DIR_ALIASES = {"bestiary": "beastiary"}` in `shape_ledger.py`, applied only to
which directory `build_corpus_index` *walks* — the returned index is still keyed under the
inventory's own book spelling (`"bestiary"`), so `classify_unit`'s join (which reads a unit's own
`book` field) needs no change and no caller needs to know the alias exists.

**RED → GREEN**: added `test_bestiary_book_walks_the_beastiary_corpus_directory`, reproducing the
real defect with a `beastiary/monster_ability/*.json` record at the same
`(book="bestiary", basename, line)` shape as the live corpus's `ce_abilities_race.lst:1280`.
Confirmed it failed for the intended reason before the fix (`AssertionError: ('bestiary',
'ce_abilities_race.lst', 1280) not found in {}`), then GREEN after.

```
$ python3 -m unittest scripts.tests.test_shape_ledger -v   # 30 passed, 0 failed (was 29/1 RED)
$ python3 -m unittest discover -s scripts/tests -p "test_*.py"   # 421 passed, 0 failed, 1 skipped
```

## Finding 2 (measurement catch-up, no code change): `docs/work-inventory.json` was stale relative to already-committed corpus additions

Two prior committed cycles (`7072f323e` monster_ability generic ingest +190, `43c3e4bde`
`MonsterAbilityFacet` widening +442) each recorded in their own receipts that their addition was
"not reflected in the checked-in inventory until a future regen." Regenerated it this cycle, per
the workflow-instruction near-miss warning: built `corpus_literal_sweep --json-out` and
`derived_evaluator_fixture_check --json-out` reports first, set both
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`, ran `v06_work_inventory` **without**
`--allow-stamp-loss` (which would have refused on any stamp loss), then diffed the full status
distribution before/after:

```
status               before   after
not-ingested          29106   28312   (-794)
literal-verified        6506    6506   (unchanged — no stamp loss)
fixture-verified         1741    1741   (unchanged — no stamp loss)
text-complete            3869    4395
grounded                 2515    2724
ingested-magnitude       1474    1515
unknown                  4264    4282
deferred-with-reason       46      46
not-started                19      19
```
No `literal-verified`/`fixture-verified` stamp was lost — the exact near-miss the warning names,
checked and clean.

## Net result, per kind, re-derived against the pinned oracle after both fixes

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l_after.json
population (not-done units considered): 35418
no_record            20572  (58.1%)   [was 20889 -- -317 overall]
```

| kind | before | after | delta |
|---|---:|---:|---:|
| `monster_ability` | 1210 | 1146 | -64 |
| `companion` | 773 | 769 | -4 |
| `monster` | 141 | 28 | **-113** |
| `race` | 59 | 59 | 0 |

(Overall -317 also includes `race_trait` -30 and small shifts elsewhere from the inventory catch-up,
unscoped to this cycle but real and not double-counted — the `shape-coverage-standing-gate` command
below is the single source of truth for the totals.)

```
$ scripts/verify.sh --only shape-coverage-standing-gate
PASS  shape-coverage-standing-gate  (population=35418 unclassified=0 no_record=20572 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
```

Budget constants (`NO_RECORD_BUDGET_COUNT`/`POPULATION`) **left untouched** per the dispatch
instruction — a repin is a separate evidence-gated cycle.

## `monster`'s -113: mostly the alias, the rest already independently closed

Of `monster`'s remaining 28 `no_record` units (down from 141), a sibling `t9-onboarding` cycle's own
retro log (`docs/retro/events/t9-onboarding.jsonl`, corrections at `29f3bca6d`/`fb4f28dad`, read
before duplicating any of this work) already traced and closed the non-bestiary population at zero
real gap: `bestiary_4`'s 4 (Hydra/Iron Cobra colour variants) are `.MOD` overlay deltas on records
defined elsewhere; `bestiary_2`'s 2 (Gug Savant, Magma Ooze (Poisonous)) are `.COPY=` derived deltas;
`occult_adventures`'s 1 (Kami (Shikigami)) loads only under a negated `PRECAMPAIGN` gate that never
fires. The remaining 28 are unique named creatures in `bestiary_4`/`inner_sea_bestiary`/
`inner_sea_world_guide`/`occult_adventures` (Demon Lords, Empyreal Lords, Great Old Ones, Kaiju,
`Star-Spawn of Cthulhu`) — several names PCGen itself declares `NAMEISPI:YES` elsewhere (`decisions.md
§19b`'s own recorded caveat about the `Cthulhu` inconsistency). Per-record review, not attempted
this cycle — see "What remains."

## `companion`'s 769: the real lever is a shape this cycle did not build, named precisely

`python3 scripts/classify_companion_rows.py <all 9 no_record books>` (all already registered in
`companion_chassis::COMPANION_BOOKS` — no unregistered-book gap exists for `companion`) shows **730
of 769** are the tool's own `ORPHAN` disposition: an ability row whose `KEY` no creature row's
`ABILITY:`/prerace/prefix/relay reference names. Grouped by `KEY` prefix, the top shapes are
`Evolution` (212), `Temp Evolution` (118), `Animal Companion Feat` (64), `Animal Trick` (53), `Imp
Companion Trick` (23), `Companion Archetype` (16), `Familiar Archetype` (14) — **one shape, not many**:
every one of these is granted through a `BONUS:ABILITYPOOL|<PoolName>|<Count>` token, not a direct
`ABILITY:` reference, so none of the six existing ownership shapes (`classify_companion_rows.py`'s
row-named/prerace/prefix/relay/granted/`.COPY=`) see them.

Traced one concretely (`advanced_players_guide:companion:evolution_ability_increase_cha`,
`apg_abilities_companion.lst:121`, `magnitude_token_count: 3` — real formula content, not flavour
text): its pool is granted by `apg_abilities_companion.lst:50`'s `Standard Eidolon` row
(`BONUS:ABILITYPOOL|Eidolon Evolution|EidolonEvolution`), which is itself a `CATEGORY:Internal` row
— **not an inventory unit at all**, exactly the two-hop-relay shape `classify_companion_rows.py`'s
own "Shape 6" comment already documents for a different case. Worse, the pool's own name (`"Eidolon
Evolution"`) does not equal the ability `KEY` prefix (`"Evolution"`) — the pool→prefix correspondence
is not a clean rule, and generalising it wrong across all 730 orphans risks manufacturing false
ownership claims (`decisions.md §1a`/`§3`), which is worse than leaving them named. **Not attempted
this cycle** — a real "Shape 7, ABILITYPOOL grants" needs its own dedicated, adversarially-verified
cycle per the same `decisions.md §16` caution the `monster_ability` facet-widening cycle already
applied correctly once.

## `race`'s 59: not investigated this cycle

No time spent past confirming the baseline (59, unchanged by both fixes above — none of its books
matched `"bestiary"`/`"beastiary"`). Genuinely open; next cycle's first move should be the same
per-book `classify_*`-style orphan/gate breakdown this receipt ran for `companion` and `monster`.

## §15 / PI

Read-only measurement and one join-key fix this cycle — no record was transcribed, so nothing was
stopped on. The `bestiary_4`/`inner_sea_world_guide`/`occult_adventures` `monster` names surfaced
above (Great Old Ones, Kaiju, `Cthulhu`) are flagged, not touched, for the next cycle's attention —
several are exactly the shape `decisions.md §19b`'s caveat already names as PCGen-inconsistent.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §20` — Gate 3's closure condition is `no_record == 0`.
  This cycle reduces it (20889 → 20572, -317; my scope -181 direct) and does not claim closure.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).
- **Status:** complete (real, verified reduction landed; large remaining populations named by exact
  shape, not rounded into "done").
- **Notes:** `docs/work-inventory.json` regenerated safely (stamp-loss checked, see Finding 2); no
  Gate 3 budget constant touched.
- **Discovery forwards:** none filed as `## DISCOVERED` — both open shapes (`companion`'s ABILITYPOOL
  grants, `race`'s un-investigated 59) are named above with enough detail for direct pickup, not
  raised as blockers (`AGENTS.md` Blocker Discipline: this is real work handed forward with a named
  next step, not a stall).
- **Next-cycle plan:** (1) `race`'s 59 — run the same per-book classify-and-group pass this receipt
  ran for `companion`/`monster`. (2) `companion`'s `BONUS:ABILITYPOOL` shape — trace the
  pool-name→`KEY`-prefix correspondence for at least 3 more pools (`Animal Companion Feat`, `Animal
  Trick`, `Imp Companion Trick`) before generalising, since `Eidolon Evolution`→`Evolution` already
  shows the mapping is not literal equality. (3) `monster`'s remaining 28 unique-named creatures —
  PI-screen each by name against the signed-off `ogl-pi-blacklist.md` before any transcription
  attempt.
