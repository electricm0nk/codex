# Cycle 004 — Gate 1 / `equipment`, `equipment_modifier`, `class` `no_record` closure (decisions.md §20)

- **Card ID:** `gate-1-shape-closure` (row 5) — `no_record` ingestion mandate, `decisions.md §20`;
  scope was T9-onboarding's wave-2 lane: `equipment` (316), `equipment_modifier` (237), `class`
  (157), `race` (59) — 769 units across four kinds.
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — `book_routing()` gained `"ISTEM"`/`"ISM"`/`"AG"`
    arms it was missing entirely; test list widened to cover every gap book.
  - `src/bin/gen_equipment_gap_tables.rs` — new `EQUIPMENT_BOOK_AG` book (`adventurers_guide`, no
    prior config at all), new `ultimate_magic` (`EQUIPMENT_BOOK_UM`, already-routed code, no prior
    config), `inner_sea_magic`'s stale "zero not-ingested equipment units" comment corrected and
    `ism_equipmods.lst` added back to its citation files.
  - `scripts/ingest_class.py` (**new**) — generic ingest for `Kind::Class`'s enumerated-but-not-
    ingested units, all books, one `CLASS:<Name>` LST-row citation per unit, verbatim transcription.
  - `scripts/tests/test_ingest_class.py` (**new**) — unit tests for the `CLASS:` identity-prefix
    strip and the `BOOK_CORPUS_DIR_ALIASES` output-directory fix (see "What was found" below).
  - `tests/equipment_gap_tables.rs`, `src/rules_core/equipment_resolver.rs`,
    `apps/desktop/src-tauri/src/equipment_catalog.rs` — every pinned count these three files carry
    over the corpus gap lane's row count, re-derived and updated (not hand-adjusted): gap-table
    total 1720 -> 1879, catalog total 7866 -> 8025, `ISM`/`AG` per-book and per-category counts,
    one new cross-book key collision (`"Rod (Storm Kindler's)"`, `AG`), description-coverage totals.
  - `data/corpus/**/equipment/**/*.json` (146 new, via `gen_cache_equipment_gap`)
  - `data/corpus/**/equipment/equipmods/**/*.json` (62 new, same generator)
  - `data/corpus/**/class/**/*.json` (137 new, via `scripts/ingest_class.py`)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/20-class-pi-skipped.json` (**new**) — the 21 name-blacklisted `class` units, named per `decisions.md §15`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff)
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`" —
  applied to this cycle's four kinds.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete for `equipment`/`equipment_modifier`/`class` (this cycle's ingestible
  population, PI residuals named not closed); `race` **not started** — named as next-cycle scope
  below, per `workflow-instruction.md`'s "landing two cleanly and naming the other two's blockers
  beats four half-built paths."
- **Notes:** see below.

## Before / after (re-derived, `scripts/shape_ledger.py` against the pinned oracle)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "
import json,collections
r=json.load(open('/tmp/l.json'))['rows']
print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Kind | Before | After | Closed | Residual (named, not fabricated) |
|---|---:|---:|---:|---|
| `equipment` | 316 | 170 | 146 | see "What's left" |
| `equipment_modifier` | 237 | 175 | 62 | see "What's left" |
| `class` | 157 | 21 | 136 | 21 PI-blocked (`20-class-pi-skipped.json`) |
| `race` | 59 | 59 | 0 | not started this cycle |

Bundle-wide `no_record` at cycle start (re-derived, this cycle's own first run): 8,434. At cycle
end: 8,226 (equipment fix) then 8,089 (class ingest) = **345 units closed this cycle**, before the
wave's `race` residual.

## What was found

Per `decisions.md §17`, checked for an existing mechanism before building one, per the brief's
explicit instruction.

### `equipment` / `equipment_modifier` (208 closed)

`src/rules_core/cache_gen/equipment_gap.rs` + `src/bin/gen_equipment_gap_tables.rs` already existed
and already generated a config table with the CORRECT row count for two of the missing books
(`inner_sea_temples`/`inner_sea_magic`) — but `cache_gen::equipment_gap::book_routing()`'s
`match` had **no arm at all** for the `"ISTEM"`/`"ISM"` codes those `BOOK_INPUT` entries used, so
`generate()`'s own `let Some((book_id, book_rel_dir)) = book_routing(book) else { continue }`
silently dropped every one of those rows before they ever reached `data/corpus/` — the config table
and the cache writer had drifted out of sync (a two-file mechanism, one file updated without the
other). Separately, `inner_sea_magic`'s `BOOK_INPUT` carried a comment asserting "zero
`not-ingested` equipment units cite `ism_equipmods.lst`" that had gone stale — 62 `equipment_
modifier` units do, re-derived against the pinned oracle.

Root-caused, not patched around: added the two routing arms, corrected the stale comment, added
`ism_equipmods.lst` back to `inner_sea_magic`'s citation files, and — the single largest residual
population in this scope — added a brand-new `BOOK_INPUT`/`book_routing` pair for
`adventurers_guide` (115 `not-ingested` equipment units, no config entry at all before this cycle;
97 resolve and clear PI screening). Also added `ultimate_magic` (its `EQUIPMENT_BOOK_UM` code was
already routed for the compiled catalog, but had no `BOOK_INPUT` config); it wrote 0 rows because
its real residual (19 units) is `status == "unknown"`/`"ingested-magnitude"` in
`docs/work-inventory.json`, not `"not-ingested"` — this generator's own selection predicate only
covers the latter (see its module doc comment). Left named, not widened untested in the same cycle.

Regenerated via `cargo run --locked --bin gen_equipment_gap_tables` then `gen_cache_equipment_gap`
(additive-only: verified `data/corpus/**/*.json` count 41,403 -> 41,611, exactly +208, no file
removed or overwritten — `write_json_never_overwrites_an_existing_file` covers this generator's own
non-destructive contract).

### `class` (136 closed, new mechanism — `scripts/ingest_class.py`)

No corpus writer for the `class` kind existed at all outside the 11 base classes
(`gen_core_rulebook_cache.rs`'s `ClassId::ALL`) and 6 APG/6 ACG/1 PU hybrid/unchained classes. Every
other class — prestige classes and NPC classes, 157 units across 19 books — had `join_status:
no_record` because nothing had ever transcribed them. `Kind::Class`'s corpus rows turn out to be the
same flat, single-line, tab-delimited shape `scripts/ingest_simple_filename_kinds.py` already
handles for five other kinds (verified directly: `cr_classes.lst:367`'s real `Assassin` row), so
this is a `decisions.md §17` "generic pass" case, not per-class work — but not literally that
script's sixth kind: a class row's leading field carries a `CLASS:` tag
(`CLASS:Assassin\tHD:8\t...`) the other five kinds' bare-identity leading field never does, so a
byte-exact port would have rejected every single real row (proven directly by
`ClassIdentityStripTests` in the new test file). Wrote `scripts/ingest_class.py`: same citation-
resolution, PI-screening, and provenance-stamping discipline as `ingest_simple_filename_kinds.py`,
generalized to strip exactly one leading tag.

**Self-caught defect, fixed before commit — the wave-1 lesson landing on this cycle directly.** The
first pass wrote the 28 `bestiary`-book monster-HD-progression pseudo-classes (`Aberration`,
`Animal`, `Construct`, ... `Vermin` — `core_essentials/ce_classes_race.lst`, PCGen's per-creature-
type save/BAB progression table, itself a real `CLASS:` row, not a defect in the corpus) to the
UNALIASED `data/corpus/bestiary/class/` directory. `shape_ledger.py`'s `BOOK_CORPUS_DIR_ALIASES`
(`bestiary` -> `beastiary`) means a `--books`-restricted reader (which is how `main()` always calls
it) never walks `data/corpus/bestiary/`, so those 28 records stayed `no_record` despite being
written — caught by re-deriving the ledger after the first write rather than trusting the write
count (`decisions.md §17a`'s standing instruction). Fixed by importing `BOOK_CORPUS_DIR_ALIASES`
from `shape_ledger.py` into the writer's own output-directory computation, removing the 28 stray
files, and rewriting them under `data/corpus/beastiary/class/`. Regression-tested directly
(`BookAliasOutputDirTests`), not just fixed and re-run.

21 units skip on a name-blacklist hit (a class whose own name is a Paizo-original proper noun:
Aldori Swordlord, Hellknight [two books], Red Mantis Assassin [two books], etc — see
`20-class-pi-skipped.json` for the full list and reasoning) — per `decisions.md §15`, never
transcribed, never silently skipped.

Additive-only, verified: `data/corpus/**/*.json` count 41,611 -> 41,748, exactly +137.

### `race` (not started this cycle)

Investigated but not implemented, per this wave's instruction to land two kinds cleanly rather than
four half-built. `IN_SCOPE_RACES` (`src/bin/ingest_races.rs`) is the machinery, per T2b precedent.
The 59 `race` `no_record` units are a **mixed population**, confirmed by direct inspection of the
pinned oracle against `IN_SCOPE_RACES`'s existing `core_essentials/races/<dir>/` shape:

- **At least 6 genuine, un-widened playable races** with a matching `core_essentials/races/<dir>/`
  chassis directory present in the corpus (same flat `<dir>_races.lst` +
  `<dir>_abilities_race.lst` + `<dir>_abilities_globalvar.lst` shape `IN_SCOPE_RACES`'s doc comment
  already describes as the widening precedent): `changeling`, `trox`, `wyrwood`, `wyvaran`,
  `ghoran`, `kasatha`. Widening `IN_SCOPE_RACES` for these 6 (a config-only extension, same shape as
  every prior `IN_SCOPE_RACES` batch) is the next cycle's first move.
- **A substantial share is `decisions.md §16`'s classifier-noise pattern repeating in a fourth
  place** (T2b found it for `race_trait`; this is the `race` kind's own version): units like
  `bestiary:race:skeleton`, `bestiary:race:zombie`, `bestiary_2:race:hydra_cryohydra`,
  `bestiary_2:race:iron_cobra_*`, `occult_adventures:race:phantom`/`homunculus_companion`,
  `ultimate_psionics:race:horror`, and multiple `companion_<animal>` ids are monster stat blocks and
  animal-companion entries, not playable races — `refine_kind`-shaped mistyping, not content to
  ingest. Building race chassis for these would be exactly the `decisions.md §1a`-forbidden
  "fabricate content to close a counter." A next-cycle classifier fix (same adversarially-verified
  discipline `decisions.md §16` required) is the correct path, not a per-race ingest lane.
- A residual (`inner_sea_world_guide`'s named locations/creatures, `advanced_race_guide:race:race`
  — a garbage identity, likely a census artifact) needs direct per-record review before either
  bucket.

**Reachability claim:** none made. `reach_gate.rs` was not checked for `class`/`equipment`/
`equipment_modifier` entries this cycle; Gate-1 measurability (this receipt's actual claim) and
player-reachability are different claims, per the brief's standing lesson.

## Verification

- `cargo test --locked --lib rules_core::cache_gen::equipment_gap::tests` — 15/15 pass (RED
  confirmed first: `book_routing_covers_every_non_ue_gap_book` widened to include `ISTEM`/`ISM`/
  `AG`, failed for `ISTEM` before the routing fix, passed after).
- `cargo test --locked --test equipment_gap_tables` — 7/7 pass.
- `cargo test --locked --lib rules_core::equipment_resolver::tests` — 14/14 pass (2 pinned-count
  regressions found and fixed: catalog total 7866 -> 8025, one new cross-book collision named).
- `cd apps/desktop/src-tauri && cargo test --locked equipment_catalog::` — 17/17 pass (3 pinned-
  count regressions found and fixed: ArmsArmor category total, description-coverage totals, cross-
  book collision total — all re-derived from the regenerated table, not hand-adjusted).
- `python3 -m unittest scripts.tests.test_ingest_class` (run from `scripts/`) — 8/8 pass.
- `data/corpus/**/*.json` file count: 41,403 -> 41,748 (exactly +345, additive-only, verified by
  direct count before/after each write, never a deletion or overwrite).
- `corpus_literal_sweep` could not be run to completion: it exits 2 on the **first** malformed
  `source.path` it walks (`data/corpus/advanced_class_guide/domain/battle_spirit.json`, a
  pre-existing, unrelated `domain`-kind record missing its `pathfinder/` path segment — the exact
  wave-1 defect this bundle's brief already flags as "a repair lane is running now"). Not this
  cycle's regression: confirmed directly this cycle's own new records (`equipment`, `equipment_
  modifier`, `class`, all three write paths) carry the correctly-shaped `pathfinder/...` prefix by
  direct inspection of a sample from each.

## Next-cycle plan

1. Widen `IN_SCOPE_RACES` for the 6 confirmed races (`changeling`, `trox`, `wyrwood`, `wyvaran`,
   `ghoran`, `kasatha`) — config-only, same shape as every prior batch.
2. Investigate `race` kind's classifier-noise share (monster/companion mistyping) with the same
   adversarial-verification discipline `decisions.md §16` required for `race_trait`.
3. `equipment` residual (170): re-derive per-book after this cycle's fixes; `ultimate_magic`'s 19
   units need this generator's selection predicate widened to cover `status ==
   "ingested-magnitude"`/`"unknown"`, tested before landing.
4. `equipment_modifier` residual (175): not yet investigated per-book this cycle.
