# Cycle t9-monster-ability-owner-less-ingest-round5 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-owner-less-ingest-round5`)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded
  in `progress.md`'s appended receipt)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — `BOOKS` dict gains 1 new entry
    (`mythic_adventures`); the mechanism itself is **unchanged**.
  - `src/bin/gen_book_cache.rs` — 1 new `MonsterBookSpec` row for
    `mythic_adventures`. Unlike round 4's `pathfinder_unchained`/
    `advanced_race_guide`, this book has **no** hand-rolled generator
    function, so it is reached entirely through `main`'s existing generic
    `monster_book_spec` fallback arm — zero new generator code, only the
    registry row.
  - `src/rules_core/rules_tables/monster_chassis.rs` — 1 new `MonsterBook`
    row in `MONSTER_BOOKS`;
    `widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`'s
    pin repinned 3683 → 3704 (count) and digest `0x2fa5c4578c0267bb` →
    `0xd732c20ec4c2a946`, re-derived from a live test run, never guessed.
  - `src/rules_core/rules_tables/mythic_adventures/mod.rs` — gains
    `mod monster_data;`, a `monster_chassis` re-export
    (`MonsterAbilityDelivery`/`MonsterAbilityFacet`/`MonsterAbilityRecord`/
    `MonsterStatBlock`), and `monsters_static()`/`monster_abilities_static()`
    wrappers. **The module directory itself already existed** — a sibling
    `spell` lane (commit `3f8ddca7fd`) created it for `spell_list` before
    this cycle ran; this closes the round-4 receipt's own "needs a new
    module scaffolded from scratch" note, which the dispatch brief carried
    forward but which was stale by the time this cycle started.
  - `src/rules_core/rules_tables/mythic_adventures/monster_data.rs` (new,
    generated via `scripts/transcribe_monster_tables.py mythic_adventures`).
  - `data/corpus/mythic_adventures/monster_ability/*.json` (21 new files, via
    `gen_book_cache mythic_adventures`) and the book's `LICENSE.json`
    (screening-note append).
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — 1 new wire code
    (`BOOK_MYTHIC`, reusing the code `equipment_catalog`/`reach_gate`
    already serve this book's `equipment`/`spell` families under, "MYTHIC")
    wired into `book_display_name`/`book_wire_code`; the corpus-wide
    owner-less-count pin moves 1027 → 1048 (+21).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — 1 new
    `("mythic_adventures", "monster_abilities") => Some(chassis_monster_abilities_reach(...))`
    reach-claim arm, 1 new `UNREACHED_RECORD_FINDINGS` entry (21 exact keys),
    1 new `OPEN_FINDINGS` entry.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff against this
  cycle's own start point `a32e235321` (== PIN), the substantive files above
  — 0 hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits).
- **Acceptance criterion:** `decisions.md §20` — drive `monster_ability`
  `no_record` toward zero.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (partial application of the overall `no_record==0`
  goal — card 11 stays `in-progress`; see "What remains" below).
- **Notes:** see full body below.
- **Discovery forwards:** two pre-existing, unrelated test failures observed
  this cycle in `corpus_ingest_diagnostic.rs` — named in §5 below, neither
  caused by this cycle's diff (that file is untouched by this cycle) and
  neither in this cycle's `monster_ability` scope.
- **Next-cycle plan:** see §6.

---

## 1. `mythic_adventures` — the last of the original 8 zero-monster books, taken first

Round 4's own receipt named this book as needing "a `rules_tables/mythic_adventures/`
directory scaffolded... not just a bare registry row." That was true when
round 4 wrote it. It is **stale now**: a sibling `spell` lane (SD-32 `spell`
no_record round, commit `3f8ddca7fd`) created
`src/rules_core/rules_tables/mythic_adventures/` for its own `spell_list`
module in the interim, and `rules_tables/mod.rs` already carries
`pub mod mythic_adventures;`. This cycle re-derived the state fresh rather
than trusting the dispatch brief's carried-forward note
(`decisions.md §17a`) and found the scaffold already present — so this cycle
only extends the existing module, the identical shape round 4 applied to
`pathfinder_unchained`/`advanced_race_guide`.

Verified before touching anything:

```
python3 scripts/classify_monster_ability_rows.py mythic_adventures
  -> mythic_adventures  0  21  0  0  21  0  0
  (0 monster rows, 21 ability rows, 0 row-named, 0 prefix, 21 orphan, 0 PI, 0 .COPY=)
```

`ma_abilities_race.lst` (21 rows) loads UNGATED at the book's own `.pcc` root
(`grep -n 'ABILITY:ma_abilities_race.lst' _mythic_adventures.pcc` → line 40,
no `PRECAMPAIGN`). Zero Product Identity rows
(`grep -c 'NAMEISPI:YES\|DESCISPI:YES' ma_abilities_race.lst` → 0).

## 2. Real per-record shape found: none refused, all 21 shipped

Unlike round 4's `pathfinder_unchained` (3 of 72 orphan candidates refused
as a multi-`DESC:` shape), `mythic_adventures`'s transcription printed and
shipped all 21 candidates with zero refusals
(`python3 scripts/transcribe_monster_tables.py mythic_adventures` stderr:
"21 orphan ability row(s) transcribed WITHOUT an owner ... reachability NOT
claimed"). No parse-refusal shape was forced through — there was none to
force.

## 3. `mythic_adventures` has no hand-rolled `gen_book_cache.rs` generator

Round 4's two books (`pathfinder_unchained`/`advanced_race_guide`) each
already had a dedicated generator function for their other families
(feats/equipment), so round 4 extended those functions to also call
`gen_monster_book`. `mythic_adventures` carries no such function — its
`spell` family is reached through `src/bin/ingest_spells.rs`'s config-driven
path instead — so this cycle only added a `MonsterBookSpec` row;
`main`'s existing generic `monster_book_spec(other) => gen_monster_book(spec)`
fallback arm reaches it with **zero new generator code**. This also means
round 4's §0 near-miss (a bundled generator function silently deleting
unrelated pre-existing corpus files on any re-run) **cannot recur here** —
`gen_monster_book` writes only the `monster_ability`/`monster` families it
owns, nothing else.

`git status --porcelain` before commit: 8 entries, all either modified files
this receipt names or new (`??`) files under the 1 new `monster_ability/`
directory and 1 new `monster_data.rs` file — **zero deletions**, checked
explicitly (`git status --porcelain | grep '^ D\|^D '` → empty).

## 4. Re-derived `no_record` — real closure, not a relabel (`decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after_mythic.json
```
```
no_record (bundle total, pre-cycle):  326
no_record (bundle total, post-cycle): 305  (-21)
monster_ability (pre-cycle):          121
monster_ability (post-cycle):         100  (-21)
```
Per-kind (pre-cycle, unchanged, this cycle touched no other kind):
`equipment` 113, `spell` 57, `equipment_modifier` 33, `companion` 2 — all
byte-identical before/after, confirmed by direct re-run above.

Per-book `monster_ability` `no_record`, post-cycle: `bestiary` 23,
`bestiary_3` 21, `inner_sea_bestiary` 12, `bestiary_2` 10, `horror_adventures`
9, `bestiary_4` 7, `inner_sea_gods` 6, `occult_adventures` 5,
`inner_sea_world_guide` 3, `pathfinder_unchained` 3, `bestiary_5` 1.
`mythic_adventures` no longer appears — 0 remaining.

**No unit was reclassified out of `monster_ability` into another kind** —
this is a genuine ingestion closure.

## 5. Two pre-existing, unrelated test failures observed (not caused by this cycle)

`cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins
corpus_ingest_diagnostic::` shows 13 passed, 2 failed:

1. `the_two_ingested_books_totals_reconcile_with_their_license_artifacts` —
   fails on `advanced_race_guide` alone (`left: 1579, right: 2157`, a stale
   `rules_tables`-vs-`LICENSE.json` reconciliation for a book/kind this
   cycle never touched). `corpus_ingest_diagnostic.rs` and
   `data/corpus/advanced_race_guide/LICENSE.json` are both untouched by this
   cycle's diff (`git status --porcelain` confirms neither file appears in
   this cycle's changes), so this failure predates this cycle and is not
   this cycle's to fix — it is outside `monster_ability` scope and outside
   this cycle's granted files.
2. `every_book_landed_in_rules_tables_is_reported` — names
   `["inner_sea_races", "mythic_adventures"]` as landed-but-unreported.
   `mythic_adventures`'s absence here predates this cycle too: the sibling
   `spell` lane's commit `3f8ddca7fd` created the module without adding a
   `book_status(..)` row to this diagnostic, and this cycle's own diff never
   touches `corpus_ingest_diagnostic.rs`.

Both are named here per `decisions.md §22`'s divergence-visibility rule
(discovered, not silently worked around) and left for whichever lane owns
`corpus_ingest_diagnostic.rs`/the `advanced_race_guide` reconciliation — not
this cycle's `monster_ability` territory.

`monster_catalog::` — 26 passed, 0 failed (same as round 4's own baseline,
after the 1 new wire code and the 1027 → 1048 pin).
`reach_gate::` — 23 passed, 8 failed, **the identical 23/8 split round 4
recorded as its own baseline**; none of the 8 failures names
`mythic_adventures` or `monster_ability` (all 8 are pre-existing gaps in
OTHER families across OTHER books — `advanced_race_guide/companions`,
`beastiary1/*`, etc.), confirmed by reading each failure's own printed
detail.

## 6. What remains (explicit, three separate figures per `decisions.md §16`)

**Closure this cycle: 21 units, real ingestion, 0 reclassified.**

`monster_ability` `no_record` remaining: **100**, in two shapes:

1. **Real per-record/per-facet engineering — 95 units across 10
   already-registered books, unchanged from round 4** (`bestiary` 23,
   `bestiary_3` 21, `inner_sea_bestiary` 12, `bestiary_2` 10,
   `horror_adventures` 9, `bestiary_4` 7, `inner_sea_gods` 6,
   `inner_sea_world_guide` 3, `pathfinder_unchained` 3, `bestiary_5` 1):
   multi-`DESC:` parse refusals, `TYPE:`-facet-vocabulary gaps, and
   PI-declared rows correctly excluded per `decisions.md §15`. Re-confirmed
   present by this cycle's own ledger re-run — identical population round 4
   named.
2. **`occult_adventures` (5 units) — correctly out of scope, not a gap.**
   Unchanged from round 3/4: its monster row loads under a NEGATED
   `PRECAMPAIGN` gate this repo's campaign set fails
   (`grep -n 'PRECAMPAIGN' _occult_adventures.pcc` shows the gate; the repo's
   book set never satisfies it).

`monster` kind (28 units, sibling lane's scope) — untouched.

**No further apply-the-mechanism-to-a-zero-monster-book cycles remain.**
All 8 of the original zero-monster books this dict tracked
(`ultimate_wilderness`, `ultimate_intrigue`, `ultimate_magic`, `bestiary_6`,
`bestiary_5`, `pathfinder_unchained`, `advanced_race_guide`,
`mythic_adventures`) are now registered. The residual 100 units are real
per-record engineering against already-registered books plus one confirmed
out-of-scope population, not a missing config row.

## 7. Tests

```
cargo build --locked --lib                                                clean, 9 warnings (matching prior rounds' identical unused-import shape, +1 for the new mythic_adventures monster_data.rs)
cargo test --locked --lib monster_chassis::                               8 passed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean, finished in 2m51s
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (after the new wire code and the 1027 -> 1048 pin)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed (all 8 confirmed pre-existing, none naming mythic_adventures/monster_ability;
  identical 23/8 split round 4 recorded as its own baseline)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  13 passed, 2 failed (both pre-existing, named and explained in §5; neither
  file either failure depends on was touched by this cycle)
```
RED→GREEN: `monster_chassis::tests::widening_the_facet_vocabulary_does_not_
reclassify_any_existing_record` failed with the real count mismatch
(`left: 3704, right: 3683`) on first run after registering the book but
before repinning the count; repinning the count (3704) produced a real
digest mismatch (`left: 15506669835676461382` / `0xd732c20ec4c2a946`,
`right: 3433366171230037947` / `0x2fa5c4578c0267bb`); repinning both from
the live failure's own printed values (never guessed) turned it green.

## 8. Pinned base

`PIN=a32e23532181d205dde22327c89203bd73052c5a` — this worktree's HEAD
matched on first check via `git merge-base --is-ancestor`. (A prior isolated
worktree at this same task had drifted onto a later `tranche/11`-merge
commit not descended from PIN; `git reset --hard "$PIN"` corrected it before
any other work began.)
