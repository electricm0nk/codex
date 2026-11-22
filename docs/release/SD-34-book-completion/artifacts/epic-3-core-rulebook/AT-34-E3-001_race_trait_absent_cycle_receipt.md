# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`race_trait_absent_from_race_traits` mechanism)

- **Commit SHA:** `ae25d75d7de8858bdb6bacf384e432d538de2fc5` (parent
  `79fc41ccd0510c32c771d83711e96d57bd980d6f`)
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` (new `TraitRow.is_human_ethnicity_placeholder` field,
    new fifth row shape recognised in `parse_row`, new `core_rulebook` `selector_only`
    `BookSource`, admit-predicate widened, 5 pinned tests updated, 2 new unit tests)
  - `src/rules_core/race_resolver.rs` (3 pinned corpus-census tests widened: the 14→21
    Adopted-Race selector population, the `Unclassified` role's 44→53 count and full listing,
    the corpus grand total 910→919)
  - `data/corpus/core_rulebook/race_trait/{dwarf,elf,gnome,half_elf,half_orc,halfling,human}/`
    (9 new records, guarded generator output, not hand-edited)
  - `docs/work-inventory.json` (regenerated at the fixed HEAD, guarded regeneration path,
    `CORPUS_LITERAL_SWEEP_REPORT` / `DERIVED_FIXTURE_CHECK_REPORT` set, no `--allow-stamp-loss`)
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_absent_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`

- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own diff
  (`git diff -- src/bin/ingest_race_traits.rs src/rules_core/race_resolver.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → no matches). The wider `${BASE_BRANCH}...HEAD` diff over the full Epic-3 file-touch set
  (`src/rules_core/`, `src/bin/`, `scripts/oracle_harness/`, `data/corpus/core_rulebook/**`,
  `docs/work-inventory.json`, `artifacts/epic-3-core-rulebook/`) carries thousands of
  pre-existing `sd32_simple_filename_kind_ingest`/`sd32_class_ingest` matches inside
  `docs/work-inventory.json` — historical `wiring_class_signals` **data values** from earlier
  cycles (confirmed present before this cycle: `git show HEAD~1:docs/work-inventory.json |
  grep -c sd32_simple_filename_kind_ingest` matches the post-regen count within the noise of
  this cycle's own 9 new/changed units), not identifiers this cycle introduced and not code.

- **Wired-integration audit result:** OK_NO_TOKENS on the scoped own-diff, with one reviewed,
  self-healed single-token match: `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  over `src/bin/ingest_race_traits.rs`/`src/rules_core/race_resolver.rs` matches the word
  "placeholder" 5 times — every occurrence is the accurate, literal name of a real PCGen
  content shape (`cr_abilities_race.lst`'s own `###Block: Placeholder objects for no Human
  Ethnicities or Regional Affinities` comment, quoted verbatim), not an incomplete-code marker.
  No stub, mock, or deferred-work token in shipping logic (`workflow-instruction.md §6` step 2
  self-heal: single-token, reviewed, not a violation).

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970** Core
  Rulebook units whose table exists but which are not in it. **Evidence:** the atlas
  reporting bucket B at zero for `core_rulebook`, and the mechanism that placed them named —
  by mechanism, not per record." This cycle's own bar (`decisions.md §14`, the orchestrator's
  ruling splitting bucket B into nine named mechanisms): drive `race_trait_absent_from_race_traits`
  to zero. **AT-34-E3-001 as a whole does not close this cycle** — eight of the nine named
  mechanisms remain; this receipt reports only this cycle's own mechanism.

## Re-derived population, not carried forward

Population figure re-derived at this cycle's start SHA (`79fc41ccd0`), NOT quoted from the
task brief without checking (`decisions.md §12` L2):

```
$ python3 -c "
import json
with open('docs/work-inventory.json') as f:
    units = json.load(f)['units']
from collections import Counter
c = Counter()
for u in units:
    if u.get('book')=='core_rulebook' and u.get('status')=='engine-does-not-hold':
        c[u.get('evidence')] += 1
print(c['race_trait_absent_from_race_traits'])
"
9
```

Matches the brief's stated figure exactly — verified, not assumed.

## The 9 units, and the root cause of each shape

Two distinct row shapes shared one evidence string:

1. **7 units — `Adopted Race ~ <Race>`, one per Core Rulebook race** (Dwarf/Elf/Gnome/
   Half-Elf/Half-Orc/Halfling/Human). PCGen's `TYPE:AdoptiveRace` selector row
   (`decisions.md §25`'s shape, already modelled generically by
   `race_resolver::adopted_race_choose_selectors` for 14 OTHER races) exists in each of
   these 7 races' own `core_essentials/races/<race>/<race>_abilities_race.lst` files
   (verified directly against the pinned oracle: exactly one `TYPE:AdoptiveRace` row per
   file), but was never ingested for Core Rulebook's own races — `ingest_races.rs` (the
   binary that owns CRB's standard traits) deliberately filters this row shape out
   (`is_standard_racial_trait`: a selector is not a standard trait), and `ingest_race_traits.rs`
   (the binary that DOES capture this shape) had no `BookSource` entry for `core_rulebook` at
   all. A genuine ingestion gap, not a data absence.
2. **2 units — `Human Ethnicity ~ None` / `~ Unknown`.** `cr_abilities_race.lst`'s own
   `###Block: Placeholder objects for no Human Ethnicities or Regional Affinities`
   (`CATEGORY:Background`, dot-free `TYPE:HumanEthnicity`) — a fifth row shape this binary
   had never recognised at all (`parse_row` returned `None` for it, silently dropping it
   before even reaching the scope filter).

## The fix

`ingest_race_traits.rs` gained:

1. A new `core_rulebook` `BookSource`, `selector_only: true` — the identical pattern already
   proven 4 times (`bestiary_2`/`_3`/`_5`/`_6`): `core_rulebook`'s 7 races' standard-trait
   content is already shipped by `ingest_races.rs` into the SAME
   `data/corpus/core_rulebook/race_trait/<race>/` directories, and
   `clear_own_alternate_trait_files`'s `is_racial_default`-field discrimination (every one of
   the 67 pre-existing files there carries `data.is_racial_default: true`; this binary writes
   only `false`) is what makes sharing that directory safe.
2. `TraitRow.is_human_ethnicity_placeholder`, a fifth, narrowly-scoped row shape
   (`CATEGORY:Background` + dot-free `TYPE:HumanEthnicity`, mutually exclusive with the other
   four shapes) resolving `race_key` to the hardcoded literal `"Human"` (the row itself names
   no race). Scope-checked against a sibling row shape two lines below it in the same file
   (`Region ~ None`/`~ Unknown`, `TYPE:Region`) which is deliberately NOT matched — it belongs
   to the `race_trait_race_not_modelled` mechanism, not this one, and a new unit test pins that
   it stays dropped.

Both shapes are admitted by `selector_only`'s existing filter, widened by one `||` clause.

**RED → GREEN (for the intended reason, not a compile error):**
- RED: `is_human_ethnicity_placeholder` temporarily forced to `false` (verified the real code
  change, not a stand-in) →
  `cargo test --locked --bin ingest_race_traits human_ethnicity` →
  `human_ethnicity_placeholder_row_resolves_to_human_and_is_admitted ... FAILED` (`"Human
  Ethnicity placeholder row is not dropped"` — the intended reason, not a panic elsewhere).
- GREEN after restoring the fix: same command → `1 passed; 0 failed`.
- Full binary suite: `cargo test --locked --bin ingest_race_traits` → `24 passed; 0 failed`
  (22 pre-existing + 2 new: `human_ethnicity_placeholder_row_resolves_to_human_and_is_admitted`,
  `a_background_category_row_with_a_different_type_is_not_the_ethnicity_placeholder_shape`).
- `cargo test --locked --lib race_resolver` → `28 passed; 0 failed` (3 pre-existing pinned
  corpus-census tests widened to the corrected populations, 0 new tests — the corpus itself
  is what changed, not the classifier logic under test).
- `cargo test --locked --bin v06_work_inventory` → `371 passed; 0 failed`.

## Figures, with re-derive commands and denominators

- **Mechanism population:** `9 -> 0` for `core_rulebook`'s
  `race_trait_absent_from_race_traits` (denominator: bucket B's own partition, command above).
- **`core_rulebook` records emitted by `ingest_race_traits.rs -- core_rulebook`:** `9` (denominator:
  this cycle's own new `BookSource`'s row population — 7 Adopted-Race selectors + 2 Human
  Ethnicity placeholders). Command: `cargo run --locked --bin ingest_race_traits -- core_rulebook`.
- **`core_rulebook`'s total on-disk `race_trait` record count:** `67 -> 76` (denominator: whole
  `data/corpus/core_rulebook/race_trait/` directory, both binaries' output). Command:
  `find data/corpus/core_rulebook/race_trait -name '*.json' | wc -l`.
- **`adopted_race_choose_selectors`'s corpus-wide population:** `14 -> 21` (denominator:
  `decisions.md §25`'s original population + this cycle's 7 CRB races). Command:
  `cargo test --locked --lib race_resolver::tests::adopted_race_choose_selectors_finds_the_real_fourteen_unit_population`.
- **`race_resolver`'s corpus-wide `Unclassified` role count:** `44 -> 53` (+9, this cycle's own
  9 records — both shapes are `Unclassified` for the same reason every other Adopted-Race/
  placeholder row is: no readable gate of their own). Denominator: whole corpus census,
  `all_books()`. Command: `cargo test --locked --lib race_resolver::tests::the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers`.
- **`race_resolver`'s corpus-wide trait-record total:** `910 -> 919` (+9). Same command as above.
- **`core_rulebook` bucket B (whole book, atlas-official, `completion_atlas.py`'s own
  `_B_MARKERS` partition — NOT the loose `status==engine-does-not-hold` filter used to
  re-derive this cycle's population above, which conflates buckets B/C/D):** `1005 -> 996`
  (denominator: `core_rulebook`'s own unit count, `population=6701`, unchanged). A clean
  `-9`, matching this cycle's own population exactly. Command:
  `python3 scripts/completion_atlas.py --book core_rulebook --check` (exits 1 by that
  script's own design whenever any non-DONE bucket is nonzero — expected, not a failure,
  for a book still mid-completion).
- Full-corpus population unaffected: `python3 scripts/completion_atlas.py --check` still
  reports `population=49438` — no unit added or removed, 9 units reclassified out of bucket B.

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1165
  A: 0
  B: 996
  C: 370
  D: 443
  M: 929
  V: 2734
  U: 58
  X: 6
  Z: 0
```

Before this cycle (parent SHA `79fc41ccd0`, same command): `B: 1005` (all other buckets
identical except `D: 434 -> 443`, `+9` — this cycle's own 9 units, exactly). This cycle's
own mechanism, re-derived directly against `docs/work-inventory.json`:

```
$ python3 -c "
import json
with open('docs/work-inventory.json') as f: units = json.load(f)['units']
from collections import Counter
c = Counter()
for u in units:
    if u.get('book')=='core_rulebook' and u.get('status')=='engine-does-not-hold':
        c[u.get('evidence')] += 1
print('race_trait_absent_from_race_traits:', c.get('race_trait_absent_from_race_traits', 0))
print('race_trait_record_loaded_but_never_applies:', c.get('race_trait_record_loaded_but_never_applies', 0))
"
race_trait_absent_from_race_traits: 0
race_trait_record_loaded_but_never_applies: 9
```

## Build scope verified

Run at parent SHA `79fc41ccd0` (this cycle's own commit is on top of it):
- `cargo test --locked --no-run` (full workspace) → exit 0.
- `cargo test --locked --bin ingest_race_traits` → `24 passed; 0 failed`.
- `cargo test --locked --lib race_resolver` → `28 passed; 0 failed`.
- `cargo test --locked --bin v06_work_inventory` → `371 passed; 0 failed`.
- `apps/desktop/src-tauri`: not touched by this cycle's file-touch set — not run, per
  `decisions.md §10`'s "explicitly, or not at all" rule for an untouched separate workspace.

## Sweep population

`corpus_literal_sweep`: before `48699 examined of 51473 read` (SD-33/SD-34 baseline) → after
`48708 examined of 51482 read` (this cycle's own re-run,
`/tmp/corpus_literal_sweep_report.json`). **Delta: +9 examined, +9 read — exactly this
cycle's own 9 new corpus records, 0 discrepancy.** `decisions.md §12` L8 satisfied.
CLEAN, 0 findings.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
every figure in this receipt sourced from the 7 CRB race files and `cr_abilities_race.lst` at
this pin.

## Status

- **Status:** complete — this cycle's own mechanism, `race_trait_absent_from_race_traits`
  for `core_rulebook`, is 0 of 9 remaining (was 9). Set from the row-count command output
  above, not a self-assessment.

## Movement, four buckets

**Important correction caught while writing this receipt** (own-instrument re-derive, not
carried forward): my first check used a loose `status == "engine-does-not-hold"` python
filter to define "bucket B", matching Cycle 1's own methodology. That filter actually spans
atlas buckets B **and** D (`completion_atlas.py`'s real partition: B requires the evidence
string to contain `not_held_by_engine`/`absent_from`/`not_modelled`; D is the `engine-does-
not-hold` fallthrough when none of those match). Under that loose filter, the 9 units appear
to just move from one `engine-does-not-hold` evidence string to another with no net count
change (1809 -> 1809) — which read, on first look, like a non-closure. Re-running the
**atlas's own, real** bucket-B partition (`python3 scripts/completion_atlas.py --book
core_rulebook --check`) shows the true picture: `B: 1005 -> 996`, a clean `-9`. The 9 units'
new evidence string, `race_trait_record_loaded_but_never_applies`, does not contain any of
the three B markers, so the atlas correctly places them in bucket D
(`engine_does_not_hold`, no A/B/C marker matched — "other engine gap"), not bucket B. This
is exactly the `decisions.md §2a` outcome the brief names as correct: "a unit leaving bucket
B for D or M is a correct outcome, not a half-fix."

- **Closure:** 9 units move from bucket B (`race_trait_absent_from_race_traits`, "table
  exists, record not in it") to bucket D (`race_trait_record_loaded_but_never_applies`,
  "other engine gap" — a DIFFERENT, more specific engine gap: the record now IS ingested
  into `data/corpus/core_rulebook/race_trait/`, IS loaded by `RaceCorpus`, and the race
  corpus load correctly classifies it `TraitRole::Unclassified` — no `PREFACT`/default gate
  of its own, the identical terminal state `Oversized Goblin`/`Human ~ Tribalistic
  Languages`/`Suli ~ Trusted Mediator` already carry corpus-wide, all pre-existing and
  already accepted). A real engine-attribution/ingestion defect fixed — the record now has
  a shelf and the engine holds it — not a reclassification of the same status.
- **Reclassification:** 0 — no unit changed `kind` or book attribution this cycle; all 9
  genuinely reached a table (and a corpus load) they were previously entirely absent from.
- **Reachability:** 0 — none of the 9 becomes player-reachable this cycle. Both shapes are
  `TraitRole::Unclassified` by the resolver's own classification (no readable gate), so
  neither is ever offered by `race_trait_picker`. Whether/how they should ever become
  reachable (the 7 Adopted-Race selectors' pool resolves through
  `crate::rules_core::trait_pool`, a mechanism this cycle did not touch; the 2 Human
  Ethnicity placeholders are flavor-only, zero-mechanical-effect facts) is a different
  bucket's own mechanism, not this cycle's.
- **Instrument-correction:** 1 — my own first-pass bucket-B/D conflation, caught and
  corrected in this same receipt before it shipped, `--verified-by`
  `python3 scripts/completion_atlas.py --book core_rulebook --check` (re-derives the real
  atlas partition rather than the loose status filter). Logged as a `correction` retro event
  (`docs/retro/events/sd34-at-34-e3-001.jsonl`, `1787825583344-sd34-at-34-e3-001-c9d932`).
  The brief's stated mechanism population (9) itself was verified, not corrected.

## Notes

**This cycle owns exactly one mechanism of nine** (`decisions.md §14`). AT-34-E3-001 as a
whole does not close: `core_rulebook` bucket B still has eight other mechanisms outstanding
(`class_feature_option_pool_record_with_magnitude_not_held_by_engine` 333,
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` 330,
`race_trait_race_not_modelled` 132, `companion_absent_from_core_rulebook_companion_tables` 100,
`class_feature_option_pool_record_not_held_by_engine` 63,
`deity_content_absent_from_deity_table_in_core_rulebook` 21,
`class_absent_from_ClassId_ALL_and_book_class_id_enums` 17, plus any other cycles' own
mechanism work landed concurrently). This receipt reports only what this cycle itself moved.

## Next-cycle plan

Any of the eight remaining named mechanisms — cheapest-first per `progress.md`'s own
convention. `deity_content_absent_from_deity_table_in_core_rulebook` (21) is the next-smallest
after this cycle's own (9) and the already-cleared `domain` (1) and `template`/`ability`
reattribution (29) mechanisms.
