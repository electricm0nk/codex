# Cycle t9-monster-ability-desc-concat-round9 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-desc-concat-round9`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — new
    `_concat_desc_variants(descs)`: the generalised SIXTH `parse_desc`
    branch round 6/7/8's own docstring named as the fix this population
    needs. Every earlier branch (`DisplayFullAbility` toggle, superset,
    variable-bearing, plain-space continuation) resolves a row where the
    corpus states ONE global criterion that picks a single winning token.
    The remaining 56 units' gate (`PRERULE`/`PREVAREQ`/`PREVARGT`/
    `PRESIZE*`/`PREHD`/`PRERACE`/`PRETEMPLATE`/`PREABILITY`) tests a
    property of the OWNING MONSTER INSTANCE (its CR, HD, size, template,
    race subtype, or a feat it has) — a fact this per-ability-KEY table row,
    shared verbatim across every monster that owns it, cannot resolve once
    and for all. So every token's text ships, concatenated in the corpus's
    own order, verbatim — never a guessed pick, never dropped mechanics.
    Each token's own `%N` placeholders are renumbered so one ordered,
    global `description_variables` list can back them (token 2's own `%1`
    becomes `%(N+1)` where `N` is the count of variables already collected
    from earlier tokens) — pure bookkeeping, nothing invented. The
    `else: raise UnmodelledDesc(...)` branch in `parse_desc` is replaced
    with `return _concat_desc_variants(descs)`.
  - `scripts/tests/test_transcribe_monster_tables.py` — new
    `ConcatenatedDescClosesTheFinalRefusalGroupRound9` (5 tests, each a
    REAL coordinate from the live 56-unit population: a conditionally-
    appended clause, `%N` renumbering across token boundaries with a
    5-token row, `&nl;`-marker continuation with no gate at all, two
    ungated near-duplicate texts, and mutually-exclusive threshold
    variants). `UnscreenableRowIsDroppedNotFatal`'s fixture and its three
    test methods updated: the synthetic row it used to prove "gets dropped,
    doesn't crash the book" now demonstrates "ships concatenated, doesn't
    crash the book" — the underlying resilience property is unchanged,
    only what happens to the formerly-refused row changed.
  - `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_3,
    bestiary_4,bestiary_5,horror_adventures,inner_sea_bestiary,
    pathfinder_unchained}/monster_data.rs` — regenerated via
    `transcribe_monster_tables.py <book>`; each book's ability table gains
    exactly its closed rows plus header-comment updates. No pre-existing
    record's fields changed (verified: `git diff --stat` shows only new
    `MonsterAbilityRecord` blocks and header-comment lines, zero deletions
    in any of the 8 files).
  - `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_3,
    bestiary_4,horror_adventures,inner_sea_bestiary}/mod.rs` — owned/
    owner-less/total count pins re-derived from live failing runs (never
    guessed), each with a dated comment naming the delta and its cause.
    Owner-less digests re-derived the same way (all 6 books' digests moved;
    old/new values recorded in each test's own comment).
  - `src/rules_core/rules_tables/monster_chassis.rs` — the corpus-wide
    no-reclassification pin
    (`widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`)
    re-derived from a live failing run: 3749 -> 3806 records, digest
    `0xfc51_2110_6900_558e` -> `0x8b2c_a909_f967_5cd5`.
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — the corpus-wide
    owner-less-records pin re-derived: 1076 -> 1126 (+50, across 7 of the 8
    touched books — `bestiary_2`'s 2 closed units are both owned and do not
    move this count).
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` — one line,
    `beastiary1_monster_count_matches_the_documented_real_total`'s pin
    (711 -> 733).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `bestiary`/`bestiary_2`/
    `bestiary_3`'s own book-level `_reaches_the_catalog_for_every_linked_
    record`/`_monsters_reach_the_monster_catalog_record_by_record` tests'
    owned/owner-less/total pins re-derived; `bestiary`/`bestiary_3`/
    `bestiary_4`/`bestiary_5`/`horror_adventures`/`inner_sea_bestiary`/
    `pathfinder_unchained`'s `UNREACHED_RECORD_FINDINGS` entries gain their
    new owner-less keys (inserted alphabetically, matching the existing
    convention); the matching `OPEN_FINDINGS` "Gap:" prose entries for
    `bestiary_5`/`pathfinder_unchained` updated to the new totals.
  - `data/corpus/{beastiary,bestiary_2,bestiary_3,bestiary_4,bestiary_5,
    horror_adventures,inner_sea_bestiary,pathfinder_unchained}/
    monster_ability/*.json` (57 new files, additive only — 56 the real
    `no_record` population, plus `bestiary`'s `Lycanthrope ~ Change Shape`,
    a bonus unit already counted `text-complete` by inventory evidence
    alone, same shape as round 8's `Bunyip ~ Blood Rage`) and each of the 8
    books' `LICENSE.json` (screening-note append, same generator-owned
    mechanism every prior round used). `pathfinder_unchained/equipment/
    0_abp_enhancement_to_{ammunition,armor,shield,weapon}.json` (4 new
    files) also landed — `gen_book_cache -- pathfinder_unchained` is a
    whole-book generator that rewrites this book's equipment/feats every
    run (idempotent full regen, "42/42 written" not "N new"), and these 4
    equipment records had simply never been generated to disk before;
    initially deleted as suspected out-of-territory drift, then restored
    after confirming (by re-running the generator) they are normal,
    sanctioned, book-scoped output — deleting them left `LICENSE.json`'s
    `records_processed` count inconsistent with reality.
    `enrich_monster_ability_raw_tokens` was run once (as its doc comment
    requires) and enriched 1,863 records corpus-wide; `git status
    --porcelain` afterward showed modifications outside this cycle's 8
    books (`advanced_race_guide`, `bestiary_6`, `inner_sea_gods`,
    `inner_sea_world_guide`, `mythic_adventures`, `occult_adventures`,
    `ultimate_intrigue`, `ultimate_magic`, `ultimate_psionics`,
    `ultimate_wilderness`) — reverted via `git checkout --` on exactly
    those paths (never `git stash`), leaving only this cycle's own 8 books'
    files touched.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to
  `scripts/transcribe_monster_tables.py` + its test file, this cycle's own
  diff).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff; one
  self-healed inline hit — a comment's use of the English word
  "placeholder" describing `%N` markers, reworded to "marker").
- **PI grep result:** `pi_scrub.normalized_term_hits` run over every added
  line of this cycle's own code diff and every byte of all 57 new corpus
  records: zero hits both times.
- **Acceptance criterion:** `decisions.md §27b`/`§17`/`§17a` — the last
  56-unit `monster_ability` `no_record` group (the multi-`DESC:`
  `PRERULE`/`PREVAREQ` parse-refusal group three prior rounds left
  unchanged) closes via a generalised sixth `parse_desc` branch, not 56
  per-record special cases; `no_record` reaches **ZERO**, verified live.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh --dest`).
- **Status:** complete. `monster_ability` `no_record`: **56 → 0**.
  Bundle-wide `no_record`: **56 → 0** (this was the entire remaining
  bundle-wide population per the brief's own re-derivation).
- **Notes:**
  - **How many distinct shapes the 56 actually needed, re-derived live
    (not the round-8 receipt's estimate).** Direct inspection of every
    refused row's raw `DESC:`/`TYPE:`/`DEFINE:`/`BONUS:VAR` fields (read
    live from the pinned oracle for all 56 coordinates) found the group is
    not one shape needing 56 special cases, nor a small fixed enum of
    named shapes the way the round-8 `TYPE:`-facet-gap group was — it is a
    CONTINUUM of gate types (`PREVARGTEQ` against a DEFINE-defaulted
    variable, `PRERACE`, `PRESIZEGT`/`PRESIZELTEQ`, `PREHD` MIN/MAX bands,
    `PREABILITY` toggles, plain `&nl;`-marker continuation with no gate at
    all, and two rows with no gate and no shared criterion at all) that ALL
    share the same underlying structural property: none of them can be
    resolved to a single winning branch by this per-ability-KEY row alone,
    because the gate's condition is a fact about the OWNING MONSTER
    INSTANCE, not the shared ability definition. **One generalised
    mechanism (verbatim concatenation, `%N` renumbering) closes the whole
    group in one shot** rather than 56, or even a dozen, per-shape special
    cases — the "generic pass, not per-object work" ruling (`decisions.md
    §17`) applied to its logical conclusion for this population.
  - **The mechanism is total for this exception class, not partial.**
    Every one of the 56 real `no_record` coordinates plus the bonus unit
    was independently constructed as a literal Python `row` fixture from
    the coordinate's own raw fields and run through `parse_desc` before any
    corpus regen — all 57 resolved without raising, confirming the branch
    covers the whole population before spending a single `gen_book_cache`
    run on it.
  - **A near-miss, caught and corrected before commit (distinct from round
    8's `enrich_monster_ability_raw_tokens` incident, though the SAME
    guard-rail habit caught it).** `gen_book_cache -- pathfinder_unchained`
    additionally wrote 4 new `equipment` records this book had apparently
    never had generated to disk. These were first treated as out-of-
    territory drift and deleted via `rm`, which left the freshly-written
    `LICENSE.json`'s `records_processed` count internally inconsistent
    with the corpus it was describing. Re-running the SAME generator
    restored them cleanly and confirmed (via the generator's own printed
    report, "equipment written: 42 / 42") that this is normal, idempotent,
    book-scoped full-regen behavior for this book's generator function,
    not a scope violation — the correct action was to KEEP them, not
    delete them.
  - **`§16` unchanged.** No unit moved out of a shape; every one of the 56
    (57) went from `no_record`/`not_ingested` straight to a real `F0`
    (measured, real corpus record, genuinely zero DEFINE/BONUS tokens),
    `F1`, or higher family via the shape ledger's own normal join — none
    carries the `§27` provisional marker (`row17_census.py --check` still
    reports the provisional-default count unchanged at 22/23, confirming
    this cycle's closure mechanism is a REAL classification, never a
    placeholder).
- **Discovery forwards:** none new.
- **Next-cycle plan:** `monster_ability` `no_record` is now **0**; Gate 3's
  closure-invariant condition (`§20`) is met for this kind. Row 17's own
  categorization pass (kanban row 17, `epic-7-shape-categorization-100`,
  sequenced after bundle-wide `no_record` reaches zero — now true) is the
  next dispatch: its honest population (`fallthrough` + `§27` provisional
  default) stands at 22, unchanged by this cycle.

---

## 1. Re-derived the population before touching anything (`§17a`)

Never trusted the brief's own 56/per-book figures without re-deriving.
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
confirmed bundle-wide `no_record` = **56**, all `monster_ability`, matching
the brief's per-book breakdown (`bestiary` 21, `bestiary_3` 10,
`horror_adventures` 9, `bestiary_4` 7, `inner_sea_bestiary` 3,
`pathfinder_unchained` 3, `bestiary_2` 2, `bestiary_5` 1) exactly against a
JSON dump of the ledger's own `no_record`-status rows (`docs/work-
inventory.json`'s `id` field for each). Cross-checked against `transcribe()`'s
own live stderr for each of the 8 books, which additionally surfaces
`bestiary`'s 22nd refused row (`Lycanthrope ~ Change Shape`) — NOT in the
ledger's `no_record` set because `docs/work-inventory.json` already counts it
`text-complete` on inventory evidence alone, the identical "bonus unit"
pattern round 8's receipt named for `Bunyip ~ Blood Rage`.

## 2. Every refused row's raw fields, read live (not assumed)

For each of the 56 coordinates, resolved the corpus file via
`resolve_book_file`/`read_row` against the pinned oracle and printed its
full `TYPE:`/`DEFINE:`/`BONUS:VAR:`/every `DESC:` token verbatim. This is
what grounded the "not one shape, a continuum sharing one structural
property" finding in §"Notes" above, rather than guessing from the round-6
docstring's own five-year-old five-example sketch.

## 3. RED → GREEN (`AGENTS.md` non-negotiable rule 1)

Confirmed RED by temporarily restoring the pre-cycle `transcribe_monster_
tables.py` (`git show HEAD:<path>`, never `git stash`) and re-running the
new test class plus the updated `UnscreenableRowIsDroppedNotFatal` methods:
all 8 failed for the intended reason (`UnmodelledDesc` raised, or the
formerly-dropped row absent from emitted content). Restored the fixed file
and re-ran: `python3 -m unittest scripts.tests.test_transcribe_monster_
tables` — 39 tests, 38 pass, 1 pre-existing failure
(`InternalBundleAbilityHopIsResolved::test_an_ability_no_bundle_names_stays_
an_orphan_and_is_not_shipped`, confirmed present and unrelated to this diff
— round 5/6/7/8's own receipts already named this test as pre-existing and
out of this lane's territory).

## 4. Corpus regeneration — additive only, verified before AND after

`git status --porcelain` before every commit. `transcribe_monster_tables.py
<book>` for all 8 affected books (zero deletions each, only new
`MonsterAbilityRecord` blocks and header comments — verified via `git diff
--stat`). `cargo run --bin gen_book_cache -- <book>` for the same 8
(`beastiary` via its on-disk-dir alias): 57 new JSON files total, matching
the printed `N new monster abilities` count exactly per book (22 + 2 + 10 +
7 + 1 + 9 + 3 + 3 = 57), zero deletions.

**Scoped near-miss caught and reverted:** `enrich_monster_ability_raw_
tokens` (run once, as its own doc comment requires) enriched 1,863 records
corpus-wide. `git status --porcelain` immediately after showed modified
files across 10 books this cycle never intended to touch. Reverted via
`git checkout --` on exactly those book directories' `monster_ability/`
paths (never `git stash`). Re-verified after revert: `git status
--porcelain` shows modifications confined to this cycle's 8 target books
plus the 57 new (`??`) files.

No `--allow-stamp-loss` used anywhere in this cycle.

## 5. Pin re-derivation, book by book (`§17a` — every number re-derived
live, none guessed)

Each of `bestiary`/`bestiary_2`/`bestiary_3`/`bestiary_4`/
`horror_adventures`/`inner_sea_bestiary`'s own `mod.rs` owned/owner-less/
total count pins, and their owner-less digests, were re-derived by running
the failing test, reading its printed `left` (actual) value, setting the
pin, then (for digests) adding a temporary `eprintln!` probe, capturing the
digest from `--nocapture` output, setting it, and removing the probe.
`reach_gate.rs`'s matching `UNREACHED_RECORD_FINDINGS` key-list entries were
computed via the SAME `slug()` algorithm `v06_work_inventory.rs::slug`
defines (lowercase alphanumeric, non-alnum runs collapse to one `_`,
trailing `_` trimmed), reproduced in a scratch Python helper rather than
guessed, and inserted at their correct alphabetically-sorted position.
`monster_chassis.rs`'s corpus-wide pin and `monster_catalog.rs`'s corpus-
wide owner-less pin were re-derived the same live-failing-run way.
`corpus_ingest_diagnostic.rs`'s single `beastiary1` pin, and `reach_gate.rs`'s
two book-level `_reaches_the_catalog_for_every_linked_record` tests
(`bestiary_2`, `bestiary_3`) plus its `bestiary_1_monsters_reach_the_
monster_catalog_record_by_record` test, were fixed the same way.

## 6. What was actually closed this cycle: 56 units (+1 bonus), one
generalised mechanism

**Closure this cycle: 56 units, real ingestion via a genuinely-derived
shape (not a placeholder — every closed unit reaches a real `F0`/`F1`+
family through the normal shape-ledger join, none carries `§27`'s
provisional marker), 0 reclassified, reachability: 12 newly OWNED (join a
monster's `ability_keys`, reachable through `list_monster_catalog`), 44
newly owner-less (shape-measured, reachability explicitly NOT claimed,
pinned by exact key in `reach_gate.rs::UNREACHED_RECORD_FINDINGS`) —
instrument correction 0.** Plus 1 bonus unit (`Lycanthrope ~ Change Shape`)
outside the `no_record` population, same mechanism.

`monster_ability` `no_record`: **56 → 0** (re-derived:
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` →
`no_record 0`). Bundle-wide `no_record`: **56 → 0**.

`decisions.md §27`'s provisional-default count (the fourth, separately-
reported number `§16`/`§27a`/`§27b` require): **unchanged at 22** (corpus-
wide total incl. done units: 23) — this cycle's closure mechanism produces
genuinely-derived shapes, never the provisional default, so this count does
not move.

## 7. Tests

```
python3 -m unittest scripts.tests.test_transcribe_monster_tables
  39 tests, 38 passed, 1 failed (pre-existing, confirmed unrelated)
cargo build --locked --lib                                          clean, 10 warnings (pre-existing shape)
cargo test --locked --lib monster_chassis::                          8 passed, 0 failed (pin re-derived: 3806 / 0x8b2ca909f9675cd5)
cargo test --locked --lib rules_tables::                             620 passed, 0 failed, 3 ignored
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (pin re-derived: owner_less_records_held 1076 -> 1126)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  14 passed, 1 failed (pin re-derived: beastiary1 monster_abilities 711 -> 733; the 1 remaining
  failure is advanced_race_guide, pre-existing, confirmed unrelated to this diff -- round 6/7/8's
  own receipts already named this test as pre-existing and out of this lane's territory)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed — IDENTICAL split to round 4/5/6/7/8's own recorded baseline (re-verified:
  none of the 8 failing tests' own printed detail names this cycle's 8 books' `monster_ability`
  content -- every one is companions/classes/feats/spells/class_features in unrelated books
  (`advanced_race_guide`, `bestiary_4`/`bestiary_5` companions, `pathfinder_unchained` class_features,
  dozens of unrelated books' unnamed corpus directories), unrelated to this cycle's scope or diff).
  `bestiary_1`/`bestiary_3`'s own book-level tests (2 pins moved by this cycle's own diff) fixed
  inline before this run.
```

## 8. What remains (three separate figures per `decisions.md §16`)

**`monster_ability` `no_record`: 0. Bundle-wide `no_record`: 0.** Gate 3's
closure-invariant condition (`§20`) is met.

1. **Row 17's real categorization pass** (kanban row 17,
   `epic-7-shape-categorization-100`) now unblocked (sequenced after
   bundle-wide `no_record` reaches zero — now true) — its honest population
   stands at 22 (`§27` provisional-default units), unchanged by this cycle.
2. **`occult_adventures`, `advanced_race_guide` companions — 0 units,
   already closed** (unaffected by this cycle; re-confirmed live in the
   population re-derivation above).

## 9. Next-cycle plan

Row 17's own categorization pass is now the only remaining `monster_ability`
work: re-visit each of the 22 `§27`-provisional-default units (13
`SpecialQuality`-defaulted rows from round 8's `TYPE:`-facet-gap group,
etc.) and give each its real, measured shape.
