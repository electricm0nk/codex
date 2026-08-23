# Cycle 003 — Gate 1 / `class_feature` `no_record` closure (decisions.md §20)

- **Card ID:** `gate-1-shape-closure` (row 5) — `no_record` ingestion mandate, `decisions.md §20`
- **Commit SHA:** 649c072ae
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` (scope widening + `foreign_citations` guard)
  - `data/corpus/**/class_feature/**/*.json` (12,384 refreshed `ingested_at`/citation-path stamps
    from the idempotent regen + ~5,464 newly-written records; regenerated through
    `cargo run --locked --bin gen_cache_class_feature`, never hand-edited)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff of
  `src/rules_core/cache_gen/class_feature.rs`, per workflow-instruction.md §6's guidance that the
  full `BASE_BRANCH...HEAD` form returns pre-existing tagged lines and is not a per-cycle signal)
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`,
  not 'budget not exceeded'" — applied to this cycle's scope, the `class_feature` kind (the
  largest of the 18 named populations, 5,604 units at wave start).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete (this kind's ingestible population); 140-unit PI-protected residual named,
  not closed (see below)
- **Notes:**

## What was found

Per `decisions.md §17`'s "find the lever, don't build a snowflake pass" instruction, this cycle
started from the existing `src/rules_core/cache_gen/class_feature.rs` +
`src/bin/gen_cache_class_feature.rs` generic transcription pipeline (already found by an earlier
T2a/T12 cycle) rather than writing a new one. `BOOK_PRIMARY_FILES` scoped it to 21 of 23 real
books' PRIMARY `*_abilities_class.lst` file only, excluding `ultimate_psionics` and
`pathfinder_unchained` entirely, and excluding every book's nested `support/*abilities_class*.lst`
variant files. All three restrictions turned out to be stale or over-broad:

1. **`ultimate_psionics` exclusion was stale.** Its module doc comment cited a `book_dir_of`
   5-segment-path bug (`src/bin/corpus_literal_sweep.rs`) that was independently fixed in commit
   `014f210b9` (a 4-segment `dreamscarred_press` branch), landed before this exclusion's own commit
   but never noticed. Re-verified this cycle: `book_dir_of("pathfinder/dreamscarred_press/
   ultimate_psionics/up_abilities_class.lst")` already resolves correctly.
2. **Nested `support/*abilities_class*.lst` files were never in scope at all** — 100% of this
   kind's `no_record` population (56 distinct filenames, re-derived from `docs/work-inventory.json`)
   matches the `*abilities_class*.lst` naming convention, primary or nested.
3. **`pathfinder_unchained` was excluded wholesale** to protect 64 hand-curated `class_feature`
   records (different schema, `data.class_key`/`base_class_key`, from an earlier mechanism-wiring
   cycle) — but that left 536 OTHER `class_feature` units in the same book, ones the hand-curation
   never touched, permanently `no_record`.

## What changed

- `BOOK_PRIMARY_FILES` gained `ultimate_psionics` and `pathfinder_unchained` (21 → 23 books).
- `units_from_inventory_json` widened from "book's own listed primary file only" to "any
  `class_feature` unit of a known book whose `source_file` contains `abilities_class`" — closing
  the nested-support-file gap for every book at once, not per-book.
- `generate()` gained `resolve_book_file` (a recursive basename search under the book directory,
  mirroring `wiring_class::resolve_corpus_file`'s existing shape) so a nested file's real path is
  found instead of assumed flat, and now writes each record's `source.path` as the REAL relative
  path it read from (previously always the flat `<book-dir>/<primary-file>` assumption).
- `generate()` gained `foreign_citations` — a per-unit guard, keyed on `data.class_key` presence,
  that skips (never overwrites, never duplicates) any citation a foreign (non-generic) record
  already covers. This is what let `pathfinder_unchained` re-enter scope without touching its 64
  hand-curated records — verified: `git status --porcelain` on all four hand-curated class
  directories is empty after the regen.
- 5 new unit tests (`book_primary_files_covers_the_23_in_scope_books`,
  `units_from_inventory_json_accepts_any_abilities_class_file_of_a_known_book`,
  `resolve_book_file_finds_a_nested_support_file_by_basename`,
  `foreign_citations_finds_only_records_carrying_class_key_never_this_generators_own`, plus the
  renamed primary-files count test). RED→GREEN proven by temporarily reverting the
  `ultimate_psionics` entry and re-running `book_primary_files_covers_the_23_in_scope_books`
  (failed `left: 21, right: 22` for the intended reason), then restoring — see command log below.

## Re-derived, not assumed (`§17a`)

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "
import json,collections
r=json.load(open('/tmp/l.json'))['rows']
nr=[x for x in r if x['join_status']=='no_record']
print(collections.Counter(x.get('kind','?') for x in nr).most_common())"
```

| Population (`class_feature`, `join_status == no_record`) | Before | After |
|---|---:|---:|
| Total | 5,604 | **140** |

Bundle-wide `no_record` total (all 18 kinds, `decisions.md §20`'s population): **20,889 → 15,425**
(population unchanged at 36,028 — this cycle only ingests already-enumerated units, adds/removes
none). Gate 3's evidence-gated budget (`NO_RECORD_BUDGET_COUNT=21521`/`POPULATION=36028`,
untouched this cycle per the brief) now reads `15425/36028 vs. baseline 21521/36028 — exceeded:
False` (`python3 scripts/shape_coverage_standing_gate.py`).

## The 140-unit residual — named, not silently rounded into "done" (`§16`)

100% of the remaining `class_feature` `no_record` units are PI-protected: their PCGen row declares
`NAMEISPI:YES`, and the generator's own pre-existing screen (this cycle did not touch it) correctly
refuses to transcribe a redacted name (`decisions.md §15`). Verified exact-match, not
coincidental:

```
grep -c "NAMEISPI:Yes" .../inner_sea_world_guide/iswg_abilities_class.lst   # 29
```
— equals the 29 `inner_sea_world_guide` units in the 140-row residual. Per-book: `adventurers_guide`
49, `inner_sea_world_guide` 29, `inner_sea_combat` 21, `inner_sea_magic` 21, `inner_sea_intrigue`
11, `book_of_the_damned_volume_2` 7, `advanced_class_guide` 1, `advanced_players_guide` 1 — sums to
140, the generator's own reported `name_pi_skipped` count for this run. **This residual is a
correct PI disposition, not a defect**, and is out of `§15`'s "transcribe only PI-clear units" scope
by construction — no operator ruling needed, no blacklist amendment applies (`class_feature` is not
one of T9's kinds).

## Fixture discipline (`§3`)

`corpus_literal_sweep` (whole-repo, no book filter available) run against the pinned oracle after
the regen: **`corpus-literal-sweep: CLEAN`** — every shipped record, including the ~5,464 new
`class_feature` ones, verifies byte-for-byte against its cited corpus line.

## Reachability scope (`§3`/brief)

This cycle's claim is scoped to **ingestion**, not per-record player-reachability proof: every
newly-written record carries a `wiring_class` the existing `WiringClassIndex` machinery computes
from the corpus row itself (unchanged logic, this cycle only widened which rows reach it). Proving
every one of ~5,464 new records reaches a live character sheet is a distinct, much larger scope
(per-class dispatch wiring) that this cycle does not claim and does not attempt — named here rather
than silently implied. `reach_gate.rs`'s existing `("class_feature", "class_features")` /
`pu_class_features_reach()` machinery is unchanged.

## Known downstream shortfall, named not hidden

Nested-support-file and `ultimate_psionics`/`pathfinder_unchained` records now cite a real,
non-flat `source.path`. `corpus_literal_sweep`'s SEPARATE `--json-out` writer derives a verified
triple's `"book"` field from `source_path.parent().file_name()` (pre-existing bug,
`OPEN-ISSUES.md` SD-31 row 22, `src/bin/corpus_literal_sweep.rs:267-276`, not edited this cycle) —
so a nested-file record's `--json-out` "book" comes out as `"support"`, blocking
`literal-verified` stamping via `v06_work_inventory`'s `apply_done_rung_stamps` join for that
subset. This does **not** block `shape_ledger.py`'s join (confirmed: `no_record` closed for these
units) and does not block `corpus_literal_sweep`'s own CLEAN verdict (which uses `book_dir_of`,
not the `--json-out` writer). A future cycle fixing row 22 will pick up `literal-verified` for this
subset for free; not attempted here (out of this cycle's file-touch scope).

`data/class_feature_grants` (wave 22's grant-fact tree, used only as the first-priority `class`-
field resolution tier) was not regenerated for the two newly-added books — `true_class_by_key`
falls back to its next resolution tiers unchanged, the same fallback every other ungranted book
already uses. Named as a quality-enhancement opportunity, not a `no_record` blocker.

- **Discovery forwards:** none filed as `## DISCOVERED` — both findings above are self-contained
  and already logged in this receipt and the module's own doc comment.
- **Next-cycle plan:** the remaining bundle-wide `no_record` (15,425, 17 kinds) is out of this
  cycle's scope (`class_feature` only). Sibling cycles per `decisions.md §20`'s per-kind table.
