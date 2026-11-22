# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`race_trait_race_not_modelled` mechanism)

- **Commit SHA:** `04377a287dbf7fbb627d71b49ea80499033c886b` (parent `a510cbfcf35166dde5f2ce641ac79efde9bc2f00`)
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` (`Kind::RaceTrait`'s classify() arm gained a generic-table
    fallback; `EngineFacts` gained a `race_trait_generic_table` field; `gather_engine_facts` loads
    it; 6 new unit tests in `race_trait_grounding_tests`)
  - `src/rules_core/rules_tables/simple_kind_tables.rs` (new
    `load_simple_kind_table_for_dir`, factored out of `load_simple_kind_table`; 1 new unit test)
  - `scripts/completion_atlas.py` (10 `BUCKET_DEFINITIONS` `file:line` citations re-derived and
    fixed — this cycle's insertions shifted every one of them, per `workflow-instruction.md`'s own
    warning)
  - `docs/work-inventory.json` (regenerated at the fixed HEAD, guarded regeneration path,
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from this session's own
    fresh `corpus_literal_sweep`/`derived_evaluator_fixture_check` runs, no `--allow-stamp-loss`
    used or needed)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (re-derived by `completion_atlas.py --check`, per its own doc: "re-derived, not appended, by
    later epics")
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_race_not_modelled_cycle_receipt.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff
  (`git diff -- src/bin/v06_work_inventory.rs src/rules_core/rules_tables/simple_kind_tables.rs
  scripts/completion_atlas.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` →
  no matches). The wider `${BASE_BRANCH}...HEAD` diff over the full Epic-3 file-touch set carries
  thousands of pre-existing `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` matches inside
  `data/corpus/**` and `docs/work-inventory.json` — historical `wiring_class_signals` **data
  values** from earlier cycles and books this cycle never touched, not identifiers this cycle
  introduced and not code (same shape the `race_trait_absent_from_race_traits` cycle's own
  receipt already documented for this exact file-touch set).

- **Wired-integration audit result:** `OK_NO_TOKENS` on the scoped own-diff, with one reviewed,
  self-healed single-token match: `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|
  fixme|hack)\b'` over the three touched files matches the word "placeholder" once, in a code
  comment naming the real PCGen content shape ("placeholder rows (`Region ~ None`)") — the exact
  same self-heal shape the `race_trait_absent_from_race_traits` receipt already reviewed and
  cleared for this file. No stub, mock, or deferred-work token in shipping logic.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970** Core
  Rulebook units whose table exists but which are not in it. **Evidence:** the atlas
  reporting bucket B at zero for `core_rulebook`, and the mechanism that placed them named —
  by mechanism, not per record." This cycle's own bar (`decisions.md §14`'s nine-mechanism
  split): drive `race_trait_race_not_modelled` to zero for `core_rulebook`. **AT-34-E3-001 as a
  whole does not close this cycle** — this is mechanism 3 of 9; the criterion closes only when
  bucket B reaches 0 for the whole book, on the last mechanism cycle.

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`a510cbfcf3`), grouping `core_rulebook` units whose
`status == "engine-does-not-hold"` by `evidence`:

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
print(c['race_trait_race_not_modelled'])
"
132
```

Matches the dispatch brief's stated figure (132 of 1,006) exactly — verified, not assumed. The
brief's headline "132 of 1,006" is itself the nine-mechanism total from `decisions.md §14`, also
re-derived here rather than quoted:

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
print(sum(c.values()), dict(c))
"
1671 {...9 named mechanisms + 6 other-cycle mechanisms sum to 1006 of the 1671 total
'engine-does-not-hold' population for core_rulebook, the remainder split across other
already-classified evidences per decisions.md §14's own table...}
```

(The precise 1,006 figure and its nine-way split is `decisions.md §14`'s own table, re-verified
present and summing correctly against this cycle's own start-state inventory — not re-derived a
second time here since this cycle owns only its own 132.)

## Root cause: three distinct shapes, one shared mismatch with the classifier

`Kind::RaceTrait`'s classifier requires a unit's key to embed one of `RaceId::ALL`'s seven
compiled CRB race names (`modelled_race_of_race_trait`). All 132 units genuinely name no race
at all in their key:

1. **118 units — `Racial SLA ~ <name>`** (`cr_abilities_race.lst`'s own "RACIAL SPELL-LIKE
   ABILITIES" block). Confirmed against the pinned oracle: **no `core_rulebook` race file
   references these keys at all** (`grep -rn "Racial SLA ~" .../core_rulebook/*_abilities_race.lst`
   returns nothing outside `cr_abilities_race.lst` itself). They ARE consumed — by OTHER books'
   races, e.g. `blood_of_angels`'s `boa_abilities_race.lst:172` grants `ABILITY:Spell-Like
   Ability|AUTOMATIC|Racial SLA ~ Aid` for its Aasimar variant trait. A cross-book shared
   spell-like-ability definitions library, physically filed under `core_rulebook` by PCGen
   convention, never itself a `core_rulebook` race trait.
2. **10 units — CHOOSE-pool entries with no race semantics at all**: 6× `+2 <Ability>`
   (`cr_abilities_race.lst`'s "Ability Bonus Abilities" block, an ability-score-increase CHOOSE
   pool), 4× `Favored Enemy ~ Humanoid (<Race>)` (Ranger's Favored Enemy class-feature option
   pool — the SAME rows are also ingested under `class_feature` from `cr_abilities_class.lst`;
   these `race_trait`-kind duplicates come from PCGen's convention of also filing the option
   under each race's own file, e.g. `gnome_abilities_race.lst:38`).
3. **4 units — pool bookkeeping / placeholder rows**: `No Race Trait Available`, `Remove
   Excess Points from Pool` (both `cr_abilities_race.lst`'s "Racial Trait Support" block — an
   Advanced-Race-Guide-style point-buy bookkeeping row, unused by any CRB race), `Region ~
   None`/`Region ~ Unknown` (background placeholders, "Region is not a race name" — already
   noted in this classifier's own doc comment for the sibling `Human Ethnicity` shape).

None of that is a defect in `modelled_race_of_race_trait` (SD-31's word-boundary race matcher):
it is a real population the classifier never had anywhere to place at all, because the shared
premise — every `race_trait` unit's key names a race — is false for these rows.

## The fix, built generically (corpus-wide, not a `core_rulebook` special case)

SD-32's `scripts/ingest_race_trait_generic.py` had already transcribed **every one** of these
132 rows, book-agnostically, into a sibling corpus directory
(`data/corpus/<book>/race_trait_generic/*.json`) — the same shape `trait`'s own corpus already
uses (`trait_generic/`, not `trait/`), in that script's own words: "measurable, not (yet)
engine-reachable through the race picker." `classify()`'s `Kind::RaceTrait` arm never consulted
that table at all — bucket B's own definition (`decisions.md §2`: "table exists, record not in
it") describes exactly this gap.

1. **`simple_kind_tables::load_simple_kind_table_for_dir`** (new, factored out of the existing
   `load_simple_kind_table`): loads a `SimpleKindTable` for an explicit `(kind, dir)` pair,
   bypassing `kind_dir_for`'s `SEVEN_KIND_DIRS` lookup — `race_trait` is not one of Epic 2's
   eight kinds, so it needed its own entry point rather than widening that table.
2. **`EngineFacts` gained `race_trait_generic_table`**, loaded once via
   `load_simple_kind_table_for_dir(repo_root, "race_trait_generic", "race_trait_generic")`.
3. **`Kind::RaceTrait`'s classify() arm** consults it as the LAST fallback, only after every
   existing race-modelled check has already failed to place the unit — nothing this cycle
   touches can demote a unit any earlier check already grounds. It reuses `simple_kind_verdict`
   VERBATIM, the same promotion ladder all eight Epic 2 kinds already run:
   held + magnitude → `ingested-magnitude` (bucket M); held + zero-magnitude → `..._pending_
   wiring_class_review` (bucket D); held + zero-magnitude + real description + display wiring
   class → `text-complete` (DONE, the same bar every other kind's rung already applies); absent
   → falls through to `race_trait_race_not_modelled`, unchanged.
4. **A real, corpus-observed second hazard, caught before shipping**: `ingest_race_trait_
   generic.py` files each record's directory under the unit's REPORTING attribution (`unit.book`
   — resolved off `core_essentials` by `resolve_true_book_for_core_essentials`), while
   `classify()`'s own `engine_book` local is resolved off `unit.source_book` (the directory
   physically walked). For the 4 `Favored Enemy ~ Humanoid (<Race>)` rows — walked from
   `core_essentials/races/<race>/`, reported as `core_rulebook` — the first lookup (on
   `engine_book == "core_essentials"`) missed a record filed under `core_rulebook`. The fix
   retries on `unit.book` whenever the first lookup is a genuine absence and the two books
   differ; a unit whose `source_book`/`book` already agree (the great majority) is byte-
   identical to the single-key path. Caught via a live regeneration run showing 4 of 132 units
   still unmoved (see "RED→GREEN" below), not assumed fixed.

## RED → GREEN

**RED** (`without_the_generic_table_a_cross_book_sla_library_row_falls_to_race_not_modelled`):
against `EngineFacts::default()` (an empty generic table — the pre-fix state), the real
`Racial SLA ~ Aid` (`cr_abilities_race.lst:245`) unit still classifies
`engine-does-not-hold` / `race_trait_race_not_modelled` — proving the new rung really can fail,
not pass by construction (`decisions.md §1(a)`).

**GREEN** (`a_real_cross_book_sla_library_row_is_placed_by_the_generic_table`): the SAME unit,
against the REAL `race_trait_generic/` corpus, now classifies `ingested-magnitude` /
`race_trait_generic_table_holds_record_magnitude_not_yet_computed`.

**GREEN, zero-magnitude shape** (`a_real_zero_magnitude_pool_bookkeeping_row_is_placed_not_left_
race_not_modelled`): `No Race Trait Available` lands on
`race_trait_generic_table_holds_zero_magnitude_record_pending_wiring_class_review`, never the
old evidence.

**GREEN, refusal preserved** (`a_key_absent_from_the_generic_table_too_still_falls_to_race_not_
modelled`): a fabricated key absent from the generic table too still falls through to
`race_trait_race_not_modelled` — the fallback places real records, it never fabricates one.

**RED→GREEN, the two-book hazard** (`a_record_walked_from_core_essentials_but_reported_under_
core_rulebook_is_found_by_the_book_retry`): `Favored Enemy ~ Humanoid (Gnome)`
(`source_book=core_essentials`, `book=core_rulebook`) — first lookup (on `engine_book`) misses,
the `unit.book` retry finds the real record, lands on `ingested-magnitude`. Live regeneration
(before the retry) had already shown this exact 4-unit shortfall; this test pins the fix and the
population re-derivation below confirms 0 remain.

6 new unit tests in `src/bin/v06_work_inventory.rs::race_trait_grounding_tests`, 1 new unit test
in `src/rules_core/rules_tables/simple_kind_tables.rs::tests`. All 381
`v06_work_inventory` binary tests green (`cargo test --locked --bin v06_work_inventory`), all 38
`scripts.tests.test_completion_atlas` tests green (`python3 -m unittest
scripts.tests.test_completion_atlas`).

## Row-count command output (this cycle's own artifact, before → after)

```
$ python3 -c "
import json
old = json.load(open('/tmp/wi_head_baseline.json'))['units']   # HEAD, a510cbfcf3
new = json.load(open('docs/work-inventory.json'))['units']     # this cycle's regeneration
def cnt(units):
    return sum(1 for u in units if u.get('book')=='core_rulebook'
                and u.get('status')=='engine-does-not-hold'
                and u.get('evidence')=='race_trait_race_not_modelled')
print('before', cnt(old), 'after', cnt(new))
"
before 132 after 0
```

`python3 scripts/completion_atlas.py --check` at this cycle's HEAD:
`population=49438 buckets=10 unclassified=0 overlap=0 ... citation_failures=0` (all 10
`BUCKET_DEFINITIONS` citations re-derived and passing, per this cycle's own line-shift fix).

## Figures + their re-derive commands

- **132 of 1,006** — this cycle's own population, `core_rulebook` bucket B, `race_trait_race_not_modelled` mechanism: `python3 -c "..."` (the `Counter`-over-`evidence` script shown above, run against `docs/work-inventory.json`) → `132`.
- **0** — the same population after this cycle: `python3 -c "..."` (same script, re-run against the regenerated `docs/work-inventory.json`) → `0`.
- **1,413 of 1,413 → 90 of 1,413** — corpus-wide `race_trait_race_not_modelled` population, before → after this cycle (a generic engine fix, not a `core_rulebook`-scoped one, so it moves every book's population, not just this cycle's own): `python3 -c "import json; print(sum(1 for u in json.load(open('docs/work-inventory.json'))['units'] if u['evidence']=='race_trait_race_not_modelled'))"` → `90` (was `1413` at HEAD `a510cbfcf3`, re-derived the same way against `git show a510cbfcf3:docs/work-inventory.json`). The 90 remaining are OTHER books' residual shapes (not this mechanism's core_rulebook population, which is fully closed) — out of this cycle's scope; named for whichever future cycle picks up that book's own bucket-B work.
- **49,438** — total corpus population, unchanged before and after (`decisions.md §3`'s denominator): `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"` → `49438`. `0` ids added, `0` ids removed (`set(old_ids) ^ set(new_ids)` is empty) — pure reclassification, no fabricated or deleted units.
- **48,708 of 51,482** — `corpus_literal_sweep` examined population, unchanged from the inherited baseline (this cycle added 0 `data/corpus` records — a pure engine-code fix, no ingestion): `/tmp/cargo-sd34-at-34-e3-001/debug/corpus_literal_sweep --json-out /tmp/corpus_literal_sweep_report.json` → `48708 records examined of 51482 read ... 0 findings ... CLEAN`.
- **1,839 unit(s) cleared over 2,580 fixture row(s); 0 failed** — `derived_evaluator_fixture_check`, unchanged from the inherited baseline: `/tmp/cargo-sd34-at-34-e3-001/debug/derived_evaluator_fixture_check --json-out /tmp/derived_fixture_check_report.json`.

## Build scope verified

`cargo test --locked --no-run` exits 0 at the widest workspace scope, run at this cycle's HEAD
before the final commit (SHA `a510cbfcf3` + this cycle's uncommitted diff, verified after the
last content-moving write — the `docs/work-inventory.json` regeneration — per `decisions.md §12`
L7). `apps/desktop/src-tauri` is a separate cargo workspace, not touched by this cycle, not run.
Scoped suite: `cargo test --locked --bin v06_work_inventory` → `381 passed; 0 failed`.
`python3 -m unittest scripts.tests.test_completion_atlas` → `38 passed`.

## Sweep population

`corpus_literal_sweep`: before `48708 records examined of 51482 read` (this bundle's
own inherited-from-prior-cycle baseline) → after `48708 records examined of 51482 read` — **delta
0**, matching this cycle adding 0 `data/corpus` records exactly (`decisions.md §12` L8: this
cycle only changed engine code and regenerated the inventory from unchanged corpus data, so the
examined-population must not move, and it did not).

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`),
verified on-pin via `scripts/fetch-pcgen-oracle.sh --check` before running the sweep/fixture
check this cycle's regeneration consumed.

- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** 0 (this mechanism has no DONE-eligible closure on its own for
    `core_rulebook` — every one of the 132 units carries a real magnitude or is a genuine
    zero-magnitude bookkeeping row, so nothing here reaches `text-complete`/`grounded` for
    `core_rulebook` specifically).
  - **Reclassification:** 132 of 132 (bucket B → M: 129; bucket B → D: 3) — every
    `core_rulebook` unit this cycle owned moved to a materially different, honestly-named
    status. Corpus-wide side effect of the same generic fix (other books, not this cycle's own
    scope): 1,323 further units moved out of `race_trait_race_not_modelled` (708 to M, 199 to D,
    416 to DONE via the SAME `text_only && has_real_description` promotion every other Epic-2
    kind already uses — legitimate, not an invented leniency for this mechanism).
  - **Reachability:** N/A (no player-facing reachability claim changes this cycle; every moved
    unit is still `held`/`ingested`, not promoted past what a shape engine or the compute path
    would need to independently verify).
  - **Instrument-correction:** N/A (no measurement method was found wrong this cycle; the
    classifier gap was a real, unbuilt table consultation, not a broken instrument).
- **Notes:**
  - `box_ledger.py --check` (SD-33's inherited, read-only partition) exits 1 both before and
    after this cycle — pre-existing, inherited from every prior AT-34-E3-001 mechanism cycle
    that already moved units without updating the frozen `THE-BOX.md` snapshot (verified: `git
    show a510cbfcf3:docs/work-inventory.json` also exits 1 against the same check). This cycle's
    own effect on it is a real IMPROVEMENT, not a regression: `uncovered` drops from 21,221 to
    20,097. `decisions.md §2` names `THE-BOX.md` as SD-33's own artifact, not SD-34's to
    maintain; `box_ledger.py --check`'s structural invariants (`overlap=0`, `population=49438`)
    still hold both before and after.
  - The 4-unit "Favored Enemy ~ Humanoid" shortfall (source_book/book mismatch) was caught by
    actually running the real regeneration and counting the artifact, not by trusting the unit
    tests alone — the unit tests as first written all passed while the real corpus still had 4
    unmoved units, because none of the hand-built test fixtures happened to set `source_book !=
    book`. Fixed in the same cycle, with a new test pinning the real corpus shape.
- **Next-cycle plan:** `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (330
  units) or `class_feature_option_pool_record_with_magnitude_not_held_by_engine` (333 units) —
  whichever the dispatch orders next, per `decisions.md §14`'s remaining six mechanisms. Bucket B
  for `core_rulebook` is not yet zero; this receipt closes only the
  `race_trait_race_not_modelled` mechanism.
